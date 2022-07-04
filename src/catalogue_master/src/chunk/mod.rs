mod storage;

pub use storage::StorageMode;

use crate::Result;
use storage::{Storage, StorageFactory};

use futures::Stream;
use tokio::sync::{mpsc, Mutex};
use tokio_stream::{wrappers::ReceiverStream, StreamExt};
use tonic::{transport::Server, Request, Response, Status, Streaming};

use std::sync::Arc;
use std::{error::Error, io::ErrorKind};
use std::{pin::Pin, time::Duration};

// pub mod chunk_handler {
//     tonic::include_proto!("/proto/chunk_handler_service");
// }

use crate::proto::chunk_handler::{chunk_handler_service_server::*, *};

fn match_for_io_error(err_status: &Status) -> Option<&std::io::Error> {
    let mut err: &(dyn Error + 'static) = err_status;

    loop {
        if let Some(io_err) = err.downcast_ref::<std::io::Error>() {
            return Some(io_err);
        }

        // h2::Error do not expose std::io::Error with `source()`
        // https://github.com/hyperium/h2/pull/462
        if let Some(h2_err) = err.downcast_ref::<h2::Error>() {
            if let Some(io_err) = h2_err.get_io() {
                return Some(io_err);
            }
        }

        err = match err.source() {
            Some(err) => err,
            None => return None,
        };
    }
}

type HeartbeatResponseStream =
    Pin<Box<dyn Stream<Item = std::result::Result<HeartbeatResponse, Status>> + Send>>;

pub struct ChunkHandlerServiceImpl {
    storage: Arc<Mutex<Box<dyn Storage + Sync + Send>>>,
}

impl ChunkHandlerServiceImpl {
    fn handle_heartbeat(&mut self) -> Result<()> {
        unimplemented!();
    }

    fn new(storage: Arc<Mutex<Box<dyn Storage + Sync + Send>>>) -> Self {
        Self { storage: storage }
    }
}

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

    type heartbeatStream =
        Pin<Box<dyn Stream<Item = std::result::Result<HeartbeatResponse, Status>> + Send>>;

    // async fn heartbeat(
    //     &self,
    //     request: Request<Streaming<HeartbeatRequest>>,
    // ) -> std::result::Result<Response<Self::heartbeatStream>, Status> {
    //     println!("\tclient connected from: {:?}", request.remote_addr());

    //     // creating infinite stream with requested message
    //     let repeat = std::iter::repeat(HeartbeatResponse {
    //         // message: request.into_inner().message,
    //     });
    //     let mut stream = Box::pin(tokio_stream::iter(repeat).throttle(Duration::from_millis(200)));

    //     // spawn and channel are required if you want handle "disconnect" functionality
    //     // the `out_stream` will not be polled after client disconnect
    //     let (tx, rx) = mpsc::channel(128);
    //     tokio::spawn(async move {
    //         while let Some(item) = stream.next().await {
    //             match tx.send(std::result::Result::<_, Status>::Ok(item)).await {
    //                 Ok(_) => {
    //                     // item (server response) was queued to be send to client
    //                 }
    //                 Err(_item) => {
    //                     // output_stream was build from rx and both are dropped
    //                     break;
    //                 }
    //             }
    //         }
    //         println!("\tclient disconnected");
    //     });

    //     let output_stream = ReceiverStream::new(rx);
    //     Ok(Response::new(
    //         Box::pin(output_stream) as Self::heartbeatStream
    //     ))
    // }

    async fn heartbeat(
        &self,
        request: Request<Streaming<HeartbeatRequest>>,
    ) -> std::result::Result<Response<Self::heartbeatStream>, Status> {
        println!("\tclient connected from: {:?}", request.remote_addr());

        let mut in_stream = request.into_inner();
        let (tx, rx) = mpsc::channel(128);

        tokio::spawn(async move {
            while let Some(item) = in_stream.next().await {
                match item {
                    Ok(v) => {
                        self.tx
                            .send(Ok(HeartbeatResponse {}))
                            .await
                            .expect("working rx");
                    }
                    Err(err) => {
                        if let Some(io_err) = match_for_io_error(&err) {
                            if io_err.kind() == ErrorKind::BrokenPipe {
                                eprintln!("client disconnected: broken pipe");
                                break;
                            }
                        }

                        match tx.send(Err(err)).await {
                            Ok(_) => (),
                            Err(_err) => break,
                        }
                    }
                }
            }
            println!("stream ended");
        });

        let out_stream = ReceiverStream::new(rx);

        Ok(Response::new(Box::pin(out_stream) as Self::heartbeatStream))
    }
}

pub struct ChunkHandler {
    storage: Arc<Mutex<Box<dyn Storage + Sync>>>,
}

impl ChunkHandler {
    pub fn start(&mut self, storage_mode: StorageMode) -> Result<()> {
        let storage = StorageFactory::new_storage(storage_mode)?;

        use tokio::runtime::Runtime;
        let rt = Runtime::new()?;

        let addr = "[::1]:10000".parse().unwrap();
        let s = ChunkHandlerServiceImpl::new(Arc::new(Mutex::new(storage)));

        rt.block_on(async {
            Server::builder()
                .add_service(ChunkHandlerServiceServer::new(s))
                .serve(addr)
                .await
                .unwrap();
        });

        Ok(())
    }

    fn new(storage: Arc<Mutex<Box<dyn Storage + Sync>>>) -> Result<Self> {
        Ok(Self { storage: storage })
    }
}
