# A web scraper for news headlines on Hacker News

This project is a web scraper for news headlines on Hacker News. It is written in Rust and uses the `reqwest` and `scraper` crates.

The objective of this project is to learn compilation and deployment of Rust projects to WASIX and Wasmer Edge.

## Usage

### Pre-requisites

You must have the following installed on your machine:

- [Rust](https://rustup.rs/)
- [Wasmer](https://docs.wasmer.io/install)
- [cargo-wasix](https://crates.io/crates/cargo-wasix)

### Running the project

#### Running using Wasmer Registry

To run the project from the `wasmer/news-scraper` package you can simply do:

```shell
$ wasmer run wasmer/news-scraper --net
```

#### Running locally

To run the project, clone the repository and run the following command:

```bash
cargo wasix build --release
```

This will compile the project to WASIX and output the compiled binary to `target/wasm32-wasmer-wasi/release/news-scraper.wasm`.

To run the compiled binary, run the following command:

```bash
wasmer run target/wasm32-wasmer-wasi/release/news-scraper.wasm --net
```

This will run the compiled binary on port 8080. You can then access the web scraper at `http://localhost:8080` using curl or a web browser.

## Deployment

To deploy the project on wasmer edge simply do:

```bash
wasmer deploy
```

> Note: you must change the configurations in `wasmer.toml` and `app.yaml` to match your own wasmer profile.

You can also follow the full tutorial for this project [here](/todo/).

### Access Link

You can also access the deployed project and read the news from your terminal.

```bash
curl https://news-scraper.wasmer.app
```
