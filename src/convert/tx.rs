use shared_crypto::intent::{AppId, Intent, IntentMessage, IntentScope, IntentVersion};
use sui_indexer::types::IndexedTransaction;
use sui_types::{
  authenticator_state::ActiveJwk, base_types::ObjectID,
  messages_consensus::{ConsensusCommitPrologue, ConsensusCommitPrologueV2},
  transaction::{
    Argument, AuthenticatorStateUpdate, CallArg, ChangeEpoch, Command, EndOfEpochTransactionKind, GasData, GenesisObject,
    GenesisTransaction, ObjectArg, ProgrammableMoveCall, ProgrammableTransaction, RandomnessStateUpdate, SenderSignedData,
    TransactionData, TransactionDataAPI, TransactionExpiration, TransactionKind,
  }, TypeTag
};
use crate::pb::sui::checkpoint::{self as pb};

use super::{
  common::{
    convert_data, convert_object_ref, convert_owner, convert_sui_address, convert_sui_argument, convert_sui_object,
    convert_type_tag,
  }, sui_effects::convert_sui_effects, sui_event::convert_event, sui_object::convert_tx_object_change
};

fn convert_intent_message(source: &IntentMessage<TransactionData>) -> pb::IntentMessage {
  pb::IntentMessage {
    intent: Some(convert_intent(&source.intent)),
    value: Some(convert_intent_value(&source.value)),
  }
}

fn convert_intent_value(source: &TransactionData) -> pb::TransactionData {
  let tx_data = match source {
    TransactionData::V1(tx_data_v1) => {
      pb::transaction_data::TxData::V1(pb::TransactionDataV1 {
        kind: Some(convert_tx_kind(tx_data_v1.kind())),
        sender: convert_sui_address(&tx_data_v1.sender()),
        gas_data: Some(convert_gas_data(&tx_data_v1.gas_data)),
        expiration: Some(convert_tx_expiration(&tx_data_v1.expiration)),
      })
    }
  };

  pb::TransactionData {
    tx_data: Some(tx_data),
  }
}

fn convert_tx_expiration(source: &TransactionExpiration) -> pb::TransactionExpiration {
  let tx_expiration = match *source {
    TransactionExpiration::None => pb::transaction_expiration::TxExpiration::None(()),
    TransactionExpiration::Epoch(epoch_id) => pb::transaction_expiration::TxExpiration::Epoch(epoch_id),
  };

  pb::TransactionExpiration {
    tx_expiration: Some(tx_expiration),
  }
}

fn convert_gas_data(source: &GasData) -> pb::GasData {
  pb::GasData {
    payment: source.payment.iter().map(convert_object_ref).collect::<Vec<_>>(),
    owner: convert_sui_address(&source.owner),
    price: source.price,
    budget: source.budget,
  }
}

fn convert_tx_kind(source: &TransactionKind) -> pb::TransactionKind {
  let kind = match source {
    TransactionKind::ProgrammableTransaction(pt) => convert_programmable_tx_kind(pt),
    TransactionKind::ChangeEpoch(ce) => convert_change_epoch_tx_kind(ce),
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

fn convert_commit_prologue_v2(source: &ConsensusCommitPrologueV2) -> pb::transaction_kind::TransactionKind {
  pb::transaction_kind::TransactionKind::ConsensusCommitPrologueV2(pb::ConsensusCommitPrologueV2 {
    epoch: source.epoch,
    round: source.round,
    commit_timestamp_ms: source.commit_timestamp_ms,
    consensus_commit_digest: source.consensus_commit_digest.base58_encode(),
  })
}

fn convert_randomeness_state_update(source: &RandomnessStateUpdate) -> pb::transaction_kind::TransactionKind {
  pb::transaction_kind::TransactionKind::RandomnessStateUpdate(pb::RandomnessStateUpdate {
    epoch: source.epoch,
    randomness_round: source.randomness_round.0,
    random_bytes: source.random_bytes.clone(),
    randomness_obj_initial_shared_version: source.randomness_obj_initial_shared_version.value(),
  })
}

fn convert_end_of_epoch_transaction(source: &[EndOfEpochTransactionKind]) -> pb::transaction_kind::TransactionKind {
  let end_of_epoch_transaction_kind = source.iter().map(|tk| {
    let kind = match tk {
      EndOfEpochTransactionKind::ChangeEpoch(c) => pb::end_of_epoch_transaction_kind::Kind::ChangeEpoch(convert_change_epoch(c)),
      EndOfEpochTransactionKind::AuthenticatorStateCreate => pb::end_of_epoch_transaction_kind::Kind::AuthenticatorStateCreate(()),
      EndOfEpochTransactionKind::AuthenticatorStateExpire(source) =>  pb::end_of_epoch_transaction_kind::Kind::AuthenticatorStateExpire(
        pb::AuthenticatorStateExpire {
            min_epoch: source.min_epoch,
            authenticator_obj_initial_shared_version: source.authenticator_obj_initial_shared_version.value(),
        }
      ),
      EndOfEpochTransactionKind::RandomnessStateCreate => pb::end_of_epoch_transaction_kind::Kind::RandomnessStateCreate(()),
      EndOfEpochTransactionKind::DenyListStateCreate => pb::end_of_epoch_transaction_kind::Kind::DenyListStateCreate(()),
    };

    pb::EndOfEpochTransactionKind {
      kind: Some(kind),
    }
  }).collect::<Vec<_>>();

  pb::transaction_kind::TransactionKind::EndOdEpochTransaction(pb::EndOfEpochTransaction {
    end_of_epoch_transaction_kind,
  })
}

fn convert_authenticator_state_update(source: &AuthenticatorStateUpdate) -> pb::transaction_kind::TransactionKind {
  pb::transaction_kind::TransactionKind::AuthenticatorStateUpdate(pb::AuthenticatorStateUpdate {
    epoch: source.epoch,
    round: source.round,
    new_active_jwks: source.new_active_jwks.iter().map(convert_active_jwks).collect::<Vec<_>>(),
    authenticator_obj_initial_shared_version: source.authenticator_obj_initial_shared_version.value(),
  })
}

fn convert_active_jwks(source: &ActiveJwk) -> pb::ActiveJwk {
  let jwk_id = pb::JwkId {
    iss: source.jwk_id.iss.clone(),
    kid: source.jwk_id.kid.clone(),
  };

  let jwk = pb::Jwk {
    kty: source.jwk.kty.clone(),
    e: source.jwk.e.clone(),
    n: source.jwk.n.clone(),
    alg: source.jwk.alg.clone(),
  };

  pb::ActiveJwk {
    jwk_id: Some(jwk_id),
    jwk: Some(jwk),
    epoch: source.epoch,
  }
}

fn convert_commit_prologue(source: &ConsensusCommitPrologue) -> pb::transaction_kind::TransactionKind {
  pb::transaction_kind::TransactionKind::ConsensusCommitPrologue(pb::ConsensusCommitPrologue {
    epoch: source.epoch,
    round: source.round,
    commit_timestamp_ms: source.commit_timestamp_ms,
  })
}

fn convert_genesis(source: &GenesisTransaction) -> pb::transaction_kind::TransactionKind {
  pb::transaction_kind::TransactionKind::Genesis(pb::GenesisTransaction {
    objects: source.objects.iter().map(convert_genesis_obj).collect::<Vec<_>>(),
  })
}

fn convert_genesis_obj(source: &GenesisObject) -> pb::GenesisObject {
  let genesis_obj = match source {
    GenesisObject::RawObject {data, owner} => pb::genesis_object::GenesisObject::RawObject(pb::genesis_object::RawObject {
      owner: Some(convert_owner(owner)),
      data: Some(convert_data(data)),
    }),
  };

  pb::GenesisObject {
    genesis_object: Some(genesis_obj)
  } 
}

fn convert_change_epoch_tx_kind(source: &ChangeEpoch) -> pb::transaction_kind::TransactionKind {
  pb::transaction_kind::TransactionKind::ChangeEpoch(convert_change_epoch(source))
}

fn convert_change_epoch(source: &ChangeEpoch) -> pb::ChangeEpoch {
  pb::ChangeEpoch {
    epoch: source.epoch,
    storage_charge: source.storage_charge,
    computation_charge: source.computation_charge,
    storage_rebate: source.storage_rebate,
    epoch_start_timestamp_ms: source.epoch_start_timestamp_ms,
  }
}

fn convert_programmable_tx_kind(source: &ProgrammableTransaction) -> pb::transaction_kind::TransactionKind {
  let inputs = source.inputs.iter().map(|c| {
    let call_arg = match c {
      CallArg::Pure(p) => pb::call_arg::CallArg::Pure(p.clone()),
      CallArg::Object(o) => convert_obj_arg(o),
    };

    pb::CallArg {
      call_arg: Some(call_arg)
    }
  }).collect::<Vec<_>>();

  let commands = source.commands.iter().map(|c| {
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
    one: a.as_ref().map(convert_type_tag),
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
    module: mc.module.clone().into_string(),
    function: mc.function.clone().into_string(),
    type_arguments: mc.type_arguments.iter().map(convert_type_tag).collect::<Vec<_>>(),
    arguments: mc.arguments.iter().map(convert_sui_argument).collect::<Vec<_>>(),
  })
}

fn convert_obj_arg(o: &ObjectArg) -> pb::call_arg::CallArg {
  let sui_object_arg = match o {
    ObjectArg::ImmOrOwnedObject(obj_ref) => pb::sui_object_arg::SuiObjectArg::ImmOrOwnedObject(convert_object_ref(obj_ref)),
    ObjectArg::SharedObject {id, initial_shared_version, mutable} => pb::sui_object_arg::SuiObjectArg::SharedObject(pb::SharedObject{
        object_id: Some(convert_sui_object(&id)),
        initial_shared_version: initial_shared_version.value(),
        mutable: *mutable,
    }),
    ObjectArg::Receiving(obj_ref) => pb::sui_object_arg::SuiObjectArg::Receiving(convert_object_ref(obj_ref)),
  };

  pb::call_arg::CallArg::Object(pb::SuiObjectArg {
    sui_object_arg: Some(sui_object_arg),
  })
}

fn convert_intent(source: &Intent) -> pb::Intent {
  pb::Intent {
    scope: Some(convert_intent_scope(source.scope)),
    version: Some(convert_version(source.version)),
    app_id: Some(convert_app_id(source.app_id)),
  }
}

fn convert_app_id(source: AppId) -> pb::AppId {
  let app_id = match source {
    AppId::Sui => pb::app_id::AppId::Sui(()),
    AppId::Narwhal => pb::app_id::AppId::Narwhal(()),
    AppId::Consensus => pb::app_id::AppId::Consensus(()),
  };

  pb::AppId {
    app_id: Some(app_id),
  }
}

fn convert_version(source: IntentVersion) -> pb::IntentVersion {
  let intent_version = match source {
    IntentVersion::V0 => pb::intent_version::IntentVersion::V0(()),
  };

  pb::IntentVersion {
    intent_version: Some(intent_version),
  }
}

fn convert_intent_scope(source: IntentScope) -> pb::IntentScope {
  let intent_scope = match source {
    IntentScope::TransactionData => pb::intent_scope::IntentScope::TransactionData(()),
    IntentScope::TransactionEffects => pb::intent_scope::IntentScope::TransactionEffects(()),
    IntentScope::CheckpointSummary => pb::intent_scope::IntentScope::CheckpointSummary(()),
    IntentScope::PersonalMessage => pb::intent_scope::IntentScope::PersonalMessage(()),
    IntentScope::SenderSignedTransaction => pb::intent_scope::IntentScope::SenderSignedTransaction(()),
    IntentScope::ProofOfPossession => pb::intent_scope::IntentScope::ProofOfPossession(()),
    IntentScope::HeaderDigest => pb::intent_scope::IntentScope::HeaderDigest(()),
    IntentScope::BridgeEventUnused => pb::intent_scope::IntentScope::BridgeEventUnused(()),
    IntentScope::ConsensusBlock => pb::intent_scope::IntentScope::ConsensusBlock(()),
  };

  pb::IntentScope {
    intent_scope: Some(intent_scope),
  }
}

fn convert_sender_signed_data(source: &SenderSignedData) -> pb::SenderSignedTransaction {
  let sender_signed_tx = source.inner();

  pb::SenderSignedTransaction {
    intent_message: Some(convert_intent_message(&sender_signed_tx.intent_message)),
  }
}

fn convert_transaction_kind(source: &sui_indexer::types::TransactionKind) -> pb::GenericTransactionKind {
  let kind = match source {
    sui_indexer::types::TransactionKind::SystemTransaction => pb::generic_transaction_kind::Kind::SystemTransaction(()),
    sui_indexer::types::TransactionKind::ProgrammableTransaction => pb::generic_transaction_kind::Kind::ProgrammableTransaction(()),
  };

  pb::GenericTransactionKind {
    kind: Some(kind),
  }
}

pub fn convert_tx_balance_change(source: &sui_json_rpc_types::BalanceChange) -> pb::BalanceChange {
  pb::BalanceChange {
    owner: Some(convert_owner(&source.owner)),
    coin_type: Some(convert_type_tag(&source.coin_type)),
    amount: source.amount.to_string(),
  }
}

pub fn convert_transaction(source: &IndexedTransaction) -> pb::Transaction {
  pb::Transaction {
    sequence_number: source.tx_sequence_number,
    digest: source.tx_digest.base58_encode(),
    // it should only have one item
    sender_signed_data: vec![convert_sender_signed_data(&source.sender_signed_data)],
    effects: Some(convert_sui_effects(&source.effects)),
    checkpoint_sequence_number: source.checkpoint_sequence_number,
    timestamp_ms: source.timestamp_ms,
    object_changes: source.object_changes.iter().map(convert_tx_object_change).collect::<Vec<_>>(),
    balance_change: source.balance_change.iter().map(convert_tx_balance_change).collect::<Vec<_>>(),
    events: source.events.iter().map(convert_event).collect::<Vec<_>>(),
    transaction_kind: Some(convert_transaction_kind(&source.transaction_kind)),
    successful_tx_num: source.successful_tx_num,
  }
}

#[cfg(test)]
mod tests {
  use std::str::FromStr;
  use move_core_types::{account_address::AccountAddress, language_storage::StructTag};
  use shared_crypto::intent::{AppId, Intent, IntentScope};
  use sui_indexer::types::{TransactionKind as IndexerTxKind, IndexedObjectChange, IndexedTransaction};
  use sui_types::{
    base_types::{ObjectID, SequenceNumber, SuiAddress}, crypto::{Ed25519SuiSignature, Signature},
    digests::{ObjectDigest, TransactionDigest}, effects::TransactionEffects, execution_status::ExecutionStatus,
    gas::GasCostSummary, messages_consensus::ConsensusCommitPrologue, object::Owner, signature::GenericSignature,
    transaction::{GasData, SenderSignedData, TransactionData, TransactionDataV1, TransactionExpiration, TransactionKind},
    Identifier
  };

use crate::{convert::tx::convert_transaction, pb::sui::checkpoint::{self as pb, TransactionBlockEffectsV1}};

  #[test]
  fn converts_transaction() {
    let source = IndexedTransaction {
        tx_sequence_number: 1449227,
        tx_digest: TransactionDigest::from_str("D7CBWgtjcgMyn1YhRZ2q7okrmiCUYW4QA5gPZT6CRa2n").unwrap(),
        sender_signed_data: SenderSignedData::new(
          TransactionData::V1(TransactionDataV1 {
            kind: TransactionKind::ConsensusCommitPrologue(ConsensusCommitPrologue {
              epoch: 19,
              round: 58682,
              commit_timestamp_ms: 1682990756147,
            }),
            sender: SuiAddress::from_str("0x0000000000000000000000000000000000000000000000000000000000000000").unwrap(),
            gas_data: GasData {
              payment: vec![(
                ObjectID::from_str("0x0000000000000000000000000000000000000000000000000000000000000000").unwrap(),
                SequenceNumber::from_u64(0),
                ObjectDigest::from_str("11111111111111111111111111111111").unwrap(),
              )],
              owner: SuiAddress::from_str("0x0000000000000000000000000000000000000000000000000000000000000000").unwrap(),
              price: 1,
              budget: 0,
            },
            expiration: TransactionExpiration::None,
          }),
          Intent {
            scope: IntentScope::TransactionData,
            version: shared_crypto::intent::IntentVersion::V0,
            app_id: AppId::Sui,
          },
          vec![GenericSignature::Signature(Signature::Ed25519SuiSignature(Ed25519SuiSignature::default()))]
        ),
        effects: TransactionEffects::new_from_execution_v1(
          ExecutionStatus::Success,
          19,
          GasCostSummary {
            computation_cost: 0,
            storage_cost: 0,
            storage_rebate: 0,
            non_refundable_storage_fee: 0,
          },
          vec![(ObjectID::from_str("0x0000000000000000000000000000000000000000000000000000000000000006").unwrap(), SequenceNumber::from_u64(1448000))],
          vec![(
            ObjectID::from_str("0x0000000000000000000000000000000000000000000000000000000000000006").unwrap(),
            SequenceNumber::from_u64(1448000),
            ObjectDigest::from_str("2GB5NVhagD4fQ9P85WqtgX3nwFwVdqDPbKYBtGcziQYM").unwrap(),
          )],
          TransactionDigest::from_str("D7CBWgtjcgMyn1YhRZ2q7okrmiCUYW4QA5gPZT6CRa2n").unwrap(),
          vec![],
          vec![(
            (
              ObjectID::from_str("0x0000000000000000000000000000000000000000000000000000000000000006").unwrap(),
              SequenceNumber::from_u64(1448001),
              ObjectDigest::from_str("CDdzbah88YnaMJXjhpnqHy5BTo3YBAqckuD5uzfs2kyX").unwrap(),
            ),
            Owner::Shared {
              initial_shared_version: SequenceNumber::from_u64(1),
            },
          )],
          vec![],
          vec![],
          vec![],
          vec![],
          (
            (
              ObjectID::from_str("0x0000000000000000000000000000000000000000000000000000000000000000").unwrap(),
              SequenceNumber::from_u64(0),
              ObjectDigest::from_str("11111111111111111111111111111111").unwrap(),
            ),
            Owner::AddressOwner(SuiAddress::from_str("0x0000000000000000000000000000000000000000000000000000000000000000").unwrap()),
          ),
          None,
          vec![TransactionDigest::from_str("A5PHzo8quSTJGpay1S5Q6AKTjtYpSJPMUi3wchQkzBSX").unwrap()],
        ),
        checkpoint_sequence_number: 1448000,
        timestamp_ms: 1682990756147,
        object_changes: vec![
          IndexedObjectChange::Mutated {
            sender: SuiAddress::from_str("0x0000000000000000000000000000000000000000000000000000000000000000").unwrap(),
            owner: Owner::Shared {
              initial_shared_version: SequenceNumber::from_u64(1),
            },
            object_type: StructTag {
              address: AccountAddress::from_str("0000000000000000000000000000000000000000000000000000000000000002").unwrap(),
              module: Identifier::new("clock").unwrap(),
              name: Identifier::new("Clock").unwrap(),
              type_params: vec![],
            },
            object_id: ObjectID::from_str("0x0000000000000000000000000000000000000000000000000000000000000006").unwrap(),
            version: SequenceNumber::from_u64(1448001),
            previous_version: SequenceNumber::from_u64(1448000),
            digest: ObjectDigest::from_str("CDdzbah88YnaMJXjhpnqHy5BTo3YBAqckuD5uzfs2kyX").unwrap(),
          }
        ],
        balance_change: vec![],
        events: vec![],
        transaction_kind: IndexerTxKind::SystemTransaction,
        successful_tx_num: 1,
    };
    let pb_tx = convert_transaction(&source);
    let expected = pb::Transaction {
        sequence_number: 1449227,
        digest: "D7CBWgtjcgMyn1YhRZ2q7okrmiCUYW4QA5gPZT6CRa2n".to_string(),
        sender_signed_data: vec![
          pb::SenderSignedTransaction {
            intent_message: Some(pb::IntentMessage {
              intent: Some(pb::Intent {
                scope: Some(pb::IntentScope {
                  intent_scope: Some(pb::intent_scope::IntentScope::TransactionData(())),
                }),
                version: Some(pb::IntentVersion {
                  intent_version: Some(pb::intent_version::IntentVersion::V0(()))
                }),
                app_id: Some(pb::AppId {
                  app_id: Some(pb::app_id::AppId::Sui(())),
                }),
              }),
              value: Some(pb::TransactionData {
                tx_data: Some(pb::transaction_data::TxData::V1(pb::TransactionDataV1 {
                  kind: Some(pb::TransactionKind {
                    transaction_kind: Some(pb::transaction_kind::TransactionKind::ConsensusCommitPrologue(pb::ConsensusCommitPrologue {
                      epoch: 19,
                      round: 58682,
                      commit_timestamp_ms: 1682990756147,
                    })),
                  }),
                  sender: "0000000000000000000000000000000000000000000000000000000000000000".to_string(),
                  gas_data: Some(pb::GasData {
                    payment: vec![pb::ObjectRef {
                      object_id: Some(pb::ObjectId {account_address: "0000000000000000000000000000000000000000000000000000000000000000".to_string() }),
                      sequence_number: 0,
                      digest: "11111111111111111111111111111111".to_string(),
                    }],
                    owner: "0000000000000000000000000000000000000000000000000000000000000000".to_string(),
                    price: 1,
                    budget: 0,
                  }),
                  expiration: Some(pb::TransactionExpiration {
                    tx_expiration: Some(pb::transaction_expiration::TxExpiration::None(())),
                  }),
                })),
              }),
            })
          } 
        ],
        effects: Some(pb::TransactionBlockEffects {
          transaction_block_effects: Some(
            pb::transaction_block_effects::TransactionBlockEffects::V1(TransactionBlockEffectsV1 {
              status: Some(pb::ExecutionStatus {
                execution_status: Some(pb::execution_status::ExecutionStatus::Success(())),
              }),
              executed_epoch: 19,
              gas_used: Some(pb::GasCostSummary {
                computation_cost: 0,
                storage_cost: 0,
                storage_rebate: 0,
                non_refundable_storage_fee: 0,
              }),
              modified_at_versions: vec![
                pb::TransactionBlockEffectsModifiedAtVersions {
                  object_id: Some(pb::ObjectId {account_address: "0000000000000000000000000000000000000000000000000000000000000006".to_string() }),
                  sequence_number: 1448000,
                }
              ],
              shared_objects: vec![pb::ObjectRef {
                object_id: Some(pb::ObjectId {account_address: "0000000000000000000000000000000000000000000000000000000000000006".to_string() }),
                sequence_number: 1448000,
                digest: "2GB5NVhagD4fQ9P85WqtgX3nwFwVdqDPbKYBtGcziQYM".to_string(),
              }],
              transaction_digest: "D7CBWgtjcgMyn1YhRZ2q7okrmiCUYW4QA5gPZT6CRa2n".to_string(),
              created: vec![],
              mutated: vec![pb::OwnedObjectRef {
                owner: Some(pb::Owner {owner: Some(pb::owner::Owner::Shared(pb::Shared { initial_shared_version: 1 }))}),
                reference: Some(pb::ObjectRef {
                  object_id: Some(pb::ObjectId {account_address: "0000000000000000000000000000000000000000000000000000000000000006".to_string() }),
                  sequence_number: 1448001,
                  digest: "CDdzbah88YnaMJXjhpnqHy5BTo3YBAqckuD5uzfs2kyX".to_string(),
                }),
              }],
              unwrapped: vec![],
              deleted: vec![],
              unwrapped_then_deleted: vec![],
              wrapped: vec![],
              gas_object: Some(pb::OwnedObjectRef {
                owner: Some(pb::Owner {owner: Some(pb::owner::Owner::AddressOwner("0000000000000000000000000000000000000000000000000000000000000000".to_string()))}),
                reference: Some(pb::ObjectRef {
                  object_id: Some(pb::ObjectId {account_address: "0000000000000000000000000000000000000000000000000000000000000000".to_string() }),
                  sequence_number: 0,
                  digest: "11111111111111111111111111111111".to_string(),
                }),
              }),
              events_digest: None,
              dependencies: vec!["A5PHzo8quSTJGpay1S5Q6AKTjtYpSJPMUi3wchQkzBSX".to_string()],
            })
          ),
        }),
        checkpoint_sequence_number: 1448000,
        timestamp_ms: 1682990756147,
        object_changes: vec![pb::ObjectChange {
          object_change: Some(pb::object_change::ObjectChange::Mutated(pb::Mutated {
            sender: "0000000000000000000000000000000000000000000000000000000000000000".to_string(),
            owner: Some(pb::Owner {owner: Some(pb::owner::Owner::Shared(pb::Shared {initial_shared_version: 1}))}),
            object_type: Some(pb::StructTag {
              address: "0000000000000000000000000000000000000000000000000000000000000002".to_string(),
              module: "clock".to_string(),
              name: "Clock".to_string(),
              type_params: Some(pb::ListOfTypeTags {
                list: vec![],
              }),
            }),
            object_id: Some(pb::ObjectId {
              account_address: "0000000000000000000000000000000000000000000000000000000000000006".to_string(),
            }),
            version: 1448001,
            previous_version: 1448000,
            digest: "CDdzbah88YnaMJXjhpnqHy5BTo3YBAqckuD5uzfs2kyX".to_string(),
          })),
        }],
        balance_change: vec![],
        events: vec![],
        transaction_kind: Some(pb::GenericTransactionKind {
          kind: Some(pb::generic_transaction_kind::Kind::SystemTransaction(())),
        }),
        successful_tx_num: 1,
    };

    assert_eq!(expected, pb_tx);
  }
}
