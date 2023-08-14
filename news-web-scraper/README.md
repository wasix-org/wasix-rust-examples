# A web scraper for news headlines on Hacker News

This project is a web scraper for news headlines on Hacker News. It is written in Rust and uses the `reqwest` and `scraper` crates.

The objective of this project is to learn compilation and deployment of Rust projects to WASIX and Wasmer Edge.

## Usage

To run the project, you must have Rust installed. You can install Rust using [rustup](https://rustup.rs/).

To run the project, clone the repository and run the following command:

```bash
cargo wasix build --release
```

This will compile the project to WASIX and output the compiled binary to `target/wasm32-wasmer-wasi/release/news-scraper.wasm`.

To run the compiled binary, run the following command:

```bash
wasmer run target/wasm32-wasmer-wasi/release/news-scraper.wasm --net --env PORT=3000
```

This will run the compiled binary on port 3000. You can then access the web scraper at `http://localhost:3000` using curl or a web browser.

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
