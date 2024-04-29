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

#[cfg(test)]
mod tests {
    use std::str::FromStr;
    use sui_indexer::types::IndexedCheckpoint;
    use sui_types::{crypto::{AggregateAuthoritySignature, ToFromBytes}, digests::{CheckpointDigest, TransactionDigest}};
    use crate::pb::sui::checkpoint as pb;
    use super::convert_checkpoint;

  #[test]
  fn coverts_to_checkpoint() {
    let source_checkpoint = IndexedCheckpoint {
      sequence_number: 1448000,
      checkpoint_digest: CheckpointDigest::from_str("8Y1Kx4BJxbAq2WvJcHmrbgq9VewnZ6um6biCDeb7XbCR").unwrap(),
      epoch: 19,
      tx_digests: vec![
        TransactionDigest::from_str("D7CBWgtjcgMyn1YhRZ2q7okrmiCUYW4QA5gPZT6CRa2n").unwrap(),
      ],
      network_total_transactions: 1449228,
      previous_checkpoint_digest: Some(
        CheckpointDigest::from_str("GuvEqJeH5uzfxpXk4js5HXe3NMSr7yfx8Xc4At4wXrKR").unwrap(),
      ),
      timestamp_ms: 1682990756147,
      total_gas_cost: 790183816,
      computation_cost: 100000000,
      storage_cost: 16350381600,
      storage_rebate: 15660197784,
      non_refundable_storage_fee: 158183816,
      checkpoint_commitments: vec![],
      validator_signature: AggregateAuthoritySignature::default(),
      successful_tx_num: 1,
      end_of_epoch_data: None,
      end_of_epoch: false,
    };

    let pb_checkpoint = convert_checkpoint(&source_checkpoint);
    let expected = pb::Checkpoint {
      epoch: 19,
      sequence_number: 1448000,
      digest: "8Y1Kx4BJxbAq2WvJcHmrbgq9VewnZ6um6biCDeb7XbCR".to_string(),
      network_total_transactions: 1449228,
      previous_digest: Some("GuvEqJeH5uzfxpXk4js5HXe3NMSr7yfx8Xc4At4wXrKR".to_string()),
      gas_cost_summary: Some(pb::GasCostSummary {
        computation_cost: 100000000,
        storage_cost: 16350381600,
        storage_rebate: 15660197784,
        non_refundable_storage_fee: 158183816,
      }),
      timestamp_ms: 1682990756147,
      end_of_epoch_data: None,
      checkpoint_commitments: vec![],
      validator_signature: AggregateAuthoritySignature::default().as_bytes().to_vec(),
      successful_tx_num: 1,
      end_of_epoch: false,
    };

    assert_eq!(expected, pb_checkpoint)
  }
}
