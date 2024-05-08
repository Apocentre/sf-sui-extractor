use base58::ToBase58;
use sui_types::{
  base_types::ObjectID, effects::{
    EffectsObjectChange, IDOperation, ObjectIn, ObjectOut, TransactionEffects, TransactionEffectsAPI, UnchangedSharedKind
  },
};
use crate::pb::sui::checkpoint as pb;
use super::{
  common::{
    convert_gas_cost_summary, convert_object_ref, convert_owned_object_ref, convert_owner, convert_sui_object, convert_tx_block_effects_modified_at_versions
  },
  execution_status::convert_sui_execution_status,
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
        unchanged_shared_objects: source.unchanged_shared_objects.iter().map(convert_unchanged_shared_objects).collect::<Vec<_>>(),
        aux_data_digest: source.aux_data_digest.map(|e| e.base58_encode()),
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
    ObjectIn::NotExist => pb::object_in::ObjectIn::NotExist(()),
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
    ObjectOut::NotExist =>  pb::object_out::ObjectOut::NotExist(()),
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
      IDOperation::None => Some(pb::id_operation::IdOperation::None(())),
      IDOperation::Created => Some(pb::id_operation::IdOperation::Created(())),
      IDOperation::Deleted => Some(pb::id_operation::IdOperation::Deleted(())),
    }
  };

  pb::EffectsObjectChange {
    input_state: Some(input_state),
    output_state: Some(output_state),
    id_operation: Some(id_operation),
  }
}

fn convert_unchanged_shared_objects(source: &(ObjectID, UnchangedSharedKind)) -> pb::UnchangedSharedObject {
  let kind = match source.1 {
    UnchangedSharedKind::ReadOnlyRoot(source) => {
      pb::unchanged_shared_kind::UnchangedSharedKind::ReadOnlyRoot(pb::VersionDigest {
        sequence_number: source.0.value(),
        object_digest: source.1.base58_encode(),
      })
    },
    UnchangedSharedKind::MutateDeleted(source) => {
      pb::unchanged_shared_kind::UnchangedSharedKind::MutateDeleted(source.value())
    },
    UnchangedSharedKind::ReadDeleted(source) => {
      pb::unchanged_shared_kind::UnchangedSharedKind::ReadDeleted(source.value())
    }
  };

  pb::UnchangedSharedObject {
    object_id: Some(convert_sui_object(&source.0)),
    kind: Some(pb::UnchangedSharedKind {
      unchanged_shared_kind: Some(kind),
    }),
  }
}

#[cfg(test)]
mod tests {
  use std::str::FromStr;
  use sui_types::{
    base_types::{ObjectID, SequenceNumber, SuiAddress}, digests::{ObjectDigest, TransactionDigest}, 
    effects::TransactionEffects, execution_status::ExecutionStatus, gas::GasCostSummary, object::Owner,
  };
  use crate::{convert::sui_effects::convert_sui_effects, pb::sui::checkpoint::{self as pb, ObjectId, TransactionBlockEffectsV1}};

  #[test]
  fn converts_sui_effects() {
    let source = TransactionEffects::new_from_execution_v1(
      ExecutionStatus::Success,
      19,
      GasCostSummary {
        computation_cost: 0,
        storage_cost: 0,
        storage_rebate: 0,
        non_refundable_storage_fee: 0,
      },
      vec![(ObjectID::from_str("0x0000000000000000000000000000000000000000000000000000000000000006").unwrap(), SequenceNumber::from_u64(1448000))],
      vec![(
        ObjectID::from_str("0x0000000000000000000000000000000000000000000000000000000000000006").unwrap(),
        SequenceNumber::from_u64(1448000),
        ObjectDigest::from_str("2GB5NVhagD4fQ9P85WqtgX3nwFwVdqDPbKYBtGcziQYM").unwrap(),
      )],
      TransactionDigest::from_str("D7CBWgtjcgMyn1YhRZ2q7okrmiCUYW4QA5gPZT6CRa2n").unwrap(),
      vec![],
      vec![(
        (
          ObjectID::from_str("0x0000000000000000000000000000000000000000000000000000000000000006").unwrap(),
          SequenceNumber::from_u64(1448001),
          ObjectDigest::from_str("CDdzbah88YnaMJXjhpnqHy5BTo3YBAqckuD5uzfs2kyX").unwrap(),
        ),
        Owner::Shared {
          initial_shared_version: SequenceNumber::from_u64(1),
        },
      )],
      vec![],
      vec![],
      vec![],
      vec![],
      (
        (
          ObjectID::from_str("0x0000000000000000000000000000000000000000000000000000000000000000").unwrap(),
          SequenceNumber::from_u64(0),
          ObjectDigest::from_str("11111111111111111111111111111111").unwrap(),
        ),
        Owner::AddressOwner(SuiAddress::from_str("0x0000000000000000000000000000000000000000000000000000000000000000").unwrap()),
      ),
      None,
      vec![TransactionDigest::from_str("A5PHzo8quSTJGpay1S5Q6AKTjtYpSJPMUi3wchQkzBSX").unwrap()],
    );

    let pb_transaction_effects = convert_sui_effects(&source);
    let expected = pb::TransactionBlockEffects {
      transaction_block_effects: Some(
        pb::transaction_block_effects::TransactionBlockEffects::V1(TransactionBlockEffectsV1 {
          status: Some(pb::ExecutionStatus {
            execution_status: Some(pb::execution_status::ExecutionStatus::Success(())),
          }),
          executed_epoch: 19,
          gas_used: Some(pb::GasCostSummary {
            computation_cost: 0,
            storage_cost: 0,
            storage_rebate: 0,
            non_refundable_storage_fee: 0,
          }),
          modified_at_versions: vec![
            pb::TransactionBlockEffectsModifiedAtVersions {
              object_id: Some(ObjectId {account_address: "0000000000000000000000000000000000000000000000000000000000000006".to_string() }),
              sequence_number: 1448000,
            }
          ],
          shared_objects: vec![pb::ObjectRef {
            object_id: Some(ObjectId {account_address: "0000000000000000000000000000000000000000000000000000000000000006".to_string() }),
            sequence_number: 1448000,
            digest: "2GB5NVhagD4fQ9P85WqtgX3nwFwVdqDPbKYBtGcziQYM".to_string(),
          }],
          transaction_digest: "D7CBWgtjcgMyn1YhRZ2q7okrmiCUYW4QA5gPZT6CRa2n".to_string(),
          created: vec![],
          mutated: vec![pb::OwnedObjectRef {
            owner: Some(pb::Owner {owner: Some(pb::owner::Owner::Shared(pb::Shared { initial_shared_version: 1 }))}),
            reference: Some(pb::ObjectRef {
              object_id: Some(ObjectId {account_address: "0000000000000000000000000000000000000000000000000000000000000006".to_string() }),
              sequence_number: 1448001,
              digest: "CDdzbah88YnaMJXjhpnqHy5BTo3YBAqckuD5uzfs2kyX".to_string(),
            }),
          }],
          unwrapped: vec![],
          deleted: vec![],
          unwrapped_then_deleted: vec![],
          wrapped: vec![],
          gas_object: Some(pb::OwnedObjectRef {
            owner: Some(pb::Owner {owner: Some(pb::owner::Owner::AddressOwner("0000000000000000000000000000000000000000000000000000000000000000".to_string()))}),
            reference: Some(pb::ObjectRef {
              object_id: Some(ObjectId {account_address: "0000000000000000000000000000000000000000000000000000000000000000".to_string() }),
              sequence_number: 0,
              digest: "11111111111111111111111111111111".to_string(),
            }),
          }),
          events_digest: None,
          dependencies: vec!["A5PHzo8quSTJGpay1S5Q6AKTjtYpSJPMUi3wchQkzBSX".to_string()],
        })
      ),
    };

    assert_eq!(expected, pb_transaction_effects);
  }
}
