#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CheckpointData {
    #[prost(message, optional, tag = "1")]
    pub checkpoint: ::core::option::Option<Checkpoint>,
    #[prost(message, repeated, tag = "2")]
    pub transactions: ::prost::alloc::vec::Vec<Transaction>,
    #[prost(message, repeated, tag = "3")]
    pub events: ::prost::alloc::vec::Vec<IndexedEvent>,
    #[prost(message, repeated, tag = "4")]
    pub tx_indices: ::prost::alloc::vec::Vec<TxIndex>,
    #[prost(map = "string, message", tag = "5")]
    pub display_updates: ::std::collections::HashMap<
        ::prost::alloc::string::String,
        StoredDisplay,
    >,
    #[prost(message, optional, tag = "6")]
    pub packages: ::core::option::Option<IndexedPackage>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Checkpoint {
    /// Checkpoint's epoch ID
    #[prost(uint64, tag = "1")]
    pub epoch: u64,
    /// Checkpoint sequence number
    #[prost(uint64, tag = "2")]
    pub sequence_number: u64,
    /// Checkpoint digest (base58 encoded)
    #[prost(string, tag = "3")]
    pub digest: ::prost::alloc::string::String,
    /// Total number of transactions committed since genesis, including those in this checkpoint.
    #[prost(uint64, tag = "4")]
    pub network_total_transactions: u64,
    /// Digest of the previous checkpoint
    #[prost(string, optional, tag = "5")]
    pub previous_digest: ::core::option::Option<::prost::alloc::string::String>,
    /// The running total gas costs of all transactions included in the current epoch so far until this checkpoint.
    #[prost(message, optional, tag = "6")]
    pub gas_cost_summary: ::core::option::Option<GasCostSummary>,
    /// Timestamp of the checkpoint - number of milliseconds from the Unix epoch
    /// Checkpoint timestamps are monotonic, but not strongly monotonic - subsequent
    /// checkpoints can have same timestamp if they originate from the same underlining consensus commit
    #[prost(uint64, tag = "7")]
    pub timestamp_ms: u64,
    /// Present only on the final checkpoint of the epoch.
    #[prost(message, optional, tag = "8")]
    pub end_of_epoch_data: ::core::option::Option<EndOfEpochData>,
    /// Commitments to checkpoint state
    #[prost(message, repeated, tag = "9")]
    pub checkpoint_commitments: ::prost::alloc::vec::Vec<CheckpointCommitment>,
    /// Validator Signature (base64  encoded). This is a BLS signature
    #[prost(bytes = "vec", tag = "10")]
    pub validator_signature: ::prost::alloc::vec::Vec<u8>,
    #[prost(uint64, tag = "11")]
    pub successful_tx_num: u64,
    #[prost(bool, tag = "12")]
    pub end_of_epoch: bool,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GasCostSummary {
    /// total gas cost could be negative
    #[prost(int64, tag = "12")]
    pub total_gas_cost: i64,
    /// Cost of computation/execution
    #[prost(uint64, tag = "1")]
    pub computation_cost: u64,
    /// Storage cost, it's the sum of all storage cost for all objects created or mutated.
    #[prost(uint64, tag = "2")]
    pub storage_cost: u64,
    /// The amount of storage cost refunded to the user for all objects deleted or mutated in the transaction.
    #[prost(uint64, tag = "3")]
    pub storage_rebate: u64,
    /// The fee for the rebate. The portion of the storage rebate kept by the system.
    #[prost(uint64, tag = "4")]
    pub non_refundable_storage_fee: u64,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Transaction {
    #[prost(uint64, tag = "1")]
    pub sequence_number: u64,
    #[prost(string, tag = "2")]
    pub digest: ::prost::alloc::string::String,
    #[prost(message, repeated, tag = "3")]
    pub sender_signed_data: ::prost::alloc::vec::Vec<SenderSignedTransaction>,
    /// Transaction input data
    /// TransactionBlock transaction = 2;
    /// bytes raw_transaction = 3;
    #[prost(message, optional, tag = "4")]
    pub effects: ::core::option::Option<TransactionBlockEffects>,
    #[prost(uint64, tag = "5")]
    pub checkpoint_sequence_number: u64,
    #[prost(uint64, tag = "6")]
    pub timestamp_ms: u64,
    #[prost(message, repeated, tag = "7")]
    pub object_changes: ::prost::alloc::vec::Vec<ObjectChange>,
    #[prost(message, repeated, tag = "8")]
    pub balance_change: ::prost::alloc::vec::Vec<BalanceChange>,
    #[prost(message, repeated, tag = "9")]
    pub events: ::prost::alloc::vec::Vec<Event>,
    #[prost(message, optional, tag = "10")]
    pub transaction_kind: ::core::option::Option<GenericTransactionKind>,
    #[prost(uint64, tag = "11")]
    pub successful_tx_num: u64,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GenericTransactionKind {
    #[prost(oneof = "generic_transaction_kind::Kind", tags = "1, 2")]
    pub kind: ::core::option::Option<generic_transaction_kind::Kind>,
}
/// Nested message and enum types in `GenericTransactionKind`.
pub mod generic_transaction_kind {
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum Kind {
        #[prost(uint32, tag = "1")]
        SystemRansaction(u32),
        #[prost(uint32, tag = "2")]
        ProgrammableTransaction(u32),
    }
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Event {
    #[prost(message, optional, tag = "1")]
    pub package_id: ::core::option::Option<ObjectId>,
    #[prost(string, tag = "2")]
    pub transaction_module: ::prost::alloc::string::String,
    #[prost(string, tag = "3")]
    pub sender: ::prost::alloc::string::String,
    #[prost(message, optional, tag = "4")]
    pub r#type: ::core::option::Option<StructTag>,
    #[prost(bytes = "vec", tag = "5")]
    pub contents: ::prost::alloc::vec::Vec<u8>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct BalanceChange {
    /// / Owner of the balance change
    #[prost(message, optional, tag = "1")]
    pub owner: ::core::option::Option<Owner>,
    #[prost(message, optional, tag = "2")]
    pub coin_type: ::core::option::Option<TypeTag>,
    /// / The amount indicate the balance value changes,
    /// / negative amount means spending coin value and positive means receiving coin value.
    /// / covnert this to i128 later on
    #[prost(string, tag = "3")]
    pub amount: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ObjectChange {
    #[prost(oneof = "object_change::ObjectChange", tags = "1, 2, 3, 4, 5, 6")]
    pub object_change: ::core::option::Option<object_change::ObjectChange>,
}
/// Nested message and enum types in `ObjectChange`.
pub mod object_change {
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum ObjectChange {
        #[prost(message, tag = "1")]
        Published(super::Published),
        #[prost(message, tag = "2")]
        Transferred(super::Transferred),
        #[prost(message, tag = "3")]
        Mutated(super::Mutated),
        #[prost(message, tag = "4")]
        Deleted(super::Deleted),
        #[prost(message, tag = "5")]
        Wrapped(super::Wrapped),
        #[prost(message, tag = "6")]
        Created(super::Created),
    }
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Published {
    #[prost(message, optional, tag = "1")]
    pub package_id: ::core::option::Option<ObjectId>,
    #[prost(uint64, tag = "2")]
    pub version: u64,
    #[prost(string, tag = "3")]
    pub digest: ::prost::alloc::string::String,
    #[prost(string, repeated, tag = "4")]
    pub modules: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Transferred {
    #[prost(string, tag = "1")]
    pub sender: ::prost::alloc::string::String,
    #[prost(message, optional, tag = "2")]
    pub owner: ::core::option::Option<Owner>,
    #[prost(message, optional, tag = "3")]
    pub object_type: ::core::option::Option<StructTag>,
    #[prost(message, optional, tag = "4")]
    pub object_id: ::core::option::Option<ObjectId>,
    #[prost(uint64, tag = "5")]
    pub version: u64,
    #[prost(string, tag = "6")]
    pub digest: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Mutated {
    #[prost(string, tag = "1")]
    pub sender: ::prost::alloc::string::String,
    #[prost(message, optional, tag = "2")]
    pub owner: ::core::option::Option<Owner>,
    #[prost(message, optional, tag = "3")]
    pub object_type: ::core::option::Option<StructTag>,
    #[prost(message, optional, tag = "4")]
    pub object_id: ::core::option::Option<ObjectId>,
    #[prost(uint64, tag = "5")]
    pub version: u64,
    #[prost(uint64, tag = "6")]
    pub previous_version: u64,
    #[prost(string, tag = "7")]
    pub digest: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Deleted {
    #[prost(string, tag = "1")]
    pub sender: ::prost::alloc::string::String,
    #[prost(message, optional, tag = "2")]
    pub object_type: ::core::option::Option<StructTag>,
    #[prost(message, optional, tag = "3")]
    pub object_id: ::core::option::Option<ObjectId>,
    #[prost(uint64, tag = "4")]
    pub version: u64,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Wrapped {
    #[prost(string, tag = "1")]
    pub sender: ::prost::alloc::string::String,
    #[prost(message, optional, tag = "2")]
    pub object_type: ::core::option::Option<StructTag>,
    #[prost(message, optional, tag = "3")]
    pub object_id: ::core::option::Option<ObjectId>,
    #[prost(uint64, tag = "4")]
    pub version: u64,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Created {
    #[prost(string, tag = "1")]
    pub sender: ::prost::alloc::string::String,
    #[prost(message, optional, tag = "2")]
    pub owner: ::core::option::Option<Owner>,
    #[prost(message, optional, tag = "3")]
    pub object_type: ::core::option::Option<StructTag>,
    #[prost(message, optional, tag = "4")]
    pub object_id: ::core::option::Option<ObjectId>,
    #[prost(uint64, tag = "5")]
    pub version: u64,
    #[prost(string, tag = "6")]
    pub digest: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SenderSignedTransaction {
    #[prost(message, optional, tag = "1")]
    pub intent_message: ::core::option::Option<IntentMessage>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct IntentMessage {
    #[prost(message, optional, tag = "1")]
    pub intent: ::core::option::Option<Intent>,
    #[prost(message, optional, tag = "2")]
    pub value: ::core::option::Option<TransactionData>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Intent {
    #[prost(message, optional, tag = "1")]
    pub scope: ::core::option::Option<IntentScope>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct IntentScope {
    #[prost(oneof = "intent_scope::Scope", tags = "1, 2, 3, 4, 5, 6, 7, 8, 9")]
    pub scope: ::core::option::Option<intent_scope::Scope>,
}
/// Nested message and enum types in `IntentScope`.
pub mod intent_scope {
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum Scope {
        /// Used for a user signature on a transaction data.
        #[prost(uint32, tag = "1")]
        TransactionData(u32),
        /// Used for an authority signature on transaction effects.
        #[prost(uint32, tag = "2")]
        TransactionEffects(u32),
        /// Used for an authority signature on a checkpoint summary.
        #[prost(uint32, tag = "3")]
        CheckpointSummary(u32),
        /// Used for a user signature on a personal message.
        #[prost(uint32, tag = "4")]
        PersonalMessage(u32),
        /// Used for an authority signature on a user signed transaction.
        #[prost(uint32, tag = "5")]
        SenderSignedTransaction(u32),
        /// Used as a signature representing an authority's proof of possession of its authority protocol key.
        #[prost(uint32, tag = "6")]
        ProofOfPossession(u32),
        /// Used for narwhal authority signature on header digest.
        #[prost(uint32, tag = "7")]
        HeaderDigest(u32),
        /// for bridge purposes but it's currently not included in messages.
        #[prost(uint32, tag = "8")]
        BridgeEventUnused(u32),
        /// Used for consensus authority signature on block's digest
        #[prost(uint32, tag = "9")]
        ConsensusBlock(u32),
    }
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct TransactionData {
    #[prost(oneof = "transaction_data::TxData", tags = "1")]
    pub tx_data: ::core::option::Option<transaction_data::TxData>,
}
/// Nested message and enum types in `TransactionData`.
pub mod transaction_data {
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum TxData {
        #[prost(message, tag = "1")]
        V1(super::TransactionDataV1),
    }
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct TransactionDataV1 {
    #[prost(message, optional, tag = "1")]
    pub kind: ::core::option::Option<TransactionKind>,
    #[prost(string, tag = "2")]
    pub sender: ::prost::alloc::string::String,
    #[prost(message, optional, tag = "3")]
    pub gas_data: ::core::option::Option<GasData>,
    #[prost(message, optional, tag = "4")]
    pub expiration: ::core::option::Option<TransactionExpiration>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct TransactionExpiration {
    #[prost(oneof = "transaction_expiration::TxExpiration", tags = "1, 2")]
    pub tx_expiration: ::core::option::Option<transaction_expiration::TxExpiration>,
}
/// Nested message and enum types in `TransactionExpiration`.
pub mod transaction_expiration {
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum TxExpiration {
        #[prost(uint32, tag = "1")]
        None(u32),
        #[prost(uint64, tag = "2")]
        Epoch(u64),
    }
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct TransactionKind {
    #[prost(
        oneof = "transaction_kind::TransactionKind",
        tags = "1, 2, 3, 4, 5, 6, 7, 8"
    )]
    pub transaction_kind: ::core::option::Option<transaction_kind::TransactionKind>,
}
/// Nested message and enum types in `TransactionKind`.
pub mod transaction_kind {
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum TransactionKind {
        /// / A transaction that allows the interleaving of native commands and Move calls
        #[prost(message, tag = "1")]
        ProgrammableTx(super::ProgrammableTransaction),
        #[prost(message, tag = "2")]
        ChangeEpoch(super::ChangeEpoch),
        #[prost(message, tag = "3")]
        Genesis(super::GenesisTransaction),
        #[prost(message, tag = "4")]
        ConsensusCommitPrologue(super::ConsensusCommitPrologue),
        #[prost(message, tag = "5")]
        AuthenticatorStateUpdate(super::AuthenticatorStateUpdate),
        /// / EndOfEpochTransaction replaces ChangeEpoch with a list of transactions that are allowed to
        /// / run at the end of the epoch.
        #[prost(message, tag = "6")]
        EndOdEpochTransaction(super::EndOfEpochTransaction),
        #[prost(message, tag = "7")]
        RandomnessStateUpdate(super::RandomnessStateUpdate),
        /// V2 ConsensusCommitPrologue also includes the digest of the current consensus output.
        #[prost(message, tag = "8")]
        ConsensusCommitPrologueV2(super::ConsensusCommitPrologueV2),
    }
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ConsensusCommitPrologueV2 {
    /// / Epoch of the commit prologue transaction
    #[prost(uint64, tag = "1")]
    pub epoch: u64,
    /// / Consensus round of the commit
    #[prost(uint64, tag = "2")]
    pub round: u64,
    /// / Unix timestamp from consensus
    #[prost(uint64, tag = "3")]
    pub commit_timestamp_ms: u64,
    /// / Digest of consensus output
    #[prost(string, tag = "4")]
    pub consensus_commit_digest: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RandomnessStateUpdate {
    /// / Epoch of the randomness state update transaction
    #[prost(uint64, tag = "1")]
    pub epoch: u64,
    /// / Randomness round of the update
    #[prost(uint64, tag = "2")]
    pub randomness_round: u64,
    /// / Updated random bytes
    #[prost(bytes = "vec", tag = "3")]
    pub random_bytes: ::prost::alloc::vec::Vec<u8>,
    /// / The initial version of the randomness object that it was shared at.
    #[prost(uint64, tag = "4")]
    pub randomness_obj_initial_shared_version: u64,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct EndOfEpochTransaction {
    #[prost(message, repeated, tag = "1")]
    pub end_od_epoch_transaction_kind: ::prost::alloc::vec::Vec<
        EndOfEpochTransactionKind,
    >,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct EndOfEpochTransactionKind {
    #[prost(oneof = "end_of_epoch_transaction_kind::Kind", tags = "1, 2, 3, 4, 5")]
    pub kind: ::core::option::Option<end_of_epoch_transaction_kind::Kind>,
}
/// Nested message and enum types in `EndOfEpochTransactionKind`.
pub mod end_of_epoch_transaction_kind {
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum Kind {
        #[prost(message, tag = "1")]
        ChangeEpoch(super::ChangeEpoch),
        #[prost(uint32, tag = "2")]
        AuthenticatorStateCreate(u32),
        #[prost(message, tag = "3")]
        AuthenticatorStateExpire(super::AuthenticatorStateExpire),
        #[prost(uint32, tag = "4")]
        RandomnessStateCreate(u32),
        #[prost(uint32, tag = "5")]
        DenyListStateCreate(u32),
    }
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AuthenticatorStateUpdate {
    /// / Epoch of the authenticator state update transaction
    #[prost(uint64, tag = "1")]
    pub epoch: u64,
    /// / Consensus round of the authenticator state update
    #[prost(uint64, tag = "2")]
    pub round: u64,
    /// / newly active jwks
    #[prost(message, repeated, tag = "3")]
    pub new_active_jwks: ::prost::alloc::vec::Vec<ActiveJwk>,
    /// / The initial version of the authenticator object that it was shared at.
    #[prost(uint64, tag = "4")]
    pub authenticator_obj_initial_shared_version: u64,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ActiveJwk {
    #[prost(message, optional, tag = "1")]
    pub jwk_id: ::core::option::Option<JwkId>,
    #[prost(message, optional, tag = "2")]
    pub jwk: ::core::option::Option<Jwk>,
    /// the most recent epoch in which the jwk was validated
    #[prost(uint64, tag = "3")]
    pub epoch: u64,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct JwkId {
    /// / iss string that identifies the OIDC provider.
    #[prost(string, tag = "1")]
    pub iss: ::prost::alloc::string::String,
    /// / kid string that identifies the JWK.
    #[prost(string, tag = "2")]
    pub kid: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Jwk {
    /// / Key type parameter, <https://datatracker.ietf.org/doc/html/rfc7517#section-4.1>
    #[prost(string, tag = "1")]
    pub kty: ::prost::alloc::string::String,
    /// / RSA public exponent, <https://datatracker.ietf.org/doc/html/rfc7517#section-9.3>
    #[prost(string, tag = "2")]
    pub e: ::prost::alloc::string::String,
    /// / RSA modulus, <https://datatracker.ietf.org/doc/html/rfc7517#section-9.3>
    #[prost(string, tag = "3")]
    pub n: ::prost::alloc::string::String,
    /// / Algorithm parameter, <https://datatracker.ietf.org/doc/html/rfc7517#section-4.4>
    #[prost(string, tag = "4")]
    pub alg: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AuthenticatorStateExpire {
    /// / expire JWKs that have a lower epoch than this
    #[prost(uint64, tag = "1")]
    pub min_epoch: u64,
    /// / The initial version of the authenticator object that it was shared at.
    #[prost(uint64, tag = "2")]
    pub authenticator_obj_initial_shared_version: u64,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct IndexedEvent {
    #[prost(uint64, tag = "1")]
    pub tx_sequence_number: u64,
    #[prost(uint64, tag = "2")]
    pub event_sequence_number: u64,
    #[prost(uint64, tag = "3")]
    pub checkpoint_sequence_number: u64,
    #[prost(string, tag = "4")]
    pub transaction_digest: ::prost::alloc::string::String,
    #[prost(string, repeated, tag = "5")]
    pub senders: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    #[prost(message, optional, tag = "6")]
    pub package: ::core::option::Option<ObjectId>,
    #[prost(string, tag = "7")]
    pub module: ::prost::alloc::string::String,
    #[prost(string, tag = "8")]
    pub event_type: ::prost::alloc::string::String,
    #[prost(bytes = "vec", tag = "9")]
    pub bsc: ::prost::alloc::vec::Vec<u8>,
    #[prost(uint64, tag = "10")]
    pub timestamp_ms: u64,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct TxIndex {
    #[prost(uint64, tag = "1")]
    pub tx_sequence_number: u64,
    #[prost(string, tag = "2")]
    pub transaction_digest: ::prost::alloc::string::String,
    #[prost(uint64, tag = "3")]
    pub checkpoint_sequence_number: u64,
    #[prost(message, repeated, tag = "4")]
    pub input_objects: ::prost::alloc::vec::Vec<ObjectId>,
    #[prost(message, repeated, tag = "5")]
    pub changed_objects: ::prost::alloc::vec::Vec<ObjectId>,
    #[prost(string, repeated, tag = "6")]
    pub payers: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    #[prost(string, repeated, tag = "7")]
    pub senders: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    #[prost(string, repeated, tag = "8")]
    pub recipients: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    #[prost(message, repeated, tag = "9")]
    pub move_calls: ::prost::alloc::vec::Vec<MoveCallOverview>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MoveCallOverview {
    /// The package containing the module and function.
    #[prost(message, optional, tag = "1")]
    pub package: ::core::option::Option<ObjectId>,
    /// The specific module in the package containing the function.
    #[prost(string, tag = "2")]
    pub module: ::prost::alloc::string::String,
    /// The function to be called.
    #[prost(string, tag = "3")]
    pub function: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct StoredDisplay {
    #[prost(string, tag = "1")]
    pub object_type: ::prost::alloc::string::String,
    #[prost(bytes = "vec", tag = "2")]
    pub id: ::prost::alloc::vec::Vec<u8>,
    #[prost(int32, tag = "3")]
    pub version: i32,
    #[prost(bytes = "vec", tag = "4")]
    pub bcs: ::prost::alloc::vec::Vec<u8>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct TransactionObjectChange {
    #[prost(message, repeated, tag = "1")]
    pub changed_objects: ::prost::alloc::vec::Vec<IndexedObject>,
    #[prost(message, repeated, tag = "2")]
    pub deleted_objects: ::prost::alloc::vec::Vec<IndexedDeletedObject>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct IndexedObject {
    #[prost(message, optional, tag = "1")]
    pub object_id: ::core::option::Option<ObjectId>,
    #[prost(uint64, tag = "2")]
    pub object_version: u64,
    #[prost(string, tag = "3")]
    pub object_digest: ::prost::alloc::string::String,
    #[prost(uint64, tag = "4")]
    pub checkpoint_sequence_number: u64,
    #[prost(message, optional, tag = "5")]
    pub owner_type: ::core::option::Option<OwnerType>,
    #[prost(string, optional, tag = "6")]
    pub owner_id: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(message, optional, tag = "7")]
    pub object: ::core::option::Option<Object>,
    #[prost(string, optional, tag = "8")]
    pub coin_type: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(uint64, tag = "9")]
    pub coin_balance: u64,
    #[prost(message, optional, tag = "10")]
    pub df_info: ::core::option::Option<DynamicFieldInfo>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Object {
    /// / The meat of the object
    #[prost(message, optional, tag = "1")]
    pub data: ::core::option::Option<Data>,
    /// / The owner that unlocks this object
    #[prost(message, optional, tag = "2")]
    pub owner: ::core::option::Option<Owner>,
    /// / The digest of the transaction that created or last mutated this object
    #[prost(string, tag = "3")]
    pub previous_transaction: ::prost::alloc::string::String,
    /// / The amount of SUI we would rebate if this object gets deleted.
    /// / This number is re-calculated each time the object is mutated based on
    /// / the present storage gas price.
    #[prost(uint64, tag = "4")]
    pub storage_rebate: u64,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct OwnerType {
    #[prost(oneof = "owner_type::OwnerType", tags = "1, 2, 3, 4")]
    pub owner_type: ::core::option::Option<owner_type::OwnerType>,
}
/// Nested message and enum types in `OwnerType`.
pub mod owner_type {
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum OwnerType {
        #[prost(uint32, tag = "1")]
        Immutable(u32),
        #[prost(uint32, tag = "2")]
        Address(u32),
        #[prost(uint32, tag = "3")]
        Object(u32),
        #[prost(uint32, tag = "4")]
        Shared(u32),
    }
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Data {
    #[prost(oneof = "data::Data", tags = "1, 2")]
    pub data: ::core::option::Option<data::Data>,
}
/// Nested message and enum types in `Data`.
pub mod data {
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum Data {
        #[prost(message, tag = "1")]
        Move(super::MoveObject),
        #[prost(message, tag = "2")]
        Package(super::MovePackage),
    }
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MoveObject {
    /// / The type of this object. Immutable
    #[prost(message, optional, tag = "1")]
    pub r#type: ::core::option::Option<MoveObjectType>,
    /// / DEPRECATED this field is no longer used to determine whether a tx can transfer this
    /// / object. Instead, it is always calculated from the objects type when loaded in execution
    #[prost(bool, tag = "2")]
    pub has_public_transfer: bool,
    /// / Number that increases each time a tx takes this object as a mutable input
    /// / This is a lamport timestamp, not a sequentially increasing version
    #[prost(uint64, tag = "3")]
    pub version: u64,
    #[prost(bytes = "vec", tag = "4")]
    pub contents: ::prost::alloc::vec::Vec<u8>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MovePackage {
    #[prost(message, optional, tag = "1")]
    pub id: ::core::option::Option<ObjectId>,
    /// / Most move packages are uniquely identified by their ID (i.e. there is only one version per
    /// / ID), but the version is still stored because one package may be an upgrade of another (at a
    /// / different ID), in which case its version will be one greater than the version of the
    /// / upgraded package.
    /// /
    /// / Framework packages are an exception to this rule -- all versions of the framework packages
    /// / exist at the same ID, at increasing versions.
    /// /
    /// / In all cases, packages are referred to by move calls using just their ID, and they are
    /// / always loaded at their latest version.
    #[prost(uint64, tag = "2")]
    pub version: u64,
    #[prost(map = "string, bytes", tag = "3")]
    pub module_map: ::std::collections::HashMap<
        ::prost::alloc::string::String,
        ::prost::alloc::vec::Vec<u8>,
    >,
    /// / Maps struct/module to a package version where it was first defined, stored as a vector for
    /// / simple serialization and deserialization.
    #[prost(message, repeated, tag = "4")]
    pub type_origin_table: ::prost::alloc::vec::Vec<TypeOrigin>,
    /// For each dependency, maps original package ID to the info about the (upgraded) dependency
    /// version that this package is using
    #[prost(message, repeated, tag = "5")]
    pub linkage_table: ::prost::alloc::vec::Vec<LinkageTablePair>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct LinkageTablePair {
    #[prost(message, optional, tag = "1")]
    pub key: ::core::option::Option<ObjectId>,
    #[prost(message, optional, tag = "2")]
    pub value: ::core::option::Option<UpgradeInfo>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DynamicFieldInfo {
    #[prost(message, optional, tag = "1")]
    pub name: ::core::option::Option<DynamicFieldName>,
    #[prost(bytes = "vec", tag = "2")]
    pub bcs_name: ::prost::alloc::vec::Vec<u8>,
    #[prost(message, optional, tag = "3")]
    pub r#type: ::core::option::Option<DynamicFieldType>,
    #[prost(string, tag = "4")]
    pub object_type: ::prost::alloc::string::String,
    #[prost(message, optional, tag = "5")]
    pub object_id: ::core::option::Option<ObjectId>,
    #[prost(uint64, tag = "6")]
    pub version: u64,
    #[prost(string, tag = "7")]
    pub digest: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DynamicFieldName {
    #[prost(message, optional, tag = "1")]
    pub r#type: ::core::option::Option<TypeTag>,
    #[prost(message, optional, tag = "2")]
    pub value: ::core::option::Option<Value>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DynamicFieldType {
    #[prost(oneof = "dynamic_field_type::DynamicFieldType", tags = "1, 2")]
    pub dynamic_field_type: ::core::option::Option<dynamic_field_type::DynamicFieldType>,
}
/// Nested message and enum types in `DynamicFieldType`.
pub mod dynamic_field_type {
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum DynamicFieldType {
        #[prost(uint32, tag = "1")]
        DynamicField(u32),
        #[prost(uint32, tag = "2")]
        DynamicObject(u32),
    }
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct IndexedDeletedObject {
    #[prost(message, optional, tag = "1")]
    pub object_id: ::core::option::Option<ObjectId>,
    #[prost(uint64, tag = "2")]
    pub object_version: u64,
    #[prost(uint64, tag = "3")]
    pub checkpoint_sequence_number: u64,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct IndexedPackage {
    #[prost(message, optional, tag = "1")]
    pub package_id: ::core::option::Option<ObjectId>,
    #[prost(message, optional, tag = "2")]
    pub move_package: ::core::option::Option<MovePackage>,
    #[prost(uint64, tag = "3")]
    pub checkpoint_sequence_number: u64,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Value {
    #[prost(oneof = "value::Value", tags = "1, 2, 3, 4, 5, 6")]
    pub value: ::core::option::Option<value::Value>,
}
/// Nested message and enum types in `Value`.
pub mod value {
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum Value {
        #[prost(message, tag = "1")]
        Null(()),
        #[prost(bool, tag = "2")]
        Bool(bool),
        #[prost(string, tag = "3")]
        Number(::prost::alloc::string::String),
        #[prost(string, tag = "4")]
        String(::prost::alloc::string::String),
        #[prost(message, tag = "5")]
        Array(super::ListOfValues),
        #[prost(message, tag = "6")]
        Object(super::ValueMap),
    }
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ValueMap {
    #[prost(map = "string, message", tag = "6")]
    pub map: ::std::collections::HashMap<::prost::alloc::string::String, Value>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListOfValues {
    #[prost(message, repeated, tag = "1")]
    pub list: ::prost::alloc::vec::Vec<Value>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct EndOfEpochData {
    /// next_epoch_committee is `Some` if and only if the current checkpoint is
    /// the last checkpoint of an epoch.
    /// Therefore next_epoch_committee can be used to pick the last checkpoint of an epoch,
    /// which is often useful to get epoch level summary stats like total gas cost of an epoch,
    /// or the total number of transactions from genesis to the end of an epoch.
    /// he committee is stored as a vector of validator pub key and stake pairs. The vector
    /// should be sorted based on the Committee data structure.
    #[prost(message, repeated, tag = "1")]
    pub next_epoch_committee: ::prost::alloc::vec::Vec<NextEpochCommittee>,
    /// The protocol version that is in effect during the epoch that starts immediately after this checkpoint.
    #[prost(uint64, tag = "2")]
    pub next_epoch_protocol_version: u64,
    /// Commitments to epoch specific state (e.g. live object set)
    #[prost(message, repeated, tag = "3")]
    pub epoch_commitments: ::prost::alloc::vec::Vec<CheckpointCommitment>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct NextEpochCommittee {
    /// base64 value
    #[prost(string, tag = "1")]
    pub authority_name: ::prost::alloc::string::String,
    #[prost(uint64, tag = "2")]
    pub stake_unit: u64,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CheckpointCommitment {
    #[prost(oneof = "checkpoint_commitment::CheckpointCommitment", tags = "1")]
    pub checkpoint_commitment: ::core::option::Option<
        checkpoint_commitment::CheckpointCommitment,
    >,
}
/// Nested message and enum types in `CheckpointCommitment`.
pub mod checkpoint_commitment {
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum CheckpointCommitment {
        #[prost(message, tag = "1")]
        EcmhLiveObjectSetDigest(super::EcmhLiveObjectSetDigest),
    }
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct EcmhLiveObjectSetDigest {
    /// base58 encoded
    #[prost(string, tag = "1")]
    pub digest: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct TransactionBlock {
    #[prost(message, optional, tag = "1")]
    pub data: ::core::option::Option<TransactionBlockData>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct TransactionBlockData {
    #[prost(oneof = "transaction_block_data::SuiTransactionBlockData", tags = "1")]
    pub sui_transaction_block_data: ::core::option::Option<
        transaction_block_data::SuiTransactionBlockData,
    >,
}
/// Nested message and enum types in `TransactionBlockData`.
pub mod transaction_block_data {
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum SuiTransactionBlockData {
        #[prost(message, tag = "1")]
        V1(super::TransactionBlockDataV1),
    }
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct TransactionBlockDataV1 {
    #[prost(message, optional, tag = "1")]
    pub transaction: ::core::option::Option<TransactionBlockKind>,
    #[prost(string, tag = "2")]
    pub sender: ::prost::alloc::string::String,
    #[prost(message, optional, tag = "3")]
    pub gas_data: ::core::option::Option<GasData>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct TransactionBlockKind {
    #[prost(
        oneof = "transaction_block_kind::SuiTransactionBlockKind",
        tags = "1, 2, 3, 4"
    )]
    pub sui_transaction_block_kind: ::core::option::Option<
        transaction_block_kind::SuiTransactionBlockKind,
    >,
}
/// Nested message and enum types in `TransactionBlockKind`.
pub mod transaction_block_kind {
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum SuiTransactionBlockKind {
        /// A system transaction that will update epoch information on-chain.
        #[prost(message, tag = "1")]
        ChangeEpoch(super::ChangeEpoch),
        /// A system transaction used for initializing the initial state of the chain.
        #[prost(message, tag = "2")]
        Genesis(super::GenesisTransaction),
        /// A system transaction marking the start of a series of transactions scheduled as part of a checkpoint
        #[prost(message, tag = "3")]
        ConsensusCommitPrologue(super::ConsensusCommitPrologue),
        /// A series of transactions where the results of one transaction can be used in future transactions
        #[prost(message, tag = "4")]
        ProgrammableTransaction(super::ProgrammableTransaction),
    }
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GasData {
    #[prost(message, repeated, tag = "1")]
    pub payment: ::prost::alloc::vec::Vec<ObjectRef>,
    #[prost(string, tag = "2")]
    pub owner: ::prost::alloc::string::String,
    #[prost(uint64, tag = "3")]
    pub price: u64,
    #[prost(uint64, tag = "4")]
    pub budget: u64,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ObjectRef {
    /// Hex code as string representing the object id
    #[prost(message, optional, tag = "1")]
    pub object_id: ::core::option::Option<ObjectId>,
    /// Object version.
    #[prost(uint64, tag = "2")]
    pub version: u64,
    /// Base58 string representing the object digest
    #[prost(string, tag = "3")]
    pub digest: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ChangeEpoch {
    #[prost(uint64, tag = "1")]
    pub epoch: u64,
    #[prost(uint64, tag = "2")]
    pub storage_charge: u64,
    #[prost(uint64, tag = "3")]
    pub computation_charge: u64,
    #[prost(uint64, tag = "4")]
    pub storage_rebate: u64,
    #[prost(uint64, tag = "5")]
    pub epoch_start_timestamp_ms: u64,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GenesisTransaction {
    #[prost(message, repeated, tag = "1")]
    pub objects: ::prost::alloc::vec::Vec<ObjectId>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ObjectId {
    #[prost(string, tag = "1")]
    pub account_address: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ConsensusCommitPrologue {
    #[prost(uint64, tag = "1")]
    pub epoch: u64,
    #[prost(uint64, tag = "2")]
    pub round: u64,
    #[prost(uint64, tag = "3")]
    pub commit_timestamp_ms: u64,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ProgrammableTransaction {
    /// Input objects or primitive values
    #[prost(message, repeated, tag = "1")]
    pub inputs: ::prost::alloc::vec::Vec<SuiCallArg>,
    /// The transactions to be executed sequentially. A failure in any transaction will
    /// result in the failure of the entire programmable transaction block.
    #[prost(message, repeated, tag = "2")]
    pub commands: ::prost::alloc::vec::Vec<SuiCommand>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SuiCallArg {
    #[prost(oneof = "sui_call_arg::SuiCallArg", tags = "1, 2")]
    pub sui_call_arg: ::core::option::Option<sui_call_arg::SuiCallArg>,
}
/// Nested message and enum types in `SuiCallArg`.
pub mod sui_call_arg {
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum SuiCallArg {
        /// Needs to become an Object Ref or Object ID, depending on object type
        #[prost(message, tag = "1")]
        Object(super::SuiObjectArg),
        /// pure value, bcs encoded
        #[prost(message, tag = "2")]
        Pure(super::SuiPureValue),
    }
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SuiObjectArg {
    #[prost(oneof = "sui_object_arg::SuiObjectArg", tags = "1, 2")]
    pub sui_object_arg: ::core::option::Option<sui_object_arg::SuiObjectArg>,
}
/// Nested message and enum types in `SuiObjectArg`.
pub mod sui_object_arg {
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum SuiObjectArg {
        /// A Move object, either immutable, or owned mutable.
        #[prost(message, tag = "1")]
        ImmOrOwnedObject(super::ImmOrOwnedObject),
        /// A Move object that's shared.
        /// SharedObject::mutable controls whether caller asks for a mutable reference to shared object.
        #[prost(message, tag = "2")]
        SharedObject(super::SharedObject),
    }
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ImmOrOwnedObject {
    #[prost(message, optional, tag = "1")]
    pub object_id: ::core::option::Option<ObjectId>,
    #[prost(uint64, tag = "2")]
    pub version: u64,
    #[prost(string, tag = "3")]
    pub digest: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SharedObject {
    #[prost(message, optional, tag = "1")]
    pub object_id: ::core::option::Option<ObjectId>,
    #[prost(uint64, tag = "2")]
    pub initial_shared_version: u64,
    #[prost(bool, tag = "3")]
    pub mutable: bool,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SuiPureValue {
    #[prost(message, optional, tag = "1")]
    pub value_type: ::core::option::Option<TypeTag>,
    #[prost(message, optional, tag = "2")]
    pub value: ::core::option::Option<Value>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct TypeTag {
    #[prost(oneof = "type_tag::TypeTag", tags = "1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11")]
    pub type_tag: ::core::option::Option<type_tag::TypeTag>,
}
/// Nested message and enum types in `TypeTag`.
pub mod type_tag {
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum TypeTag {
        #[prost(message, tag = "1")]
        Bool(()),
        #[prost(message, tag = "2")]
        U8(()),
        #[prost(message, tag = "3")]
        U64(()),
        #[prost(message, tag = "4")]
        U128(()),
        #[prost(message, tag = "5")]
        Address(()),
        #[prost(message, tag = "6")]
        Signer(()),
        #[prost(message, tag = "7")]
        Vector(::prost::alloc::boxed::Box<super::TypeTag>),
        #[prost(message, tag = "8")]
        Struct(super::StructTag),
        #[prost(message, tag = "9")]
        U16(()),
        #[prost(message, tag = "10")]
        U32(()),
        #[prost(message, tag = "11")]
        U256(()),
    }
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListOfTypeTags {
    #[prost(message, repeated, tag = "1")]
    pub list: ::prost::alloc::vec::Vec<TypeTag>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct StructTag {
    #[prost(string, tag = "1")]
    pub address: ::prost::alloc::string::String,
    #[prost(string, tag = "2")]
    pub module: ::prost::alloc::string::String,
    #[prost(string, tag = "3")]
    pub name: ::prost::alloc::string::String,
    #[prost(message, optional, tag = "4")]
    pub type_params: ::core::option::Option<ListOfTypeTags>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SuiCommand {
    #[prost(oneof = "sui_command::SuiCommand", tags = "1, 2, 3, 4, 5, 6, 7")]
    pub sui_command: ::core::option::Option<sui_command::SuiCommand>,
}
/// Nested message and enum types in `SuiCommand`.
pub mod sui_command {
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum SuiCommand {
        /// A call to either an entry or a public Move function
        #[prost(message, tag = "1")]
        MoveCall(super::SuiProgrammableMoveCall),
        /// `(Vec<forall T:key+store. T>, address)`
        /// It sends n-objects to the specified address. These objects must have store
        /// (public transfer) and either the previous owner must be an address or the object must
        /// be newly created.
        #[prost(message, tag = "2")]
        TransferObjects(super::TransferObjectsPair),
        /// `(&mut Coin<T>, Vec<u64>)` -> `Vec<Coin<T>>`
        /// It splits off some amounts into a new coins with those amounts
        #[prost(message, tag = "3")]
        SplitCoins(super::SplitCoinsPair),
        /// `(&mut Coin<T>, Vec<Coin<T>>)`
        /// It merges n-coins into the first coin
        #[prost(message, tag = "4")]
        MergeCoins(super::MergeCoinsPair),
        /// Publishes a Move package. It takes the package bytes and a list of the package's transitive
        /// dependencies to link against on-chain.
        #[prost(message, tag = "5")]
        Publish(super::ListOfObjects),
        /// Upgrades a Move package
        #[prost(message, tag = "6")]
        Upgrade(super::SuiCommandUpgrade),
        /// `forall T: Vec<T> -> vector<T>`
        /// Given n-values of the same type, it constructs a vector. For non objects or an empty vector,
        /// the type tag must be specified.
        #[prost(message, tag = "7")]
        MakeMoveVec(super::MakeMoveVecPair),
    }
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct TransferObjectsPair {
    #[prost(message, repeated, tag = "1")]
    pub one: ::prost::alloc::vec::Vec<SuiArgument>,
    #[prost(message, optional, tag = "2")]
    pub two: ::core::option::Option<SuiArgument>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SplitCoinsPair {
    #[prost(message, optional, tag = "1")]
    pub one: ::core::option::Option<SuiArgument>,
    #[prost(message, repeated, tag = "2")]
    pub two: ::prost::alloc::vec::Vec<SuiArgument>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MergeCoinsPair {
    #[prost(message, optional, tag = "1")]
    pub one: ::core::option::Option<SuiArgument>,
    #[prost(message, repeated, tag = "2")]
    pub two: ::prost::alloc::vec::Vec<SuiArgument>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListOfObjects {
    #[prost(message, repeated, tag = "1")]
    pub list: ::prost::alloc::vec::Vec<ObjectId>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MakeMoveVecPair {
    #[prost(string, optional, tag = "1")]
    pub one: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(message, repeated, tag = "2")]
    pub two: ::prost::alloc::vec::Vec<SuiArgument>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SuiCommandUpgrade {
    #[prost(message, optional, tag = "1")]
    pub one: ::core::option::Option<ListOfObjects>,
    #[prost(message, optional, tag = "2")]
    pub two: ::core::option::Option<ObjectId>,
    #[prost(message, optional, tag = "3")]
    pub three: ::core::option::Option<SuiArgument>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SuiProgrammableMoveCall {
    /// The package containing the module and function.
    #[prost(message, optional, tag = "1")]
    pub package: ::core::option::Option<ObjectId>,
    /// The specific module in the package containing the function.
    #[prost(string, tag = "2")]
    pub module: ::prost::alloc::string::String,
    /// The function to be called.
    #[prost(string, tag = "3")]
    pub function: ::prost::alloc::string::String,
    /// The type arguments to the function.
    #[prost(string, repeated, tag = "4")]
    pub type_arguments: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    #[prost(message, repeated, tag = "5")]
    pub arguments: ::prost::alloc::vec::Vec<SuiArgument>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SuiArgument {
    #[prost(oneof = "sui_argument::SuiArguments", tags = "1, 2, 3, 4")]
    pub sui_arguments: ::core::option::Option<sui_argument::SuiArguments>,
}
/// Nested message and enum types in `SuiArgument`.
pub mod sui_argument {
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum SuiArguments {
        /// The gas coin. The gas coin can only be used by-ref, except for with
        /// `TransferObjects`, which can use it by-value.
        #[prost(message, tag = "1")]
        GasCoin(()),
        /// One of the input objects or primitive values (from
        /// `ProgrammableTransactionBlock` inputs)
        #[prost(uint32, tag = "2")]
        Input(u32),
        /// The result of another transaction (from `ProgrammableTransactionBlock` transactions)
        #[prost(uint32, tag = "3")]
        Result(u32),
        /// Like a `Result` but it accesses a nested result. Currently, the only usage
        /// of this is to access a value from a Move call with multiple return values.
        #[prost(message, tag = "4")]
        NestedResult(super::PairOfU32),
    }
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PairOfU32 {
    #[prost(uint32, tag = "1")]
    pub one: u32,
    #[prost(uint32, tag = "2")]
    pub two: u32,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct TransactionBlockEffects {
    #[prost(
        oneof = "transaction_block_effects::SuiTransactionBlockEffects",
        tags = "1, 2"
    )]
    pub sui_transaction_block_effects: ::core::option::Option<
        transaction_block_effects::SuiTransactionBlockEffects,
    >,
}
/// Nested message and enum types in `TransactionBlockEffects`.
pub mod transaction_block_effects {
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum SuiTransactionBlockEffects {
        #[prost(message, tag = "1")]
        V1(super::TransactionBlockEffectsV1),
        #[prost(message, tag = "2")]
        V2(super::TransactionBlockEffectsV2),
    }
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct TransactionBlockEffectsV1 {
    /// The status of the execution
    #[prost(message, optional, tag = "1")]
    pub status: ::core::option::Option<ExecutionStatus>,
    /// The epoch when this transaction was executed.
    #[prost(uint64, tag = "2")]
    pub executed_epoch: u64,
    /// Gas cost summary of the transaction
    #[prost(message, optional, tag = "3")]
    pub gas_used: ::core::option::Option<GasCostSummary>,
    /// The version that every modified (mutated or deleted) object had before it was modified by this transaction.
    #[prost(message, repeated, tag = "4")]
    pub modified_at_versions: ::prost::alloc::vec::Vec<
        TransactionBlockEffectsModifiedAtVersions,
    >,
    /// The object references of the shared objects used in this transaction. Empty if no shared objects were used.
    #[prost(message, repeated, tag = "5")]
    pub shared_objects: ::prost::alloc::vec::Vec<ObjectRef>,
    /// The transaction digest
    #[prost(string, tag = "6")]
    pub transaction_digest: ::prost::alloc::string::String,
    /// ObjectRef and owner of new objects created.
    #[prost(message, repeated, tag = "7")]
    pub created: ::prost::alloc::vec::Vec<OwnedObjectRef>,
    /// ObjectRef and owner of mutated objects, including gas object.
    #[prost(message, repeated, tag = "8")]
    pub mutated: ::prost::alloc::vec::Vec<OwnedObjectRef>,
    /// ObjectRef and owner of objects that are unwrapped in this transaction.
    /// Unwrapped objects are objects that were wrapped into other objects in the past,
    /// and just got extracted out.
    #[prost(message, repeated, tag = "9")]
    pub unwrapped: ::prost::alloc::vec::Vec<OwnedObjectRef>,
    /// Object Refs of objects now deleted (the old refs).
    #[prost(message, repeated, tag = "10")]
    pub deleted: ::prost::alloc::vec::Vec<ObjectRef>,
    /// Object refs of objects previously wrapped in other objects but now deleted.
    #[prost(message, repeated, tag = "11")]
    pub unwrapped_then_deleted: ::prost::alloc::vec::Vec<ObjectRef>,
    /// Object refs of objects now wrapped in other objects.
    #[prost(message, repeated, tag = "12")]
    pub wrapped: ::prost::alloc::vec::Vec<ObjectRef>,
    /// The updated gas object reference. Have a dedicated field for convenient access.
    /// It's also included in mutated.
    #[prost(message, optional, tag = "13")]
    pub gas_object: ::core::option::Option<OwnedObjectRef>,
    /// The digest of the events emitted during execution,
    /// can be None if the transaction does not emit any event.
    #[prost(string, optional, tag = "14")]
    pub events_digest: ::core::option::Option<::prost::alloc::string::String>,
    /// The set of transaction digests this transaction depends on.
    #[prost(string, repeated, tag = "15")]
    pub dependencies: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct TransactionBlockEffectsV2 {
    /// The status of the execution
    #[prost(message, optional, tag = "1")]
    pub status: ::core::option::Option<ExecutionStatus>,
    /// The epoch when this transaction was executed.
    #[prost(uint64, tag = "2")]
    pub executed_epoch: u64,
    /// Gas cost summary of the transaction
    #[prost(message, optional, tag = "3")]
    pub gas_used: ::core::option::Option<GasCostSummary>,
    /// The transaction digest
    #[prost(string, tag = "4")]
    pub transaction_digest: ::prost::alloc::string::String,
    /// / The updated gas object reference, as an index into the `changed_objects` vector.
    /// / Having a dedicated field for convenient access.
    /// / System transaction that don't require gas will leave this as None.
    #[prost(uint32, optional, tag = "5")]
    pub gas_object_index: ::core::option::Option<u32>,
    /// / The digest of the events emitted during execution,
    /// / can be None if the transaction does not emit any event.
    #[prost(string, optional, tag = "6")]
    pub events_digest: ::core::option::Option<::prost::alloc::string::String>,
    /// The set of transaction digests this transaction depends on.
    #[prost(string, repeated, tag = "7")]
    pub dependencies: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    /// / The version number of all the written Move objects by this transaction.
    #[prost(uint64, tag = "8")]
    pub lamport_version: u64,
    /// / Objects whose state are changed in the object store.
    #[prost(message, repeated, tag = "9")]
    pub changed_objects: ::prost::alloc::vec::Vec<ChangedObjectV2>,
    /// / Shared objects that are not mutated in this transaction. Unlike owned objects,
    /// / read-only shared objects' version are not committed in the transaction,
    /// / and in order for a node to catch up and execute it without consensus sequencing,
    /// / the version needs to be committed in the effects.
    #[prost(message, repeated, tag = "10")]
    pub unchanged_shared_objects: ::prost::alloc::vec::Vec<UnchangedSharedObject>,
    /// / Auxiliary data that are not protocol-critical, generated as part of the effects but are stored separately.
    /// / Storing it separately allows us to avoid bloating the effects with data that are not critical.
    /// / It also provides more flexibility on the format and type of the data.
    #[prost(string, optional, tag = "11")]
    pub aux_data_digest: ::core::option::Option<::prost::alloc::string::String>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct UnchangedSharedObject {
    #[prost(oneof = "unchanged_shared_object::UnchangedSharedObject", tags = "1, 2, 3")]
    pub unchanged_shared_object: ::core::option::Option<
        unchanged_shared_object::UnchangedSharedObject,
    >,
}
/// Nested message and enum types in `UnchangedSharedObject`.
pub mod unchanged_shared_object {
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum UnchangedSharedObject {
        /// / Read-only shared objects from the input. We don't really need ObjectDigest
        /// / for protocol correctness, but it will make it easier to verify untrusted read.
        #[prost(message, tag = "1")]
        ReadOnlyRoot(super::VersionDigest),
        /// / Deleted shared objects that appear mutably/owned in the input.
        #[prost(string, tag = "2")]
        MutateDeleted(::prost::alloc::string::String),
        /// / Deleted shared objects that appear as read-only in the input.
        #[prost(string, tag = "3")]
        ReadDeleted(::prost::alloc::string::String),
    }
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ChangedObjectV2 {
    #[prost(message, optional, tag = "1")]
    pub object_id: ::core::option::Option<ObjectId>,
    #[prost(message, optional, tag = "2")]
    pub effects: ::core::option::Option<EffectsObjectChange>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct EffectsObjectChange {
    /// input_state and output_state are the core fields that's required by
    /// the protocol as it tells how an object changes on-chain.
    /// / State of the object in the store prior to this transaction.
    #[prost(message, optional, tag = "1")]
    pub input_state: ::core::option::Option<ObjectIn>,
    /// / State of the object in the store after this transaction.
    #[prost(message, optional, tag = "2")]
    pub output_state: ::core::option::Option<ObjectOut>,
    /// / Whether this object ID is created or deleted in this transaction.
    /// / This information isn't required by the protocol but is useful for providing more detailed
    /// / semantics on object changes.
    #[prost(message, optional, tag = "3")]
    pub id_operation: ::core::option::Option<IdOperation>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct IdOperation {
    #[prost(oneof = "id_operation::IdoOperation", tags = "1, 2, 3")]
    pub ido_operation: ::core::option::Option<id_operation::IdoOperation>,
}
/// Nested message and enum types in `IDOperation`.
pub mod id_operation {
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum IdoOperation {
        #[prost(uint32, tag = "1")]
        None(u32),
        #[prost(uint32, tag = "2")]
        Created(u32),
        #[prost(uint32, tag = "3")]
        Deleted(u32),
    }
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ObjectOut {
    #[prost(oneof = "object_out::ObjectId", tags = "1, 2")]
    pub object_id: ::core::option::Option<object_out::ObjectId>,
}
/// Nested message and enum types in `ObjectOut`.
pub mod object_out {
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum ObjectId {
        #[prost(uint32, tag = "1")]
        NotExist(u32),
        /// / The old version, digest and owner.
        #[prost(message, tag = "2")]
        Exist(super::ObjectInExist),
    }
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ObjectIn {
    #[prost(oneof = "object_in::ObjectId", tags = "1, 2, 3")]
    pub object_id: ::core::option::Option<object_in::ObjectId>,
}
/// Nested message and enum types in `ObjectIn`.
pub mod object_in {
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum ObjectId {
        #[prost(uint32, tag = "1")]
        NotExist(u32),
        /// / Any written object, including all of mutated, created, unwrapped today.
        #[prost(message, tag = "2")]
        ObjectWrite(super::ObjectWrite),
        /// / Packages writes need to be tracked separately with version because
        /// / we don't use lamport version for package publish and upgrades.
        #[prost(message, tag = "3")]
        PackageWrite(super::PackageWrite),
    }
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ObjectWrite {
    #[prost(string, tag = "1")]
    pub object_digest: ::prost::alloc::string::String,
    #[prost(message, optional, tag = "2")]
    pub owner: ::core::option::Option<Owner>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PackageWrite {
    #[prost(message, optional, tag = "1")]
    pub version_digest: ::core::option::Option<VersionDigest>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ObjectInExist {
    #[prost(message, optional, tag = "1")]
    pub version_digest: ::core::option::Option<VersionDigest>,
    #[prost(message, optional, tag = "2")]
    pub owner: ::core::option::Option<Owner>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct VersionDigest {
    #[prost(uint32, tag = "1")]
    pub sequence_number: u32,
    #[prost(string, tag = "2")]
    pub object_digest: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct OwnedObjectRef {
    #[prost(message, optional, tag = "1")]
    pub owner: ::core::option::Option<Owner>,
    #[prost(message, optional, tag = "2")]
    pub reference: ::core::option::Option<ObjectRef>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Owner {
    #[prost(oneof = "owner::Owner", tags = "1, 2, 3, 4")]
    pub owner: ::core::option::Option<owner::Owner>,
}
/// Nested message and enum types in `Owner`.
pub mod owner {
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum Owner {
        /// Object is exclusively owned by a single address, and is mutable.
        #[prost(string, tag = "1")]
        AddressOwner(::prost::alloc::string::String),
        /// Object is exclusively owned by a single object, and is mutable.
        /// The object ID is converted to SuiAddress as SuiAddress is universal.
        #[prost(string, tag = "2")]
        ObjectOwner(::prost::alloc::string::String),
        /// Object is shared, can be used by any address, and is mutable.
        #[prost(message, tag = "3")]
        Shared(super::Shared),
        /// Object is immutable, and hence ownership doesn't matter.
        #[prost(message, tag = "4")]
        Immutable(()),
    }
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Shared {
    /// The version at which the object became shared
    #[prost(uint64, tag = "1")]
    pub initial_shared_version: u64,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ExecutionStatus {
    #[prost(oneof = "execution_status::SuiExecutionStatus", tags = "1, 2")]
    pub sui_execution_status: ::core::option::Option<
        execution_status::SuiExecutionStatus,
    >,
}
/// Nested message and enum types in `ExecutionStatus`.
pub mod execution_status {
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum SuiExecutionStatus {
        /// Gas used in the success case.
        #[prost(message, tag = "1")]
        Success(()),
        /// Gas used in the failed case, and the error.
        #[prost(message, tag = "2")]
        Failure(super::Failure),
    }
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Failure {
    #[prost(string, tag = "1")]
    pub error: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct TransactionBlockEffectsModifiedAtVersions {
    #[prost(message, optional, tag = "1")]
    pub object_id: ::core::option::Option<ObjectId>,
    #[prost(uint64, tag = "2")]
    pub sequence_number: u64,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ObjectType {
    #[prost(oneof = "object_type::ObjectType", tags = "1, 2")]
    pub object_type: ::core::option::Option<object_type::ObjectType>,
}
/// Nested message and enum types in `ObjectType`.
pub mod object_type {
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum ObjectType {
        /// Move package containing one or more bytecode modules
        #[prost(message, tag = "1")]
        Package(()),
        /// A Move struct of the given type
        #[prost(message, tag = "2")]
        Struct(super::MoveObjectType),
    }
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MoveObjectType {
    #[prost(oneof = "move_object_type::MoveObjectType", tags = "1, 2, 3, 4")]
    pub move_object_type: ::core::option::Option<move_object_type::MoveObjectType>,
}
/// Nested message and enum types in `MoveObjectType`.
pub mod move_object_type {
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum MoveObjectType {
        /// A type that is not `0x2::coin::Coin<T>`
        #[prost(message, tag = "1")]
        Other(super::StructTag),
        /// A SUI coin (i.e., `0x2::coin::Coin<0x2::sui::SUI>`)
        #[prost(message, tag = "2")]
        GasCoin(()),
        /// A record of a staked SUI coin (i.e., `0x3::staking_pool::StakedSui`)
        #[prost(message, tag = "3")]
        StakedSui(()),
        /// A non-SUI coin type (i.e., `0x2::coin::Coin<T> where T != 0x2::sui::SUI`)
        #[prost(message, tag = "4")]
        Coin(super::TypeTag),
    }
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct TypeOrigin {
    #[prost(string, tag = "1")]
    pub module_name: ::prost::alloc::string::String,
    #[prost(string, tag = "2")]
    pub struct_name: ::prost::alloc::string::String,
    #[prost(message, optional, tag = "3")]
    pub package: ::core::option::Option<ObjectId>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct UpgradeInfo {
    #[prost(message, optional, tag = "1")]
    pub upgraded_id: ::core::option::Option<ObjectId>,
    #[prost(uint64, tag = "2")]
    pub upgraded_version: u64,
}
