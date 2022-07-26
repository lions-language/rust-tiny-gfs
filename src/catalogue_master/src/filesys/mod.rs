use crate::ChunkId;

pub(crate) struct Path {
    dir: String,
    name: String,
}

impl Path {
    pub(crate) fn full_name(&self) -> String {
        std::path::Path::new(&self.dir)
            .join(&self.name)
            .to_str()
            .unwrap()
            .to_string()
    }
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
    path: Path,
    objects: Vec<FileObject>,
    status: Status,
}

impl File {
    pub(crate) fn is_finish(&self) -> bool {
        self.status == Status::Finish
    }

    pub(crate) fn full_name(&self) -> String {
        self.path.full_name()
    }
}
