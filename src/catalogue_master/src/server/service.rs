use crate::Result;

use tonic::{Request, Response, Status};

use log::info;

use crate::proto::catalogue::{catalogue_service_server::*, *};

use super::filemgr::{new_shared_file_mgr, FileMgrArc};
use super::metadata::MetadataMode;

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

        match self
            .file_mgr
            .write()
            .await
            .create_file(request.into_inner())
            .await
        {
            Ok(file) => {}
            Err(err) => {}
        }
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
