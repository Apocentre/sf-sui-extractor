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

#[cfg(test)]
mod tests {
  use sui_indexer::models::display::StoredDisplay;
  use crate::pb::sui::checkpoint::{self as pb};
  use super::convert_display_update;

  #[test]
  fn converts_display_update() {
    let source = StoredDisplay {
        object_type: "some_object_tupe".to_string(),
        id: vec![1, 2, 3],
        version: 1,
        bcs: vec![1, 2, 3, 4, 5, 6, 7, 8],
    };
    let pb_stored_display = convert_display_update(&source);
    let expected = pb::StoredDisplay {
      object_type: "some_object_tupe".to_string(),
      id: vec![1, 2, 3],
      version: 1,
      bcs: vec![1, 2, 3, 4, 5, 6, 7, 8],
    };

    assert_eq!(expected, pb_stored_display);
  }
}
