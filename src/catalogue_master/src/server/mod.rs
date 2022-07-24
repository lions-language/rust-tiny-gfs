mod allocer;
mod filemgr;
mod metadata;
mod service;

use std::sync::Arc;

use tokio::sync::RwLock;

use crate::Result;

use crate::chunk::{ChunkHandler, ChunkOperator, IdGeneratorMode, StorageFactory, StorageMode};

use allocer::Allocer;

pub struct Server {}

impl Server {
    pub fn start(&self, chunk_storage_mode: StorageMode) -> Result<()> {
        let chunk_storage = Arc::new(RwLock::new(StorageFactory::new_storage(
            chunk_storage_mode,
        )?));

        let mut chunk_handler = ChunkHandler::new(chunk_storage.clone())?;
        chunk_handler.start(IdGeneratorMode::Memory)?;

        let mut chunk_operator = ChunkOperator::new(chunk_storage);

        let allocer = Allocer::new();

        Ok(())
    }

    pub fn new() -> Self {
        Self {}
    }
}
