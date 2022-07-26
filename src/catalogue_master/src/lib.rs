mod chunk;
mod error;
mod filesys;
mod iowrap;
mod serdeser;
mod server;
mod service;

pub mod proto;

pub use chunk::StorageMode as ChunkStorageMode;
pub use server::Server;

pub(crate) use chunk::StorageFactory as ChunkStorageFactory;
pub use error::{Error, Result};

pub(crate) struct Chunk {
    chunk_server: ChunkServer,
}

impl Chunk {
    pub(crate) fn new(chunk_server: ChunkServer) -> Self {
        Self {
            chunk_server: chunk_server,
        }
    }
}

type Chunks = Vec<Chunk>;

pub(crate) struct ChunkId(String);

impl From<String> for ChunkId {
    fn from(v: String) -> Self {
        Self(v)
    }
}

struct ChunkServer(String);

impl From<String> for ChunkServer {
    fn from(v: String) -> Self {
        Self(v)
    }
}

pub(crate) use filesys::{File, FileObject, Path};
