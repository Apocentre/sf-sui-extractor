use env_logger::Env;
use eyre::Result;
use sui_sf_indexer::{
  process_manager::ProcessManager,
};

#[tokio::main]
async fn main() -> Result<()> {
  env_logger::Builder::from_env(Env::default().default_filter_or("info")).init();

  let pm = Box::leak(Box::new(ProcessManager::new()));
  pm.start().await;

  Ok(())
}
