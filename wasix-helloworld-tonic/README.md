# WASIX + gRPC

This example is compiled from the sample on tonic's official github repo.

## Compiling the application

```shell
cargo wasix build
```

## Running the application

### Server

```shell
$ wasmer run target/wasm32-wasmer-wasi/debug/helloworld-client.wasm --net
Server listening on 127.0.0.1:50051
```

### client

> Note: The client works only for IPv4 addresses.

```shell
$ wasmer run target/wasm32-wasmer-wasi/debug/helloworld-server.wasm --net

RESPONSE=Response { metadata: MetadataMap { headers: {"content-type": "application/grpc", "date": "Sat, 12 Aug 2023 23:48:29 GMT", "grpc-status": "0"} }, message: HelloReply { message: "Hello Tonic!" }, extensions: Extensions }
```

Or you can use `grpcurl` to verify the server:

```shell
$ grpcurl -plaintext -import-path ./proto -proto helloworld.proto -d '{"name": "Tonic"}' '[::1]:50051' helloworld.Greeter/SayHello
{
  "message": "Hello Tonic!"
}
```
