use crate::{Error, File, Result};

use super::Metadata;

pub(crate) struct Memory {}

#[tonic::async_trait]
impl Metadata for Memory {
    async fn get_file(&self, _dir: String, _name: String) -> Result<Option<File>> {
        Err(Error::Unimplemented("Metadata::get_file".into()))
    }

    async fn alloc(&mut self, _total: i64) -> Result<File> {
        Err(Error::Unimplemented("Metadata::alloc".into()))
    }
}

impl Memory {
    pub(crate) fn new() -> Self {
        Self {}
    }
}
