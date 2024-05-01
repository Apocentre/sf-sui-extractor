use clap::Parser;
use env_logger::Env;
use eyre::Result;
use sui_sf_indexer::{args::Args, process_manager::ProcessManager, logger::StdoutLogger};

#[tokio::main]
async fn main() -> Result<()> {
  env_logger::Builder::from_env(Env::default().default_filter_or("info")).init();

  let args = Args::parse();
  let mut pm = ProcessManager::new(args);
  pm.start::<StdoutLogger>().await;

  Ok(())
}
