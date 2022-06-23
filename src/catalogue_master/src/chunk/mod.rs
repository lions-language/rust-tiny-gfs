mod storage;

pub use storage::StorageMode;

use crate::Result;
use storage::{Storage, StorageFactory};

use tonic::{transport::Server, Request, Response, Status};

pub mod chunk_handler {
    tonic::include_proto!("chunk_handler");
}

#[derive(Default)]
pub struct Service {}

#[tonic::async_trait]
impl ChunkHandlerService for Service {
    async fn say_hello(
        &self,
        request: Request<HelloRequest>,
    ) -> Result<Response<HelloReply>, Status> {
        println!("Got a request from {:?}", request.remote_addr());

        let reply = hello_world::HelloReply {
            message: format!("Hello {}!", request.into_inner().name),
        };
        Ok(Response::new(reply))
    }
}

pub struct ChunkHandler {
    storage: Box<dyn Storage>,
}

impl ChunkHandler {
    pub fn start(&mut self) -> Result<()> {
        use tokio::runtime::Runtime;
        let rt = Runtime::new()?;

        let addr = "[::1]:10000".parse().unwrap();
        let greeter = MyGreeter::default();

        rt.block_on(async {
            Server::builder()
                .add_service(GreeterServer::new(greeter))
                .serve(addr)
                .await?;
        })
    }

    fn handle_heartbeat(&mut self) -> Result<()> {
        unimplemented!();
    }

    pub fn new(storage_mode: StorageMode) -> Result<Self> {
        Ok(Self {
            storage: StorageFactory::new_storage(storage_mode)?,
        })
    }
}
