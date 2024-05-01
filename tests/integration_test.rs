use simple_home_dir::*;
use tokio::{
  spawn, sync::oneshot::channel
};
use sui_sf_indexer::{args::Args, logger::Logger, process_manager::ProcessManager};

#[tokio::test(flavor = "multi_thread", worker_threads = 4)]
async fn test_raw_data_printed_in_stdout() {
  let (tx, rx) = channel::<&str>();
  struct TestLogger;
  
  impl Logger for TestLogger {
    fn log(msg: &str) {
      let closure = move || {
        tx.send(msg).unwrap();
      };

      closure();
    }
  }

  fn sui_config_path() -> String {
    format!("{}/.sf_sui/sui_config/full_node.yaml", home_dir().unwrap().display().to_string())
  }

  let args = Args {
    sui_node_config: sui_config_path(),
    chain_id: "4btiuiMPvEENsttpZC7CZ53DruC3MAgfznDbASZ7DR6S".to_string(),
    starting_checkpoint_seq: 1948619,
    rpc_client_url: None
  };
  
  let mut pm = ProcessManager::new(args);

  spawn(async move {

  });

  pm.start::<TestLogger>().await;

  pm.kill_all();
}
