use base58::ToBase58;
use sui_types::{base_types::ObjectID, effects::{EffectsObjectChange, IDOperation, ObjectIn, ObjectOut, TransactionEffects, TransactionEffectsAPI}};
use crate::pb::sui::checkpoint as pb;
use super::common::{
  convert_gas_cost_summary, convert_object_ref, convert_owned_object_ref, convert_owner, convert_sui_execution_status, convert_sui_object, convert_tx_block_effects_modified_at_versions
};

pub fn convert_sui_effects(source: &TransactionEffects) -> pb::TransactionBlockEffects {
  let transaction_block_effects = match source {
    TransactionEffects::V1(source) => pb::transaction_block_effects::TransactionBlockEffects::V1(
      pb::TransactionBlockEffectsV1 {
        status: Some(convert_sui_execution_status(&source.status)),
        executed_epoch: source.executed_epoch,
        gas_used: Some(convert_gas_cost_summary(&source.gas_used)),
        modified_at_versions: source.modified_at_versions.iter().map(convert_tx_block_effects_modified_at_versions).collect(),
        shared_objects: source.shared_objects.iter().map(convert_object_ref).collect(),
        transaction_digest: source.transaction_digest.base58_encode(),
        created: source.created.iter().map(convert_owned_object_ref).collect(),
        mutated: source.mutated.iter().map(convert_owned_object_ref).collect(),
        unwrapped: source.unwrapped.iter().map(convert_owned_object_ref).collect(),
        deleted: source.deleted.iter().map(convert_object_ref).collect(),
        unwrapped_then_deleted: source.unwrapped_then_deleted.iter().map(convert_object_ref).collect(),
        wrapped: source.wrapped.iter().map(convert_object_ref).collect(),
        gas_object: Some(convert_owned_object_ref(&source.gas_object)),
        events_digest: source.events_digest.map(|d| d.into_inner().to_base58()),
        dependencies: source.dependencies.iter().map(|v| v.base58_encode()).collect()
      }
    ),
    TransactionEffects::V2(source) => pb::transaction_block_effects::TransactionBlockEffects::V2(
      pb::TransactionBlockEffectsV2 {
        status: Some(convert_sui_execution_status(&source.status)),
        executed_epoch: source.executed_epoch(),
        gas_used: Some(convert_gas_cost_summary(&source.gas_used)),
        transaction_digest: source.transaction_digest.base58_encode(),
        gas_object_index: source.gas_object_index,
        events_digest: source.events_digest.map(|e| e.base58_encode()),
        dependencies: source.dependencies.iter().map(|v| v.base58_encode()).collect::<Vec<_>>(),
        lamport_version: source.lamport_version.value(),
        changed_objects: source.changed_objects.iter().map(convert_changed_object_v2).collect::<Vec<_>>(),
        unchanged_shared_objects: todo!(),
        aux_data_digest: source.aux_data_digest().map(|e| e.clone()),
    }
    ),
  };

  pb::TransactionBlockEffects {
    transaction_block_effects: Some(transaction_block_effects),
  }
}

fn convert_changed_object_v2(source: &(ObjectID, EffectsObjectChange)) -> pb::ChangedObjectV2 {
  pb::ChangedObjectV2 {
    object_id: Some(convert_sui_object(&source.0)),
    effects: Some(convert_effects_object_change(&source.1)),
  }
}

fn convert_effects_object_change(source: &EffectsObjectChange) -> pb::EffectsObjectChange {
  let object_in = match source.input_state {
    ObjectIn::NotExist => pb::object_in::ObjectIn::NotExist(0),
    ObjectIn::Exist(source) => pb::object_in::ObjectIn::Exist(pb::ObjectInExist {
        version_digest: Some(pb::VersionDigest {
          sequence_number: source.0.0.value(),
          object_digest: source.0.1.base58_encode(),
        }),
        owner: Some(convert_owner(&source.1)),
    }),
  };

  let input_state = pb::ObjectIn {
    object_in: Some(object_in),
  };

  let object_out = match source.output_state {
    ObjectOut::NotExist =>  pb::object_out::ObjectOut::NotExist(0),
    ObjectOut::ObjectWrite(source) => {
      pb::object_out::ObjectOut::ObjectWrite(pb::ObjectWrite {
        object_digest: source.0.base58_encode(),
        owner: Some(convert_owner(&source.1)),
      })
    },
    ObjectOut::PackageWrite(source) => {
      pb::object_out::ObjectOut::PackageWrite(pb::PackageWrite {
        version_digest: Some(pb::VersionDigest {
          sequence_number: source.0.value(),
          object_digest: source.1.base58_encode(),
        }),
      })
    },
  };

  let output_state = pb::ObjectOut {
    object_out: Some(object_out),
  };

  let id_operation = pb::IdOperation {
    id_operation: match source.id_operation {
      IDOperation::None => Some(pb::id_operation::IdOperation::None(0)),
      IDOperation::Created => Some(pb::id_operation::IdOperation::Created(1)),
      IDOperation::Deleted => Some(pb::id_operation::IdOperation::Deleted(2)),
    }
  };

  pb::EffectsObjectChange {
    input_state: Some(input_state),
    output_state: Some(output_state),
    id_operation: Some(id_operation),
  }
}
