mod memory;

use crate::Result;

#[tonic::async_trait]
trait IdGenerator {
    async fn next(&mut self) -> Result<String>;
}
