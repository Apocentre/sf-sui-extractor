Sui StreamingFast Indexer
===

This module provides a StreamingFast Firehose Streamer for pushing protobuf data from the Sui Blockchain

Download `sui-node`
===

1. Download the latest mainnet version of the sui node from [here](https://github.com/MystenLabs/sui/releases).
2. Extract the zip file and move the `sui-node-macos-arm64` to a folder that is within your system PATH.

```bash
mv sui-node-macos-arm64 ~/.sui/sui-node
```

> Note that yu should add `~/.sui/` to your system PATH.


Alternatively, you can build it from source. Note it might take some time until the build is completed.

```bash
git clone https://github.com/MystenLabs/sui
cargo build --release --bin sui-node
```
