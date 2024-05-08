use std::{marker::PhantomData, panic, sync::{mpsc::{sync_channel, SyncSender}, Arc, Mutex}};
use eyre::{eyre, ensure, Result};
use simple_home_dir::*;
use tokio::spawn;
use sui_sf_indexer::{args::Args, logger::Logger, process_manager::ProcessManager};
use sysinfo::System;

struct InitState;
struct BlockStartState;
struct BlockEndState;
struct CheckpointState;
struct TransactionState;
struct ObjectChangeState;
struct EventState;
struct DisplayUpdateState;

trait LineValidation {
  fn validate(&self, line: &str) -> Result<Box<dyn LineValidation>>;
  fn full_cycle(&self) -> bool {false}
}

struct Test <State = InitState> {
  state: PhantomData<State>,
}

struct TestLogger {
  tx: SyncSender<String>,
}

impl Logger for TestLogger {
  fn log(&self, msg: &str) {
    let _ = self.tx.send(msg.to_string());
  }
}

impl LineValidation for Test<InitState> {
  fn validate(&self, line: &str) -> Result<Box<dyn LineValidation>> {
    ensure!(line.eq("INIT"), "expected INIT");
    Ok(Box::new(Test::<BlockStartState> {state: PhantomData}))
  }
}

impl LineValidation for Test<BlockStartState> {
  fn validate(&self, line: &str) -> Result<Box<dyn LineValidation>> {
    ensure!(line.eq("BLOCK_START"), "expected BLOCK_START");
    Ok(Box::new(Test::<CheckpointState> {state: PhantomData}))
  }
}

impl LineValidation for Test<CheckpointState> {
  fn validate(&self, line: &str) -> Result<Box<dyn LineValidation>> {
    ensure!(line.eq("CHECKPOINT"), "expected CHECKPOINT");
    Ok(Box::new(Test::<TransactionState> {state: PhantomData}))
  }
}

impl LineValidation for Test<TransactionState> {
  fn validate(&self, line: &str) -> Result<Box<dyn LineValidation>> {
    if line.eq("TRX") {
      Ok(Box::new(Test::<TransactionState> {state: PhantomData}))
    } else if line.eq("OBJ_CHANGE") {
      Ok(Box::new(Test::<ObjectChangeState> {state: PhantomData}))
    } else if line.eq("EVT") {
      Ok(Box::new(Test::<EventState> {state: PhantomData}))
    } else if line.eq("DSP_UPDATE") {
      Ok(Box::new(Test::<DisplayUpdateState> {state: PhantomData}))
    } else if line.eq("BLOCK_END") {
      Ok(Box::new(Test::<BlockEndState> {state: PhantomData}))
    } else {
      Err(eyre!("expected either TRX or BLOCK_END"))
    }
  }
}

impl LineValidation for Test<ObjectChangeState> {
  fn validate(&self, line: &str) -> Result<Box<dyn LineValidation>> {
    if line.eq("OBJ_CHANGE") {
      Ok(Box::new(Test::<ObjectChangeState> {state: PhantomData}))
    } else if line.eq("EVT") {
      Ok(Box::new(Test::<EventState> {state: PhantomData}))
    } else if line.eq("DSP_UPDATE") {
      Ok(Box::new(Test::<DisplayUpdateState> {state: PhantomData}))
    } else if line.eq("BLOCK_END") {
      Ok(Box::new(Test::<BlockEndState> {state: PhantomData}))
    } else {
      Err(eyre!("expected either OBJ_CHANGE, EVT, DSP_UPDATE or BLOCK_END"))
    }
  }
}

impl LineValidation for Test<EventState> {
  fn validate(&self, line: &str) -> Result<Box<dyn LineValidation>> {
    if line.eq("EVT") {
      Ok(Box::new(Test::<EventState> {state: PhantomData}))
    } else if line.eq("DSP_UPDATE") {
      Ok(Box::new(Test::<DisplayUpdateState> {state: PhantomData}))
    } else if line.eq("BLOCK_END") {
      Ok(Box::new(Test::<BlockEndState> {state: PhantomData}))
    } else {
      Err(eyre!("expected either EVT, DSP_UPDATE or BLOCK_END"))
    }
  }
}

impl LineValidation for Test<DisplayUpdateState> {
  fn validate(&self, line: &str) -> Result<Box<dyn LineValidation>> {
    if line.eq("BLOCK_START") {
      Ok(Box::new(Test::<BlockStartState> {state: PhantomData}))
    } else {
      Err(eyre!("expected BLOCK_START"))
    }
  }
}

impl LineValidation for Test<BlockEndState> {
  fn validate(&self, line: &str) -> Result<Box<dyn LineValidation>> {
    ensure!(line.eq("BLOCK_START"), "expected BLOCK_START");
    Ok(Box::new(Test::<CheckpointState> {state: PhantomData}))
  }

  fn full_cycle(&self) -> bool {true}
}

#[tokio::test(flavor = "multi_thread", worker_threads = 4)]
async fn test_raw_data_printed_in_stdout() {
  let cycles = Arc::new(Mutex::new(0));
  let cycles_clone = cycles.clone();

  panic::set_hook(Box::new(move |_| {
    let lock = cycles_clone.lock().unwrap();
    assert_eq!(*lock, 2);

    println!(">>>>>>>>>>>>>>> Test Passes <<<<<<<<<<<<<<<<");
    std::process::exit(0);
  }));

  fn sui_config_path() -> String {
    format!("{}/.sf_sui/sui_config/full_node.yaml", home_dir().unwrap().display().to_string())
  }

  let args = Args {
    sui_node_config: sui_config_path(),
    chain_id: "4btiuiMPvEENsttpZC7CZ53DruC3MAgfznDbASZ7DR6S".to_string(),
    starting_checkpoint_seq: 1948619,
    rpc_client_url: None
  };
  
  let pm = ProcessManager::new(args);

  let (tx, rx) = sync_channel::<String>(10);
  let test_logger = TestLogger {tx};

  let cycles_clone = cycles.clone();
  spawn(async move {
    let mut test: Box<dyn LineValidation> = Box::new(Test::<InitState>{state: PhantomData});

    while let Ok(line) = rx.recv() {
      let parts = line.split(" ").collect::<Vec<_>>();
      // First line must start with FIRE BLOCK_START
      // Followed by FIRE CHECKPOINT
      // Followed by 0 or more FIRE TRX
      // Followed by 0 or more FIRE OBJ_CHANGE
      // Followed by 0 or more FIRE EVT
      // Followed by 0 or more FIRE DSP_UPDATE
      // Followed by FIRE BLOCK_END
      let result = test.validate(&parts[1]);
      assert!(result.is_ok());
      test = result.unwrap();

      if test.full_cycle() {
        let mut lock = cycles_clone.lock().unwrap();
        *lock += 1;
        println!("Full cycle {lock:?}");

        if *lock == 2 {
          let sys = System::new_all();
          for (_, process) in sys.processes() {
            if process.name().eq("sui-node") {
              process.kill();
            }
          }

          break;     
        }
      }
    }
  });

  pm.start(test_logger).await;
}
