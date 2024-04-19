use sui_indexer::types::IndexedCheckpoint;
use sui_types::messages_checkpoint::EndOfEpochData;
use crate::pb::sui::checkpoint as pb;
use super::common::{
  convert_gas_cost_summary, convert_checkpoint_commitment, convert_next_epoch_committee
};

fn convert_end_of_epoch_data(source: &EndOfEpochData) -> pb::EndOfEpochData {
  pb::EndOfEpochData {
    next_epoch_committee: source.next_epoch_committee.iter().map(convert_next_epoch_committee).collect(),
    next_epoch_protocol_version: source.next_epoch_protocol_version.as_u64(),
    epoch_commitments: source.epoch_commitments.iter().map(convert_checkpoint_commitment).collect(),
  }
}

pub fn convert_checkpoint(source: &IndexedCheckpoint) -> pb::Checkpoint {
  pb::Checkpoint {
    epoch: source.epoch,
    sequence_number: source.sequence_number,
    digest: source.digest.base58_encode(),
    network_total_transactions: source.network_total_transactions,
    previous_digest: source.previous_digest.map(|pd| pd.base58_encode()),
    epoch_rolling_gas_cost_summary: Some(convert_gas_cost_summary(&source.epoch_rolling_gas_cost_summary)),
    timestamp_ms: source.timestamp_ms,
    end_of_epoch_data: source.end_of_epoch_data.as_ref().map(convert_end_of_epoch_data),
    transactions: source.transactions.iter().map(|t| t.base58_encode()).collect(),
    checkpoint_commitments: source.checkpoint_commitments.iter().map(convert_checkpoint_commitment).collect(),
    validator_signature: source.validator_signature.as_ref().to_vec(),
  }
}
