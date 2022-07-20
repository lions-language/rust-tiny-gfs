use crate::{Chunk, Result};

pub(crate) struct Allocer {}

impl Allocer {
    pub(crate) fn alloc(&mut self) -> Result<Chunk> {
        // 1. file is exists (read from metadata)
        // - exist:
        //  - read finish marker / init state
        //      - finish: return error
        // - not exist:
        //  - create metadata
        unimplemented!();
    }

    pub(crate) fn new() -> Self {
        Self {}
    }
}
