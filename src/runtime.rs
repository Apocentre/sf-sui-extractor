use eyre::{Result, Report};
use jsonrpsee::http_client::{HeaderMap, HeaderValue, HttpClient, HttpClientBuilder};
use backoff::{ExponentialBackoff, future::retry};
use log::error;
use sui_json_rpc::{CLIENT_SDK_TYPE_HEADER};
use crate::checkpoint_handler::CheckpointHandler;

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
    // TODO: add the reamining parts
    println!(
      "\nFIRE INIT sui-node {} sui 1 1 {}",
      env!("CARGO_PKG_VERSION"), self.chain_id,
    );

    let checkpoint_handler = retry(ExponentialBackoff::default(), || async {
      let http_client = get_http_client(&self.rpc_client_url).map_err(|err| {
        error!("Failed to create HTTP client: {}", err);
        err
      })?;
      let cp = CheckpointHandler::new(http_client);

      Ok(cp)
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

    for _onchain_txn in &checkpoint_data.transactions {
      //TODO: convert transaction data and print to stdout
    }

    Ok(())
  }
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
