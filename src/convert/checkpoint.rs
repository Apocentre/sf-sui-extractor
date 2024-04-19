use sui_indexer::types::IndexedCheckpoint;
use sui_types::messages_checkpoint::EndOfEpochData;
use crate::pb::sui::checkpoint as pb;
use super::common::{
  convert_checkpoint_commitment, convert_next_epoch_committee
};

fn convert_end_of_epoch_data(source: &EndOfEpochData) -> pb::EndOfEpochData {
  pb::EndOfEpochData {
    next_epoch_committee: source.next_epoch_committee.iter().map(convert_next_epoch_committee).collect(),
    next_epoch_protocol_version: source.next_epoch_protocol_version.as_u64(),
    epoch_commitments: source.epoch_commitments.iter().map(convert_checkpoint_commitment).collect(),
  }
}

pub fn convert_checkpoint(source: &IndexedCheckpoint) -> pb::Checkpoint {
  let gas_cost_summary = pb::GasCostSummary {
    total_gas_cost: source.total_gas_cost,
    computation_cost: source.computation_cost,
    storage_cost: source.storage_cost,
    storage_rebate: source.storage_rebate,
    non_refundable_storage_fee: source.non_refundable_storage_fee,
  };

  pb::Checkpoint {
    epoch: source.epoch,
    sequence_number: source.sequence_number,
    digest: source.checkpoint_digest.base58_encode(),
    network_total_transactions: source.network_total_transactions,
    previous_digest: source.previous_checkpoint_digest.map(|pd| pd.base58_encode()),
    gas_cost_summary: Some(gas_cost_summary),
    timestamp_ms: source.timestamp_ms,
    end_of_epoch_data: source.end_of_epoch_data.as_ref().map(convert_end_of_epoch_data),
    checkpoint_commitments: source.checkpoint_commitments.iter().map(convert_checkpoint_commitment).collect(),
    validator_signature: source.validator_signature.as_ref().to_vec(),
    successful_tx_num: source.successful_tx_num as u64,
    end_of_epoch: source.end_of_epoch,
  }
}
