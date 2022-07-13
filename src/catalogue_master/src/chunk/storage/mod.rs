mod memory;

use std::collections::HashMap;

use crate::{Chunk, ChunkId, Result};
use memory::Memory;

pub enum StorageMode {
    Memory,
}

pub(crate) struct StorageFactory {}

impl StorageFactory {
    pub(crate) fn new_storage(mode: StorageMode) -> Result<Box<dyn Storage + Sync + Send>> {
        match mode {
            StorageMode::Memory => Ok(Box::new(Memory::new())),
        }
    }
}

#[tonic::async_trait]
pub(crate) trait Storage {
    async fn insert(&mut self, chunk_id: &ChunkId, chunk: Chunk) -> Result<()>;
    fn find(&self, chunk_id: &ChunkId) -> Result<Chunk>;
    async fn update_state_multi(&mut self, heartbeat_datas: HashMap<String, u64>) -> Result<()>;
}
