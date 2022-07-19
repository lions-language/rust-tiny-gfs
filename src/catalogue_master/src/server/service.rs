use crate::{Chunk, Error, Result};

use futures::Stream;
use tokio::sync::{mpsc, RwLock};
use tokio::time::{self, Duration, Interval};
use tonic::{transport::Server, Request, Response, Status, Streaming};

use std::collections::HashMap;
use std::pin::Pin;
use std::sync::Arc;
use std::{error::Error as StdError, io::ErrorKind};

use log::{error, info, warn};

use crate::proto::catalogue::{catalogue_service_server::*, *};

pub(crate) struct CatalogueServiceImpl {}

impl CatalogueServiceImpl {
    pub(crate) fn new() -> Result<Self> {
        Ok(Self {})
    }
}

#[tonic::async_trait]
impl CatalogueService for CatalogueServiceImpl {
    async fn create_file(
        &self,
        request: Request<CreateFileRequest>,
    ) -> std::result::Result<Response<CreateFileResponse>, Status> {
        info!("Got a request from {:?}", request.remote_addr());

        // Ok(Response::new(CreateFileResponse::new_ok(chunk_id)))
        unimplemented!();
    }

    async fn delete_file(
        &self,
        request: Request<DeleteFileRequest>,
    ) -> std::result::Result<Response<DeleteFileResponse>, Status> {
        unimplemented!();
    }

    async fn get_file(
        &self,
        request: Request<GetFileRequest>,
    ) -> std::result::Result<Response<GetFileResponse>, Status> {
        unimplemented!();
    }
}
