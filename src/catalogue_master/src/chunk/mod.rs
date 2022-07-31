mod idgenerator;
mod operator;
mod storage;

mod service;

pub use idgenerator::IdGeneratorMode;
use idgenerator::{IdGenerator, IdGeneratorFactory};
pub(crate) use operator::ChunkOperator;
use storage::Storage;
pub(crate) use storage::StorageFactory;
pub use storage::StorageMode;

use service::ChunkHandlerServiceImpl;

use crate::{Chunk, Error, Result};

use futures::Stream;
use tokio::sync::{mpsc, RwLock};
use tokio::time::{self, Duration};
use tonic::transport::Server;

use common_tracing::tracing::{error, info, warn};

use std::collections::HashMap;
use std::pin::Pin;
use std::sync::Arc;
use std::{error::Error as StdError, io::ErrorKind};

// use tiny_gfs_utils::init_simple_file_log;

use crate::proto::chunk_handler::{chunk_handler_service_server::*, *};

type ArcStorage = Arc<RwLock<Box<dyn Storage + Sync + Send>>>;

pub(crate) struct HeartbeatBuffer {
    chunk_ids: HashMap<String, u64>,
}

impl HeartbeatBuffer {
    fn update(&mut self, chunk_id: &str) {
        use std::time::*;
        let t = SystemTime::now()
            .duration_since(SystemTime::UNIX_EPOCH)
            .expect("successful get timestamp")
            .as_secs();
        match self.chunk_ids.get_mut(chunk_id) {
            Some(v) => {
                *v = t;
            }
            None => {
                self.chunk_ids.insert(chunk_id.into(), t);
            }
        }
    }

    fn fetch_all(&mut self) -> HashMap<String, u64> {
        let ids = self.chunk_ids.clone();
        self.chunk_ids.clear();
        ids
    }

    fn new() -> Self {
        Self {
            chunk_ids: HashMap::new(),
        }
    }
}

pub(crate) struct ChunkHandler {
    storage: Arc<RwLock<Box<dyn Storage + Sync + Send>>>,
}

impl ChunkHandler {
    async fn handle_heartbeat(
        hb: Arc<RwLock<HeartbeatBuffer>>,
        storage: Arc<RwLock<Box<dyn Storage + Sync + Send>>>,
    ) {
        info!("handle heartbeat");

        let chunks_heartbeat_data = hb.write().await.fetch_all();

        if let Err(err) = storage
            .write()
            .await
            .update_state_multi(chunks_heartbeat_data)
            .await
        {
            error!("update state multi to storage failed = {}", err);
        }
    }

    fn start_async_tasks(&mut self, heartbeat_buffer: Arc<RwLock<HeartbeatBuffer>>) -> Result<()> {
        let hb = heartbeat_buffer;
        let storage = self.storage.clone();

        std::thread::spawn(move || {
            use tokio::runtime::Runtime;
            let rt = Runtime::new().unwrap();

            let hb = hb.clone();
            let storage = storage.clone();

            rt.block_on(async {
                let mut sleep = time::sleep(Duration::from_millis(1000));
                tokio::pin!(sleep);

                loop {
                    let hb = hb.clone();
                    let storage = storage.clone();

                    tokio::select! {
                        _ = &mut sleep => {
                            ChunkHandler::handle_heartbeat(hb, storage).await;
                            sleep.as_mut().reset(time::Instant::now() + Duration::from_millis(1000));
                        }
                        // _ = some_async_work() => {
                        //     println!("operation completed");
                        // }
                    }
                }
            })
        });

        Ok(())
    }

    pub(crate) fn start(&mut self, id_generator_mod: IdGeneratorMode) -> Result<()> {
        use tokio::runtime::Runtime;
        let rt = Runtime::new()?;

        let storage = self.storage.clone();

        let addr = "[::1]:10000".parse().unwrap();
        let heartbeat_buffer = Arc::new(RwLock::new(HeartbeatBuffer::new()));
        let mut s =
            ChunkHandlerServiceImpl::new(heartbeat_buffer.clone(), storage, id_generator_mod)?;

        if let Err(err) = self.start_async_tasks(heartbeat_buffer) {
            error!("chunk handler service start failed");
            return Err(err);
        }

        let (_guards, _subscriber) = common_tracing::init_tracing_log(
            "chunk_handler",
            "logs/chunk_handler",
            log::LevelFilter::Info.as_str(),
        );

        info!("chunk handler start success");

        rt.block_on(async {
            Server::builder()
                .add_service(ChunkHandlerServiceServer::new(s))
                .serve(addr)
                .await
                .unwrap();
        });

        Ok(())
    }

    pub(crate) fn new(storage: ArcStorage) -> Result<Self> {
        Ok(Self { storage: storage })
    }
}
