use crate::ChunkId;

pub(crate) struct NSNode {
    name: String,
}

pub(crate) struct Path {
    namespace: Vec<NSNode>,
    name: String,
}

pub(crate) struct FileObject {
    chunk_id: ChunkId,
    pos: usize,
    length: usize,
}

pub(crate) struct File {
    objects: Vec<FileObject>,
}
