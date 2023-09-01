# WASIX - NATS

## Description

This project uses the sample code from the [async_nats](https://docs.rs/async-nats/latest/async_nats/#complete-example) crate to create a NATS client that can be used to publish and subscribe to messages.

The NATS client is created using the `async_nats::connect` function. This function returns a `async_nats::Connection` object that can be used to publish and subscribe to messages.

- `wasix.messages.rust` is the subject used to publish messages aka. `PUBLISH_SUBJECT`.
- `wasix.messages.cli` is the subject used to receive messages aka. `SUBSCRIBE_SUBJECT`.

## Compiling to WASIX

```shell
$ cargo wasix build --release
```

## Running the NATS server

You can use the file `nats-server.conf` to run the NATS server.

```shell
$ nats-server -c nats-server.conf
```

## Running the WASM module

```shell
$ wasix run target/wasm32-wasi/release/wasix-nats.wasm --net
```

### Output

> use the nats cli for publishing and subscribing to channels

#### Published messages

> you can use the nats cli to publish messages to the `wasix.messages.cli` subject

```shell
$ nats publish wasix.messages.cli "Hello from the CLI"
10:57:18 Published 18 bytes to "wasix.messages.cli"
```

> you should see the message in the output of the WASM module

```shell
Received message Message { subject: "wasix.messages.cli", reply: None, payload: b"Hello from the CLI", headers: None, status: None, description: None, length: 36 }
```

#### Subscribed messages

> you can use the nats cli to subscribe to the `wasix.messages.rust` subject

```shell
$ nats subscribe wasix.messages.rust
10:58:47 Subscribing on wasix.messages.rust
[#1] Received on "wasix.messages.rust"
Hello - 0

[#2] Received on "wasix.messages.rust"
Hello - 1

[#3] Received on "wasix.messages.rust"
Hello - 2

[#4] Received on "wasix.messages.rust"
Hello - 3

[#5] Received on "wasix.messages.rust"
Hello - 4

[#6] Received on "wasix.messages.rust"
Hello - 5

[#7] Received on "wasix.messages.rust"
Hello - 6

[#8] Received on "wasix.messages.rust"
Hello - 7

[#9] Received on "wasix.messages.rust"
Hello - 8

[#10] Received on "wasix.messages.rust"
Hello - 9
```

> You should run this command before starting your wasm module
