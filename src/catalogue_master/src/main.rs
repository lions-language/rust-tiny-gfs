use catalogue_master::{ChunkHandler, ChunkStorageMode};

fn main() {
    let mut chunk_handler = ChunkHandler::new(ChunkStorageMode::Memory).unwrap();
    chunk_handler.start().unwrap();
}
