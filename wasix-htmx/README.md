# WASIX - HTMX Example

## Overview

This is a simple example of using [htmx](https://htmx.org/) with [axum](https://github.com/tokio-rs/axum) and [askama](https://github.com/djc/askama) to create a simple web application.

This example was inspired by [this example](https://github.com/JoeyMckenzie/axum-htmx-templates/tree/main) by [JoeyMckenzie](https://github.com/JoeyMckenzie).

## Pre-requisites

- [Rust](https://rustup.rs/)
- [`cargo-wasix`](https://github.com/wasix-org/cargo-wasix)
- [`wasmer`](https://wasmer.io/)

## Local Development

### Building tailwind css

```shell
$ cargo make styles
```

> This runs a watch task that will rebuild the css when the `styles/tailwind.css` file is changed.

### Running the Development Server

```shell
$ cargo make run
```

> This will run a daemon that will rebuild the project when the source code is changed.

## Building

To run this example, you need to have [Rust] installed.

```shell
$ cargo wasix build --release
```

## Running

### Using source code

```shell
$ wasmer run target/wasm32-wasmer-wasi/release/wasix-htmx.wasm --net --mapdir /assets:./assets
```

### Using Published Package

```shell
$ wasmer run wasmer/wasix-htmx --net
```

Now you can open your browser and go to http://127.0.0.1:8080 to access the website.

This package is available on [wasmer registry](https://wasmer.io/wasmer/wasix-htmx) as `wasmer/wasix-htmx`.
