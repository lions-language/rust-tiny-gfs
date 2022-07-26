use crate::{Chunks, File, Result};

use super::Metadata;

pub(crate) struct Memory {}

#[tonic::async_trait]
impl Metadata for Memory {
    async fn get_file(&self, dir: String, name: String) -> Result<Option<File>> {
        unimplemented!();
    }

    async fn alloc(&mut self) -> Result<File> {
        unimplemented!();
    }
}

impl Memory {
    pub(crate) fn new() -> Self {
        Self {}
    }
}
