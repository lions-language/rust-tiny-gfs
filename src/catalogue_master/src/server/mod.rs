use std::sync::Arc;

use tokio::sync::RwLock;

use crate::{ChunkIdGeneratorMode, ChunkStorageFactory, ChunkStorageMode, Result};

use crate::chunk::ChunkHandler;

pub struct Server {}

impl Server {
    pub fn start(&self, chunk_storage_mode: ChunkStorageMode) -> Result<()> {
        let chunk_storage = Arc::new(RwLock::new(ChunkStorageFactory::new_storage(
            chunk_storage_mode,
        )?));

        let mut chunk_handler = ChunkHandler::new(chunk_storage)?;
        chunk_handler.start(ChunkIdGeneratorMode::Memory)?;

        Ok(())
    }

    pub fn new() -> Self {
        Self {}
    }
}
