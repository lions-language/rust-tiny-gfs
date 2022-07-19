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

use crate::proto::catalogue::{catalogue_service_server::*, *};

use super::idgenerator::IdGeneratorMode;
use super::idgenerator::{IdGenerator, IdGeneratorFactory};
use super::storage::Storage;
use super::HeartbeatBuffer;

pub(crate) struct CatalogueServiceImpl {}

impl ChunkHandlerServiceImpl {
    pub(crate) fn new() -> Result<Self> {
        Ok(Self {})
    }
}

#[tonic::async_trait]
impl CatalogueService for CatalogueServiceImpl {
    async fn register(
        &self,
        request: Request<RegisterRequest>,
    ) -> std::result::Result<Response<RegisterResponse>, Status> {
        info!("Got a request from {:?}", request.remote_addr());

        let chunk_id = match self.id_generator.write().await.next().await {
            Ok(id) => id,
            Err(err) => return Err(Status::internal(err.description().to_string())),
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
