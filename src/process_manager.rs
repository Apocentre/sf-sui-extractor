use std::{
  sync::{mpsc::sync_channel, Arc, Mutex}, panic, process, mem
};
use ctrlc;
use log::info;
use tokio::{
  spawn, sync::oneshot::{channel, Sender}, task::JoinHandle,
};
use crate::{args::Args, logger::Logger, runtime::FirehoseStreamer, sui::sui_node::SuiNode};


#[derive(Default)]
struct ProcessManagerInner {
  args: Args,
  tasks: Vec<Sender<()>>,
}

pub struct ProcessManager(Arc<Mutex<ProcessManagerInner>>);

impl ProcessManager {
  pub fn new(args: Args) -> Self {
    let pm = ProcessManagerInner {args, tasks: Vec::new()};

    ProcessManager(Arc::new(Mutex::new(pm)))
  }

  fn register_hooks(&self)  {
    let (tx, rx) = sync_channel(2);
    let tx_2 = tx.clone();
    let orig_hook = panic::take_hook();

    // this hook will be called if any of the threads panics
    panic::set_hook(Box::new(move |panic_info| {
      println!("Thread Panicked {:?}", panic_info);

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

  pub async fn start<L>(&self, logger: L)
  where
    L: Logger + Sync + Send + 'static
  {
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

    tasks.push(self.spawn_firehose_streamer::<L>(rpc_client_url, logger));
    self.register_hooks();
  }

  fn spawn_sui_node (&self) -> JoinHandle<()> {
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

  fn spawn_firehose_streamer<L>(&self, rpc_client_url: String, logger: L) -> JoinHandle<()>
  where
    L: Logger + Sync + Send + 'static
  {
    let pm = Arc::clone(&self.0);
    let pm = pm.lock().unwrap();
    let chain_id = pm.args.chain_id.clone();
    let starting_checkpoint_seq = pm.args.starting_checkpoint_seq.clone();

    spawn(async move {
      let mut fireshose_streamer = FirehoseStreamer::<L>::new(chain_id, rpc_client_url, starting_checkpoint_seq, logger);
      if let Err(e) = fireshose_streamer.start().await {
        panic!("{}", e);
      }
    })
  }

  pub fn kill_all(&self) {
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
