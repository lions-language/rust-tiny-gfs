use catalogue_master::proto::chunk_handler::{chunk_handler_service_client::*, *};

use futures::stream::Stream;
use std::time::Duration;
use tokio_stream::{wrappers::IntervalStream, StreamExt};
use tonic::transport::Channel;

pub struct HeartbeatStream {
    inner: Heartbeat,
}

impl HeartbeatStream {
    /// Create a new `HeartbeatStream`.
    pub fn new(interval: Heartbeat) -> Self {
        Self { inner: interval }
    }

    /// Get back the inner `Heartbeat`.
    pub fn into_inner(self) -> Heartbeat {
        self.inner
    }
}

impl Stream for HeartbeatStream {
    type Item = Instant;

    fn poll_next(mut self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Option<Instant>> {
        self.inner.poll_tick(cx).map(Some)
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        (std::usize::MAX, None)
    }
}

impl AsRef<Heartbeat> for HeartbeatStream {
    fn as_ref(&self) -> &Heartbeat {
        &self.inner
    }
}

impl AsMut<Heartbeat> for HeartbeatStream {
    fn as_mut(&mut self) -> &mut Heartbeat {
        &mut self.inner
    }
}

fn heartbeat_requests_iter() -> impl Stream<Item = HeartbeatRequest> {
    tokio_stream::iter(1..usize::MAX).map(|i| HeartbeatRequest {
        chunk_id: "xxx".into(),
    })
}

async fn streaming_heartbeat(client: &mut ChunkHandlerServiceClient<Channel>, num: usize) {
    let interval = tokio::time::interval(Duration::from_millis(10));
    let in_stream = IntervalStream::new(interval);
    // let in_stream = heartbeat_requests_iter().throttle(Duration::from_secs(1));

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
