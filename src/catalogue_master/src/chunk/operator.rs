use super::ArcStorage;
use crate::ChunkId;

pub(crate) struct ChunkOperator {
    storage: ArcStorage,
}

pub(crate) struct ApplyChunkData {
    num_chunks: usize,
}

impl ChunkOperator {
    pub(crate) fn apply_chunks(&mut self, data: ApplyChunkData) -> Vec<ChunkId> {
        unimplemented!();
    }

    pub(crate) fn new(storage: ArcStorage) -> Self {
        Self { storage: storage }
    }
}
