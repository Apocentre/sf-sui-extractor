use sui_json_rpc_types::{SuiTransactionBlockKind};
use crate::pb::sui::checkpoint as pb;

pub fn convert_sui_transaction_block_kind(source: &SuiTransactionBlockKind) -> Option<pb::SuiTransactionBlockKind> {
  let sui_transaction_block_kind = match source {
    SuiTransactionBlockKind::ChangeEpoch(source) => pb::sui_transaction_block_kind::SuiTransactionBlockKind::ChangeEpoch(
      pb::SuiChangeEpoch {
        epoch: todo!(),
        storage_charge: todo!(),
        computation_charge: todo!(),
        storage_rebate: todo!(),
        epoch_start_timestamp_ms: todo!(),
      }
    ),
    SuiTransactionBlockKind::Genesis(source) => pb::sui_transaction_block_kind::SuiTransactionBlockKind::Genesis(
      pb::SuiGenesisTransaction {
        objects: todo!(),
      }
    ),
    SuiTransactionBlockKind::ConsensusCommitPrologue(source) =>  pb::sui_transaction_block_kind::SuiTransactionBlockKind::ConsensusCommitPrologue(
      pb::SuiConsensusCommitPrologue {
        epoch: todo!(),
        round: todo!(),
        commit_timestamp_ms: todo!(),
    }
    ),
    SuiTransactionBlockKind::ProgrammableTransaction(source) => pb::sui_transaction_block_kind::SuiTransactionBlockKind::ProgrammableTransaction(
      pb::SuiProgrammableTransactionBlock {
        inputs: todo!(),
        commands: todo!(),
    }
    ),
  };
  
  Some(
    pb::SuiTransactionBlockKind {
      sui_transaction_block_kind: Some(sui_transaction_block_kind),
    }
  )
}
