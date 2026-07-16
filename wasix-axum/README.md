# AXUM WASIX STARTER

This repo contains a starter project for building a **Wasix** based axum web server.

## Compiling the project

```bash
$ cargo wasix build --release
```

## Running the project

```bash
$ cargo wasix run --release -W,--net
Listening on http://127.0.0.1:8080
```

> Note: `-W,` passes the comma-separated flags after it through to the Wasmer runtime — here `--net`, which enables networking.
>
> The server listens on the `PORT` environment variable and defaults to 8080 locally. On edge, set it to 80 because Wasmer edge expects the server to listen on port 80 and proxies requests to it.

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
