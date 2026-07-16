# WASIX Subprocesses

This example shows how to spawn subprocesses in WASIX. This would be primarily used in conjunction with other modules that can be loaded in using the `--use` flag.

## Usage

### Compiling to WASIX

```shell
cargo wasix build
```

### Running the application

```shell
cargo wasix run -W,--use,sharrattj/coreutils
```

> `-W,` passes the comma-separated flags after it through to the Wasmer runtime — here `--use`, which loads the coreutils commands the example spawns.

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
