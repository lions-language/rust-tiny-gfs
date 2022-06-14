mod memory;

use crate::{Chunk, ChunkId, Result};

pub enum StorageMode {}

pub trait Storage {
    fn insert(&mut self, chunk_id: &ChunkId, chunk: Chunk) -> Result<()>;
    fn find(&self, chunk_id: &ChunkId) -> Result<Chunk>;
}
