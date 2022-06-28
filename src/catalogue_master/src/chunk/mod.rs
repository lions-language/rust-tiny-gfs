mod storage;

pub use storage::StorageMode;

use crate::Result;
use storage::{Storage, StorageFactory};

use tonic::{transport::Server, Request, Response, Status, Streaming};

// pub mod chunk_handler {
//     tonic::include_proto!("/proto/chunk_handler_service");
// }

use crate::proto::chunk_handler::{chunk_handler_service_server::*, *};

#[derive(Default)]
pub struct ChunkHandlerServiceImpl {}

#[tonic::async_trait]
impl ChunkHandlerService for ChunkHandlerServiceImpl {
    async fn register(
        &self,
        request: Request<RegisterRequest>,
    ) -> std::result::Result<Response<RegisterResponse>, Status> {
        println!("Got a request from {:?}", request.remote_addr());

        let reply = RegisterResponse {};
        Ok(Response::new(reply))
    }

    async fn heartbeat(
        &self,
        request: Request<Streaming<HeartbeatRequest>>,
    ) -> Result<Response<HeartbeatResponse>, Status> {
        println!("EchoServer::server_streaming_echo");
        println!("\tclient connected from: {:?}", req.remote_addr());

        // creating infinite stream with requested message
        let repeat = std::iter::repeat(EchoResponse {
            message: req.into_inner().message,
        });
        let mut stream = Box::pin(tokio_stream::iter(repeat).throttle(Duration::from_millis(200)));

        // spawn and channel are required if you want handle "disconnect" functionality
        // the `out_stream` will not be polled after client disconnect
        let (tx, rx) = mpsc::channel(128);
        tokio::spawn(async move {
            while let Some(item) = stream.next().await {
                match tx.send(Result::<_, Status>::Ok(item)).await {
                    Ok(_) => {
                        // item (server response) was queued to be send to client
                    }
                    Err(_item) => {
                        // output_stream was build from rx and both are dropped
                        break;
                    }
                }
            }
            println!("\tclient disconnected");
        });

        let output_stream = ReceiverStream::new(rx);
        Ok(Response::new(
            Box::pin(output_stream) as Self::ServerStreamingEchoStream
        ))
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
        let s = ChunkHandlerServiceImpl::default();

        rt.block_on(async {
            Server::builder()
                .add_service(ChunkHandlerServiceServer::new(s))
                .serve(addr)
                .await
                .unwrap();
        });

        Ok(())
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
