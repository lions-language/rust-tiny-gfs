use catalogue_master::proto::chunk_handler::{chunk_handler_service_client::*, *};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut client = ChunkHandlerServiceClient::connect("http://[::1]:10000")
        .await
        .unwrap();

    let request = tonic::Request::new(RegisterRequest {
        server_addr: "[::1]:8081".into(),
    });

    if let Err(err) = client.register(request).await {
        println!("{}", err);
    };

    Ok(())
}
