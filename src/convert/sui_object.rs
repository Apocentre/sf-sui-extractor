use sui_indexer::types::IndexedObjectChange;
use crate::pb::sui::checkpoint::{self as pb};
use super::common::{convert_owner, convert_struct_tag, convert_sui_address, convert_sui_object};

pub fn convert_tx_object_change(source: &IndexedObjectChange) -> pb::ObjectChange {
  let object_change = match source {
    IndexedObjectChange::Published {package_id, version, digest, modules} => {
      pb::object_change::ObjectChange::Published(pb::Published {
        package_id: Some(convert_sui_object(&package_id)),
        version: version.value(),
        digest: digest.base58_encode(),
        modules: modules.clone(),
      })
    }
    IndexedObjectChange::Transferred {sender, recipient, object_type, object_id, version, digest} => {
      pb::object_change::ObjectChange::Transferred(pb::Transferred {
        sender: convert_sui_address(&sender),
        recipient: Some(convert_owner(&recipient)),
        object_type: Some(convert_struct_tag(&object_type)),
        object_id: Some(convert_sui_object(&object_id)),
        version: version.value(),
        digest: digest.base58_encode(),
    })
    },
    IndexedObjectChange::Mutated {sender, owner, object_type, object_id, version, previous_version, digest} => {
      pb::object_change::ObjectChange::Mutated(pb::Mutated {
        sender: convert_sui_address(&sender),
        owner: Some(convert_owner(&owner)),
        object_type: Some(convert_struct_tag(&object_type)),
        object_id: Some(convert_sui_object(&object_id)),
        version: version.value(),
        previous_version: previous_version.value(),
        digest: digest.base58_encode(),
      })
    },
    IndexedObjectChange::Deleted {sender, object_type, object_id, version} => {
      pb::object_change::ObjectChange::Deleted(pb::Deleted {
        sender: convert_sui_address(&sender),
        object_type: Some(convert_struct_tag(&object_type)),
        object_id: Some(convert_sui_object(&object_id)),
        version: version.value(),
      })
    },
    IndexedObjectChange::Wrapped {sender, object_type, object_id, version} => {
      pb::object_change::ObjectChange::Wrapped(pb::Wrapped {
        sender: convert_sui_address(&sender),
        object_type: Some(convert_struct_tag(&object_type)),
        object_id: Some(convert_sui_object(&object_id)),
        version: version.value(),
      })
    },
    IndexedObjectChange::Created {sender, owner, object_type, object_id, version, digest} => {
      pb::object_change::ObjectChange::Created(pb::Created {
        sender: convert_sui_address(&sender),
        owner: Some(convert_owner(&owner)),
        object_type: Some(convert_struct_tag(&object_type)),
        object_id: Some(convert_sui_object(&object_id)),
        version: version.value(),
        digest: digest.base58_encode(),
      })
    },
  };

  pb::ObjectChange {
    object_change: Some(object_change),
  }
}

#[cfg(test)]
mod tests {
  use std::str::FromStr;
  use move_core_types::{account_address::AccountAddress, language_storage::StructTag};
use sui_indexer::types::IndexedObjectChange;
  use sui_types::{base_types::{ObjectID, SequenceNumber, SuiAddress}, digests::ObjectDigest, object::Owner, Identifier};
  use crate::{convert::sui_object::convert_tx_object_change, pb::sui::checkpoint::{self as pb, ListOfTypeTags}};

  #[test]
  fn converts_tx_object_change() {
    let source = IndexedObjectChange::Mutated {
      sender: SuiAddress::from_str("0x0000000000000000000000000000000000000000000000000000000000000000").unwrap(),
      owner: Owner::Shared {
        initial_shared_version: SequenceNumber::from_u64(1),
      },
      object_type: StructTag {
        address: AccountAddress::from_str("0000000000000000000000000000000000000000000000000000000000000002").unwrap(),
        module: Identifier::new("clock").unwrap(),
        name: Identifier::new("Clock").unwrap(),
        type_params: vec![],
      },
      object_id: ObjectID::from_str("0x0000000000000000000000000000000000000000000000000000000000000006").unwrap(),
      version: SequenceNumber::from_u64(1448001),
      previous_version: SequenceNumber::from_u64(1448000),
      digest: ObjectDigest::from_str("CDdzbah88YnaMJXjhpnqHy5BTo3YBAqckuD5uzfs2kyX").unwrap(),
    };
    let pb_object_change = convert_tx_object_change(&source);
    let expected = pb::ObjectChange {
        object_change: Some(pb::object_change::ObjectChange::Mutated(pb::Mutated {
          sender: "0000000000000000000000000000000000000000000000000000000000000000".to_string(),
          owner: Some(pb::Owner {owner: Some(pb::owner::Owner::Shared(pb::Shared {initial_shared_version: 1}))}),
          object_type: Some(pb::StructTag {
            address: "0000000000000000000000000000000000000000000000000000000000000002".to_string(),
            module: "clock".to_string(),
            name: "Clock".to_string(),
            type_params: Some(ListOfTypeTags {
              list: vec![],
            }),
          }),
          object_id: Some(pb::ObjectId {
            account_address: "0000000000000000000000000000000000000000000000000000000000000006".to_string(),
          }),
          version: 1448001,
          previous_version: 1448000,
          digest: "CDdzbah88YnaMJXjhpnqHy5BTo3YBAqckuD5uzfs2kyX".to_string(),
        })),
    };

    assert_eq!(expected, pb_object_change);
  }
}
