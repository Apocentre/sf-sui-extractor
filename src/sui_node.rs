use std::{process::{Command, Child}};

pub struct ChildGuard(Child);

impl Drop for ChildGuard {
  fn drop(&mut self) {
    match self.0.kill() {
      Err(e) => println!("Could not kill child process: {}", e),
      Ok(_) => println!("Successfully killed child process"),
    }
  }
}

pub async fn start_sui_node(config_file_path: String) {
  tokio::spawn(async move {
    let mut child = Command::new("sui-node")
    .arg(format!("--config-path={}", config_file_path))
    .spawn()
    .expect("start sui-node child process");
  
    if let Ok(status) = child.wait() {
      panic!("result: {status:?}");
    }
  });
}
