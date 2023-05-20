use sui_json_rpc_types::{SuiTransactionBlockKind, SuiCallArg, SuiCommand, SuiObjectArg};
use crate::pb::sui::checkpoint as pb;
use super::common::{
  convert_sui_object, convert_type_tag, convert_sui_json_value, convert_sui_argument
};

pub fn convert_sui_call_arg(source: &SuiCallArg) -> pb::SuiCallArg {
  let sui_call_arg = match source {
    SuiCallArg::Object(source) => match source {
      SuiObjectArg::ImmOrOwnedObject {object_id, version, digest} => pb::sui_call_arg::SuiCallArg::Object(pb::SuiObjectArg {
        sui_object_arg: Some(pb::sui_object_arg::SuiObjectArg::ImmOrOwnedObject(pb::ImmOrOwnedObject {
          object_id: Some(convert_sui_object(&object_id)),
          version: version.value(),
          digest: digest.base58_encode(),
        }))
      }),
      SuiObjectArg::SharedObject {object_id, initial_shared_version, mutable} => pb::sui_call_arg::SuiCallArg::Object(pb::SuiObjectArg {
        sui_object_arg: Some(pb::sui_object_arg::SuiObjectArg::SharedObject(pb::SharedObject {
          object_id: Some(convert_sui_object(&object_id)),
          initial_shared_version: initial_shared_version.value(),
          mutable: *mutable,
        }))
      }),
    },
    SuiCallArg::Pure(source) => pb::sui_call_arg::SuiCallArg::Pure(pb::SuiPureValue {
      value_type: source.value_type().map(|v| convert_type_tag(&v)),
      value: Some(convert_sui_json_value(&source.value().to_json_value())),
    })
  };

  pb::SuiCallArg {
    sui_call_arg: Some(sui_call_arg),
  }
}

pub fn convert_sui_command(source: &SuiCommand) -> pb::SuiCommand {
  let sui_command = match source {
    SuiCommand::MoveCall(source) => pb::sui_command::SuiCommand::MoveCall(pb::SuiProgrammableMoveCall {
      package: Some(convert_sui_object(&source.package)),
      module: source.module.to_string(),
      function: source.function.to_string(),
      type_arguments: source.type_arguments.clone(),
      arguments: source.arguments.iter().map(convert_sui_argument).collect(),
    }),
    SuiCommand::TransferObjects(val1, val2) => pb::sui_command::SuiCommand::TransferObjects(pb::TransferObjectsPair {
      one: val1.iter().map(convert_sui_argument).collect(),
      two: Some(convert_sui_argument(val2)),
    }),
    SuiCommand::SplitCoins(val1, val2) => pb::sui_command::SuiCommand::SplitCoins(pb::SplitCoinsPair {
      one: Some(convert_sui_argument(val1)),
      two: val2.iter().map(convert_sui_argument).collect(),
    }),
    SuiCommand::MergeCoins(val1, val2) => pb::sui_command::SuiCommand::MergeCoins(pb::MergeCoinsPair {
      one: Some(convert_sui_argument(val1)),
      two: val2.iter().map(convert_sui_argument).collect(),
    }),
    SuiCommand::Publish(val) => pb::sui_command::SuiCommand::Publish(pb::ListOfObjects {
      list: val.iter().map(convert_sui_object).collect(),
    }),
    SuiCommand::Upgrade(val1, val2, val3) => pb::sui_command::SuiCommand::Upgrade(pb::SuiCommandUpgrade {
      one: Some(pb::ListOfObjects{
        list: val1.iter().map(convert_sui_object).collect(),
      }),
      two: Some(convert_sui_object(val2)),
      three: Some(convert_sui_argument(val3)),
    }),
    SuiCommand::MakeMoveVec(val1, val2) => pb::sui_command::SuiCommand::MakeMoveVec(pb::MakeMoveVecPair {
      one: val1.clone(),
      two: val2.iter().map(convert_sui_argument).collect(),
    }),
  };

  pb::SuiCommand {
    sui_command: Some(sui_command),
  }
}

pub fn convert_sui_transaction_block_kind(source: &SuiTransactionBlockKind) -> Option<pb::SuiTransactionBlockKind> {
  let sui_transaction_block_kind = match source {
    SuiTransactionBlockKind::ChangeEpoch(source) => pb::sui_transaction_block_kind::SuiTransactionBlockKind::ChangeEpoch(
      pb::SuiChangeEpoch {
        epoch: source.epoch,
        storage_charge: source.storage_charge,
        computation_charge: source.computation_charge,
        storage_rebate: source.storage_rebate,
        epoch_start_timestamp_ms: source.epoch_start_timestamp_ms,
      }
    ),
    SuiTransactionBlockKind::Genesis(source) => pb::sui_transaction_block_kind::SuiTransactionBlockKind::Genesis(
      pb::SuiGenesisTransaction {
        objects: source.objects.iter().map(convert_sui_object).collect(),
      }
    ),
    SuiTransactionBlockKind::ConsensusCommitPrologue(source) =>  pb::sui_transaction_block_kind::SuiTransactionBlockKind::ConsensusCommitPrologue(
      pb::SuiConsensusCommitPrologue {
        epoch: source.epoch,
        round: source.round,
        commit_timestamp_ms: source.commit_timestamp_ms,
    }
    ),
    SuiTransactionBlockKind::ProgrammableTransaction(source) => pb::sui_transaction_block_kind::SuiTransactionBlockKind::ProgrammableTransaction(
      pb::SuiProgrammableTransactionBlock {
        inputs: source.inputs.iter().map(convert_sui_call_arg).collect(),
        commands: source.commands.iter().map(convert_sui_command).collect(),
      }
    ),
  };
  
  Some(
    pb::SuiTransactionBlockKind {
      sui_transaction_block_kind: Some(sui_transaction_block_kind),
    }
  )
}
