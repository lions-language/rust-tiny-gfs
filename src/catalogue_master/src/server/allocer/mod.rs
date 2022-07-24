use crate::{Chunk, Result};

pub(crate) struct Allocer {}

impl Allocer {
    pub(crate) fn alloc(&mut self) -> Result<Vec<Chunk>> {
        unimplemented!();
    }

    pub(crate) fn new() -> Self {
        Self {}
    }
}
