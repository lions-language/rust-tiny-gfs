use tokio::sync::RwLock;
use tonic::metadata;

use crate::server::metadata::{Metadata, MetadataMode, MetadataPtr, MetadataPtrArc};

use crate::proto::catalogue::CreateFileRequest;

use crate::filesys::File;
use crate::{Error, Result};

use super::metadata::MetadataFactory;

pub(crate) struct FileMgr {
    metadata: MetadataPtrArc,
}

impl FileMgr {
    pub(crate) async fn create_file(&mut self, req: CreateFileRequest) -> Result<File> {
        // 1. file is exists (read from metadata)
        // - exist:
        //  - read finish marker
        //      - finish: return error
        //      - init state / writing state: return success
        // - not exist:
        //  - alloc balance chunks
        //  - create metadata
        match self
            .metadata
            .read()
            .await
            .get_file(req.dir.into(), req.name.into())
            .await
        {
            Ok(file) => match file {
                Some(file) => {
                    // check
                    if file.is_finish() {
                        return Err(Error::AlreadyExist(file.full_name()));
                    } else {
                        Ok(file)
                    }
                }
                None => {
                    // alloc
                    self.metadata.write().await.alloc(req.total).await
                }
            },
            Err(err) => Err(err),
        }
    }

    pub(crate) fn new(metadata: MetadataPtrArc) -> Self {
        Self { metadata }
    }
}

pub(crate) type FileMgrArc = std::sync::Arc<tokio::sync::RwLock<FileMgr>>;

pub(crate) fn new_shared_file_mgr(metadata_mode: MetadataMode) -> Result<FileMgrArc> {
    Ok(std::sync::Arc::new(tokio::sync::RwLock::new(FileMgr::new(
        MetadataFactory::new().create_metadata_arc(metadata_mode)?,
    ))))
}
