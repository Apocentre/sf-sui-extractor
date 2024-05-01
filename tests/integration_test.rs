use std::sync::mpsc::{sync_channel, SyncSender};

use simple_home_dir::*;
use tokio::spawn;
use sui_sf_indexer::{args::Args, logger::Logger, process_manager::ProcessManager};

#[tokio::test(flavor = "multi_thread", worker_threads = 4)]
async fn test_raw_data_printed_in_stdout() {
  struct TestLogger {
    tx: SyncSender<String>,
  }
  
  impl Logger for TestLogger {
    fn log(&self, msg: &str) {
      self.tx.send(msg.to_string()).unwrap();
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

  let (tx, rx) = sync_channel::<String>(2);
  let test_logger = TestLogger {tx};

  spawn(async move {
    while let Ok(line) = rx.recv() {
      println!(">>>>>>> {line}");
    }  
  });

  pm.start(test_logger).await;

  pm.kill_all();
}
