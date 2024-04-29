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
    bcs: source.bcs.clone(),
    timestamp_ms: source.timestamp_ms,
  }
}

#[cfg(test)]
mod tests {
  use std::str::FromStr;

use sui_indexer::types::IndexedEvent;
  use sui_types::{base_types::{ObjectID, SuiAddress}, digests::TransactionDigest};
  use crate::{convert::sui_event::convert_indexed_event, pb::sui::checkpoint::{self as pb, ObjectId}};

  #[test]
  fn converts_indexed_event() {
    let source = IndexedEvent {
        tx_sequence_number: 1,
        event_sequence_number: 10,
        checkpoint_sequence_number: 1000,
        transaction_digest: TransactionDigest::from_str("2GB5NVhagD4fQ9P85WqtgX3nwFwVdqDPbKYBtGcziQYM").unwrap(),
        senders: vec![SuiAddress::from_str("0x0000000000000000000000000000000000000000000000000000000000000000").unwrap()],
        package: ObjectID::from_str("0x0000000000000000000000000000000000000000000000000000000000000006").unwrap(),
        module: "module_id".to_string(),
        event_type: "event_1".to_string(),
        bcs: vec![1,2 ,3 , 4, 5],
        timestamp_ms: 1682990756147,
    };
    let pb_indexed_event = convert_indexed_event(&source);
    let expected = pb::IndexedEvent {
      tx_sequence_number: 1,
      event_sequence_number: 10,
      checkpoint_sequence_number: 1000,
      transaction_digest: "2GB5NVhagD4fQ9P85WqtgX3nwFwVdqDPbKYBtGcziQYM".to_string(),
      senders: vec!["0000000000000000000000000000000000000000000000000000000000000000".to_string()],
      package: Some(ObjectId {
        account_address: "0000000000000000000000000000000000000000000000000000000000000006".to_string(),
      }),
      module: "module_id".to_string(),
      event_type: "event_1".to_string(),
      bcs: vec![1,2 ,3 , 4, 5],
      timestamp_ms: 1682990756147,
    };

    assert_eq!(expected, pb_indexed_event);
  }
}
