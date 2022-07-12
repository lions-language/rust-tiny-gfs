use catalogue_master::{ChunkHandler, ChunkIdGeneratorMode, ChunkStorageMode};

fn main() {
    let mut chunk_handler = ChunkHandler::new(ChunkStorageMode::Memory).unwrap();
    chunk_handler.start(ChunkIdGeneratorMode::Memory).unwrap();
}
