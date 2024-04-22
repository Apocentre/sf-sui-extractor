use eyre::Result;
use futures::StreamExt;
use mysten_metrics::{
  init_metrics, get_metrics, metered_channel::{channel, Sender, Receiver, ReceiverStream},
};
use sui_indexer::{
  framework::{fetcher::{CheckpointFetcher, CheckpointDownloadData}, Handler},
  handlers::{
    checkpoint_handler::CheckpointHandler, CheckpointDataToCommit,
  },
  metrics::IndexerMetrics,
};
use sui_rest_api::Client;
use backoff::{ExponentialBackoff, future::retry};
use prost::Message;
use log::{debug, error};
use prometheus::Registry;
use tokio::spawn;
use crate::{
  convert::{
    checkpoint::convert_checkpoint, display_update::convert_display_update, sui_event::convert_indexed_event,
    tx::convert_transaction,
  },
  pb::sui::checkpoint as pb,
};

const DOWNLOAD_QUEUE_SIZE: usize = 1000;
const CHECKPOINT_QUEUE_SIZE: usize = 1000;
const CHECKPOINT_PROCESSING_BATCH_SIZE: usize = 25;

pub struct FirehoseStreamer {
  pub current_checkpoint_seq: u64,
  rpc_client_url: String,
  chain_id: String,
  metrics: IndexerMetrics,
}

impl FirehoseStreamer {
  pub fn new(chain_id: String, rpc_client_url: String, starting_checkpoint_seq: u64) -> Self {
    let registry = Registry::default();
    init_metrics(&registry);
    let metrics = IndexerMetrics::new(&registry);

    Self {
      current_checkpoint_seq: starting_checkpoint_seq,
      rpc_client_url,
      chain_id,
      metrics,
    }
  }

  pub async fn start(&mut self) -> Result<()> {
    // Format is FIRE INIT sui-node <PACKAGE_VERSION> <MAJOR_VERSION> <MINOR_VERSION> <CHAIN_ID>
    println!(
      "\nFIRE INIT sui-node {} sui 0 0 {}",
      env!("CARGO_PKG_VERSION"), self.chain_id,
    );

    let (
      downloaded_checkpoint_data_sender,
      downloaded_checkpoint_data_receiver
    ) = channel(
        DOWNLOAD_QUEUE_SIZE,
        &get_metrics()
        .unwrap()
        .channels
        .with_label_values(&["checkpoint_tx_downloading"]),
    );

    self.spawn_fetcher(downloaded_checkpoint_data_sender).await?;

    let (
      handle_checkpoint_sender,
      handle_checkpoint_receiver
    ) = channel::<CheckpointDataToCommit>(
      CHECKPOINT_QUEUE_SIZE,
      &get_metrics().unwrap()
      .channels
      .with_label_values(&["checkpoint_indexing"]),
    );
    
    self.spawn_checkpoint_handler(handle_checkpoint_sender, downloaded_checkpoint_data_receiver).await?;
    self.commit_checkpoint_data(handle_checkpoint_receiver).await;

    Ok(())
  }

  async fn spawn_fetcher(&self, downloaded_checkpoint_data_sender: Sender<CheckpointDownloadData>) -> Result<()> {
    let http_client = retry(ExponentialBackoff::default(), || async {
      let http_client = Self::get_http_client(&self.rpc_client_url).map_err(|err| {
        error!("Failed to create HTTP client: {}", err);
        err
      })?;
      
      Ok(http_client)
    }).await?;

    let checkpoint_fetcher = CheckpointFetcher::new(
      http_client,
      Some(self.current_checkpoint_seq),
      downloaded_checkpoint_data_sender,
      self.metrics.clone(),
    );

    spawn(async move {
      checkpoint_fetcher.run().await;
    });

    Ok(())
  }

  async fn spawn_checkpoint_handler(
    &self,
    handle_checkpoint_sender: Sender<CheckpointDataToCommit>,
    downloaded_checkpoint_data_receiver: Receiver<CheckpointDownloadData>
  ) -> Result<()> {
    let mut checkpoint_handler = self.create_handler(handle_checkpoint_sender).await?;
    let stream = ReceiverStream::new(downloaded_checkpoint_data_receiver);
    let mut chunks = stream.ready_chunks(CHECKPOINT_PROCESSING_BATCH_SIZE);

    spawn(async move {
      while let Some(checkpoints) = chunks.next().await {
        let checkpoint_data = checkpoints.iter().map(|c| c.data.clone()).collect::<Vec<_>>();
        checkpoint_handler.process_checkpoints(&checkpoint_data).await.expect("process checkpoints"); 
      }
    });

    Ok(())
  }

  async fn commit_checkpoint_data(&mut self, handle_checkpoint_receiver: Receiver<CheckpointDataToCommit>) {
    let mut stream = ReceiverStream::new(handle_checkpoint_receiver);

    while let Some(checkpoint_data) = stream.next().await {
      // Convert and log data to the stdout
      // We would need to ignore the following fields from CheckpointDataToCommit:
      // 1. epoch
      // 2. object_changes
      // 3. object_history_changes
      //
      // These fields are computed and rely on state being stored which we don't want to do here.
      //
      // We will have to update the proto buf models and thus all convertsion logic that exist in the
      // convert module.
      assert!(self.current_checkpoint_seq == checkpoint_data.checkpoint.sequence_number, "sequence number mismatch");
      println!("\nFIRE BLOCK_START {}", self.current_checkpoint_seq);

      if checkpoint_data.transactions.is_empty() {
        debug!("[fh-stream] no transactions to send");
      }

      debug!(
        "[fh-stream] got {} transactions from  {}",
        checkpoint_data.transactions.len(),
        self.current_checkpoint_seq,
      );

      Self::print_checkpoint_overview(&convert_checkpoint(&checkpoint_data.checkpoint));

      for tx in &checkpoint_data.transactions {
        let txn_proto = convert_transaction(&tx);
        Self::print_transaction(&txn_proto);
      }

      // Not that the transaction data does also include event data but here we explicitely log
      // events if one is interested in just that
      for event in &checkpoint_data.events {
        let event_proto = convert_indexed_event(event);
        Self::print_event(&event_proto);
      }

      for (_, store_display) in &checkpoint_data.display_updates {
        let store_display_proto = convert_display_update(store_display);
        Self::print_display_update(&store_display_proto);
      }

      println!("\nFIRE BLOCK_END {}", self.current_checkpoint_seq);
      self.current_checkpoint_seq += 1;

      println!(
        "Block {} -----> Tx count {:?}",
        checkpoint_data.checkpoint.sequence_number, checkpoint_data.transactions.len(),
      )
    }
  }

  async fn create_handler(&self, handle_checkpoint_sender: Sender<CheckpointDataToCommit>) -> Result<CheckpointHandler> {
    let checkpoint_handler = CheckpointHandler::new(
      self.metrics.clone(),
      handle_checkpoint_sender,
    );

    Ok(checkpoint_handler)
  }

  fn get_http_client(rpc_client_url: &str) -> Result<Client> {
    let rest_api_url = format!("{}/rest", rpc_client_url);
    let rest_client = Client::new(&rest_api_url);

    Ok(rest_client)
  }

  fn print_checkpoint_overview(checkpoint: &pb::Checkpoint) {
    let mut buf = vec![];
    checkpoint.encode(&mut buf).unwrap_or_else(|_| {
      panic!(
        "Could not convert protobuf checkpoint to bytes '{:?}'",
        checkpoint
      )
    });

    println!("\nFIRE CHECKPOINT {}", base64::encode(buf));
  }

  fn print_transaction(transaction: &pb::Transaction) {
    let mut buf = vec![];
    transaction.encode(&mut buf).unwrap_or_else(|_| {
      panic!(
        "Could not convert protobuf transaction to bytes '{:?}'",
        transaction
      )
    });

    println!("\nFIRE TRX {}", base64::encode(buf));
  }

  fn print_event(event: &pb::IndexedEvent) {
    let mut buf = vec![];
    event.encode(&mut buf).unwrap_or_else(|_| {
      panic!(
        "Could not convert protobuf event to bytes '{:?}'",
        event
      )
    });

    println!("\nFIRE EVT {}", base64::encode(buf));
  }

  fn print_display_update(display_update: &pb::StoredDisplay) {
    let mut buf = vec![];
    display_update.encode(&mut buf).unwrap_or_else(|_| {
      panic!(
        "Could not convert protobuf display update to bytes '{:?}'",
        display_update
      )
    });

    println!("\nFIRE DSP_UPDATE {}", base64::encode(buf));
  }
}
