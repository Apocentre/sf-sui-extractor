use sui_types::event::Event;

use crate::pb::sui::checkpoint::{self as pb};

use super::common::{convert_struct_tag, convert_sui_address, convert_sui_object};

pub fn convert_event(source: &Event) -> pb::Event {
  pb::Event {
    package_id: Some(convert_sui_object(&source.package_id)),
    transaction_module: source.transaction_module.clone().into_string(),
    sender: convert_sui_address(&source.sender),
    r#type: Some(convert_struct_tag(&source.type_)),
    contents: source.contents.clone(),
  }
}
