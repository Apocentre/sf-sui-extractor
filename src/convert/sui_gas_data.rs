use sui_json_rpc_types::SuiGasData;
use crate::pb::sui::checkpoint as pb;
use super::common::convert_sui_object_ref;

pub fn convert_sui_gas_data(source: &SuiGasData) -> pb::SuiGasData {
  pb::SuiGasData {
    payment: source.payment.iter().map(convert_sui_object_ref).collect(),
    owner: hex::encode(source.owner),
    price: source.price,
    budget: source.budget,
  }
}
