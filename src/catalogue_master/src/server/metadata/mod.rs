mod memory;

use std::sync::Arc;

use tokio::sync::RwLock;

use crate::Result;
use crate::{Chunks, File};

#[tonic::async_trait]
pub(crate) trait Metadata {
    async fn get_file(&self, dir: String, name: String) -> Result<Option<File>>;
    async fn alloc(&mut self) -> Result<File>;
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

    pub(crate) fn create_metadata_arc(&self, mode: MetadataMode) -> Result<MetadataPtrArc> {
        Ok(Arc::new(RwLock::new(self.create_metadata(mode)?)))
    }

    pub(crate) fn new() -> Self {
        Self {}
    }
}
