use serde_json::Value;
use sui_json_rpc_types::{
  SuiArgument, SuiObjectRef, SuiExecutionStatus, SuiTransactionBlockEffectsModifiedAtVersions, OwnedObjectRef,
  SuiTransactionBlockEvents
};
use sui_types::{base_types::ObjectID, TypeTag, gas::GasCostSummary, object::Owner, event::EventID};
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

pub fn convert_owned_object_ref(source: &OwnedObjectRef) -> pb::OwnedObjectRef {
  let owner = match source.owner {
    Owner::AddressOwner(val) => pb::owner::Owner::AddressOwner(val.to_vec()),
    Owner::ObjectOwner(val) => pb::owner::Owner::ObjectOwner(val.to_vec()),
    Owner::Shared {initial_shared_version} => pb::owner::Owner::Shared(pb::Shared {
      initial_shared_version: initial_shared_version.value(),
    }),
    Owner::Immutable => pb::owner::Owner::Immutable(()),
  };

  pb::OwnedObjectRef {
    owner: Some(pb::Owner{
      owner: Some(owner)
    }),
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
