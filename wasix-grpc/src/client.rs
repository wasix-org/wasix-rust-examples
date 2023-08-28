use std::time::Duration;

use hello_world::greeter_client::GreeterClient;
use hello_world::HelloRequest;

use http_body::Body;
use hyper::{body::Bytes, client::HttpConnector, Client, Request, Uri};
use hyper_rustls::HttpsConnector;
use tokio_rustls::rustls::{ClientConfig, RootCertStore};
use tokio_stream::{Stream, StreamExt};
use tower::{util::MapRequest, ServiceExt};

pub mod hello_world {
    tonic::include_proto!("helloworld");
}

fn hello_requests_iter() -> impl Stream<Item = HelloRequest> {
    tokio_stream::iter(1..usize::MAX).map(|i| HelloRequest {
        name: format!("Hello {}", i),
    })
}

async fn say_hello<T>(client: &mut GreeterClient<T>)
where
    T: tonic::client::GrpcService<tonic::body::BoxBody> + Send + 'static,
    T::ResponseBody: Body<Data = Bytes> + Send + 'static,
    <T::ResponseBody as Body>::Error: Into<Box<dyn std::error::Error + Send + Sync>> + Send,
{
    let request = tonic::Request::new(HelloRequest {
        name: "Alice".into(),
    });

    let response = client.send(request).await.unwrap();

    println!("RESPONSE={:?}", response);
}

async fn server_streaming_hello<T>(client: &mut GreeterClient<T>, num: usize)
where
    T: tonic::client::GrpcService<tonic::body::BoxBody> + Send + 'static,
    T::ResponseBody: Body<Data = Bytes> + Send + 'static,
    <T::ResponseBody as Body>::Error: Into<Box<dyn std::error::Error + Send + Sync>> + Send,
{
    let stream = client
        .send_stream(HelloRequest { name: "bob".into() })
        .await
        .unwrap()
        .into_inner();

    // stream is infinite - take just 5 elements and then disconnect
    let mut stream = stream.take(num);
    while let Some(item) = stream.next().await {
        dbg!(&item);
        println!("\treceived: {}", item.unwrap().message);
    }
    // stream is droped here and the disconnect info is send to server
}

async fn client_streaming_hello<T>(client: &mut GreeterClient<T>, num: usize)
where
    T: tonic::client::GrpcService<tonic::body::BoxBody> + Send + 'static,
    T::ResponseBody: Body<Data = Bytes> + Send + 'static,
    <T::ResponseBody as Body>::Error: Into<Box<dyn std::error::Error + Send + Sync>> + Send,
{
    let in_stream = hello_requests_iter().take(num);

    let response = client.receive_stream(in_stream).await.unwrap();

    println!("RESPONSE={:?}", response);
}

async fn bidirectional_streaming_echo<T>(client: &mut GreeterClient<T>, num: usize)
where
    T: tonic::client::GrpcService<tonic::body::BoxBody> + Send + 'static,
    T::ResponseBody: Body<Data = Bytes> + Send + 'static,
    <T::ResponseBody as Body>::Error: Into<Box<dyn std::error::Error + Send + Sync>> + Send,
{
    let in_stream = hello_requests_iter().take(num);

    let response = client.bidirectional(in_stream).await.unwrap();

    let mut resp_stream = response.into_inner();

    while let Some(received) = resp_stream.next().await {
        let received = received.unwrap();
        println!("\treceived message: `{}`", received.message);
    }
}

async fn bidirectional_streaming_echo_throttle<T>(client: &mut GreeterClient<T>, dur: Duration)
where
    T: tonic::client::GrpcService<tonic::body::BoxBody> + Send + 'static,
    T::ResponseBody: Body<Data = Bytes> + Send + 'static,
    <T::ResponseBody as Body>::Error: Into<Box<dyn std::error::Error + Send + Sync>> + Send,
{
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

    let fd = std::fs::File::open(data_dir.join("tls/ca.pem"))?;

    let mut roots = RootCertStore::empty();

    let mut buf = std::io::BufReader::new(&fd);
    let certs = rustls_pemfile::certs(&mut buf)?;
    roots.add_parsable_certificates(&certs);

    let tls = ClientConfig::builder()
        .with_safe_defaults()
        .with_root_certificates(roots)
        .with_no_client_auth();

    let mut http = HttpConnector::new();
    http.enforce_http(false);

    let connector = tower::ServiceBuilder::new()
        .layer_fn(move |s| {
            let tls = tls.clone();

            hyper_rustls::HttpsConnectorBuilder::new()
                .with_tls_config(tls)
                .https_or_http()
                .enable_http2()
                .wrap_connector(s)
        })
        .map_request(|_: Uri| Uri::from_static("https://127.0.0.1:50051"))
        .service(http);

    let client = hyper::Client::builder().build(connector);

    // Using `with_origin` will let the codegenerated client set the `scheme` and
    // `authority` from the porvided `Uri`.
    let uri = Uri::from_static("https://example.com");

    let mut client = GreeterClient::with_origin(client, uri);
    let request = tonic::Request::new(HelloRequest {
        name: "Alice".into(),
    });

    let response = client.send(request).await.unwrap();

    println!("RESPONSE={:?}", response);
    say_hello(&mut client).await;

    // server_streaming_hello(&mut client, 5).await;

    client_streaming_hello(&mut client, 5).await;

    bidirectional_streaming_echo(&mut client, 5).await;

    // bidirectional_streaming_echo_throttle(&mut client, Duration::from_millis(200)).await;

    Ok(())
}
