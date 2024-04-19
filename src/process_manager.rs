use std::{
  sync::{mpsc::sync_channel, Arc, Mutex}, panic, process, mem
};
use ctrlc;
use log::info;
use tokio::{
  spawn, sync::oneshot::{channel, Sender}, task::JoinHandle,
};
use clap::Parser;
use crate::{sui::sui_node::SuiNode, runtime::FirehoseStreamer};

#[derive(Default, Debug, Parser)]
struct Args {
  /// The fullnode config file
  #[arg(short = 'c', long)]
  sui_node_config: String,

  /// Chain Identifier is the digest of the genesis checkpoint
  #[arg(short = 'i', long, default_value = "4btiuiMPvEENsttpZC7CZ53DruC3MAgfznDbASZ7DR6S")]
  chain_id: String,

  /// Which checkpoint should we start streaming data from
  #[arg(short = 's', long, default_value_t = 0)]
  starting_checkpoint_seq: u64,

  /// You can use https://fullnode.mainnet.sui.io:443 for mainnet
  /// Note that if one is not provided, a local sui-node will be spinned up instead
  #[arg(short = 'r', long)]
  rpc_client_url: Option<String>,
}

#[derive(Default)]
struct ProcessManagerInner {
  args: Args,
  tasks: Vec<Sender<()>>,
}

pub struct ProcessManager(Arc<Mutex<ProcessManagerInner>>);

impl ProcessManager {
  pub fn new() -> Self {
    let args = Args::parse();
    let pm = ProcessManagerInner {args, tasks: Vec::new(),};

    ProcessManager(Arc::new(Mutex::new(pm)))
  }

  fn register_hooks(&mut self)  {
    let (tx, rx) = sync_channel(2);
    let tx_2 = tx.clone();
    let orig_hook = panic::take_hook();

    // this hook will be called if any of the threads panics
    panic::set_hook(Box::new(move |panic_info| {
      tx_2.send(()).expect("send msg");
      orig_hook(panic_info);
    }));

    let tx_3 = tx.clone();
    ctrlc::set_handler(move || {
      tx_3.send(()).expect("send msg");
    }).unwrap();

    let _ = rx.recv();
    self.kill_all();
  }

  pub async fn start(&mut self) {
    let mut tasks = vec![];

    let pm = Arc::clone(&self.0);
    let rpc_client_url = pm.lock().unwrap().args.rpc_client_url.clone();

    // If no rpc url provided the we need to start a local sui-node
    let rpc_client_url = if let Some(rpc_client_url) = rpc_client_url {
      rpc_client_url.clone()
    } else {
      tasks.push(self.spawn_sui_node());
      "http://127.0.0.1:9000".to_string()
    };

    tasks.push(self.spawn_firehose_streamer(rpc_client_url));
    self.register_hooks();
  }

  fn spawn_sui_node (&mut self) -> JoinHandle<()> {
    let (tx, rx) = channel();
    let pm = Arc::clone(&self.0);
    let mut pm = pm.lock().unwrap();
    let sui_config = pm.args.sui_node_config.clone();

    pm.tasks.push(tx);

    spawn(async move {
      let sui_node = SuiNode::new(sui_config);
      sui_node.start(rx).await;
    })
  }

  fn spawn_firehose_streamer(&mut self, rpc_client_url: String) -> JoinHandle<()> {
    let pm = Arc::clone(&self.0);
    let pm = pm.lock().unwrap();
    let chain_id = pm.args.chain_id.clone();
    let starting_checkpoint_seq = pm.args.starting_checkpoint_seq.clone();

    spawn(async move {
      let mut fireshose_streamer = FirehoseStreamer::new(chain_id, rpc_client_url, starting_checkpoint_seq);
      if let Err(e) = fireshose_streamer.start().await {
        panic!("{}", e);
      }
    })
  }

  pub fn kill_all(&mut self) {
    info!("Killing all processes and exiting");

    let pm = Arc::clone(&self.0);
    let mut pm = pm.lock().unwrap();
    let tasks = mem::take(&mut pm.tasks);

    for task in tasks {
      task.send(()).expect("send task termination messages");
    }

    process::exit(1);
  }
}
