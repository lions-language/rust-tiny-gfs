use std::collections::HashMap;

use super::Storage;
use crate::{Chunk, ChunkId, Error, Result};

pub struct Memory {}

#[tonic::async_trait]
impl Storage for Memory {
    async fn insert(&mut self, chunk_id: ChunkId, chunk: Chunk) -> Result<()> {
        Err(Error::Unimplemented("Storage::insert".into()))
    }

    fn find(&self, chunk_id: &ChunkId) -> Result<Chunk> {
        Err(Error::Unimplemented("Storage::find".into()))
    }

    async fn update_state_multi(&mut self, heartbeat_datas: HashMap<String, u64>) -> Result<()> {
        Err(Error::Unimplemented("Storage::update_state_multi".into()))
    }
}

impl Memory {
    pub fn new() -> Self {
        Self {}
    }
}
