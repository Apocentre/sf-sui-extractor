use std::collections::HashMap;
use sui_json_rpc_types::{SuiObjectData, DisplayFieldsResponse};
use crate::pb::sui::checkpoint as pb;
use super::common::{
  convert_sui_object, convert_object_type, convert_owner, convert_sui_object_response_error,
};

fn convert_display_fields_response(source: &DisplayFieldsResponse) -> pb::DisplayFieldsResponse {
  pb::DisplayFieldsResponse {
    data: source.data.map(|d| Into::<HashMap<String, String>>(d)),
    error: source.error.map(convert_sui_object_response_error),
  }
}

pub fn convert_sui_object_data(source: &SuiObjectData) -> pb::SuiObjectData {
  pb::SuiObjectData {
    object_id: Some(convert_sui_object(&source.object_id)),
    version: source.version.value(),
    digest: source.digest.into_inner().to_vec(),
    r#type: source.type_.as_ref().map(|t| convert_object_type(&t)),
    owner: source.owner.as_ref().map(convert_owner),
    previous_transaction: source.previous_transaction.map(|pt| pt.into_inner().to_vec()),
    storage_rebate: source.storage_rebate.map(|sr| sr),
    display: source.display.as_ref().map(convert_display_fields_response),
    content: todo!(),
    bcs: todo!(),
  }
}
