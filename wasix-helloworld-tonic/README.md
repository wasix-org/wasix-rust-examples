# WASIX + gRPC

This example is compiled from the sample on tonic's official github repo.

## Compiling the application

```shell
cargo wasix build
```

## Running the application

### Server

```shell
wasmer run target/wasm32-wasmer-wasi/debug/helloworld-client.wasm --net
```

### client


> ⚠️
> Note: The client shows an error for now but the server is fully compatiable.

```
wasmer run target/wasm32-wasmer-wasi/debug/helloworld-server.wasm --net
```

Meanwhile you can use `grpcurl` to verify the server:

```shell
grpcurl -plaintext -import-path ./proto -proto helloworld.proto -d '{"name": "Tonic"}' '[::1]:50051' helloworld.Greeter/SayHello
{
  "message": "Hello Tonic!"
}
```

