use crate::{Chunk, Result};

pub(crate) struct Allocer {}

impl Allocer {
    pub(crate) fn alloc(&mut self) -> Result<Vec<Chunk>> {
        // 1. file is exists (read from metadata)
        // - exist:
        //  - read finish marker
        //      - finish: return error
        //      - init state / writing state: return success
        // - not exist:
        //  - alloc balance chunks
        //  - create metadata
        unimplemented!();
    }

    pub(crate) fn new() -> Self {
        Self {}
    }
}
