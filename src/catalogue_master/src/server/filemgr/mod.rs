use crate::server::metadata::{Metadata, MetadataPtr, MetadataPtrArc};

use crate::proto::catalogue::CreateFileRequest;

use crate::Result;

pub(crate) struct FileMgr {
    metadata: MetadataPtrArc,
}

impl FileMgr {
    pub(crate) async fn create_file(&mut self, req: CreateFileRequest) -> Result<()> {
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
            Ok(chunks) => match chunks {
                Some(chunks) => {
                    // check
                    unimplemented!();
                }
                None => {
                    // alloc
                    unimplemented!();
                }
            },
            Err(err) => Err(err),
        }
    }

    pub(crate) fn new(metadata: MetadataPtrArc) -> Self {
        Self { metadata }
    }
}
