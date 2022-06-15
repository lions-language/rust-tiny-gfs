use super::Storage;
use crate::{Chunk, ChunkId, Result};

pub struct Memory {}

impl Storage for Memory {
    fn insert(&mut self, chunk_id: &ChunkId, chunk: Chunk) -> Result<()> {
        unimplemented!();
    }

    fn find(&self, chunk_id: &ChunkId) -> Result<Chunk> {
        unimplemented!();
    }
}

impl Memory {
    pub fn new() -> Self {
        Self {}
    }
}
