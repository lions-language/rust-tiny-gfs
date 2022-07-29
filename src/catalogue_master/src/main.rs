use catalogue_master::{ChunkStorageMode, Result, Server, ServerMetadataMode};

fn main() -> Result<()> {
    let mut server = Server::new();
    server.start(ChunkStorageMode::Memory, ServerMetadataMode::Memory)?;

    Ok(())
}
