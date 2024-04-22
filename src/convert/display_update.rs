use sui_indexer::models::display::StoredDisplay;
use crate::pb::sui::checkpoint::{self as pb};

pub fn convert_display_update(source: &StoredDisplay) -> pb::StoredDisplay {
  pb::StoredDisplay {
    object_type: source.object_type.clone(),
    id: source.id.clone(),
    version: source.version as i32,
    bcs: source.bcs.clone(),
  }
}
