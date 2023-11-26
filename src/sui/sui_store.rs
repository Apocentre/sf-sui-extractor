use std::{collections::BTreeMap, sync::Arc};
use async_trait::async_trait;
use move_bytecode_utils::module_cache::SyncModuleCache;
use sui_types::{
  base_types::{ObjectID, SequenceNumber}, object::ObjectRead,
};
use sui_indexer::{
  store::indexer_store_v2::IndexerStoreV2, errors::IndexerError, handlers::{TransactionObjectChangesToCommit, EpochToCommit},
  types_v2::{IndexedCheckpoint, IndexedTransaction, IndexedEvent, IndexedPackage, TxIndex}, models_v2::display::StoredDisplay
};
use super::module_resolver::SuiModuleResolver;

/// Dummy implementation of the IndexerStoreV2 trait. This is currently required by the CheckpointHandler we're utilizing
/// from the sui-indexer crate.
struct SuiStore {
  module_cache: Arc<SyncModuleCache<SuiModuleResolver>>,
}

impl SuiStore {
  pub fn new() -> Self {
    let module_cache = Arc::new(SyncModuleCache::new(SuiModuleResolver));

    Self {module_cache}
  }
}

#[async_trait]
impl IndexerStoreV2 for SuiStore {
  type ModuleCache = SyncModuleCache<SuiModuleResolver>;

  async fn get_latest_tx_checkpoint_sequence_number(&self) -> Result<Option<u64>, IndexerError> {
    Ok(None)
  }

  async fn get_object_read(&self, _: ObjectID, _: Option<SequenceNumber>,
  ) -> Result<ObjectRead, IndexerError> {
    Ok(ObjectRead::NotExists(ObjectID::random()))
  }

  async fn persist_objects(&self, _: Vec<TransactionObjectChangesToCommit>) -> Result<(), IndexerError> {
    Ok(())
  }

  async fn persist_checkpoints(&self, _: Vec<IndexedCheckpoint>) -> Result<(), IndexerError> {
    Ok(())
  }

  async fn persist_transactions(&self, _: Vec<IndexedTransaction>) -> Result<(), IndexerError> {
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

  async fn persist_tx_indices(&self, _: Vec<TxIndex>) -> Result<(), IndexerError> {
    Ok(())
  }

  async fn persist_epoch(&self, _: Vec<EpochToCommit>) -> Result<(), IndexerError> {
    Ok(())
  }

  async fn get_network_total_transactions_by_end_of_epoch(&self, _: u64) -> Result<u64, IndexerError> {
   Ok(0)
  }

  fn module_cache(&self) -> Arc<Self::ModuleCache> {
    self.module_cache.clone()
  }
}
