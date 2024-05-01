use sui_sf_indexer::{args::Args, process_manager::ProcessManager};

#[tokio::test]
async fn test_raw_data_printed_in_stdout() {
  let args = Args {
    sui_node_config: todo!(),
    chain_id: "4btiuiMPvEENsttpZC7CZ53DruC3MAgfznDbASZ7DR6S".to_string(),
    starting_checkpoint_seq: todo!(),
    rpc_client_url: todo!()
  };
  
  let mut pm = ProcessManager::new(args);
  pm.start().await;

  pm.kill_all();
}
