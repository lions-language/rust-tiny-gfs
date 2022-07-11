mod storage;

pub use id_generator::IdGeneratorMode;
pub use storage::StorageMode;

use crate::{Error, Result};
use storage::{Storage, StorageFactory};

use futures::Stream;
use tokio::sync::{mpsc, RwLock};
use tokio::time::{self, Duration, Interval};
use tokio_stream::{wrappers::ReceiverStream, StreamExt};
use tonic::{transport::Server, Request, Response, Status, Streaming};

use log::{error, info, warn};

use std::collections::HashMap;
use std::pin::Pin;
use std::sync::Arc;
use std::{error::Error as StdError, io::ErrorKind};

// pub mod chunk_handler {
//     tonic::include_proto!("/proto/chunk_handler_service");
// }

use crate::proto::chunk_handler::{chunk_handler_service_server::*, *};

fn match_for_io_error(err_status: &Status) -> Option<&std::io::Error> {
    let mut err: &(dyn StdError + 'static) = err_status;

    loop {
        if let Some(io_err) = err.downcast_ref::<std::io::Error>() {
            return Some(io_err);
        }

        // h2::Error do not expose std::io::Error with `source()`
        // https://github.com/hyperium/h2/pull/462
        if let Some(h2_err) = err.downcast_ref::<h2::Error>() {
            if let Some(io_err) = h2_err.get_io() {
                return Some(io_err);
            }
        }

        err = match err.source() {
            Some(err) => err,
            None => return None,
        };
    }
}

struct HeartbeatBuffer {
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

type HeartbeatResponseStream =
    Pin<Box<dyn Stream<Item = std::result::Result<HeartbeatResponse, Status>> + Send>>;

pub struct ChunkHandlerServiceImpl {
    heartbeat_buffer: Arc<RwLock<HeartbeatBuffer>>,
    storage: Arc<RwLock<Box<dyn Storage + Sync + Send>>>,
    id_generator: Arc<RwLock<Box<dyn IdGenerator + Sync + Send>>>,
}

impl ChunkHandlerServiceImpl {
    fn new(
        heartbeat_buffer: Arc<RwLock<HeartbeatBuffer>>,
        storage: Arc<RwLock<Box<dyn Storage + Sync + Send>>>,
        id_generator_mod: IdGeneratorMode,
    ) -> Result<Self> {
        Ok(Self {
            heartbeat_buffer: heartbeat_buffer,
            storage: storage,
            id_generator: Arc::new(RwLock::new(IdGeneratorFactory::new_id_generator(
                id_generator_mod,
            )?)),
        })
    }
}

#[tonic::async_trait]
impl ChunkHandlerService for ChunkHandlerServiceImpl {
    async fn register(
        &self,
        request: Request<RegisterRequest>,
    ) -> std::result::Result<Response<RegisterResponse>, Status> {
        info!("Got a request from {:?}", request.remote_addr());

        let chunk_id = self.id_generator.generate().await;

        Ok(Response::new(RegisterResponse::new_ok(chunk_id)))
    }

    type heartbeatStream =
        Pin<Box<dyn Stream<Item = std::result::Result<HeartbeatResponse, Status>> + Send>>;

    async fn heartbeat(
        &self,
        request: Request<Streaming<HeartbeatRequest>>,
    ) -> std::result::Result<Response<Self::heartbeatStream>, Status> {
        info!(
            "client heart-beat connected from: {:?}",
            request.remote_addr()
        );

        let mut buffer = self.heartbeat_buffer.clone();

        let mut in_stream = request.into_inner();
        let (tx, rx) = mpsc::channel(128);

        tokio::spawn(async move {
            while let Some(item) = in_stream.next().await {
                let mut buffer = buffer.clone();

                match item {
                    Ok(v) => {
                        let mut b = buffer.write().await;
                        b.update(&v.chunk_id);

                        tx.send(Ok(HeartbeatResponse::new_ok()))
                            .await
                            .expect("working rx");
                    }
                    Err(err) => {
                        if let Some(io_err) = match_for_io_error(&err) {
                            if io_err.kind() == ErrorKind::BrokenPipe {
                                error!("client heart-beat disconnected: broken pipe");
                                break;
                            }
                        }

                        match tx.send(Err(err)).await {
                            Ok(_) => (),
                            Err(_err) => break,
                        }
                    }
                }
            }

            info!("heart-beat stream ended");
        });

        let out_stream = ReceiverStream::new(rx);

        Ok(Response::new(Box::pin(out_stream) as Self::heartbeatStream))
    }
}

pub struct ChunkHandler {
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

    pub fn start(&mut self) -> Result<()> {
        use tokio::runtime::Runtime;
        let rt = Runtime::new()?;

        let storage = self.storage.clone();

        let addr = "[::1]:10000".parse().unwrap();
        let heartbeat_buffer = Arc::new(RwLock::new(HeartbeatBuffer::new()));
        let mut s = ChunkHandlerServiceImpl::new(heartbeat_buffer.clone(), storage);

        if let Err(err) = self.start_async_tasks(heartbeat_buffer) {
            error!("chunk handler service start failed");
            return Err(err);
        }

        // log
        let chunk_handler_log = log4rs::append::file::FileAppender::builder()
            .encoder(Box::new(log4rs::encode::pattern::PatternEncoder::new(
                "{d} - {m}{n}",
            )))
            .build("logs/chunk_handler.log")
            .unwrap();

        let chunk_handler_log_name = "chunk_handler_log";
        let config = log4rs::config::Config::builder()
            .appender(
                log4rs::config::Appender::builder()
                    .build(chunk_handler_log_name, Box::new(chunk_handler_log)),
            )
            .logger(
                log4rs::config::Logger::builder()
                    .appender(chunk_handler_log_name)
                    .additive(false)
                    .build("app::chunk_handler_log", log::LevelFilter::Info),
            )
            .build(
                log4rs::config::Root::builder()
                    .appender(chunk_handler_log_name)
                    .build(log::LevelFilter::Info),
            )
            .unwrap();

        log4rs::init_config(config).unwrap();

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

    pub fn new(storage_mode: StorageMode) -> Result<Self> {
        Ok(Self {
            storage: Arc::new(RwLock::new(StorageFactory::new_storage(storage_mode)?)),
        })
    }
}
