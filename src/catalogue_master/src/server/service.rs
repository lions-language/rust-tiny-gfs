use crate::{Chunk, Error, Result};

use futures::Stream;
use tokio::sync::{mpsc, RwLock};
use tokio::time::{self, Duration, Interval};
use tonic::{transport::Server, Request, Response, Status, Streaming};

use std::collections::HashMap;
use std::fs::File;
use std::pin::Pin;
use std::sync::Arc;
use std::{error::Error as StdError, io::ErrorKind};

use log::{error, info, warn};

use crate::proto::catalogue::{catalogue_service_server::*, *};

use super::filemgr::{new_shared_file_mgr, FileMgr, FileMgrArc};

pub(crate) struct CatalogueServiceImpl {
    file_mgr: FileMgrArc,
}

impl CatalogueServiceImpl {
    pub(crate) fn new(metadata_mode: MetadataMode) -> Result<Self> {
        Ok(Self {
            file_mgr: new_shared_file_mgr(metadata_mode)?,
        })
    }
}

#[tonic::async_trait]
impl CatalogueService for CatalogueServiceImpl {
    async fn create_file(
        &self,
        request: Request<CreateFileRequest>,
    ) -> std::result::Result<Response<CreateFileResponse>, Status> {
        info!(
            "create file: got a request from {:?}",
            request.remote_addr()
        );

        // 1. get chunks via file name

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
