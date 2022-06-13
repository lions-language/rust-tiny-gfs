mod chunk;
mod error;
mod iowrap;
mod serdeser;
mod service;

pub(crate) use error::{Error, Result};

struct NSNode {
    name: String,
}

pub struct Path {
    namespace: Vec<NSNode>,
    name: String,
}

struct ChunkServer(String);

struct Chunk {
    chunk_server: ChunkServer,
}

struct ChunkId(String);

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
