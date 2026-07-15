use std::time::Duration;

use hello_world::greeter_client::GreeterClient;
use hello_world::HelloRequest;

use tokio_stream::{Stream, StreamExt};
use tonic::transport::{Certificate, Channel, ClientTlsConfig};

pub mod hello_world {
    tonic::include_proto!("helloworld");
}

fn hello_requests_iter() -> impl Stream<Item = HelloRequest> {
    tokio_stream::iter(1..usize::MAX).map(|i| HelloRequest {
        name: format!("Hello {}", i),
    })
}

async fn say_hello(client: &mut GreeterClient<Channel>) {
    let request = tonic::Request::new(HelloRequest {
        name: "Alice".into(),
    });

    let response = client.send(request).await.unwrap();

    println!("RESPONSE={:?}", response);
}

async fn server_streaming_hello(client: &mut GreeterClient<Channel>, num: usize) {
    let stream = client
        .send_stream(HelloRequest { name: "bob".into() })
        .await
        .unwrap()
        .into_inner();

    // stream is infinite - take just `num` elements and then disconnect
    let mut stream = stream.take(num);
    while let Some(item) = stream.next().await {
        println!("\treceived: {}", item.unwrap().message);
    }
    // stream is dropped here and the disconnect info is send to server
}

async fn client_streaming_hello(client: &mut GreeterClient<Channel>, num: usize) {
    let in_stream = hello_requests_iter().take(num);

    let response = client.receive_stream(in_stream).await.unwrap();

    println!("RESPONSE={:?}", response);
}

async fn bidirectional_streaming_echo(client: &mut GreeterClient<Channel>, num: usize) {
    let in_stream = hello_requests_iter().take(num);

    let response = client.bidirectional(in_stream).await.unwrap();

    let mut resp_stream = response.into_inner();

    while let Some(received) = resp_stream.next().await {
        let received = received.unwrap();
        println!("\treceived message: `{}`", received.message);
    }
}

#[allow(dead_code)]
async fn bidirectional_streaming_echo_throttle(
    client: &mut GreeterClient<Channel>,
    dur: Duration,
) {
    let in_stream = hello_requests_iter().throttle(dur);

    let response = client.bidirectional(in_stream).await.unwrap();

    let mut resp_stream = response.into_inner();

    while let Some(received) = resp_stream.next().await {
        let received = received.unwrap();
        println!("\treceived message: `{}`", received.message);
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let data_dir = if cfg!(target_os = "wasi") {
        std::env::current_dir()?
    } else {
        std::path::PathBuf::from(std::env!("CARGO_MANIFEST_DIR"))
    };

    // The server's certificate is signed by our own CA, so trust that CA.
    let ca = Certificate::from_pem(std::fs::read_to_string(data_dir.join("tls/ca.pem"))?);
    let tls = ClientTlsConfig::new()
        .ca_certificate(ca)
        .domain_name("localhost");

    let channel = Channel::from_static("https://127.0.0.1:50051")
        .tls_config(tls)?
        .connect()
        .await?;

    let mut client = GreeterClient::new(channel);

    say_hello(&mut client).await;

    server_streaming_hello(&mut client, 5).await;

    client_streaming_hello(&mut client, 5).await;

    bidirectional_streaming_echo(&mut client, 5).await;

    Ok(())
}
