mod storage;

use storage::{Storage, StorageMode};

pub struct ChunkService {
    storage: Box<dyn Storage>,
}

impl ChunkService {
    pub fn new(mode: StorageMode) -> Self {
        Self {}
    }
}
