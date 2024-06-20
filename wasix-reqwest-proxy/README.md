# Example to build a proxy server with reqwest library

This example fetches the rust's foundation url using reqwest and proxies it to the client using axum.

## Usage

```bash
cargo wasix build
```

## Run

```bash
wasmer run . --net
```

## Deploy to wasmer edge

```
wasmer deploy
```

> Available app URL:
>
> https://wasix-reqwest-proxy-example.wasmer.app
