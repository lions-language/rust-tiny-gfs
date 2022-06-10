mod mem_disk;

pub struct ChunkId(String);

impl From<String> for ChunkId {
    fn from(v: String) -> Self {
        Self(v)
    }
}
