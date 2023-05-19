use sui_indexer::{models::objects::ObjectStatus};
use sui_json_rpc_types::SuiObjectData;
use crate::pb::sui::checkpoint as pb;
use super::{sui_object_data::convert_sui_object_data};

fn convert_object_status(source: ObjectStatus) -> pb::ObjectStatus {
  let object_status = match source {
    ObjectStatus::Created => pb::object_status::ObjectStatus::Created(()),
    ObjectStatus::Mutated => pb::object_status::ObjectStatus::Mutated(()),
    ObjectStatus::Deleted => pb::object_status::ObjectStatus::Deleted(()),
    ObjectStatus::Wrapped => pb::object_status::ObjectStatus::Wrapped(()),
    ObjectStatus::Unwrapped => pb::object_status::ObjectStatus::Unwrapped(()),
    ObjectStatus::UnwrappedThenDeleted => pb::object_status::ObjectStatus::UnwrappedThenDeleted(()),
  };

  pb::ObjectStatus {
    object_status: Some(object_status),
  }
}

pub fn convert_object_change(source: &(ObjectStatus, SuiObjectData)) -> pb::ChangedObject {
  pb::ChangedObject {
    status: Some(convert_object_status(source.0)),
    data: Some(convert_sui_object_data(&source.1)),
  }
}
