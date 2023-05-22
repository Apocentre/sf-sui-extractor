use std::{sync::{Arc, Mutex}, process::Command};
use ctrlc;

pub async fn start_sui_node(config_file_path: String) {
  tokio::spawn(async move {
    let child = Arc::new(
      Mutex::new(
        Command::new("sui-node")
        .arg(format!("--config-path={}", config_file_path))
        .spawn()
        .expect("start sui-node child process")
      )
    );

    let child_clone = Arc::clone(&child);
    ctrlc::set_handler(move|| {
      let mut child = child_clone.lock().unwrap();

      child.kill().expect("kill sui-node");
    }).unwrap();

    let mut child = child.lock().unwrap();
    if let Ok(status) = child.wait() {
      panic!("result: {status:?}");
    }
  });
}
