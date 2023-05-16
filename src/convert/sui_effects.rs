use sui_json_rpc_types::SuiTransactionBlockEffects;
use crate::pb::sui::checkpoint as pb;
use super::common::{convert_sui_execution_status, convert_gas_cost_summary, convert_tx_block_effects_modified_at_versions, convert_sui_object_ref};

pub async fn convert_sui_effects(source: &SuiTransactionBlockEffects) -> pb::SuiTransactionBlockEffects {
  let sui_transaction_block_effects = match source {
    SuiTransactionBlockEffects::V1(source) => pb::sui_transaction_block_effects::SuiTransactionBlockEffects::V1(
      pb::SuiTransactionBlockEffectsV1 {
        status: Some(convert_sui_execution_status(&source.status)),
        executed_epoch: source.executed_epoch,
        gas_used: Some(convert_gas_cost_summary(&source.gas_used)),
        modified_at_versions: convert_tx_block_effects_modified_at_versions(&source.modified_at_versions),
        shared_objects: source.shared_objects.iter().map(convert_sui_object_ref).collect(),
        transaction_digest: source.transaction_digest.into_inner().to_vec(),
        created: source.created.iter().map(convert_owned_object_ref).collect(),
        mutated: todo!(),
        unwrapped: todo!(),
        deleted: todo!(),
        unwrapped_then_deleted: todo!(),
        wrapped: todo!(),
        gas_object: todo!(),
        events_digest: todo!(),
        dependencies: todo!(),
      }
    )
  };

  pb::SuiTransactionBlockEffects {
    sui_transaction_block_effects: Some(sui_transaction_block_effects),
  }
}
