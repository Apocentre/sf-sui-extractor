use eyre::Result;
use futures::StreamExt;
use mysten_metrics::{
  init_metrics, get_metrics, metered_channel::{channel, Sender, Receiver, ReceiverStream},
};
use sui_indexer::{
  framework::fetcher::CheckpointFetcher,
  handlers::{
    checkpoint_handler::CheckpointProcessor, CheckpointDataToCommit, tx_processor::IndexingPackageCache,
  },
  metrics::IndexerMetrics, store::TemporaryCheckpointStore,
};
use sui_rest_api::{Client, CheckpointData};
use backoff::{ExponentialBackoff, future::retry};
// use prost::Message;
use log::error;
use prometheus::Registry;
use tokio::{sync::watch, spawn};
use crate::{
  sui::sui_store::SuiStore,
  // pb::sui::checkpoint as pb, 
  // convert::{
  //   tx::convert_transaction, object::convert_object_change, checkpoint::convert_checkpoint,
  // },
};

const DOWNLOAD_QUEUE_SIZE: usize = 1000;
const CHECKPOINT_QUEUE_SIZE: usize = 1000;
const CHECKPOINT_PROCESSING_BATCH_SIZE: usize = 25;

pub struct FirehoseStreamer {
  pub current_checkpoint_seq: u64,
  rpc_client_url: String,
  chain_id: String,
  registry: Registry,
}

impl FirehoseStreamer {
  pub fn new(chain_id: String, rpc_client_url: String, starting_checkpoint_seq: u64) -> Self {
    let registry = Registry::default();
    init_metrics(&registry);

    Self {
      current_checkpoint_seq: starting_checkpoint_seq,
      rpc_client_url,
      chain_id,
      registry,
    }
  }

  pub async fn start(&mut self) -> Result<()> {
    // Format is FIRE INIT sui-node <PACKAGE_VERSION> <MAJOR_VERSION> <MINOR_VERSION> <CHAIN_ID>
    println!(
      "\nFIRE INIT sui-node {} sui 0 0 {}",
      env!("CARGO_PKG_VERSION"), self.chain_id,
    );

    let (
      checkpoint_data_sender,
      checkpoint_data_receiver
    ) = channel(
        DOWNLOAD_QUEUE_SIZE,
        &get_metrics()
        .unwrap()
        .channels
        .with_label_values(&["checkpoint_tx_downloading"]),
    );

    self.spawn_fetcher(checkpoint_data_sender).await?;

    let (
      handle_checkpoint_sender,
      handle_checkpoint_receiver
    ) = channel::<CheckpointDataToCommit>(
      CHECKPOINT_QUEUE_SIZE,
      &get_metrics()
      .unwrap()
      .channels
      .with_label_values(&["checkpoint_indexing"]),
    );
    
    self.spawn_checkpoint_handler(handle_checkpoint_sender, checkpoint_data_receiver).await?;
    Self::commit_checkpoint_data(handle_checkpoint_receiver).await;

    Ok(())
  }

  async fn spawn_fetcher(&self, checkpoint_data_sender: Sender<CheckpointData>) -> Result<()> {
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
      checkpoint_data_sender,
    );
    
    spawn(async move {
      checkpoint_fetcher.run().await;
    });

    Ok(())
  }

  async fn spawn_checkpoint_handler(
    &self,
    handle_checkpoint_sender: Sender<CheckpointDataToCommit>,
    checkpoint_data_receiver: Receiver<CheckpointData>
  ) -> Result<()> {
    let mut checkpoint_handler = self.create_handler(handle_checkpoint_sender).await?;
    let stream = ReceiverStream::new(checkpoint_data_receiver);
    let mut chunks = stream.ready_chunks(CHECKPOINT_PROCESSING_BATCH_SIZE);

    spawn(async move {
      while let Some(checkpoints) = chunks.next().await {
        checkpoint_handler.process_checkpoints(&checkpoints).await.expect("process checkpoints"); 
      }
    });

    Ok(())
  }

  async fn commit_checkpoint_data(handle_checkpoint_receiver: Receiver<TemporaryCheckpointStore>) {
    let mut stream = ReceiverStream::new(handle_checkpoint_receiver);

    while let Some(checkpoint_data) = stream.next().await {
      // TODO: convert and log data to the stdout
      println!("Block {} -> {:?}", checkpoint_data.checkpoint.epoch, checkpoint_data)
    }
  }

  async fn create_handler(&self, handle_checkpoint_sender: Sender<TemporaryCheckpointStore>) -> Result<CheckpointProcessor<SuiStore>> {
    let (_, rx) = watch::channel(None);
    let indexer_metrics = IndexerMetrics::new(&self.registry);
    
    let checkpoint_handler = CheckpointProcessor::new(
      SuiStore::new(),
      indexer_metrics,
      handle_checkpoint_sender,
      IndexingPackageCache::start(rx),
    );


    Ok(checkpoint_handler)
  }

  fn get_http_client(rpc_client_url: &str) -> Result<Client> {
    let rest_api_url = format!("{}/rest", rpc_client_url);
    let rest_client = Client::new(&rest_api_url);

    Ok(rest_client)
  }

  // pub async fn convert_next_block(&mut self) -> Result<()> {
  //   let checkpoint_handler = self.checkpoint_handler.as_ref().expect("Checkpoint handler should be created");
  //   let checkpoint_data = retry(ExponentialBackoff::default(), || async {
  //     Ok(checkpoint_handler.download_checkpoint_data(self.current_checkpoint_seq).await?)
  //   }).await?;

  //   println!("\nFIRE BLOCK_START {}", self.current_checkpoint_seq);

  //   if checkpoint_data.transactions.is_empty() {
  //     debug!("[fh-stream] no transactions to send");
  //     sleep(Duration::from_millis(100)).await;

  //     return Ok(())
  //   }

  //   debug!(
  //     "[fh-stream] got {} transactions from  {}",
  //     checkpoint_data.transactions.len(),
  //     self.current_checkpoint_seq,
  //   );

  //   Self::print_checkpoint_overview(&convert_checkpoint(&checkpoint_data.checkpoint));

  //   for tx in &checkpoint_data.transactions {
  //     let txn_proto = convert_transaction(&tx);
  //     Self::print_transaction(&txn_proto);
  //   }

  //   for obj_change in &checkpoint_data.changed_objects {
  //     let obj_change_proto = convert_object_change(&obj_change);
  //     Self::print_changed_object(&obj_change_proto);
  //   }

  //   println!("\nFIRE BLOCK_END {}", self.current_checkpoint_seq);
  //   self.current_checkpoint_seq += 1;

  //   Ok(())
  // }

  // fn print_checkpoint_overview(checkpoint: &pb::Checkpoint) {
  //   let mut buf = vec![];
  //   checkpoint.encode(&mut buf).unwrap_or_else(|_| {
  //     panic!(
  //       "Could not convert protobuf checkpoint to bytes '{:?}'",
  //       checkpoint
  //     )
  //   });
  //   println!("\nFIRE CHECKPOINT {}", base64::encode(buf));
  // }

  // fn print_transaction(transaction: &pb::CheckpointTransactionBlockResponse) {
  //   let mut buf = vec![];
  //   transaction.encode(&mut buf).unwrap_or_else(|_| {
  //     panic!(
  //       "Could not convert protobuf transaction to bytes '{:?}'",
  //       transaction
  //     )
  //   });
  //   println!("\nFIRE TRX {}", base64::encode(buf));
  // }

  // fn print_changed_object(obj_change: &pb::ChangedObject) {
  //   let mut buf = vec![];
  //   obj_change.encode(&mut buf).unwrap_or_else(|_| {
  //     panic!(
  //       "Could not convert protobuf object change to bytes '{:?}'",
  //       obj_change
  //     )
  //   });
  //   println!("\nFIRE OBJ {}", base64::encode(buf));
  // }
}
