use crate::server::metadata::{Metadata, MetadataPtr, MetadataPtrArc};

use crate::proto::catalogue::CreateFileRequest;

pub(crate) struct FileMgr {
    metadata: MetadataPtrArc,
}

impl FileMgr {
    pub(crate) async fn create_file(&mut self, req: CreateFileRequest) {
        // 1. file is exists (read from metadata)
        // - exist:
        //  - read finish marker
        //      - finish: return error
        //      - init state / writing state: return success
        // - not exist:
        //  - alloc balance chunks
        //  - create metadata
        match self.metadata.get_file(req.dir, req.name) {
            Ok(chunks) => {}
            Err(err) => {}
        }
        unimplemented!();
    }

    pub(crate) fn new(metadata: MetadataPtrArc) -> Self {
        Self { metadata }
    }
}
