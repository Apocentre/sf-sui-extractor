use sui_indexer::types::IndexedEvent;
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

pub fn convert_indexed_event(source: &IndexedEvent) -> pb::IndexedEvent {
  pb::IndexedEvent {
    tx_sequence_number: source.tx_sequence_number,
    event_sequence_number: source.event_sequence_number,
    checkpoint_sequence_number: source.checkpoint_sequence_number,
    transaction_digest: source.transaction_digest.base58_encode(),
    senders: source.senders.iter().map(convert_sui_address).collect::<Vec<_>>(),
    package: Some(convert_sui_object(&source.package)),
    module: source.module.clone(),
    event_type: source.event_type.clone(),
    bsc: source.bcs.clone(),
    timestamp_ms: source.timestamp_ms,
  }
}
