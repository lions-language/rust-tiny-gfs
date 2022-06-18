mod storage;

pub use storage::StorageMode;

use crate::Result;
use storage::{Storage, StorageFactory};

pub struct ChunkService {
    storage: Box<dyn Storage>,
}

impl ChunkService {
    pub fn new(storage_mode: StorageMode) -> Result<Self> {
        Ok(Self {
            storage: StorageFactory::new_storage(storage_mode)?,
        })
    }
}
