use std::time::Duration;
use eyre::{Result, Report};
use jsonrpsee::http_client::{HeaderMap, HeaderValue, HttpClient, HttpClientBuilder};
use backoff::{ExponentialBackoff, future::retry};
use prost::Message;
use log::{error, debug};
use sui_json_rpc::{CLIENT_SDK_TYPE_HEADER};
use tokio::time::{sleep};
use crate::{
  checkpoint_handler::CheckpointHandler, pb::sui::checkpoint as pb,
  convert::{
    tx::convert_transaction, object::convert_object_change,
  },
};

pub struct FirehoseStreamer {
  rpc_client_url: String,
  chain_id: String,
  checkpoint_handler: Option<CheckpointHandler>,
  pub current_checkpoint_seq: u64,
}

impl FirehoseStreamer {
  pub fn new(chain_id: String, rpc_client_url: String, starting_checkpoint_seq: u64) -> Self {
    Self {
      rpc_client_url,
      chain_id,
      current_checkpoint_seq: starting_checkpoint_seq,
      checkpoint_handler: None,
    }
  }

  pub async fn start(&mut self) -> Result<()> {
    // Format is FIRE INIT sui-node <PACKAGE_VERSION> <MAJOR_VERSION> <MINOR_VERSION> <CHAIN_ID>
    println!(
      "\nFIRE INIT sui-node {} sui 0 0 {}",
      env!("CARGO_PKG_VERSION"), self.chain_id,
    );

    let checkpoint_handler = retry(ExponentialBackoff::default(), || async {
      let http_client = Self::get_http_client(&self.rpc_client_url).map_err(|err| {
        error!("Failed to create HTTP client: {}", err);
        err
      })?;

      Ok(CheckpointHandler::new(http_client))
    }).await?;

    self.checkpoint_handler = Some(checkpoint_handler);

    loop {
      self.convert_next_block().await?;
    }
  }

  pub async fn convert_next_block(&mut self) -> Result<()> {
    println!("\nFIRE BLOCK_START {}", self.current_checkpoint_seq);
    let checkpoint_handler = self.checkpoint_handler.as_ref().expect("Checkpoint handler should be created");
    let checkpoint_data = checkpoint_handler.download_checkpoint_data(self.current_checkpoint_seq).await?;

    if checkpoint_data.transactions.is_empty() {
      debug!("[fh-stream] no transactions to send");
      sleep(Duration::from_millis(100)).await;

      return Ok(())
    }

    debug!(
      "[fh-stream] got {} transactions from  {}",
      checkpoint_data.transactions.len(),
      self.current_checkpoint_seq,
    );

    for tx in &checkpoint_data.transactions {
      let txn_proto = convert_transaction(&tx);
      Self::print_transaction(&txn_proto);
    }

    for obj_change in &checkpoint_data.changed_objects {
      let obj_change_proto = convert_object_change(&obj_change);
      Self::print_changed_object(&obj_change_proto);
    }

    println!("\nFIRE BLOCK_END {}", self.current_checkpoint_seq);
    self.current_checkpoint_seq += 1;

    Ok(())
  }

  fn get_http_client(rpc_client_url: &str) -> Result<HttpClient> {
    let mut headers = HeaderMap::new();
    headers.insert(CLIENT_SDK_TYPE_HEADER, HeaderValue::from_static("indexer"));
  
    HttpClientBuilder::default()
    .max_request_body_size(2 << 30)
    .max_concurrent_requests(usize::MAX)
    .set_headers(headers.clone())
    .build(rpc_client_url)
    .map_err(|e| {
      Report::msg(format!("Failed to initialize fullnode RPC client with error: {:?}", e))
    })
  }
  
  
  fn print_transaction(transaction: &pb::CheckpointTransactionBlockResponse) {
    let mut buf = vec![];
    transaction.encode(&mut buf).unwrap_or_else(|_| {
      panic!(
        "Could not convert protobuf transaction to bytes '{:?}'",
        transaction
      )
    });
    println!("\nFIRE TRX {}", base64::encode(buf));
  }

  fn print_changed_object(obj_change: &pb::ChangedObject) {
    let mut buf = vec![];
    obj_change.encode(&mut buf).unwrap_or_else(|_| {
      panic!(
        "Could not convert protobuf object cahange to bytes '{:?}'",
        obj_change
      )
    });
    println!("\nFIRE OBJ {}", base64::encode(buf));
  }  
}
