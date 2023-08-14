# AXUM WASIX STARTER

This repo contains a starter project for building a **Wasix** based axum web server.

## Compiling the project

```bash
$ cargo wasix build
```

## Running the project

```bash
$ wasmer run target/wasm32-wasmer-wasi/release/wasix-axum.wasm --net --env PORT=8080
Listening on http://127.0.0.1:8080
```

> Note: The `PORT` environment variable is required to run the server locally. On edge this defaults to 80 because Wasmer edge expects the server to listen on port 80 and it will proxy the request to the server.

## Deploy to Wasmer Edge

1. Login to `wasmer-cli`

```bash
$ wasmer login
```

2. Replace `wasix-org` username in `wasmer.toml` and `deploy.toml` with your username
   > Note: `wasmer.toml` is for publishing the image to **wasmer registry** and `deploy.toml` is for deploying the image to **wasmer-edge**
3. Deploy to `wasmer-edge`

```bash
$ wasmer deploy
```

Checkout the full tutorial [here](http://wasix.org/docs/language-guide/rust/tutorials/wasix-axum)

This project is available on wasmer edge:

> Available on https://wasix-axum-example.wasmer.app
