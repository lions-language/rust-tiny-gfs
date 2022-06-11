pub struct ChunkServerId(String);

pub struct Chunk {
    chunk_server_id: ChunkServerId,
}

pub struct ChunkId(String);

impl From<String> for ChunkId {
    fn from(v: String) -> Self {
        Self(v)
    }
}

pub struct Offset(i64);

pub struct FileObject {
    chunk_id: ChunkId,
    offset: Offset,
}
