use super::ArcStorage;
use crate::{Chunk, ChunkId};

pub(crate) struct ChunkOperator {
    storage: ArcStorage,
}

pub(crate) struct ApplyChunkData {
    num_chunks: usize,
}

impl ChunkOperator {
    pub(crate) async fn apply_chunks(&mut self, data: ApplyChunkData) -> Vec<ChunkId> {
        unimplemented!();
    }

    pub(crate) async fn find_chunk(&self, chunk_id: &ChunkId) -> Option<Chunk> {
        unimplemented!();
    }

    pub(crate) fn new(storage: ArcStorage) -> Self {
        Self { storage: storage }
    }
}
