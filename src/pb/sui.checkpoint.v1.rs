#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CheckpointData {
    #[prost(message, optional, tag = "1")]
    pub checkpoint: ::core::option::Option<Checkpoint>,
    #[prost(message, repeated, tag = "2")]
    pub transactions: ::prost::alloc::vec::Vec<CheckpointTransactionBlockResponse>,
    #[prost(message, repeated, tag = "3")]
    pub changed_objects: ::prost::alloc::vec::Vec<ChangedObject>,
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
    pub epoch_rolling_gas_cost_summary: ::core::option::Option<GasCostSummary>,
    /// Timestamp of the checkpoint - number of milliseconds from the Unix epoch
    /// Checkpoint timestamps are monotonic, but not strongly monotonic - subsequent
    /// checkpoints can have same timestamp if they originate from the same underlining consensus commit
    #[prost(uint64, tag = "7")]
    pub timestamp_ms: u64,
    /// Present only on the final checkpoint of the epoch.
    #[prost(message, optional, tag = "8")]
    pub end_of_epoch_data: ::core::option::Option<EndOfEpochData>,
    /// Transaction digests (base58 encoded)
    #[prost(string, repeated, tag = "9")]
    pub transactions: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    /// Commitments to checkpoint state
    #[prost(message, repeated, tag = "10")]
    pub checkpoint_commitments: ::prost::alloc::vec::Vec<CheckpointCommitment>,
    /// Validator Signature (base64  encoded). This is a BLS signature
    #[prost(bytes = "vec", tag = "11")]
    pub validator_signature: ::prost::alloc::vec::Vec<u8>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CheckpointTransactionBlockResponse {
    #[prost(string, tag = "1")]
    pub digest: ::prost::alloc::string::String,
    /// Transaction input data
    #[prost(message, optional, tag = "2")]
    pub transaction: ::core::option::Option<SuiTransactionBlock>,
    #[prost(bytes = "vec", tag = "3")]
    pub raw_transaction: ::prost::alloc::vec::Vec<u8>,
    #[prost(message, optional, tag = "4")]
    pub effects: ::core::option::Option<SuiTransactionBlockEffects>,
    #[prost(message, optional, tag = "5")]
    pub events: ::core::option::Option<SuiTransactionBlockEvents>,
    #[prost(uint64, tag = "6")]
    pub timestamp_ms: u64,
    #[prost(bool, optional, tag = "7")]
    pub confirmed_local_execution: ::core::option::Option<bool>,
    #[prost(uint64, tag = "8")]
    pub checkpoint: u64,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GasCostSummary {
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
pub struct EndOfEpochData {
    /// next_epoch_committee is `Some` if and only if the current checkpoint is
    /// the last checkpoint of an epoch.
    /// Therefore next_epoch_committee can be used to pick the last checkpoint of an epoch,
    /// which is often useful to get epoch level summary stats like total gas cost of an epoch,
    /// or the total number of transactions from genesis to the end of an epoch.
    /// The committee is stored as a vector of validator pub key and stake pairs. The vector
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
pub struct SuiTransactionBlock {
    #[prost(message, optional, tag = "1")]
    pub data: ::core::option::Option<SuiTransactionBlockData>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SuiTransactionBlockData {
    #[prost(oneof = "sui_transaction_block_data::SuiTransactionBlockData", tags = "1")]
    pub sui_transaction_block_data: ::core::option::Option<
        sui_transaction_block_data::SuiTransactionBlockData,
    >,
}
/// Nested message and enum types in `SuiTransactionBlockData`.
pub mod sui_transaction_block_data {
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum SuiTransactionBlockData {
        #[prost(message, tag = "1")]
        V1(super::SuiTransactionBlockDataV1),
    }
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SuiTransactionBlockDataV1 {
    #[prost(message, optional, tag = "1")]
    pub transaction: ::core::option::Option<SuiTransactionBlockKind>,
    #[prost(string, tag = "2")]
    pub sender: ::prost::alloc::string::String,
    #[prost(message, optional, tag = "3")]
    pub gas_data: ::core::option::Option<SuiGasData>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SuiTransactionBlockKind {
    #[prost(
        oneof = "sui_transaction_block_kind::SuiTransactionBlockKind",
        tags = "1, 2, 3, 4"
    )]
    pub sui_transaction_block_kind: ::core::option::Option<
        sui_transaction_block_kind::SuiTransactionBlockKind,
    >,
}
/// Nested message and enum types in `SuiTransactionBlockKind`.
pub mod sui_transaction_block_kind {
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum SuiTransactionBlockKind {
        /// A system transaction that will update epoch information on-chain.
        #[prost(message, tag = "1")]
        ChangeEpoch(super::SuiChangeEpoch),
        /// A system transaction used for initializing the initial state of the chain.
        #[prost(message, tag = "2")]
        Genesis(super::SuiGenesisTransaction),
        /// A system transaction marking the start of a series of transactions scheduled as part of a checkpoint
        #[prost(message, tag = "3")]
        ConsensusCommitPrologue(super::SuiConsensusCommitPrologue),
        /// A series of transactions where the results of one transaction can be used in future transactions
        #[prost(message, tag = "4")]
        ProgrammableTransaction(super::SuiProgrammableTransactionBlock),
    }
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SuiGasData {
    #[prost(message, repeated, tag = "1")]
    pub payment: ::prost::alloc::vec::Vec<SuiObjectRef>,
    #[prost(string, tag = "2")]
    pub owner: ::prost::alloc::string::String,
    #[prost(uint64, tag = "3")]
    pub price: u64,
    #[prost(uint64, tag = "4")]
    pub budget: u64,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SuiObjectRef {
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
pub struct SuiChangeEpoch {
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
pub struct SuiGenesisTransaction {
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
pub struct SuiConsensusCommitPrologue {
    #[prost(uint64, tag = "1")]
    pub epoch: u64,
    #[prost(uint64, tag = "2")]
    pub round: u64,
    #[prost(uint64, tag = "3")]
    pub commit_timestamp_ms: u64,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SuiProgrammableTransactionBlock {
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
    pub value: ::core::option::Option<SuiJsonValue>,
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
pub struct SuiJsonValue {
    #[prost(oneof = "sui_json_value::Value", tags = "1, 2, 3, 4, 5, 6")]
    pub value: ::core::option::Option<sui_json_value::Value>,
}
/// Nested message and enum types in `SuiJsonValue`.
pub mod sui_json_value {
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
        Array(super::ListOfJsonValues),
        #[prost(message, tag = "6")]
        Object(super::SuiJsonValueMap),
    }
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SuiJsonValueMap {
    #[prost(map = "string, message", tag = "6")]
    pub map: ::std::collections::HashMap<::prost::alloc::string::String, SuiJsonValue>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListOfJsonValues {
    #[prost(message, repeated, tag = "1")]
    pub list: ::prost::alloc::vec::Vec<SuiJsonValue>,
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
pub struct SuiTransactionBlockEffects {
    #[prost(
        oneof = "sui_transaction_block_effects::SuiTransactionBlockEffects",
        tags = "1"
    )]
    pub sui_transaction_block_effects: ::core::option::Option<
        sui_transaction_block_effects::SuiTransactionBlockEffects,
    >,
}
/// Nested message and enum types in `SuiTransactionBlockEffects`.
pub mod sui_transaction_block_effects {
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum SuiTransactionBlockEffects {
        #[prost(message, tag = "1")]
        V1(super::SuiTransactionBlockEffectsV1),
    }
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SuiTransactionBlockEffectsV1 {
    /// The status of the execution
    #[prost(message, optional, tag = "1")]
    pub status: ::core::option::Option<SuiExecutionStatus>,
    /// The epoch when this transaction was executed.
    #[prost(uint64, tag = "2")]
    pub executed_epoch: u64,
    /// Gas cost summary of the transaction
    #[prost(message, optional, tag = "3")]
    pub gas_used: ::core::option::Option<GasCostSummary>,
    /// The version that every modified (mutated or deleted) object had before it was modified by this transaction.
    #[prost(message, repeated, tag = "4")]
    pub modified_at_versions: ::prost::alloc::vec::Vec<
        SuiTransactionBlockEffectsModifiedAtVersions,
    >,
    /// The object references of the shared objects used in this transaction. Empty if no shared objects were used.
    #[prost(message, repeated, tag = "5")]
    pub shared_objects: ::prost::alloc::vec::Vec<SuiObjectRef>,
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
    pub deleted: ::prost::alloc::vec::Vec<SuiObjectRef>,
    /// Object refs of objects previously wrapped in other objects but now deleted.
    #[prost(message, repeated, tag = "11")]
    pub unwrapped_then_deleted: ::prost::alloc::vec::Vec<SuiObjectRef>,
    /// Object refs of objects now wrapped in other objects.
    #[prost(message, repeated, tag = "12")]
    pub wrapped: ::prost::alloc::vec::Vec<SuiObjectRef>,
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
pub struct OwnedObjectRef {
    #[prost(message, optional, tag = "1")]
    pub owner: ::core::option::Option<Owner>,
    #[prost(message, optional, tag = "2")]
    pub reference: ::core::option::Option<SuiObjectRef>,
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
pub struct SuiExecutionStatus {
    #[prost(oneof = "sui_execution_status::SuiExecutionStatus", tags = "1, 2")]
    pub sui_execution_status: ::core::option::Option<
        sui_execution_status::SuiExecutionStatus,
    >,
}
/// Nested message and enum types in `SuiExecutionStatus`.
pub mod sui_execution_status {
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
pub struct SuiTransactionBlockEffectsModifiedAtVersions {
    #[prost(message, optional, tag = "1")]
    pub object_id: ::core::option::Option<ObjectId>,
    #[prost(uint64, tag = "2")]
    pub sequence_number: u64,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SuiTransactionBlockEvents {
    #[prost(message, repeated, tag = "1")]
    pub data: ::prost::alloc::vec::Vec<SuiEvent>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SuiEvent {
    /// Sequential event ID, ie (transaction seq number, event seq number).
    /// 1) Serves as a unique event ID for each fullnode
    /// 2) Also serves to sequence events for the purposes of pagination and querying.
    ///     A higher id is an event seen later by that fullnode.
    /// This ID is the "cursor" for event querying.
    #[prost(message, optional, tag = "1")]
    pub id: ::core::option::Option<EventId>,
    /// Move package where this event was emitted.
    #[prost(message, optional, tag = "2")]
    pub package_id: ::core::option::Option<ObjectId>,
    /// Move module where this event was emitted.
    #[prost(string, tag = "3")]
    pub transaction_module: ::prost::alloc::string::String,
    /// Sender's Sui address.
    #[prost(string, tag = "4")]
    pub sender: ::prost::alloc::string::String,
    /// Move event type.
    #[prost(message, optional, tag = "5")]
    pub r#type: ::core::option::Option<StructTag>,
    /// Parsed json value of the event
    #[prost(message, optional, tag = "6")]
    pub parsed_json: ::core::option::Option<SuiJsonValue>,
    /// Base 58 encoded bcs bytes of the move event
    #[prost(string, tag = "7")]
    pub bcs: ::prost::alloc::string::String,
    /// UTC timestamp in milliseconds since epoch (1/1/1970)
    #[prost(uint64, optional, tag = "8")]
    pub timestamp_ms: ::core::option::Option<u64>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct EventId {
    #[prost(string, tag = "1")]
    pub tx_digest: ::prost::alloc::string::String,
    #[prost(uint64, tag = "2")]
    pub event_seq: u64,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ChangedObject {
    #[prost(message, optional, tag = "1")]
    pub status: ::core::option::Option<ObjectStatus>,
    #[prost(message, optional, tag = "2")]
    pub data: ::core::option::Option<SuiObjectData>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ObjectStatus {
    #[prost(oneof = "object_status::ObjectStatus", tags = "1, 2, 3, 4, 5, 6")]
    pub object_status: ::core::option::Option<object_status::ObjectStatus>,
}
/// Nested message and enum types in `ObjectStatus`.
pub mod object_status {
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum ObjectStatus {
        #[prost(message, tag = "1")]
        Created(()),
        #[prost(message, tag = "2")]
        Mutated(()),
        #[prost(message, tag = "3")]
        Deleted(()),
        #[prost(message, tag = "4")]
        Wrapped(()),
        #[prost(message, tag = "5")]
        Unwrapped(()),
        #[prost(message, tag = "6")]
        UnwrappedThenDeleted(()),
    }
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SuiObjectData {
    /// Object id
    #[prost(message, optional, tag = "1")]
    pub object_id: ::core::option::Option<ObjectId>,
    /// Object version
    #[prost(uint64, tag = "2")]
    pub version: u64,
    /// Base58 string representing the object digest
    #[prost(string, tag = "3")]
    pub digest: ::prost::alloc::string::String,
    /// The type of the object. Default to be None unless SuiObjectDataOptions.showType is set to true
    #[prost(message, optional, tag = "4")]
    pub r#type: ::core::option::Option<ObjectType>,
    /// Default to be None because otherwise it will be repeated for the getOwnedObjects endpoint
    /// The owner of this object. Default to be None unless SuiObjectDataOptions.showOwner is set to true
    #[prost(message, optional, tag = "5")]
    pub owner: ::core::option::Option<Owner>,
    /// The digest of the transaction that created or last mutated this object. Default to be None unless
    /// SuiObjectDataOptions.showPreviousTransaction is set to true
    #[prost(string, optional, tag = "6")]
    pub previous_transaction: ::core::option::Option<::prost::alloc::string::String>,
    /// The amount of SUI we would rebate if this object gets deleted.
    /// This number is re-calculated each time the object is mutated based on
    /// the present storage gas price.
    #[prost(uint64, optional, tag = "7")]
    pub storage_rebate: ::core::option::Option<u64>,
    /// The Display metadata for frontend UI rendering, default to be None unless SuiObjectDataOptions.showContent is set to true
    /// This can also be None if the struct type does not have Display defined
    /// See more details in <<https://forums.sui.io/t/nft-object-display-proposal/4872>>
    #[prost(message, optional, tag = "8")]
    pub display: ::core::option::Option<DisplayFieldsResponse>,
    /// Move object content or package content, default to be None unless SuiObjectDataOptions.showContent is set to true
    #[prost(message, optional, tag = "9")]
    pub content: ::core::option::Option<SuiParsedData>,
    /// Move object content or package content in BCS, default to be None unless SuiObjectDataOptions.showBcs is set to true
    #[prost(message, optional, tag = "10")]
    pub bcs: ::core::option::Option<SuiRawData>,
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
pub struct DisplayFieldsResponse {
    #[prost(map = "string, string", tag = "1")]
    pub data: ::std::collections::HashMap<
        ::prost::alloc::string::String,
        ::prost::alloc::string::String,
    >,
    #[prost(message, optional, tag = "2")]
    pub error: ::core::option::Option<SuiObjectResponseError>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SuiObjectResponseError {
    #[prost(
        oneof = "sui_object_response_error::SuiObjectResponseError",
        tags = "1, 2, 3, 4, 5"
    )]
    pub sui_object_response_error: ::core::option::Option<
        sui_object_response_error::SuiObjectResponseError,
    >,
}
/// Nested message and enum types in `SuiObjectResponseError`.
pub mod sui_object_response_error {
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct NotExists {
        #[prost(message, optional, tag = "1")]
        pub object_id: ::core::option::Option<super::ObjectId>,
    }
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct DynamicFieldNotFound {
        #[prost(message, optional, tag = "1")]
        pub parent_object_id: ::core::option::Option<super::ObjectId>,
    }
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct Deleted {
        #[prost(message, optional, tag = "1")]
        pub object_id: ::core::option::Option<super::ObjectId>,
        #[prost(uint64, tag = "2")]
        pub version: u64,
        #[prost(string, tag = "3")]
        pub digest: ::prost::alloc::string::String,
    }
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct DisplayError {
        #[prost(string, tag = "1")]
        pub error: ::prost::alloc::string::String,
    }
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum SuiObjectResponseError {
        #[prost(message, tag = "1")]
        NotExists(NotExists),
        #[prost(message, tag = "2")]
        DynamicFieldNotFound(DynamicFieldNotFound),
        #[prost(message, tag = "3")]
        Deleted(Deleted),
        #[prost(message, tag = "4")]
        Unknown(()),
        #[prost(message, tag = "5")]
        DisplayError(DisplayError),
    }
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SuiParsedData {
    #[prost(oneof = "sui_parsed_data::SuiParsedData", tags = "1, 2")]
    pub sui_parsed_data: ::core::option::Option<sui_parsed_data::SuiParsedData>,
}
/// Nested message and enum types in `SuiParsedData`.
pub mod sui_parsed_data {
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum SuiParsedData {
        #[prost(message, tag = "1")]
        MoveObject(super::SuiParsedMoveObject),
        #[prost(message, tag = "2")]
        Package(super::SuiMovePackage),
    }
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SuiParsedMoveObject {
    #[prost(message, optional, tag = "1")]
    pub r#type: ::core::option::Option<StructTag>,
    #[prost(bool, tag = "2")]
    pub has_public_transfer: bool,
    #[prost(message, optional, tag = "3")]
    pub fields: ::core::option::Option<SuiMoveStruct>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SuiMoveStruct {
    #[prost(oneof = "sui_move_struct::SuiMoveStruct", tags = "1, 2, 3")]
    pub sui_move_struct: ::core::option::Option<sui_move_struct::SuiMoveStruct>,
}
/// Nested message and enum types in `SuiMoveStruct`.
pub mod sui_move_struct {
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum SuiMoveStruct {
        #[prost(message, tag = "1")]
        Runtime(super::ListOfSuiMoveValues),
        #[prost(message, tag = "2")]
        WithTypes(super::WithTypes),
        #[prost(message, tag = "3")]
        WithFields(super::WithFields),
    }
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListOfSuiMoveValues {
    #[prost(message, repeated, tag = "1")]
    pub list: ::prost::alloc::vec::Vec<SuiMoveValue>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct WithTypes {
    #[prost(message, optional, tag = "1")]
    pub r#type: ::core::option::Option<StructTag>,
    #[prost(map = "string, message", tag = "2")]
    pub fields: ::std::collections::HashMap<
        ::prost::alloc::string::String,
        SuiMoveValue,
    >,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct WithFields {
    #[prost(map = "string, message", tag = "2")]
    pub fields: ::std::collections::HashMap<
        ::prost::alloc::string::String,
        SuiMoveValue,
    >,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SuiMoveValue {
    #[prost(oneof = "sui_move_value::SuiMoveValue", tags = "1, 2, 3, 4, 5, 6, 7, 8")]
    pub sui_move_value: ::core::option::Option<sui_move_value::SuiMoveValue>,
}
/// Nested message and enum types in `SuiMoveValue`.
pub mod sui_move_value {
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum SuiMoveValue {
        #[prost(uint32, tag = "1")]
        Number(u32),
        #[prost(bool, tag = "2")]
        Bool(bool),
        #[prost(string, tag = "3")]
        Address(::prost::alloc::string::String),
        #[prost(message, tag = "4")]
        Vector(super::ListOfSuiMoveValues),
        #[prost(string, tag = "5")]
        String(::prost::alloc::string::String),
        #[prost(message, tag = "6")]
        Uid(super::Uid),
        #[prost(message, tag = "7")]
        Struct(super::SuiMoveStruct),
        #[prost(message, tag = "8")]
        Option(::prost::alloc::boxed::Box<super::SuiMoveValue>),
    }
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Uid {
    #[prost(message, optional, tag = "1")]
    pub id: ::core::option::Option<ObjectId>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SuiMovePackage {
    #[prost(map = "string, message", tag = "1")]
    pub disassembled: ::std::collections::HashMap<
        ::prost::alloc::string::String,
        SuiJsonValue,
    >,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SuiRawData {
    #[prost(oneof = "sui_raw_data::SuiRawData", tags = "1, 2")]
    pub sui_raw_data: ::core::option::Option<sui_raw_data::SuiRawData>,
}
/// Nested message and enum types in `SuiRawData`.
pub mod sui_raw_data {
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum SuiRawData {
        #[prost(message, tag = "1")]
        MoveObject(super::SuiRawMoveObject),
        #[prost(message, tag = "2")]
        Package(super::SuiRawMovePackage),
    }
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SuiRawMoveObject {
    #[prost(message, optional, tag = "1")]
    pub r#type: ::core::option::Option<StructTag>,
    #[prost(bool, tag = "2")]
    pub has_public_transfer: bool,
    #[prost(uint64, tag = "3")]
    pub version: u64,
    #[prost(bytes = "vec", tag = "4")]
    pub bcs_bytes: ::prost::alloc::vec::Vec<u8>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SuiRawMovePackage {
    #[prost(message, optional, tag = "1")]
    pub id: ::core::option::Option<ObjectId>,
    #[prost(uint64, tag = "2")]
    pub version: u64,
    #[prost(map = "string, bytes", tag = "3")]
    pub module_map: ::std::collections::HashMap<
        ::prost::alloc::string::String,
        ::prost::alloc::vec::Vec<u8>,
    >,
    #[prost(message, repeated, tag = "4")]
    pub type_origin_table: ::prost::alloc::vec::Vec<TypeOrigin>,
    /// Note the key here is ObjectID, but we cannot use Message as keys in a map thus we covnert it into hex string
    /// that is key = hex(ObjectId)
    #[prost(map = "string, message", tag = "5")]
    pub linkage_table: ::std::collections::HashMap<
        ::prost::alloc::string::String,
        UpgradeInfo,
    >,
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
