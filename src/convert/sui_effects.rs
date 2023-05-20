use base58::ToBase58;
use sui_json_rpc_types::SuiTransactionBlockEffects;
use crate::pb::sui::checkpoint as pb;
use super::common::{
  convert_sui_execution_status, convert_gas_cost_summary, convert_tx_block_effects_modified_at_versions,
  convert_sui_object_ref, convert_owned_object_ref,
};

pub fn convert_sui_effects(source: &SuiTransactionBlockEffects) -> pb::SuiTransactionBlockEffects {
  let sui_transaction_block_effects = match source {
    SuiTransactionBlockEffects::V1(source) => pb::sui_transaction_block_effects::SuiTransactionBlockEffects::V1(
      pb::SuiTransactionBlockEffectsV1 {
        status: Some(convert_sui_execution_status(&source.status)),
        executed_epoch: source.executed_epoch,
        gas_used: Some(convert_gas_cost_summary(&source.gas_used)),
        modified_at_versions: source.modified_at_versions.iter().map(convert_tx_block_effects_modified_at_versions).collect(),
        shared_objects: source.shared_objects.iter().map(convert_sui_object_ref).collect(),
        transaction_digest: source.transaction_digest.base58_encode(),
        created: source.created.iter().map(convert_owned_object_ref).collect(),
        mutated: source.mutated.iter().map(convert_owned_object_ref).collect(),
        unwrapped: source.unwrapped.iter().map(convert_owned_object_ref).collect(),
        deleted: source.deleted.iter().map(convert_sui_object_ref).collect(),
        unwrapped_then_deleted: source.unwrapped_then_deleted.iter().map(convert_sui_object_ref).collect(),
        wrapped: source.wrapped.iter().map(convert_sui_object_ref).collect(),
        gas_object: Some(convert_owned_object_ref(&source.gas_object)),
        events_digest: source.events_digest.map(|d| d.into_inner().to_base58()),
        dependencies: source.dependencies.iter().map(|v| v.base58_encode()).collect()
      }
    )
  };

  pb::SuiTransactionBlockEffects {
    sui_transaction_block_effects: Some(sui_transaction_block_effects),
  }
}
