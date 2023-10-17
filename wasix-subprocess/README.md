# WASIX Subprocesses

This example shows how to spawn subprocesses in WASIX. This would be primarily used in conjunction with other modules that can be loaded in using the `--use` flag.

## Usage

### Compiling to WASIX

```shell
cargo wasix build
```

### Running the application

```shell
wasmer run ./target/wasm32-wasmer-wasi/debug/wasix-subprocess.wasm --use sharrattj/coreutils
```

or

```shell
wasmer run .
```

> The above command works using the `wasmer.toml` and the one above that is more explicit.

#### Output

```shell
status: exit code: 0
stdout: ---------- 1 somebody somegroup 0 Jan  1  1970 bin
---------- 1 somebody somegroup 0 Jan  1  1970 dev
---------- 1 somebody somegroup 0 Jan  1  1970 etc
---------- 1 somebody somegroup 0 Jan  1  1970 tmp
---------- 1 somebody somegroup 0 Jan  1  1970 usr

stderr:
```

> This application is published on wasmer registry as wasmer/wasix-subprocess. You can run it with `wasmer run wasmer/wasix-subprocess-example`
