mod chunk;
mod error;
mod idgenerator;
mod iowrap;
mod serdeser;
mod storage;

pub(crate) use error::{Error, Result};

struct NSNode {
    name: String,
}

pub struct Path {
    namespace: Vec<NSNode>,
    name: String,
}

struct ChunkServerId(String);

struct Chunk {
    chunk_server_id: ChunkServerId,
}

struct ChunkId(String);

impl From<String> for ChunkId {
    fn from(v: String) -> Self {
        Self(v)
    }
}

struct Offset(i64);

struct FileObject {
    chunk_id: ChunkId,
    offset: Offset,
}

pub struct File {
    objects: Vec<FileObject>,
}
