use catalogue_master::{ChunkHandler, ChunkIdGeneratorMode, ChunkStorageMode, Result};

fn main() -> Result<()> {
    let mut chunk_handler = ChunkHandler::new(Arc::new(RwLock::new(StorageFactory::new_storage(
        ChunkStorageMode::Memory,
    )?)))
    .unwrap();
    chunk_handler.start(ChunkIdGeneratorMode::Memory).unwrap();

    Ok(())
}
