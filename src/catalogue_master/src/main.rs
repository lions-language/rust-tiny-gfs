use catalogue_master::{ChunkStorageMode, Result, Server};

fn main() -> Result<()> {
    let server = Server::new();
    server.start(ChunkStorageMode::Memory)?;

    Ok(())
}
