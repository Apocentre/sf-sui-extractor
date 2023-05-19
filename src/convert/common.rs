use std::collections::HashMap;

use serde_json::Value;
use sui_json_rpc_types::{
  SuiArgument, SuiObjectRef, SuiExecutionStatus, SuiTransactionBlockEffectsModifiedAtVersions, OwnedObjectRef,
  SuiTransactionBlockEvents, SuiParsedData, SuiParsedMoveObject, SuiMoveStruct, SuiMoveValue, SuiMovePackage, SuiRawData, SuiRawMoveObject
};
use sui_types::{
  base_types::{ObjectID, ObjectType, MoveObjectType},
  TypeTag, gas::GasCostSummary, object::Owner, event::EventID, error::SuiObjectResponseError, id::UID
};
use crate::pb::sui::checkpoint::{self as pb};

pub fn convert_sui_object(source: &ObjectID) -> pb::ObjectId {
  pb::ObjectId {
    account_address: source.to_canonical_string(),
  }
}

pub fn convert_type_tag(source: &TypeTag) -> pb::TypeTag {
  let type_tag = match source {
    TypeTag::Bool => pb::type_tag::TypeTag::Bool(()),
    TypeTag::U8 => pb::type_tag::TypeTag::U8(()),
    TypeTag::U64 => pb::type_tag::TypeTag::U64(()),
    TypeTag::U128 => pb::type_tag::TypeTag::U128(()),
    TypeTag::Address => pb::type_tag::TypeTag::Address(()),
    TypeTag::Signer => pb::type_tag::TypeTag::Signer(()),
    TypeTag::Vector(type_tag) => pb::type_tag::TypeTag::Vector(Box::new(convert_type_tag(&*type_tag))),
    TypeTag::Struct(source) => pb::type_tag::TypeTag::Struct(pb::StructTag {
      address: source.address.to_canonical_string(),
      module: source.module.to_string(),
      name: source.name.to_string(),
      type_params: Some(pb::ListOfTypeTags {
        list: source.type_params.iter().map(convert_type_tag).collect(),
      }),
    }),
    TypeTag::U16 => pb::type_tag::TypeTag::U16(()),
    TypeTag::U32 => pb::type_tag::TypeTag::U32(()),
    TypeTag::U256 => pb::type_tag::TypeTag::U256(()),
  };

  pb::TypeTag {
    type_tag: Some(type_tag),
  }
}

pub fn convert_sui_json_value(source: &Value) -> pb::SuiJsonValue {
  let json_value = match source {
    Value::Null => pb::sui_json_value::Value::Null(()),
    Value::Bool(val) => pb::sui_json_value::Value::Bool(*val),
    Value::Number(val) => pb::sui_json_value::Value::Number(val.to_string()),
    Value::String(val) => pb::sui_json_value::Value::String(val.clone()),
    Value::Array(val) => pb::sui_json_value::Value::Array(pb::ListOfJsonValues {
      list: val.iter().map(convert_sui_json_value).collect(),
    }),
    Value::Object(_) => pb::sui_json_value::Value::Null(()),
  };

  pb::SuiJsonValue {
    value: Some(json_value),
  }
}

pub fn convert_sui_argument(source: &SuiArgument) -> pb::SuiArgument {
  let sui_arguments = match source {
    SuiArgument::GasCoin => pb::sui_argument::SuiArguments::GasCoin(()),
    SuiArgument::Input(val) => pb::sui_argument::SuiArguments::Input(*val as u32),
    SuiArgument::Result(val) => pb::sui_argument::SuiArguments::Result(*val as u32),
    SuiArgument::NestedResult(one, two) => pb::sui_argument::SuiArguments::NestedResult(pb::PairOfU32 {
      one: *one as u32,
      two: *two as u32,
    }),
  };

  pb::SuiArgument {
    sui_arguments: Some(sui_arguments),
  }
}

pub fn convert_sui_object_ref(source: &SuiObjectRef) -> pb::SuiObjectRef {
  pb::SuiObjectRef {
    object_id: Some(convert_sui_object(&source.object_id)),
    version: source.version.value(),
    digest: source.digest.into_inner().to_vec(),
  }
}

pub fn convert_sui_execution_status(source: &SuiExecutionStatus) -> pb::SuiExecutionStatus {
  let sui_execution_status = match source {
    SuiExecutionStatus::Success => pb::sui_execution_status::SuiExecutionStatus::Success(()),
    SuiExecutionStatus::Failure {error} => pb::sui_execution_status::SuiExecutionStatus::Failure(pb::Failure {
      error: error.clone(),
    })
  };
  
  pb::SuiExecutionStatus {
    sui_execution_status: Some(sui_execution_status),
  }
}

pub fn convert_gas_cost_summary(source: &GasCostSummary) -> pb::GasCostSummary {
  pb::GasCostSummary {
    computation_cost: source.computation_cost,
    storage_cost: source.storage_cost,
    storage_rebate: source.storage_rebate,
    non_refundable_storage_fee: source.non_refundable_storage_fee,
  }
}

pub fn convert_owner(source: &Owner) -> pb::Owner {
  let owner = match source {
    Owner::AddressOwner(val) => pb::owner::Owner::AddressOwner(val.to_vec()),
    Owner::ObjectOwner(val) => pb::owner::Owner::ObjectOwner(val.to_vec()),
    Owner::Shared {initial_shared_version} => pb::owner::Owner::Shared(pb::Shared {
      initial_shared_version: initial_shared_version.value(),
    }),
    Owner::Immutable => pb::owner::Owner::Immutable(()),
  };

  pb::Owner{
    owner: Some(owner)
  }
}

pub fn convert_owned_object_ref(source: &OwnedObjectRef) -> pb::OwnedObjectRef {
  pb::OwnedObjectRef {
    owner: Some(convert_owner(&source.owner)),
    reference: Some(convert_sui_object_ref(&source.reference)),
  }
}

pub fn convert_tx_block_effects_modified_at_versions(
  source: &SuiTransactionBlockEffectsModifiedAtVersions
) -> pb::SuiTransactionBlockEffectsModifiedAtVersions {
  pb::SuiTransactionBlockEffectsModifiedAtVersions {
    object_id: Some(convert_sui_object(&source.object_id())),
    sequence_number: source.sequence_number().value(),
  }
}

pub fn convert_event_id(source: &EventID) -> pb::EventId {
  pb::EventId {
    tx_digest: source.tx_digest.into_inner().to_vec(),
    event_seq: source.event_seq,
  }
}

pub fn convert_tx_block_events(source: &SuiTransactionBlockEvents) -> pb::SuiTransactionBlockEvents {
  let data = source.data.iter().map(|e| pb::SuiEvent {
    id: Some(convert_event_id(&e.id)),
    package_id: Some(convert_sui_object(&e.package_id)),
    transaction_module: e.transaction_module.clone().into_string(),
    sender: hex::encode(e.sender),
    r#type: Some(pb::StructTag {
      address: e.type_.address.to_canonical_string(),
      module: e.type_.module.to_string(),
      name: e.type_.name.to_string(),
      type_params: Some(pb::ListOfTypeTags {
        list: e.type_.type_params.iter().map(convert_type_tag).collect(),
      }),
    }),
    parsed_json: Some(convert_sui_json_value(&e.parsed_json)),
    bcs: e.bcs.clone(),
    timestamp_ms: e.timestamp_ms,
  })
  .collect();

  pb::SuiTransactionBlockEvents {
    data,
  }
}

pub fn convert_object_type(source: &ObjectType) -> pb::ObjectType {
  let object_type = match source {
    ObjectType::Package => pb::object_type::ObjectType::Package(()),
    ObjectType::Struct(source) => pb::object_type::ObjectType::Struct(convert_move_object_type(&source))
  };
  
  pb::ObjectType {
    object_type: Some(object_type)
  }
}

pub fn convert_move_object_type(source: &MoveObjectType) -> pb::MoveObjectType {
  let move_object_type = match source.clone().into_inner() {
    sui_types::base_types::MoveObjectType_::Other(source) => pb::move_object_type::MoveObjectType::Other(pb::StructTag {
      address: source.address.to_canonical_string(),
      module: source.module.to_string(),
      name: source.name.to_string(),
      type_params: Some(pb::ListOfTypeTags {
        list: source.type_params.iter().map(convert_type_tag).collect(),
      }),
    }),
    sui_types::base_types::MoveObjectType_::GasCoin => pb::move_object_type::MoveObjectType::GasCoin(()),
    sui_types::base_types::MoveObjectType_::StakedSui => pb::move_object_type::MoveObjectType::StakedSui(()),
    sui_types::base_types::MoveObjectType_::Coin(source) => pb::move_object_type::MoveObjectType::Coin(convert_type_tag(&source)),
  };
  
  pb::MoveObjectType {
    move_object_type: Some(move_object_type)
  }
}

pub fn convert_sui_object_response_error(source: &SuiObjectResponseError) -> pb::SuiObjectResponseError {
  let sui_object_response_error = match source {
    SuiObjectResponseError::NotExists {object_id} => pb::sui_object_response_error::SuiObjectResponseError::NotExists(
      pb::sui_object_response_error::NotExists {
        object_id: Some(convert_sui_object(&object_id)),
      },
    ),
    SuiObjectResponseError::DynamicFieldNotFound {parent_object_id} => pb::sui_object_response_error::SuiObjectResponseError::DynamicFieldNotFound(
      pb::sui_object_response_error::DynamicFieldNotFound {
        parent_object_id: Some(convert_sui_object(&parent_object_id)),
      },
    ),
    SuiObjectResponseError::Deleted {object_id, version, digest} => pb::sui_object_response_error::SuiObjectResponseError::Deleted(
      pb::sui_object_response_error::Deleted {
        object_id: Some(convert_sui_object(&object_id)),
        version: version.value(),
        digest: digest.into_inner().to_vec(),
      },
    ),
    SuiObjectResponseError::Unknown => pb::sui_object_response_error::SuiObjectResponseError::Unknown(()),
    SuiObjectResponseError::DisplayError {error} => pb::sui_object_response_error::SuiObjectResponseError::DisplayError(
      pb::sui_object_response_error::DisplayError {
        error: error.clone(),
      },
    ),
  };

  pb::SuiObjectResponseError {
    sui_object_response_error: Some(sui_object_response_error),
  }
}

pub fn convert_sui_parsed_data(source: &SuiParsedData) -> pb::SuiParsedData {
  let sui_parsed_data = match source {
    SuiParsedData::MoveObject(source) => pb::sui_parsed_data::SuiParsedData::MoveObject(
      convert_sui_parsed_move_object(source)
    ),
    SuiParsedData::Package(source) => pb::sui_parsed_data::SuiParsedData::Package(
      convert_sui_move_package(source)
    ),
  };

  pb::SuiParsedData {
    sui_parsed_data: Some(sui_parsed_data),
  }
}

pub fn convert_sui_parsed_move_object(source: &SuiParsedMoveObject) -> pb::SuiParsedMoveObject {
  pb::SuiParsedMoveObject {
    r#type: Some(pb::StructTag {
      address: source.type_.address.to_canonical_string(),
      module: source.type_.module.to_string(),
      name: source.type_.name.to_string(),
      type_params: Some(pb::ListOfTypeTags {
        list: source.type_.type_params.iter().map(convert_type_tag).collect(),
      }),
    }),
    has_public_transfer: source.has_public_transfer,
    fields: Some(convert_sui_move_struct(&source.fields)),
  }
}

pub fn convert_sui_move_struct(source: &SuiMoveStruct) -> pb::SuiMoveStruct {
  let sui_move_struct = match source {
    SuiMoveStruct::Runtime(source) => pb::sui_move_struct::SuiMoveStruct::Runtime(pb::ListOfSuiMoveValues {
      list: source.iter().map(convert_sui_move_value).collect(),
    }),
    SuiMoveStruct::WithTypes {type_, fields} => todo!(),
    SuiMoveStruct::WithFields(_) => todo!(),
  };

  pb::SuiMoveStruct {
    sui_move_struct: Some(sui_move_struct),
  }
}

pub fn convert_sui_move_value(source: &SuiMoveValue) -> pb::SuiMoveValue {
  let sui_move_value = match source {
    SuiMoveValue::Number(source) => pb::sui_move_value::SuiMoveValue::Number(*source),
    SuiMoveValue::Bool(source) => pb::sui_move_value::SuiMoveValue::Bool(*source),
    SuiMoveValue::Address(source) => pb::sui_move_value::SuiMoveValue::Address(source.to_vec()),
    SuiMoveValue::Vector(source) => pb::sui_move_value::SuiMoveValue::Vector(pb::ListOfSuiMoveValues {
      list: source.iter().map(convert_sui_move_value).collect(),
    }),
    SuiMoveValue::String(source) => pb::sui_move_value::SuiMoveValue::String(source.clone()),
    SuiMoveValue::UID {id} => pb::sui_move_value::SuiMoveValue::Uid(pb::Uid {
      id: Some(convert_sui_object(id)),
    }),
    SuiMoveValue::Struct(source) => pb::sui_move_value::SuiMoveValue::Struct(convert_sui_move_struct(source)),
    SuiMoveValue::Option(source) => pb::sui_move_value::SuiMoveValue::Option(
      Box::new(source.clone().map(|o| convert_sui_move_value(&o)).unwrap()),
    ),
  };

  pb::SuiMoveValue {
    sui_move_value: Some(sui_move_value),
  }
}

pub fn convert_uid(source: &UID) -> pb::Uid {
  pb::Uid {
    id: Some(convert_sui_object(source.object_id())),
  }
}

pub fn convert_sui_move_package(source: &SuiMovePackage) -> pb::SuiMovePackage {
  let mut disassembled = HashMap::new();
  for (k, v) in source.disassembled.clone() {
    disassembled.insert(k, convert_sui_json_value(&v));
  }

  pb::SuiMovePackage {
    disassembled,
  }
}

pub fn convert_sui_raw_data(source: &SuiRawData) -> pb::SuiRawData {
  let sui_raw_data = match source {
    SuiRawData::MoveObject(_) => todo!(),
    SuiRawData::Package(_) => todo!(),
  };

  pb::SuiRawData {
    sui_raw_data: Some(sui_raw_data),
  }
}

pub fn conver_sui_raw_move_object(source: &SuiRawMoveObject) -> pb::SuiRawMoveObject {
  pb::SuiRawMoveObject {
    r#type: Some(pb::StructTag {
      address: source.type_.address.to_canonical_string(),
      module: source.type_.module.to_string(),
      name: source.type_.name.to_string(),
      type_params: Some(pb::ListOfTypeTags {
        list: source.type_.type_params.iter().map(convert_type_tag).collect(),
      }),
    }),
    has_public_transfer: source.has_public_transfer,
    version: source.version.value(),
    bcs_bytes: source.bcs_bytes.clone(),
  }
}
