mod memory;

use memory::Memory;

use crate::Result;

#[tonic::async_trait]
pub(crate) trait IdGenerator {
    async fn next(&mut self) -> Result<String>;
}

pub enum IdGeneratorMode {
    Memory,
}

pub(crate) struct IdGeneratorFactory {}

impl IdGeneratorFactory {
    pub(crate) fn new_id_generator(
        mode: IdGeneratorMode,
    ) -> Result<Box<dyn IdGenerator + Sync + Send>> {
        match mode {
            IdGeneratorMode::Memory => Ok(Box::new(Memory::new())),
        }
    }
}
