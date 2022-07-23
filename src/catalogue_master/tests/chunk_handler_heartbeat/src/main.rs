use catalogue_master::proto::chunk_handler::{chunk_handler_service_client::*, *};

use futures::stream::Stream;
use std::{pin::Pin, task::Context, task::Poll, time::Duration};
use tokio_stream::{wrappers::IntervalStream, StreamExt};
use tonic::transport::Channel;

pub struct HeartbeatStream {}

impl HeartbeatStream {
    pub fn new() -> Self {
        Self {}
    }
}

impl Stream for HeartbeatStream {
    type Item = HeartbeatRequest;

    fn poll_next(mut self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Option<HeartbeatRequest>> {
        Poll::Ready(Some(HeartbeatRequest {
            chunk_id: "xxx".into(),
        }))
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        (std::usize::MAX, None)
    }
}

// fn heartbeat_requests_iter() -> impl Stream<Item = HeartbeatRequest> {
//     tokio_stream::iter(1..usize::MAX).map(|i| HeartbeatRequest {
//         chunk_id: "xxx".into(),
//     })
// }

async fn streaming_heartbeat(client: &mut ChunkHandlerServiceClient<Channel>, num: usize) {
    // let interval = tokio::time::interval(Duration::from_millis(10));
    // let in_stream = IntervalStream::new(interval);
    // let in_stream = heartbeat_requests_iter().throttle(Duration::from_secs(1));

    let in_stream = HeartbeatStream::new();

    let stream = client.heartbeat(in_stream).await.unwrap().into_inner();

    // stream is infinite - take just 5 elements and then disconnect
    let mut stream = stream.take(num);
    while let Some(item) = stream.next().await {
        println!("\treceived: {}", item.unwrap().msg);
    }
    // stream is droped here and the disconnect info is send to server
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut client = ChunkHandlerServiceClient::connect("http://[::1]:50051")
        .await
        .unwrap();

    println!("Streaming heartbeat:");
    streaming_heartbeat(&mut client, 5).await;

    Ok(())
}
