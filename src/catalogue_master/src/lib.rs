mod chunk;
mod error;
mod iowrap;
mod proto;
mod serdeser;
mod server;
mod service;

pub use chunk::StorageMode as ChunkStorageMode;
pub use server::Server;

pub(crate) use chunk::StorageFactory as ChunkStorageFactory;
pub use error::{Error, Result};

struct NSNode {
    name: String,
}

pub struct Path {
    namespace: Vec<NSNode>,
    name: String,
}

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

pub(crate) struct ChunkId(String);

impl From<String> for ChunkId {
    fn from(v: String) -> Self {
        Self(v)
    }
}

struct FileObject {
    chunk_id: ChunkId,
    pos: usize,
    length: usize,
}

pub struct File {
    objects: Vec<FileObject>,
}

struct ChunkServer(String);

impl From<String> for ChunkServer {
    fn from(v: String) -> Self {
        Self(v)
    }
}
