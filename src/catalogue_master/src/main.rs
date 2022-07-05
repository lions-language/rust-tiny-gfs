use catalogue_master::{ChunkHandler, ChunkStorageMode};

fn main() {
    let mut chunk_handler = ChunkHandler::new();
    chunk_handler.start(ChunkStorageMode::Memory).unwrap();
}
