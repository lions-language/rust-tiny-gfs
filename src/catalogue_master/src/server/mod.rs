mod allocer;
mod filemgr;
mod metadata;
mod service;

use std::sync::Arc;

use log::info;
use tokio::sync::RwLock;

use tiny_gfs_utils::init_simple_file_log;

use crate::chunk::{ChunkHandler, IdGeneratorMode, StorageFactory, StorageMode};
use crate::proto::catalogue_service_server::CatalogueServiceServer;
use crate::Result;

use service::CatalogueServiceImpl;

pub use metadata::MetadataMode;

pub struct Server {}

impl Server {
    pub fn start(
        &mut self,
        chunk_storage_mode: StorageMode,
        metadata_mode: MetadataMode,
    ) -> Result<()> {
        let chunk_storage = Arc::new(RwLock::new(StorageFactory::new_storage(
            chunk_storage_mode,
        )?));

        let cs = chunk_storage.clone();
        std::thread::spawn(move || {
            let mut chunk_handler = ChunkHandler::new(cs).unwrap();
            chunk_handler.start(IdGeneratorMode::Memory).unwrap();
        });

        self.start_service(metadata_mode)?;

        // let mut chunk_operator = ChunkOperator::new(chunk_storage);

        // let allocer = Allocer::new();

        Ok(())
    }

    fn start_service(&mut self, metadata_mode: MetadataMode) -> Result<()> {
        use tokio::runtime::Runtime;
        let rt = Runtime::new()?;

        let addr = "[::1]:10010".parse().unwrap();
        let s = CatalogueServiceImpl::new(metadata_mode)?;

        init_simple_file_log(tiny_gfs_utils::SimpleFileLog {
            name: "catalogue_master_service_log",
            app_name: "app::catalogue_master_service_log",
            path: "logs/catalogue_master_service.log",
            level: log::LevelFilter::Info,
        });

        info!("catalogue master service start success");

        rt.block_on(async {
            tonic::transport::Server::builder()
                .add_service(CatalogueServiceServer::new(s))
                .serve(addr)
                .await
                .unwrap();
        });

        Ok(())
    }

    pub fn new() -> Self {
        Self {}
    }
}
