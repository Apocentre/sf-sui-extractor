use sui_indexer::types::CheckpointTransactionBlockResponse;
use crate::pb::sui::checkpoint as pb;

pub fn convert_transaction(tx: &CheckpointTransactionBlockResponse) -> pb::CheckpointTransactionBlockResponse {
  todo!("Convert Sui transaction to our transaction protobuf object")
}
