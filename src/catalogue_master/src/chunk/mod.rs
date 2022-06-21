mod storage;

pub use storage::StorageMode;

use crate::Result;
use storage::{Storage, StorageFactory};

pub struct ChunkHandler {
    storage: Box<dyn Storage>,
}

impl ChunkHandler {
    pub fn start(&mut self) -> Result<()> {
        unimplemented!();
    }

    fn handle_heartbeat(&mut self) -> Result<()> {
        unimplemented!();
    }

    pub fn new(storage_mode: StorageMode) -> Result<Self> {
        Ok(Self {
            storage: StorageFactory::new_storage(storage_mode)?,
        })
    }
}
