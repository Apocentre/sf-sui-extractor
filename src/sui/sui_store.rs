use std::sync::Arc;
use async_trait::async_trait;
use move_bytecode_utils::module_cache::SyncModuleCache;
use prometheus::{IntCounter, Histogram};
use sui_json_rpc_types::{
  CheckpointId, Checkpoint as RpcCheckpoint, EventFilter, EventPage, SuiObjectDataFilter, SuiTransactionBlockResponse,
  SuiTransactionBlockResponseOptions, MoveCallMetrics, NetworkMetrics, EpochInfo,
};
use sui_types::{
  base_types::{ObjectID, SequenceNumber, SuiAddress}, object::ObjectRead,
  messages_checkpoint::{CheckpointDigest, CheckpointSequenceNumber}, event::EventID, Identifier, committee::EpochId,
};
use sui_indexer::{
  store::{IndexerStore, TransactionObjectChanges, TemporaryEpochStore}, errors::IndexerError, models::{checkpoints::Checkpoint, events::Event, transactions::Transaction, addresses::{ActiveAddress, Address, AddressStats}, transaction_index::{Recipient, InputObject, ChangedObject, MoveCall}, checkpoint_metrics::CheckpointMetrics}, metrics::IndexerMetrics,
};
use super::module_resolver::SuiModuleResolver;

/// Dummy implementation of the IndexerStore trait. This is currently required by the CheckpointHandler we're utilizing
/// from the sui-indexer crate.
#[derive(Clone)]
pub struct SuiStore {
  module_cache: Arc<SyncModuleCache<SuiModuleResolver>>,
}

impl SuiStore {
  pub fn new() -> Self {
    let module_cache = Arc::new(SyncModuleCache::new(SuiModuleResolver));

    Self {module_cache}
  }
}

#[async_trait]
impl IndexerStore for SuiStore {
  type ModuleCache = SyncModuleCache<SuiModuleResolver>;

  async fn get_latest_tx_checkpoint_sequence_number(&self) -> Result<i64, IndexerError> {
    todo!()
  }

  async fn get_latest_object_checkpoint_sequence_number(&self) -> Result<i64, IndexerError> {
    todo!()
  }

  async fn get_checkpoint(&self, id: CheckpointId) -> Result<RpcCheckpoint, IndexerError> {
    todo!()
  }

  async fn get_checkpoints(&self, cursor: Option<CheckpointId>, limit: usize) -> Result<Vec<RpcCheckpoint>, IndexerError> {
    todo!()
  }
  async fn get_indexer_checkpoint(&self) -> Result<Checkpoint, IndexerError> {
    todo!()
  }

  async fn get_indexer_checkpoints(&self, cursor: i64, limit: usize) -> Result<Vec<Checkpoint>, IndexerError> {
    todo!()
  }

  async fn get_checkpoint_sequence_number(&self, digest: CheckpointDigest) -> Result<CheckpointSequenceNumber, IndexerError> {
    todo!();
  }

  async fn get_event(&self, id: EventID) -> Result<Event, IndexerError> {
    todo!();
  }

  async fn get_events(
    &self,
    query: EventFilter,
    cursor: Option<EventID>,
    limit: Option<usize>,
    descending_order: bool,
  ) -> Result<EventPage, IndexerError> {
    todo!();
  }

  async fn get_object(
    &self,
    object_id: ObjectID,
    version: Option<SequenceNumber>,
  ) -> Result<ObjectRead, IndexerError> {
    todo!();
  }

  async fn query_objects_history(
    &self,
    filter: SuiObjectDataFilter,
    at_checkpoint: CheckpointSequenceNumber,
    cursor: Option<ObjectID>,
    limit: usize,
  ) -> Result<Vec<ObjectRead>, IndexerError> {
    todo!();
  }

  async fn query_latest_objects(
    &self,
    filter: SuiObjectDataFilter,
    cursor: Option<ObjectID>,
    limit: usize,
  ) -> Result<Vec<ObjectRead>, IndexerError> {
    todo!();
  }

  async fn get_total_transaction_number_from_checkpoints(&self) -> Result<i64, IndexerError> {
    todo!();
  }

  // TODO: combine all get_transaction* methods
  async fn get_transaction_by_digest(&self, tx_digest: &str) -> Result<Transaction, IndexerError> {
    todo!();
  }

  async fn multi_get_transactions_by_digests(&self, tx_digests: &[String]) -> Result<Vec<Transaction>, IndexerError> {
    todo!();
  }

  async fn compose_sui_transaction_block_response(
      &self,
      tx: Transaction,
      options: Option<&SuiTransactionBlockResponseOptions>,
  ) -> Result<SuiTransactionBlockResponse, IndexerError> {
    todo!();
  }

  async fn get_all_transaction_page(
    &self,
    start_sequence: Option<i64>,
    limit: usize,
    is_descending: bool,
  ) -> Result<Vec<Transaction>, IndexerError> {
    todo!();
  }

  async fn get_transaction_page_by_checkpoint(
    &self,
    checkpoint_sequence_number: i64,
    start_sequence: Option<i64>,
    limit: usize,
    is_descending: bool,
  ) -> Result<Vec<Transaction>, IndexerError> {
    todo!();
  }

  async fn get_transaction_page_by_transaction_kinds(
    &self,
    kind_names: Vec<String>,
    start_sequence: Option<i64>,
    limit: usize,
    is_descending: bool,
  ) -> Result<Vec<Transaction>, IndexerError> {
    todo!();
  }

  async fn get_transaction_page_by_sender_address(
    &self,
    sender_address: String,
    start_sequence: Option<i64>,
    limit: usize,
    is_descending: bool,
  ) -> Result<Vec<Transaction>, IndexerError> {
    todo!();
  }

  async fn get_transaction_page_by_recipient_address(
    &self,
    sender_address: Option<SuiAddress>,
    recipient_address: SuiAddress,
    start_sequence: Option<i64>,
    limit: usize,
    is_descending: bool,
  ) -> Result<Vec<Transaction>, IndexerError> {
    todo!();
  }

  // `address` can be either sender or recipient address of the transaction
  async fn get_transaction_page_by_address(
    &self,
    address: SuiAddress,
    start_sequence: Option<i64>,
    limit: usize,
    is_descending: bool,
  ) -> Result<Vec<Transaction>, IndexerError> {
    todo!();
  }

  async fn get_transaction_page_by_input_object(
    &self,
    object_id: ObjectID,
    version: Option<i64>,
    start_sequence: Option<i64>,
    limit: usize,
    is_descending: bool,
  ) -> Result<Vec<Transaction>, IndexerError> {
    todo!();
  }

  async fn get_transaction_page_by_changed_object(
    &self,
    object_id: ObjectID,
    version: Option<i64>,
    start_sequence: Option<i64>,
    limit: usize,
    is_descending: bool,
  ) -> Result<Vec<Transaction>, IndexerError> {
    todo!();
  }

  async fn get_transaction_page_by_move_call(
    &self,
    package: ObjectID,
    module: Option<Identifier>,
    function: Option<Identifier>,
    start_sequence: Option<i64>,
    limit: usize,
    is_descending: bool,
  ) -> Result<Vec<Transaction>, IndexerError> {
    todo!();
  }

  async fn get_transaction_sequence_by_digest(
    &self,
    tx_digest: Option<String>,
    is_descending: bool,
  ) -> Result<Option<i64>, IndexerError> {
    todo!();
  }

  async fn get_move_call_sequence_by_digest(
    &self,
    tx_digest: Option<String>,
    is_descending: bool,
  ) -> Result<Option<i64>, IndexerError> {
    todo!();
  }

  async fn get_input_object_sequence_by_digest(
    &self,
    tx_digest: Option<String>,
    is_descending: bool,
  ) -> Result<Option<i64>, IndexerError> {
    todo!();
  }

  async fn get_changed_object_sequence_by_digest(
    &self,
    tx_digest: Option<String>,
    is_descending: bool,
  ) -> Result<Option<i64>, IndexerError> {
    todo!();
  }

  async fn get_recipient_sequence_by_digest(
    &self,
    tx_digest: Option<String>,
    is_descending: bool,
  ) -> Result<Option<i64>, IndexerError> {
    todo!();
  }

  async fn get_recipients_data_by_checkpoint(
      &self,
      seq: u64,
  ) -> Result<Vec<Recipient>, IndexerError> {
    todo!();
  }

  async fn get_network_metrics(&self) -> Result<NetworkMetrics, IndexerError> {
    todo!();
  }

  async fn get_move_call_metrics(&self) -> Result<MoveCallMetrics, IndexerError> {
    todo!();
  }

  async fn persist_checkpoint_transactions(
    &self,
    checkpoints: &[Checkpoint],
    transactions: &[Transaction],
    counter_committed_tx: IntCounter,
  ) -> Result<(), IndexerError> {
    todo!();
  }
  
  async fn persist_object_changes(
    &self,
    tx_object_changes: &[TransactionObjectChanges],
    object_mutation_latency: Histogram,
    object_deletion_latency: Histogram,
    object_commit_chunk_counter: IntCounter,
  ) -> Result<(), IndexerError> {
    todo!();
  }

  async fn persist_events(&self, events: &[Event]) -> Result<(), IndexerError> {
    todo!();
  }

  async fn persist_addresses(
      &self,
      addresses: &[Address],
      active_addresses: &[ActiveAddress],
  ) -> Result<(), IndexerError> {
    todo!();
  }

  async fn persist_packages(&self, packages: &[sui_indexer::models::packages::Package]) -> Result<(), IndexerError> {
    todo!();
  }
  // NOTE: these tables are for tx query performance optimization
  async fn persist_transaction_index_tables(
      &self,
      input_objects: &[InputObject],
      changed_objects: &[ChangedObject],
      move_calls: &[MoveCall],
      recipients: &[Recipient],
  ) -> Result<(), IndexerError> {
    todo!();
  }

  async fn persist_epoch(&self, data: &TemporaryEpochStore) -> Result<(), IndexerError> {
    todo!();
  }

  async fn get_network_total_transactions_previous_epoch(&self, epoch: i64) -> Result<i64, IndexerError> {
    Ok(0)
  }

  async fn get_epochs(
    &self,
    cursor: Option<EpochId>,
    limit: usize,
    descending_order: Option<bool>,
  ) -> Result<Vec<EpochInfo>, IndexerError> {
    todo!();
  }

  async fn get_current_epoch(&self) -> Result<EpochInfo, IndexerError> {
    todo!();
  }

  fn module_cache(&self) -> &Self::ModuleCache {
    todo!();
  }

  fn indexer_metrics(&self) -> &IndexerMetrics {
    todo!();
  }

  /// methods for address stats
  async fn get_last_address_processed_checkpoint(&self) -> Result<i64, IndexerError> {
    todo!();
  }
  async fn calculate_address_stats(&self, checkpoint: i64) -> Result<AddressStats, IndexerError> {
    todo!();
  }

  async fn persist_address_stats(&self, addr_stats: &AddressStats) -> Result<(), IndexerError> {
    todo!();
  }

  async fn get_latest_address_stats(&self) -> Result<AddressStats, IndexerError> {
    todo!();
  }
  
  async fn get_checkpoint_address_stats(&self, checkpoint: i64) -> Result<AddressStats, IndexerError> {
    todo!();
  }
  
  async fn get_all_epoch_address_stats(&self, descending_order: Option<bool>) -> Result<Vec<AddressStats>, IndexerError> {
    todo!();
  }

  /// methods for checkpoint metrics
  async fn calculate_checkpoint_metrics(
    &self,
    current_checkpoint: i64,
    last_checkpoint_metrics: &CheckpointMetrics,
    checkpoints: &[Checkpoint],
  ) -> Result<CheckpointMetrics, IndexerError> {
    todo!();
  }

  async fn persist_checkpoint_metrics(&self, checkpoint_metrics: &CheckpointMetrics) -> Result<(), IndexerError> {
    todo!();
  }
  
  async fn get_latest_checkpoint_metrics(&self) -> Result<CheckpointMetrics, IndexerError> {
    todo!();
  }

  /// TPS related methods
  async fn calculate_real_time_tps(&self, current_checkpoint: i64) -> Result<f64, IndexerError> {
    todo!();
  }
  
  async fn calculate_peak_tps_30d(
    &self,
    current_checkpoint: i64,
    current_timestamp_ms: i64,
  ) -> Result<f64, IndexerError> {
    todo!();
  }
}
