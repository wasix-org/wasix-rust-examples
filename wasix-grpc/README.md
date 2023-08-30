# WASIX with gRPC

This example demonstrates how to use gRPC in WASIX to create a server that can be accessed from a gRPC client. This example also showcases how to use custom https certificates with WASIX.

## Building

```bash
$ cargo wasix build --release
```

## Running the server

```bash
$ wasmer run target/wasm32-wasmer-wasi/release/helloworld-server.wasm --net --mapdir /tls:./tls
```

## Running the client

```bash
$ wasmer run target/wasm32-wasmer-wasi/release/helloworld-client.wasm --net --mapdir /tls:./tls
```

You can also follow this example as a tutorial on the [WASIX website](http://wasix.org/docs/language-guide/rust/tutorials/wasix-grpc)
