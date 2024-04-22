use sui_indexer::types::IndexedObjectChange;
use crate::pb::sui::checkpoint::{self as pb};
use super::common::{convert_owner, convert_stuct_tag, convert_sui_address, convert_sui_object};

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
        object_type: Some(convert_stuct_tag(&object_type)),
        object_id: Some(convert_sui_object(&object_id)),
        version: version.value(),
        digest: digest.base58_encode(),
    })
    },
    IndexedObjectChange::Mutated {sender, owner, object_type, object_id, version, previous_version, digest} => {
      pb::object_change::ObjectChange::Mutated(pb::Mutated {
        sender: convert_sui_address(&sender),
        owner: Some(convert_owner(&owner)),
        object_type: Some(convert_stuct_tag(&object_type)),
        object_id: Some(convert_sui_object(&object_id)),
        version: version.value(),
        previous_version: previous_version.value(),
        digest: digest.base58_encode(),
      })
    },
    IndexedObjectChange::Deleted {sender, object_type, object_id, version} => {
      pb::object_change::ObjectChange::Deleted(pb::Deleted {
        sender: convert_sui_address(&sender),
        object_type: Some(convert_stuct_tag(&object_type)),
        object_id: Some(convert_sui_object(&object_id)),
        version: version.value(),
      })
    },
    IndexedObjectChange::Wrapped {sender, object_type, object_id, version} => {
      pb::object_change::ObjectChange::Wrapped(pb::Wrapped {
        sender: convert_sui_address(&sender),
        object_type: Some(convert_stuct_tag(&object_type)),
        object_id: Some(convert_sui_object(&object_id)),
        version: version.value(),
      })
    },
    IndexedObjectChange::Created {sender, owner, object_type, object_id, version, digest} => {
      pb::object_change::ObjectChange::Created(pb::Created {
        sender: convert_sui_address(&sender),
        owner: Some(convert_owner(&owner)),
        object_type: Some(convert_stuct_tag(&object_type)),
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
