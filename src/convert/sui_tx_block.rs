use sui_json_rpc_types::{SuiTransactionBlock, SuiTransactionBlockData};
use crate::pb::sui::checkpoint as pb;
use super::sui_tx_block_kind::convert_sui_transaction_block_kind;

pub fn convert_sui_tx_block(source: &SuiTransactionBlock) -> Option<pb::SuiTransactionBlock> {
  let sui_transaction_block_data = match &source.data {
    SuiTransactionBlockData::V1(source) => pb::sui_transaction_block_data::SuiTransactionBlockData::V1(
      pb::SuiTransactionBlockDataV1 {
        transaction: convert_sui_transaction_block_kind(&source.transaction),
        sender: source.sender.to_inner().to_vec(),
        gas_data: None,
      }
    ),
  };

  let pb_tx = pb::SuiTransactionBlock {
    data: Some(
      pb::SuiTransactionBlockData {
        sui_transaction_block_data: Some(sui_transaction_block_data),
      }
    ),
    ..Default::default()
  };

  Some(pb_tx)
}
