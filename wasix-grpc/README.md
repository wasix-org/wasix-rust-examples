# WASIX with gRPC

This example demonstrates how to use gRPC in WASIX to create a server that can be accessed from a gRPC client. This example also showcases how to use custom https certificates with WASIX.

## Building

```bash
$ cargo wasix build --release
```

## Running the server

```bash
$ cargo wasix run --release --bin helloworld-server -W,--net,--volume,./tls:/tls
```

## Running the client

```bash
$ cargo wasix run --release --bin helloworld-client -W,--net,--volume,./tls:/tls
```

You can also follow this example as a tutorial on the [WASIX website](http://wasix.org/docs/language-guide/rust/tutorials/wasix-grpc)
