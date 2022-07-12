use super::IdGenerator;
use crate::{Error, Result};

pub struct Memory {
    id: u64,
}

#[tonic::async_trait]
impl IdGenerator for Memory {
    async fn next(&mut self) -> Result<String> {
        let r: String = self.id.to_string();

        self.id += 1;

        Ok(r)
    }
}

impl Memory {
    pub(crate) fn new() -> Self {
        Self { id: 0 }
    }
}
