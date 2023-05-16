use sui_indexer::types::CheckpointTransactionBlockResponse;
use crate::pb::sui::checkpoint as pb;
use super::{
  sui_tx_block::convert_sui_tx_block,
};


pub fn convert_transaction(source: &CheckpointTransactionBlockResponse) -> pb::CheckpointTransactionBlockResponse {
  let pb_tx = pb::CheckpointTransactionBlockResponse {
    digest: source.digest.into_inner().to_vec(),
    transaction: convert_sui_tx_block(&source.transaction),
    ..Default::default()
  };
  
  
  pb_tx
}
