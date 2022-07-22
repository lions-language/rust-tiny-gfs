use catalogue_master::proto::chunk_handler::{chunk_handler_service_client::*, *};

use futures::stream::Stream;
use std::time::Duration;
use tokio_stream::StreamExt;
use tonic::transport::Channel;

async fn streaming_heartbeat(client: &mut ChunkHandlerServiceClient<Channel>, num: usize) {
    let stream = client
        .heartbeat(HeartbeatRequest {
            chunk_id: "xxx".into(),
        })
        .await
        .unwrap()
        .into_inner();

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
