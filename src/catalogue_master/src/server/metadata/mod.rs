mod memory;

use crate::Chunks;

use crate::Result;

#[tonic::async_trait]
pub(crate) trait Metadata {
    async fn get_file(&self, dir: String, name: String) -> Result<Option<Chunks>>;
}

pub(crate) type MetadataPtr = Box<dyn Metadata + Sync + Send>;
pub(crate) type MetadataPtrArc = std::sync::Arc<tokio::sync::RwLock<MetadataPtr>>;

pub(crate) enum MetadataMode {
    Memory,
}

pub(crate) struct MetadataFactory {}

impl MetadataFactory {
    pub(crate) fn create_metadata(&self, mode: MetadataMode) -> Result<MetadataPtr> {
        match mode {
            MetadataMode::Memory => Ok(Box::new(memory::Memory::new())),
        }
    }

    pub(crate) fn new() -> Self {
        Self {}
    }
}
