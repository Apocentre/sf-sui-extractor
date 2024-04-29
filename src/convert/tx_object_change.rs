
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
      tx_digest: changed_object.tx_digest.base58_encode(),
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
      tx_digest: deleted_object.tx_digest.base58_encode(),
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

#[cfg(test)]
mod tests {
  use std::str::FromStr;
  use move_core_types::{account_address::AccountAddress, language_storage::StructTag};
use sui_indexer::{handlers::TransactionObjectChangesToCommit, types::{IndexedObject, OwnerType}};
  use sui_types::{base_types::{ObjectID, SequenceNumber}, digests::{ObjectDigest, TransactionDigest}, object::{MoveObject, Object, Owner}, Identifier};
  use crate::pb::sui::checkpoint::{self as pb};
  use super::convert_tx_object_changes;

  #[test]
  fn converts_tx_object_changes() {
    let object = unsafe {
      let tag = StructTag {
        address: AccountAddress::from_str("0000000000000000000000000000000000000000000000000000000000000002").unwrap(),
        module: Identifier::from_str("clock").unwrap(),
        name: Identifier::from_str("Clock").unwrap(),
        type_params: vec![],
      };

      let object =  MoveObject::new_from_execution_with_limit(
        tag.into(),
        false,
        SequenceNumber::from_u64(1448001),
        vec![0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 6, 51, 241, 16, 218, 135, 1, 0, 0],
        u64::MAX,
      ).unwrap();

      object
    };

    let source = TransactionObjectChangesToCommit {
        changed_objects: vec![
          IndexedObject {
            object_id: ObjectID::from_str("0x0000000000000000000000000000000000000000000000000000000000000006").unwrap(),
            object_version: 1448001,
            object_digest: ObjectDigest::from_str("CDdzbah88YnaMJXjhpnqHy5BTo3YBAqckuD5uzfs2kyX").unwrap(),
            checkpoint_sequence_number: 1448000,
            tx_digest: TransactionDigest::from_str("D7CBWgtjcgMyn1YhRZ2q7okrmiCUYW4QA5gPZT6CRa2n").unwrap(),
            owner_type: OwnerType::Shared,
            owner_id: None,
            object: Object::new_move(
              object,
              Owner::Shared {
                initial_shared_version: SequenceNumber::from_u64(1),
              },
              TransactionDigest::from_str("CDdzbah88YnaMJXjhpnqHy5BTo3YBAqckuD5uzfs2kyX").unwrap(),
            ),
            coin_type: None,
            coin_balance: None,
            df_info: None,
          }
        ],
        deleted_objects: vec![],
    };
    let pb_tx_object_change = convert_tx_object_changes(&source);
    let expected = pb::TransactionObjectChange {
        changed_objects: vec![
          pb::IndexedObject {
            object_id: Some(pb::ObjectId {
              account_address: "0000000000000000000000000000000000000000000000000000000000000006".to_string(),
            }),
            object_version: 1448001,
            object_digest: "CDdzbah88YnaMJXjhpnqHy5BTo3YBAqckuD5uzfs2kyX".to_string(),
            checkpoint_sequence_number: 1448000,
            tx_digest: "D7CBWgtjcgMyn1YhRZ2q7okrmiCUYW4QA5gPZT6CRa2n".to_string(),
            owner_type: Some(pb::OwnerType {
              owner_type: Some(pb::owner_type::OwnerType::Shared(()))
            }),
            owner_id: None,
            object: Some(pb::Object {
              data: Some(pb::Data {
                data: Some(pb::data::Data::Move(pb::MoveObject {
                  r#type: Some(pb::MoveObjectType {
                    move_object_type: Some(pb::move_object_type::MoveObjectType::Other(pb::StructTag {
                      address: "0000000000000000000000000000000000000000000000000000000000000002".to_string(),
                      module: "clock".to_string(),
                      name: "Clock".to_string(),
                      type_params: Some(pb::ListOfTypeTags {
                        list: vec![],
                      }),
                    })),
                  }),
                  has_public_transfer: false,
                  version: 1448001,
                  contents: vec![0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 6, 51, 241, 16, 218, 135, 1, 0, 0],
                }))
              }),
              owner: Some(pb::Owner {owner: Some(pb::owner::Owner::Shared(pb::Shared {initial_shared_version: 1}))}),
              previous_transaction: "CDdzbah88YnaMJXjhpnqHy5BTo3YBAqckuD5uzfs2kyX".to_string(),
              storage_rebate: 0,
            }),
            coin_type: None,
            coin_balance: None,
          }
        ],
        deleted_objects: vec![],
    };

    assert_eq!(expected, pb_tx_object_change);
  }
}
