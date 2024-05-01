use simple_home_dir::*;
use clap::Parser;

fn sui_config_path() -> String {
  format!("{}/.sf_sui/sui_config/full_node.yaml", home_dir().unwrap().display().to_string())
}

#[derive(Default, Debug, Parser)]
pub struct Args {
  /// The fullnode config file
  #[arg(short = 'c', long, default_value_t = sui_config_path())]
  pub sui_node_config: String,

  /// Chain Identifier is the digest of the genesis checkpoint
  #[arg(short = 'i', long, default_value = "4btiuiMPvEENsttpZC7CZ53DruC3MAgfznDbASZ7DR6S")]
  pub chain_id: String,

  /// Which checkpoint should we start streaming data from
  #[arg(short = 's', long, default_value_t = 1)]
  pub starting_checkpoint_seq: u64,

  /// You can use https://fullnode.mainnet.sui.io:443 for mainnet
  /// Note that if one is not provided, a local sui-node will be spinned up instead
  #[arg(short = 'r', long)]
  pub rpc_client_url: Option<String>,
}
