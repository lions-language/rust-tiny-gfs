use crate::{Chunks, Result};

use super::Metadata;

pub(crate) struct Memory {}

#[tonic::async_trait]
impl Metadata for Memory {
    async fn get_file(&self, dir: String, name: String) -> Result<Option<Chunks>> {
        unimplemented!();
    }
}

impl Memory {
    pub(crate) fn new() -> Self {
        Self {}
    }
}
