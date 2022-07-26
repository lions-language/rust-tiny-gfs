use crate::{Chunk, Result};
use crate::filesys::File;

pub(crate) struct Allocer {}

impl Allocer {
    pub(crate) fn alloc(&mut self) -> Result<File> {
        unimplemented!();
    }

    pub(crate) fn new() -> Self {
        Self {}
    }
}
