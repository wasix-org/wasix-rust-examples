# Example to build a proxy server with reqwest library

This example fetches the rust's foundation url using reqwest and proxies it to the client using axum.

## Usage

```bash
cargo wasix build
```

## Run

```bash
wasmer run target/wasm32-wasmer-wasi/debug/wasix-reqwest-proxy.wasm
```

## Deploy to wasmer edge

```
wasmer deploy
```

> Available app URL:
>
> https://wasix-reqwest-proxy-example.wasmer.app
