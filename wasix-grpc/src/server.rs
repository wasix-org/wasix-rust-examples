use tonic::transport::{Identity, Server, ServerTlsConfig};
use tonic::{Request, Response, Status};

use hello_world::greeter_server::{Greeter, GreeterServer};
use hello_world::{HelloReply, HelloRequest};

use tokio::sync::mpsc;
use tokio_stream::{wrappers::ReceiverStream, Stream, StreamExt};

use std::error::Error;
use std::io::ErrorKind;
use std::pin::Pin;
use std::time::Duration;

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

    let cert = std::fs::read_to_string(data_dir.join("tls/server.pem"))?;
    let key = std::fs::read_to_string(data_dir.join("tls/server.key"))?;
    let identity = Identity::from_pem(cert, key);

    let addr = "127.0.0.1:50051".parse()?;
    eprintln!("Server listening on {}", addr);

    Server::builder()
        .tls_config(ServerTlsConfig::new().identity(identity))?
        .add_service(GreeterServer::new(MyGreeter::default()))
        .serve(addr)
        .await?;

    Ok(())
}
