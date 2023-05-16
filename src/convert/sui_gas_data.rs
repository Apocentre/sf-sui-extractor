use sui_json_rpc_types::SuiGasData;
use crate::pb::sui::checkpoint as pb;

pub fn convert_sui_gas_data(source: &SuiGasData) -> pb::SuiGasData {
  pb::SuiGasData {
    payment: source.payment.iter().map(convert_sui_object_ref).collect(),
    owner: todo!(),
    price: todo!(),
    budget: todo!(),
  }
}
