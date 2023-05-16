use sui_indexer::types::CheckpointTransactionBlockResponse;
use crate::pb::sui::checkpoint as pb;
use super::{
  sui_tx_block::convert_sui_tx_block,
};

pub fn convert_transaction(source: &CheckpointTransactionBlockResponse) -> pb::CheckpointTransactionBlockResponse {
  pb::CheckpointTransactionBlockResponse {
    digest: source.digest.into_inner().to_vec(),
    transaction: convert_sui_tx_block(&source.transaction),
    raw_transaction: source.raw_transaction.clone(),
    effects: Some(convert_sui_effects(&source.effects)),
    events: todo!(),
    timestamp_ms: todo!(),
    confirmed_local_execution: todo!(),
    checkpoint: todo!(),
  }
}
