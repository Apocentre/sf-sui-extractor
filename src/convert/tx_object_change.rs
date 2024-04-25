
use sui_indexer::{handlers::TransactionObjectChangesToCommit, types::OwnerType};
use crate::pb::sui::checkpoint::{self as pb, owner_type};
use super::common::{convert_data, convert_owner, convert_sui_address, convert_sui_object};

pub fn convert_tx_object_changes(source: &TransactionObjectChangesToCommit) -> pb::TransactionObjectChange {
  let changed_objects = source.changed_objects.iter().map(|changed_object| {
    pb::IndexedObject {
      object_id: Some(convert_sui_object(&changed_object.object_id)),
      object_version: changed_object.object_version,
      object_digest: changed_object.object_digest.base58_encode(),
      checkpoint_sequence_number: changed_object.checkpoint_sequence_number,
      owner_type: Some(convert_owner_type(&changed_object.owner_type)),
      owner_id: changed_object.owner_id.map(|owner_id| convert_sui_address(&owner_id)),
      object: Some(convert_object(&changed_object.object)),
      coin_type: changed_object.coin_type.clone(),
      coin_balance: changed_object.coin_balance,
    }
  }).collect::<Vec<_>>();

  let deleted_objects = source.deleted_objects.iter().map(|deleted_object| {
    pb::IndexedDeletedObject {
      object_id: Some(convert_sui_object(&deleted_object.object_id)),
      object_version: deleted_object.object_version,
      checkpoint_sequence_number: deleted_object.checkpoint_sequence_number,
    }
  }).collect::<Vec<_>>();
  
  pb::TransactionObjectChange {
    changed_objects,
    deleted_objects,
  }
}

fn convert_object(source: &sui_types::object::Object) -> pb::Object {
  pb::Object {
    data: Some(convert_data(&source.data)),
    owner: Some(convert_owner(&source.owner)),
    previous_transaction: source.previous_transaction.base58_encode(),
    storage_rebate: source.storage_rebate,
  }
}

fn convert_owner_type(source: &OwnerType) -> pb::OwnerType {
  let owner_type = match source {
    OwnerType::Immutable => owner_type::OwnerType::Immutable(()),
    OwnerType::Address => owner_type::OwnerType::Address(()),
    OwnerType::Object => owner_type::OwnerType::Object(()),
    OwnerType::Shared => owner_type::OwnerType::Shared(()),
  };
  pb::OwnerType {
    owner_type: Some(owner_type),
  }
}
