use crate::{Chunk, Result};

pub(crate) struct Allocer {}

impl Allocer {
    pub(crate) fn alloc(&mut self) -> Result<Chunk> {
        // 1.
    }

    pub(crate) fn new() -> Self {
        Self {}
    }
}
