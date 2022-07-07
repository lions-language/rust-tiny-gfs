mod storage;

pub use storage::StorageMode;

use crate::{Error, Result};
use storage::{Storage, StorageFactory};

use futures::Stream;
use tokio::sync::{mpsc, RwLock};
use tokio_stream::{wrappers::ReceiverStream, StreamExt};
use tonic::{transport::Server, Request, Response, Status, Streaming};

use log::{error, info, warn};

use std::collections::HashMap;
use std::sync::Arc;
use std::{error::Error as StdError, io::ErrorKind};
use std::{pin::Pin, time::Duration};

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
}

impl ChunkHandlerServiceImpl {
    fn handle_heartbeat(&mut self) -> Result<()> {
        unimplemented!();
    }

    fn start(&mut self) -> Result<()> {
        std::thread::swap(|| {
            use tokio::runtime::Runtime;
            let rt = Runtime::new().unwrap();

            rt.block_on(async {});
        });
    }

    fn new() -> Self {
        Self {
            heartbeat_buffer: Arc::new(RwLock::new(HeartbeatBuffer::new())),
        }
    }
}

#[tonic::async_trait]
impl ChunkHandlerService for ChunkHandlerServiceImpl {
    async fn register(
        &self,
        request: Request<RegisterRequest>,
    ) -> std::result::Result<Response<RegisterResponse>, Status> {
        info!("Got a request from {:?}", request.remote_addr());

        let reply = RegisterResponse {};
        Ok(Response::new(reply))
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

pub struct ChunkHandler {}

impl ChunkHandler {
    pub fn start(&mut self, storage_mode: StorageMode) -> Result<()> {
        let storage = StorageFactory::new_storage(storage_mode)?;

        use tokio::runtime::Runtime;
        let rt = Runtime::new()?;

        let addr = "[::1]:10000".parse().unwrap();
        let mut s = ChunkHandlerServiceImpl::new();

        if let Err(err) = s.start() {
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

    pub fn new() -> Self {
        Self {}
    }
}
