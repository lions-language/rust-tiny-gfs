use crate::ChunkId;

pub(crate) struct Path {
    dir: String,
    name: String,
}

pub(crate) struct FileObject {
    chunk_id: ChunkId,
    pos: usize,
    length: usize,
}

#[derive(PartialEq)]
pub(crate) enum Status {
    Init,
    Writing,
    Finish,
}

pub(crate) struct File {
    objects: Vec<FileObject>,
    status: Status,
}

impl File {
    pub(crate) fn is_finish(&self) -> bool {
        self.status == Status::Finish
    }
}
