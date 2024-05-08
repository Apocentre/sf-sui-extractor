use log::info;
use tokio::{
  process::Command, sync::oneshot::Receiver,
};

pub struct SuiNode {
  config_path: String,
}

impl SuiNode {
  pub fn new(config_path: String) -> Self {
    Self {config_path}
  }

  pub async fn start(&self, rx: Receiver<()>) {
    let mut child = Command::new("sui-node")
    .arg(format!("--config-path={}", self.config_path))
    .spawn()
    .expect("start sui-node child process");

    tokio::select! {
      status = child.wait() => {
        panic!("Sui Node exited: {status:?}");
      }
      _ = rx => {
        info!("Killing Sui Node");
        child.kill().await.expect("kill sui-node")
      },
    };
  }
}
