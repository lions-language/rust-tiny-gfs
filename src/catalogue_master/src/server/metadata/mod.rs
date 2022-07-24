use crate::Chunks;

use crate::Result;

#[tonic::async_trait]
pub(crate) trait Metadata {
    async fn get_file(&self, dir: String, name: String) -> Result<Option<Chunks>>;
}

pub(crate) type MetadataPtr = Box<dyn Metadata + Sync + Send>;
pub(crate) type MetadataPtrArc = std::sync::Arc<tokio::sync::RwLock<MetadataPtr>>;
