use std::{any::Any, collections::BTreeMap, sync::Arc};
use async_trait::async_trait;
use move_bytecode_utils::module_cache::SyncModuleCache;
use sui_types::{
  base_types::{ObjectID, SequenceNumber}, object::ObjectRead,
};
use sui_indexer::{
  store::indexer_store::IndexerStore, errors::IndexerError, handlers::{TransactionObjectChangesToCommit, EpochToCommit},
  types::{IndexedCheckpoint, IndexedTransaction, IndexedEvent, IndexedPackage, TxIndex},
  models::display::StoredDisplay,
};
use super::module_resolver::SuiModuleResolver;

/// Dummy implementation of the IndexerStore trait. This is currently required by the CheckpointHandler we're utilizing
/// from the sui-indexer crate. In theory we don't need to use this but since the core SUI indexer has a dependency on this
/// trait we just pass a dummy NULL implementation.
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

  async fn get_latest_tx_checkpoint_sequence_number(&self) -> Result<Option<u64>, IndexerError> {
    return Ok(None)
  }

  async fn get_latest_object_snapshot_checkpoint_sequence_number(&self) -> Result<Option<u64>, IndexerError> {
    return Ok(None)
  }

  async fn get_object_read(&self, _: ObjectID, _: Option<SequenceNumber>) -> Result<ObjectRead, IndexerError> {
    Ok(ObjectRead::NotExists(ObjectID::random()))
  }

  async fn persist_objects(&self, _: Vec<TransactionObjectChangesToCommit>) -> Result<(), IndexerError> {
    Ok(())
  }

  async fn persist_object_history(&self, _: Vec<TransactionObjectChangesToCommit>) -> Result<(), IndexerError> {
    Ok(())
  }

  async fn persist_object_snapshot(&self, _: u64, _: u64) -> Result<(), IndexerError> {
    Ok(())
  }

  async fn persist_checkpoints(&self, _: Vec<IndexedCheckpoint>) -> Result<(), IndexerError> {
    Ok(())
  }

  async fn persist_transactions(&self, _: Vec<IndexedTransaction>) -> Result<(), IndexerError> {
    Ok(())
  }

  async fn persist_tx_indices(&self, _: Vec<TxIndex>) -> Result<(), IndexerError> {
    Ok(())
  }

  async fn persist_events(&self, _: Vec<IndexedEvent>) -> Result<(), IndexerError> {
    Ok(())
  }

  async fn persist_displays(&self, _: BTreeMap<String, StoredDisplay>) -> Result<(), IndexerError> {
    Ok(())
  }

  async fn persist_packages(&self, _: Vec<IndexedPackage>) -> Result<(), IndexerError> {
    Ok(())
  }

  async fn persist_epoch(&self, _: EpochToCommit) -> Result<(), IndexerError> {
    Ok(())
  }

  async fn advance_epoch(&self, _: EpochToCommit) -> Result<(), IndexerError> {
    Ok(())
  }

  async fn get_network_total_transactions_by_end_of_epoch(&self, _: u64) -> Result<u64, IndexerError> {
    Ok(0)
  }

  fn module_cache(&self) -> Arc<Self::ModuleCache> {
    self.module_cache.clone()
  }

  fn as_any(&self) -> &dyn Any {
    self
  }
}
