use std::{panic, process};
use env_logger::Env;
use clap::{Parser};
use eyre::Result;
use sui_sf_indexer::{
  runtime::FirehoseStreamer, sui_node,
};

#[derive(Debug, Parser)]
struct Args {
  /// The fullnode config file
  #[arg(short = 'c', long)]
  sui_node_config: String,

  /// Chain Identifier is the digest of the genesis checkpoint
  #[arg(short = 'i', long, default_value = "4btiuiMPvEENsttpZC7CZ53DruC3MAgfznDbASZ7DR6S")]
  chain_id: String,

  /// Which checkount should we start streaming data from
  #[arg(short = 's', long, default_value_t = 0)]
  starting_checkpoint_seq: u64,

  /// You can use https://fullnode.mainnet.sui.io:443 for mainnet
  /// Note that if one is not provided, a local sui-node will be spinned up instead
  #[arg(short = 'r', long)]
  rpc_client_url: Option<String>,
}

#[tokio::main]
async fn main() -> Result<()> {
  let orig_hook = panic::take_hook();
  panic::set_hook(Box::new(move |panic_info| {
    orig_hook(panic_info);
    process::exit(1);
  }));
  
  env_logger::Builder::from_env(Env::default().default_filter_or("info")).init();

  let Args {
    sui_node_config,
    chain_id,
    rpc_client_url,
    starting_checkpoint_seq
  } = Args::parse();

  // If no rpc url provided the we need to start a local sui-node
  let rpc_client_url = if let Some(rpc_client_url) = rpc_client_url {
    rpc_client_url
  } else {
    // Start local sui node
    sui_node::start_sui_node(sui_node_config).await;
    "http://127.0.0.1:9000".to_string()
  };

  let mut fireshose_streamer = FirehoseStreamer::new(chain_id, rpc_client_url, starting_checkpoint_seq);

  if let Err(e) = fireshose_streamer.start().await {
    panic!("{:?}", e);
  }

  Ok(())
}
