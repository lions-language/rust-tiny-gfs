use catalogue_master::{ChunkHandler, ChunkIdGeneratorMode, ChunkStorageMode, Result, Server};

fn main() -> Result<()> {
    let server = Server::new();
    server.start(ChunkStorageMode::Memory)?;

    Ok(())
}
