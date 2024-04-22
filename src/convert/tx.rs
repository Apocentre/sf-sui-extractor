use shared_crypto::intent::{AppId, Intent, IntentMessage, IntentScope, IntentVersion};
use sui_indexer::types::IndexedTransaction;
use sui_types::{
  base_types::{ObjectID, ObjectRef}, messages_consensus::{ConsensusCommitPrologue, ConsensusCommitPrologueV2}, 
  transaction::{
    Argument, AuthenticatorStateUpdate, CallArg, ChangeEpoch, Command, EndOfEpochTransactionKind, GenesisObject,
    GenesisTransaction, ObjectArg, ProgrammableMoveCall, ProgrammableTransaction, RandomnessStateUpdate, SenderSignedData,
    TransactionData, TransactionDataAPI, TransactionKind,
  },
  TypeTag
};
use crate::pb::sui::checkpoint::{self as pb};

use super::common::{convert_owner, convert_sui_argument, convert_sui_object, convert_type_tag};

fn convert_intent_message(source: IntentMessage<TransactionData>) -> pb::IntentMessage {
  pb::IntentMessage {
    intent: Some(convert_intent(source.intent)),
    value: Some(convert_intent_value(source.value)),
  }
}

fn convert_intent_value(source: TransactionData) -> pb::TransactionData {
  let tx_data = match source {
    TransactionData::V1(tx_data_v1) => {
      pb::transaction_data::TxData::V1(pb::TransactionDataV1 {
        kind: Some(convert_tx_kind(tx_data_v1.kind())),
        sender: todo!(),
        gas_data: todo!(),
        expiration: todo!(),
      })
    }
  };

  pb::TransactionData {
    tx_data: Some(tx_data),
  }
}

fn convert_tx_kind(source: &TransactionKind) -> pb::TransactionKind {
  let kind = match source {
    TransactionKind::ProgrammableTransaction(pt) => convert_programmable_tx_kind(pt),
    TransactionKind::ChangeEpoch(ce) => convert_change_epoch(ce),
    TransactionKind::Genesis(g) => convert_genesis(g),
    TransactionKind::ConsensusCommitPrologue(ccp) => convert_commit_prologue(ccp),
    TransactionKind::AuthenticatorStateUpdate(asu) => convert_authenticator_state_update(asu),
    TransactionKind::EndOfEpochTransaction(eet) => convert_end_of_epoch_transaction(eet),
    TransactionKind::RandomnessStateUpdate(rsu) => convert_randomeness_state_update(rsu),
    TransactionKind::ConsensusCommitPrologueV2(ccp_v2) => convert_commit_prologue_v2(ccp_v2),
  };

  pb::TransactionKind {
    transaction_kind: Some(kind),
  }
}

fn convert_commit_prologue_v2(ccp_v2: &ConsensusCommitPrologueV2) -> pb::transaction_kind::TransactionKind {
  todo!()
}

fn convert_randomeness_state_update(rsu: &RandomnessStateUpdate) -> pb::transaction_kind::TransactionKind {
  todo!()
}

fn convert_end_of_epoch_transaction(eet: &[EndOfEpochTransactionKind]) -> pb::transaction_kind::TransactionKind {
  todo!()
}

fn convert_authenticator_state_update(asu: &AuthenticatorStateUpdate) -> pb::transaction_kind::TransactionKind {
  todo!()
}

fn convert_commit_prologue(ccp: &ConsensusCommitPrologue) -> pb::transaction_kind::TransactionKind {
  todo!()
}

fn convert_genesis(g: &GenesisTransaction) -> pb::transaction_kind::TransactionKind {
  pb::transaction_kind::TransactionKind::Genesis(pb::GenesisTransaction {
    objects: g.objects.iter().map(convert_genesis_obj).collect::<Vec<_>>(),
  })
}

fn convert_genesis_obj(genesis_obj: &GenesisObject) -> pb::GenesisObject {
  let genesis_obj = match genesis_obj {
    GenesisObject::RawObject {data, owner} => pb::genesis_object::GenesisObject {
      data: convert_data(data),
      data: convert_owner(owner),
    },
  };

  pb::GenesisObject {
    genesis_object: Some(genesis_obj)
  } 
}

fn convert_change_epoch(ce: &ChangeEpoch) -> pb::transaction_kind::TransactionKind {
  pb::transaction_kind::TransactionKind::ChangeEpoch(pb::ChangeEpoch {
    epoch: ce.epoch,
    storage_charge: ce.storage_charge,
    computation_charge: ce.computation_charge,
    storage_rebate: ce.storage_rebate,
    epoch_start_timestamp_ms: ce.epoch_start_timestamp_ms,
  })
}

fn convert_programmable_tx_kind(pt: &ProgrammableTransaction) -> pb::transaction_kind::TransactionKind {
  let inputs = pt.inputs.iter().map(|c| {
    let call_arg = match c {
      CallArg::Pure(p) => pb::call_arg::CallArg::Pure(p.clone()),
      CallArg::Object(o) => convert_obj_arg(o),
    };

    pb::CallArg {
      call_arg: Some(call_arg)
    }
  }).collect::<Vec<_>>();

  let commands = pt.commands.iter().map(|c| {
    let sui_command = match c {
      Command::MoveCall(mc) => convert_move_call_cmd(mc),
      Command::TransferObjects(a, b) => convert_transfer_objects_cmd(a, b),
      Command::SplitCoins(a, b) => convert_split_coins_cmd(a, b),
      Command::MergeCoins(a, b) => convert_merge_coins_cmd(a, b),
      Command::Publish(a, b) => convert_publish_cmd(a, b),
      Command::MakeMoveVec(a, b) => convert_make_move_vec_cmd(a, b),
      Command::Upgrade(a, b, c, d) => convert_upgrade_cmd(a, b, c, d),
    };

    pb::Command {
      sui_command: Some(sui_command),
    }
  }).collect::<Vec<_>>();

  pb::transaction_kind::TransactionKind::ProgrammableTx(pb::ProgrammableTransaction {
    inputs,
    commands,
  })
}

fn convert_upgrade_cmd(a: &[Vec<u8>], b: &[ObjectID], c: &ObjectID, d: &Argument) -> pb::command::SuiCommand {
  pb::command::SuiCommand::Upgrade(pb::UpgradeComand {
    one: a.to_vec(),
    two: b.iter().map(convert_sui_object).collect::<Vec<_>>(),
    three: Some(convert_sui_object(c)),
    four: Some(convert_sui_argument(d)),
  })
}

fn convert_make_move_vec_cmd(a: &Option<TypeTag>, b: &[Argument]) -> pb::command::SuiCommand {
  pb::command::SuiCommand::MakeMoveVec(pb::MakeMoveVecPair {
    one: a.map(|t| convert_type_tag(&t)),
    two: b.iter().map(convert_sui_argument).collect::<Vec<_>>(),
  })
}

fn convert_publish_cmd(a: &[Vec<u8>], b: &[ObjectID]) -> pb::command::SuiCommand {
  pb::command::SuiCommand::Publish(pb::PublishCommand {
    package_data: a.to_vec(),
    package: b.iter().map(convert_sui_object).collect::<Vec<_>>(),
  })
}

fn convert_split_coins_cmd(a: &Argument, b: &[Argument]) -> pb::command::SuiCommand {
  pb::command::SuiCommand::SplitCoins(pb::SplitCoinsPair {
    one: Some(convert_sui_argument(a)),
    two: b.iter().map(convert_sui_argument).collect::<Vec<_>>(),
  })
}

fn convert_merge_coins_cmd(a: &Argument, b: &[Argument]) -> pb::command::SuiCommand {
  pb::command::SuiCommand::MergeCoins(pb::MergeCoinsPair {
    one: Some(convert_sui_argument(a)),
    two: b.iter().map(convert_sui_argument).collect::<Vec<_>>(),
  })
}


fn convert_transfer_objects_cmd(a: &[Argument], b: &Argument) -> pb::command::SuiCommand {
  pb::command::SuiCommand::TransferObjects(pb::TransferObjectsPair {
    one: a.iter().map(convert_sui_argument).collect::<Vec<_>>(),
    two: Some(convert_sui_argument(b)),
  })
}

fn convert_move_call_cmd(mc: &ProgrammableMoveCall) -> pb::command::SuiCommand {
  pb::command::SuiCommand::MoveCall(pb::SuiProgrammableMoveCall {
    package: Some(convert_sui_object(&mc.package)),
    module: mc.module.into_string(),
    function: mc.function.into_string(),
    type_arguments: mc.type_arguments.iter().map(convert_type_tag).collect::<Vec<_>>(),
    arguments: mc.arguments.iter().map(convert_sui_argument).collect::<Vec<_>>(),
  })
}

fn convert_obj_arg(o: &ObjectArg) -> pb::call_arg::CallArg {
  let sui_object_arg = match o {
    ObjectArg::ImmOrOwnedObject(obj_ref) => convert_object_ref(obj_ref),
    ObjectArg::SharedObject {id, initial_shared_version, mutable} => pb::sui_object_arg::SuiObjectArg::SharedObject(pb::SharedObject{
        object_id: Some(convert_sui_object(&id)),
        initial_shared_version: initial_shared_version.value(),
        mutable: *mutable,
    }),
    ObjectArg::Receiving(obj_ref) => convert_object_ref(obj_ref),
  };

  pb::call_arg::CallArg::Object(SuiObjectArg {
    sui_object_arg: Some(sui_object_arg),
  })
}

fn convert_object_ref(obj_ref: &ObjectRef) -> pb::sui_object_arg::SuiObjectArg {
  pb::sui_object_arg::SuiObjectArg::ImmOrOwnedObject(pb::ImmOrOwnedObject {
    object_id: Some(convert_sui_object(&obj_ref.0)),
    sequence_number: obj_ref.1.value(),
    digest: obj_ref.2.base58_encode(),
  })
}

fn convert_intent(source: Intent) -> pb::Intent {
  pb::Intent {
    scope: Some(convert_intent_scope(source.scope)),
    version: Some(convert_version(source.version)),
    app_id: Some(convert_app_id(source.app_id)),
  }
}

fn convert_app_id(source: AppId) -> pb::AppId {
  let app_id = match source {
    AppId::Sui => pb::app_id::AppId::Sui(0),
    AppId::Narwhal => pb::app_id::AppId::Narwhal(1),
    AppId::Consensus => pb::app_id::AppId::Consensus(2),
  };

  pb::AppId {
    app_id: Some(app_id),
  }
}

fn convert_version(source: IntentVersion) -> pb::IntentVersion {
  let intent_version = match source {
    IntentVersion::V0 => pb::intent_version::IntentVersion::V0(0),
  };

  pb::IntentVersion {
    intent_version: Some(intent_version),
  }
}

fn convert_intent_scope(source: IntentScope) -> pb::IntentScope {
  let intent_scope = match source {
    IntentScope::TransactionData => pb::intent_scope::IntentScope::TransactionData,
    IntentScope::TransactionEffects => pb::intent_scope::IntentScope::TransactionEffects,
    IntentScope::CheckpointSummary => pb::intent_scope::IntentScope::CheckpointSummary,
    IntentScope::PersonalMessage => pb::intent_scope::IntentScope::PersonalMessage,
    IntentScope::SenderSignedTransaction => pb::intent_scope::IntentScope::SenderSignedTransaction,
    IntentScope::ProofOfPossession => pb::intent_scope::IntentScope::ProofOfPossession,
    IntentScope::HeaderDigest => pb::intent_scope::IntentScope::HeaderDigest,
    IntentScope::BridgeEventUnused => pb::intent_scope::IntentScope::BridgeEventUnused,
    IntentScope::ConsensusBlock => pb::intent_scope::IntentScope::ConsensusBlock,
  };

  pb::IntentScope {
    intent_scope: Some(intent_scope),
  }
}

fn convert_sender_signed_data(source: SenderSignedData) -> pb::SenderSignedTransaction {
  let sender_signed_tx = source.inner();

  pb::SenderSignedTransaction {
    intent_message: convert_intent_message(sender_signed_tx.intent_message)
  }
}

pub fn convert_transaction(source: &IndexedTransaction) -> pb::Transaction {
  pb::Transaction {
    sequence_number: source.sequence_number,
    digest: source.digest.base58_encode(),
    sender_signed_data: convert_sender_signed_data(source.senders_signed_data),
    // transaction: convert_sui_tx_block(&source.transaction),
    // raw_transaction: source.raw_transaction.clone(),
    // effects: Some(convert_sui_effects(&source.effects)),
    // events: Some(convert_tx_block_events(&source.events)),
    // timestamp_ms: source.timestamp_ms,
    // confirmed_local_execution: source.confirmed_local_execution,
    // checkpoint: source.checkpoint,
  }
}
