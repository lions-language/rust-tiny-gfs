mod memory;

use crate::{Chunk, ChunkId, Result};
use memory::Memory;

pub enum StorageMode {
    Memory,
}

pub struct StorageFactory {}

impl StorageFactory {
    pub(crate) fn new_storage(mode: StorageMode) -> Result<Box<dyn Storage>> {
        match mode {
            StorageMode::Memory => Ok(Box::new(Memory::new())),
        }
    }
}

pub(crate) trait Storage {
    fn insert(&mut self, chunk_id: &ChunkId, chunk: Chunk) -> Result<()>;
    fn find(&self, chunk_id: &ChunkId) -> Result<Chunk>;
}
