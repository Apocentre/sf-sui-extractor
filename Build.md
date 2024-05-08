# Cross Platform Build

Install `cross`

`cargo install cross --git https://github.com/cross-rs/cross`

## Linux

- `CROSS_CONTAINER_OPTS="--platform linux/x86-64" cross build --release --target x86_64-unknown-linux-gnu`
- `CROSS_CONTAINER_OPTS="--platform linux/amd64"  cross build --release --target aarch64-unknown-linux-gnu`

## MacOs

- `cross build --release --target x86_64-apple-darwin`
- `cross build --release --target aarch64-apple-darwin`
