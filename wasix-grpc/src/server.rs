use hyper::server::conn::Http;
use tokio_rustls::rustls::{OwnedTrustAnchor, RootCertStore, ServerConfig};
use tonic::{transport::Server, Request, Response, Status};

use hello_world::greeter_server::Greeter;
use hello_world::{HelloReply, HelloRequest};

use tokio::sync::mpsc;
use tokio_stream::{wrappers::ReceiverStream, Stream, StreamExt};

use std::error::Error;
use std::io::ErrorKind;
use std::pin::Pin;
use std::time::Duration;

use std::sync::Arc;
use tokio::net::TcpListener;
use tokio_rustls::{
    rustls::{Certificate, PrivateKey},
    TlsAcceptor,
};
use tower_http::ServiceBuilderExt;

pub mod hello_world {
    tonic::include_proto!("helloworld"); // The string specified here must match the proto package name
}

type ResponseStream = Pin<Box<dyn Stream<Item = Result<HelloReply, Status>> + Send>>;

fn match_for_io_error(err_status: &Status) -> Option<&std::io::Error> {
    let mut err: &(dyn Error + 'static) = err_status;

    loop {
        if let Some(io_err) = err.downcast_ref::<std::io::Error>() {
            return Some(io_err);
        }

        // h2::Error do not expose std::io::Error with `source()`
        // https://github.com/hyperium/h2/pull/462
        // if let Some(h2_err) = err.downcast_ref::<h2::Error>() {
        // if let Some(io_err) = h2_err.get_io() {
        // return Some(io_err);
        // }
        // }

        err = match err.source() {
            Some(err) => err,
            None => return None,
        };
    }
}

#[derive(Debug, Default)]
pub struct MyGreeter {}

#[tonic::async_trait]
impl Greeter for MyGreeter {
    async fn send(&self, request: Request<HelloRequest>) -> Result<Response<HelloReply>, Status> {
        Ok(Response::new(HelloReply {
            message: format!("hello {}", request.get_ref().name),
        }))
    }

    type SendStreamStream = ResponseStream;
    async fn send_stream(
        &self,
        request: Request<HelloRequest>,
    ) -> Result<Response<Self::SendStreamStream>, Status> {
        println!("Got a request from {:?}", request.remote_addr());

        let repeat = std::iter::repeat(HelloReply {
            message: "hello".to_string(),
        });

        let mut stream = Box::pin(tokio_stream::iter(repeat).throttle(Duration::from_millis(200)));
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
            Box::pin(output_stream) as Self::SendStreamStream
        ))
    }

    async fn receive_stream(
        &self,
        request: Request<tonic::Streaming<HelloRequest>>,
    ) -> Result<Response<HelloReply>, Status> {
        let mut stream = request.into_inner();
        let mut message = String::from("");
        while let Some(req) = stream.message().await? {
            message.push_str(&format!("Hello {}\n", req.name))
        }
        Ok(Response::new(HelloReply { message }))
    }

    type BidirectionalStream = ResponseStream;
    async fn bidirectional(
        &self,
        request: Request<tonic::Streaming<HelloRequest>>,
    ) -> Result<Response<Self::BidirectionalStream>, Status> {
        let mut in_stream = request.into_inner();
        let (tx, rx) = mpsc::channel(128);

        // this spawn here is required if you want to handle connection error.
        // If we just map `in_stream` and write it back as `out_stream` the `out_stream`
        // will be drooped when connection error occurs and error will never be propagated
        // to mapped version of `in_stream`.
        tokio::spawn(async move {
            while let Some(result) = in_stream.next().await {
                match result {
                    Ok(req) => tx
                        .send(Ok(HelloReply {
                            message: format!("hello {}", req.name),
                        }))
                        .await
                        .expect("working rx"),
                    Err(err) => {
                        if let Some(io_err) = match_for_io_error(&err) {
                            if io_err.kind() == ErrorKind::BrokenPipe {
                                // here you can handle special case when client
                                // disconnected in unexpected way
                                eprintln!("\tclient disconnected: broken pipe");
                                break;
                            }
                        }

                        match tx.send(Err(err)).await {
                            Ok(_) => (),
                            Err(_err) => break, // response was dropped
                        }
                    }
                }
            }
            println!("\tstream ended");
        });

        let out_stream = ReceiverStream::new(rx);

        Ok(Response::new(
            Box::pin(out_stream) as Self::BidirectionalStream
        ))
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let data_dir = if cfg!(target_os = "wasi") {
        std::env::current_dir()?
    } else {
        std::path::PathBuf::from(std::env!("CARGO_MANIFEST_DIR"))
    };

    let certs = {
        let fd = std::fs::File::open(data_dir.join("tls/server.pem"))?;
        let mut buf = std::io::BufReader::new(&fd);
        rustls_pemfile::certs(&mut buf)?
            .into_iter()
            .map(Certificate)
            .collect()
    };
    let key = {
        let fd = std::fs::File::open(data_dir.join("tls/server.key"))?;
        let mut buf = std::io::BufReader::new(&fd);
        rustls_pemfile::pkcs8_private_keys(&mut buf)?
            .into_iter()
            .map(PrivateKey)
            .next()
            .unwrap()

        // let key = std::fs::read(data_dir.join("tls/server.key"))?;
        // PrivateKey(key)
    };

    let mut tls = ServerConfig::builder()
        .with_safe_defaults()
        .with_no_client_auth()
        .with_single_cert(certs, key)?;
    tls.alpn_protocols = vec![b"h2".to_vec()];

    let server = MyGreeter::default();

    let svc = Server::builder()
        .add_service(hello_world::greeter_server::GreeterServer::new(server))
        .into_service();

    let mut http = Http::new();
    http.http2_only(true);

    let listener = TcpListener::bind("127.0.0.1:50051").await?;
    let tls_acceptor = TlsAcceptor::from(Arc::new(tls));

    eprintln!("Server listening on {}", listener.local_addr()?);

    loop {
        let (conn, addr) = match listener.accept().await {
            Ok(incoming) => incoming,
            Err(e) => {
                eprintln!("Error accepting connection: {}", e);
                continue;
            }
        };

        let http = http.clone();
        let tls_acceptor = tls_acceptor.clone();
        let svc = svc.clone();

        tokio::spawn(async move {
            let mut certificates = Vec::new();

            let conn = tls_acceptor
                .accept_with(conn, |info| {
                    if let Some(certs) = info.peer_certificates() {
                        for cert in certs {
                            certificates.push(cert.clone());
                        }
                    }
                })
                .await
                .unwrap();

            let svc = tower::ServiceBuilder::new()
                .add_extension(Arc::new(ConnInfo { addr, certificates }))
                .service(svc);

            http.serve_connection(conn, svc)
                .await
                .map_err(|e| {
                    eprintln!("Error serving connection: {}", e);
                    e
                })
                .unwrap();
        });
    }
}

#[derive(Debug)]
struct ConnInfo {
    addr: std::net::SocketAddr,
    certificates: Vec<Certificate>,
}
