use crate::{Chunk, Error, Result};

use futures::Stream;
use tokio::sync::{mpsc, RwLock};
use tokio::time::{self, Duration, Interval};
use tokio_stream::{wrappers::ReceiverStream, StreamExt};
use tonic::{transport::Server, Request, Response, Status, Streaming};

use std::collections::HashMap;
use std::pin::Pin;
use std::sync::Arc;
use std::{error::Error as StdError, io::ErrorKind};

use log::{error, info, warn};

use crate::proto::chunk_handler::{chunk_handler_service_server::*, *};

use super::idgenerator::IdGeneratorMode;
use super::idgenerator::{IdGenerator, IdGeneratorFactory};
use super::storage::Storage;
use super::HeartbeatBuffer;

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

type HeartbeatResponseStream =
    Pin<Box<dyn Stream<Item = std::result::Result<HeartbeatResponse, Status>> + Send>>;

pub(crate) struct ChunkHandlerServiceImpl {
    heartbeat_buffer: Arc<RwLock<HeartbeatBuffer>>,
    storage: Arc<RwLock<Box<dyn Storage + Sync + Send>>>,
    id_generator: Arc<RwLock<Box<dyn IdGenerator + Sync + Send>>>,
}

impl ChunkHandlerServiceImpl {
    pub(crate) fn new(
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

        let chunk_id = match self.id_generator.write().await.next().await {
            Ok(id) => id,
            Err(err) => {
                error!("{}", err);
                return Err(Status::internal(err.description().to_string()));
            }
        };

        let register_request = request.into_inner();

        if let Err(err) = self
            .storage
            .write()
            .await
            .insert(
                chunk_id.clone().into(),
                Chunk::new(register_request.server_addr.into()),
            )
            .await
        {
            error!("insert failed = {}", err);
            return Err(Status::internal(err.description().to_string()));
        };

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
