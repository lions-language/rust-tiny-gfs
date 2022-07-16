use super::ArcStorage;

pub(crate) struct ChunkOperator {
    storage: ArcStorage,
}

impl ChunkOperator {
    pub(crate) fn new(storage: ArcStorage) -> Self {
        Self { storage: storage }
    }
}

