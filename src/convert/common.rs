use std::collections::{BTreeMap, HashMap};
use base58::ToBase58;
use move_core_types::language_storage::{ModuleId, StructTag};
use serde_json::Value;
use sui_types::{
  base_types::{AuthorityName, MoveObjectType, ObjectID, ObjectRef, SuiAddress, SequenceNumber},
  committee::StakeUnit, gas::GasCostSummary, messages_checkpoint::CheckpointCommitment,
  move_package::{TypeOrigin, UpgradeInfo}, object::{Data, Owner}, transaction::Argument, TypeTag,
};
use crate::pb::sui::checkpoint::{self as pb};

pub fn convert_sui_object(source: &ObjectID) -> pb::ObjectId {
  pb::ObjectId {
    account_address: source.to_canonical_string(false),
  }
}

pub fn convert_sui_address(address: &SuiAddress) -> String {
  address.to_string().replace("0x", "")
}

pub fn convert_object_ref(obj_ref: &ObjectRef) -> pb::ObjectRef {
  pb::ObjectRef {
    object_id: Some(convert_sui_object(&obj_ref.0)),
    sequence_number: obj_ref.1.value(),
    digest: obj_ref.2.base58_encode(),
  }
}

pub fn convert_struct_tag(source: &StructTag) -> pb::StructTag {
  pb::StructTag {
    address: source.address.to_canonical_string(false),
    module: source.module.to_string(),
    name: source.name.to_string(),
    type_params: Some(pb::ListOfTypeTags {
      list: source.type_params.iter().map(convert_type_tag).collect(),
    }),
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
    TypeTag::Struct(source) => pb::type_tag::TypeTag::Struct(convert_struct_tag(&source)),
    TypeTag::U16 => pb::type_tag::TypeTag::U16(()),
    TypeTag::U32 => pb::type_tag::TypeTag::U32(()),
    TypeTag::U256 => pb::type_tag::TypeTag::U256(()),
  };

  pb::TypeTag {
    type_tag: Some(type_tag),
  }
}

pub fn convert_sui_json_value(source: &Value) -> pb::Value {
  let json_value = match source {
    Value::Null => pb::value::Value::Null(()),
    Value::Bool(val) => pb::value::Value::Bool(*val),
    Value::Number(val) => pb::value::Value::Number(val.to_string()),
    Value::String(val) => pb::value::Value::String(val.clone()),
    Value::Array(val) => pb::value::Value::Array(pb::ListOfValues {
      list: val.iter().map(convert_sui_json_value).collect(),
    }),
    Value::Object(_) => pb::value::Value::Null(()),
  };

  pb::Value {
    value: Some(json_value),
  }
}

pub fn convert_sui_argument(source: &Argument) -> pb::SuiArgument {
  let sui_arguments = match source {
    Argument::GasCoin => pb::sui_argument::SuiArguments::GasCoin(()),
    Argument::Input(val) => pb::sui_argument::SuiArguments::Input(*val as u32),
    Argument::Result(val) => pb::sui_argument::SuiArguments::Result(*val as u32),
    Argument::NestedResult(one, two) => pb::sui_argument::SuiArguments::NestedResult(pb::PairOfU32 {
      one: *one as u32,
      two: *two as u32,
    }),
  };

  pb::SuiArgument {
    sui_arguments: Some(sui_arguments),
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
    Owner::AddressOwner(val) => pb::owner::Owner::AddressOwner(hex::encode(val)),
    Owner::ObjectOwner(val) => pb::owner::Owner::ObjectOwner(hex::encode(val)),
    Owner::Shared {initial_shared_version} => pb::owner::Owner::Shared(pb::Shared {
      initial_shared_version: initial_shared_version.value(),
    }),
    Owner::Immutable => pb::owner::Owner::Immutable(()),
  };

  pb::Owner{
    owner: Some(owner)
  }
}

pub fn convert_data(data: &Data) -> pb::Data {
  let data = match data {
    Data::Move(m) => pb::data::Data::Move(pb::MoveObject {
      r#type: Some(convert_move_object_type(m.type_())),
      has_public_transfer: m.has_public_transfer(),
      version: m.version().value(),
      contents: m.contents().to_vec(),
    }),
    Data::Package(p) => pb::data::Data::Package(pb::MovePackage {
      id: Some(convert_sui_object(&p.id())),
      version: p.version().value(),
      type_origin_table: p.type_origin_table().iter().map(convert_type_origin).collect::<Vec<_>>(),
      module_map: convert_module_map(&p.serialized_module_map()),
      linkage_table: convert_linkage_table(p.linkage_table()),
    }),
  };
  
  pb::Data {
    data: Some(data),
  }
}

fn convert_linkage_table(linkage_table: &BTreeMap<ObjectID, UpgradeInfo>) -> Vec<pb::LinkageTablePair> {
  let mut result = Vec::new();

  for (k, v) in linkage_table.iter() {
    result.push(pb::LinkageTablePair {
      key: Some(convert_sui_object(k)),
      value: Some(convert_upgrade_info(v)),
    });
  }

  result
}

fn convert_module_map(module_map: &BTreeMap<String, Vec<u8>>) -> HashMap<String, Vec<u8>> {
  let mut map = HashMap::new();

  for (k, v) in module_map.iter() {
    map.insert(k.clone(), v.clone());
  }

  map
}

pub fn convert_owned_object_ref(source: &(ObjectRef, Owner)) -> pb::OwnedObjectRef {
  pb::OwnedObjectRef {
    owner: Some(convert_owner(&source.1)),
    reference: Some(convert_object_ref(&source.0)),
  }
}

pub fn convert_tx_block_effects_modified_at_versions(source: &(ObjectID, SequenceNumber)) -> pb::TransactionBlockEffectsModifiedAtVersions {
  pb::TransactionBlockEffectsModifiedAtVersions {
    object_id: Some(convert_sui_object(&source.0)),
    sequence_number: source.1.value(),
  }
}
pub fn convert_move_object_type(source: &MoveObjectType) -> pb::MoveObjectType {
  let move_object_type = match source.clone().into_inner() {
    sui_types::base_types::MoveObjectType_::Other(source) => pb::move_object_type::MoveObjectType::Other(pb::StructTag {
      address: source.address.to_canonical_string(false),
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

pub fn convert_type_origin(source: &TypeOrigin) -> pb::TypeOrigin {
  pb::TypeOrigin {
    module_name: source.module_name.clone(),
    struct_name: source.struct_name.clone(),
    package: Some(convert_sui_object(&source.package)),
  }
}

pub fn convert_upgrade_info(source: &UpgradeInfo) -> pb::UpgradeInfo {
  pb::UpgradeInfo {
    upgraded_id: Some(convert_sui_object(&source.upgraded_id)),
    upgraded_version: source.upgraded_version.value(),
  }
}

pub fn convert_checkpoint_commitment(source: &CheckpointCommitment) -> pb::CheckpointCommitment {
  let checkpoint_commitment = match source {
    CheckpointCommitment::ECMHLiveObjectSetDigest(source) => pb::checkpoint_commitment::CheckpointCommitment::EcmhLiveObjectSetDigest(
      pb::EcmhLiveObjectSetDigest {
        digest: source.digest.into_inner().to_base58(),
      }
    )
  };

  pb::CheckpointCommitment {
    checkpoint_commitment: Some(checkpoint_commitment),
  }
}

pub fn convert_next_epoch_committee(source: &(AuthorityName, StakeUnit)) -> pb::NextEpochCommittee {
  pb::NextEpochCommittee {
    authority_name: base64::encode(source.0.as_ref()),
    stake_unit: source.1,
  }
}

pub fn convert_module_id(source: &ModuleId) -> pb::ModuleId {
  pb::ModuleId {
    address: source.address().to_canonical_string(false),
    name: source.name().to_string(),
  }
}
