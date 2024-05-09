# Cross Platform Build

Install `cross`

`cargo install cross --git https://github.com/cross-rs/cross`

## Linux

Install the following tools

https://github.com/messense/homebrew-macos-cross-toolchains

then add new target

`rustup target add x86_64-unknown-linux-gnu`
 
 and then 

`cargo build --release --target x86_64-unknown-linux-gnu`

For more information read this article https://betterprogramming.pub/cross-compiling-rust-from-mac-to-linux-7fad5a454ab1

## MacOs

- `cross build --release --target x86_64-apple-darwin`
- `cross build --release --target aarch64-apple-darwin`
