use std::{panic, process};
use env_logger::Env;
use clap::{Parser};
use eyre::Result;
use sui_sf_indexer::runtime::FirehoseStreamer;

#[derive(Debug, Parser)]
struct Args {
  #[arg(short = 'c', long, default_value = "mainnet")]
  chain_id: String,

  #[arg(short = 'r', long, default_value = "https://fullnode.mainnet.sui.io:443")]
  rpc_client_url: String,

  #[arg(short = 's', long, default_value_t = 0)]
  starting_checkpoint_seq: u64,
}


#[tokio::main]
async fn main() -> Result<()> {
  let orig_hook = panic::take_hook();
  panic::set_hook(Box::new(move |panic_info| {
    orig_hook(panic_info);
    process::exit(1);
  }));
  
  env_logger::Builder::from_env(Env::default().default_filter_or("info")).init();

  let Args {chain_id, rpc_client_url, starting_checkpoint_seq} = Args::parse();
  let mut fireshose_streamer = FirehoseStreamer::new(chain_id, rpc_client_url, starting_checkpoint_seq);
  
  fireshose_streamer.start().await?;

  Ok(())
}
