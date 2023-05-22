use std::{panic, process, mem};
use ctrlc;
use tokio::{
  spawn, sync::{
    oneshot::{channel, Sender}, mpsc,
  },
};
use clap::{Parser};
use crate::{sui_node::SuiNode, runtime::FirehoseStreamer};

#[derive(Default, Debug, Parser)]
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

#[derive(Default)]
pub struct ProcessManager {
  args: Args,
  tasks: Vec<Sender<()>>,
}

impl ProcessManager {
  pub fn new() -> ProcessManager {
    let args = Args::parse();

    ProcessManager {
      args,
      tasks: vec![],
    }
  }

  fn register_hooks(&mut self)  {
    let pm = mem::take(self);
    
    spawn(async move {
      let (tx, mut rx) = mpsc::channel(2);

      let tx_2 = tx.clone();
      let orig_hook = panic::take_hook();

      // this hook will be called if any of the threads panics
      panic::set_hook(Box::new(move |panic_info| {
        tx_2.blocking_send(()).expect("send msg");
        orig_hook(panic_info);
        std::thread::sleep(std::time::Duration::from_secs(1000));
      }));

      let tx_3 = tx.clone();
      ctrlc::set_handler(move || {
        tx_3.blocking_send(()).expect("send msg");
      }).unwrap();

      loop {
        let _ = rx.recv().await;
        pm.kill_all();
        break;
      }
    });
  }

  pub fn start(&mut self) {
    self.register_hooks();

    // If no rpc url provided the we need to start a local sui-node
    let rpc_client_url = if let Some(rpc_client_url) = &self.args.rpc_client_url {
      rpc_client_url.clone()
    } else {
      self.spawn_sui_node();
      "http://127.0.0.1:9000".to_string()
    };

    self.spawn_firehose_streamer(rpc_client_url);
    self.register_hooks();
  }

  fn spawn_sui_node (&mut self) {
    let (tx, rx) = channel();
    self.tasks.push(tx);
    let sui_config = self.args.sui_node_config.clone();

    spawn(async move {
      let sui_node = SuiNode::new(sui_config);
      sui_node.start(rx).await;
    });
  }

  fn spawn_firehose_streamer(&mut self, rpc_client_url: String) {
    let chain_id = self.args.chain_id.clone();
    let starting_checkpoint_seq = self.args.starting_checkpoint_seq.clone();

    spawn(async move {
      let mut fireshose_streamer = FirehoseStreamer::new(chain_id, rpc_client_url, starting_checkpoint_seq);
      if let Err(e) = fireshose_streamer.start().await {
        panic!("{}", e);
      }
    });
  }

  pub fn kill_all(self) {
    for task in self.tasks {
      task.send(()).expect("send task termination messages");
    }

    process::exit(1);
  }

  // pub async fn start_sui_node(&mut self, sui_node_config: String) {
  //   let sui_node_process = sui_node::start_sui_node(sui_node_config).await;
  //   self.children.push(sui_node_process);
  // }

  // pub fn kill_all(&mut self) -> Result<()> {
  //   for child in &self.children {
  //     let mut child = child.lock().unwrap();
  //     child.kill()?;
  //   }

  //   process::exit(1);

  //   Ok(())
  // }
}
