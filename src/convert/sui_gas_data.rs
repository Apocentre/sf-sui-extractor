use base58::ToBase58;
use sui_json_rpc_types::SuiGasData;
use crate::pb::sui::checkpoint as pb;
use super::common::convert_sui_object_ref;

pub fn convert_sui_gas_data(source: &SuiGasData) -> pb::SuiGasData {
  pb::SuiGasData {
    payment: source.payment.iter().map(convert_sui_object_ref).collect(),
    owner: source.owner.as_ref().to_base58(),
    price: source.price,
    budget: source.budget,
  }
}
