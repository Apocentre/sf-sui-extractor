use sui_types::execution_status::{CommandArgumentError, ExecutionFailureStatus, ExecutionStatus, MoveLocation, MoveLocationOpt, PackageUpgradeError, TypeArgumentError};
use crate::pb::sui::checkpoint::{self as pb, execution_failure_status};
use super::common::{convert_module_id, convert_sui_object};

pub fn convert_sui_execution_status(source: &ExecutionStatus) -> pb::ExecutionStatus {
  let execution_status = match source {
    ExecutionStatus::Success => pb::execution_status::ExecutionStatus::Success(()),
    ExecutionStatus::Failure {error, command} => pb::execution_status::ExecutionStatus::Failure(pb::Failure {
      error: Some(convert_executaion_failure_status(error)),
      command_index: command.map(|i| i as u32),
    })
  };
  
  pb::ExecutionStatus {
    execution_status: Some(execution_status),
  }
}

fn convert_executaion_failure_status(source: &ExecutionFailureStatus) -> pb::ExecutionFailureStatus {
  let execution_failure_status = match source {
    ExecutionFailureStatus::InsufficientGas => pb::execution_failure_status::ExecutionFailureStatus::InsufficientGas(()),
    ExecutionFailureStatus::InvalidGasObject => pb::execution_failure_status::ExecutionFailureStatus::InvalidGasObject(()),
    ExecutionFailureStatus::InvariantViolation => pb::execution_failure_status::ExecutionFailureStatus::InvariantViolation(()),
    ExecutionFailureStatus::FeatureNotYetSupported => pb::execution_failure_status::ExecutionFailureStatus::FeatureNotYetSupported(()),
    ExecutionFailureStatus::MoveObjectTooBig {object_size, max_object_size} => {
      pb::execution_failure_status::ExecutionFailureStatus::MoveObjectTooBig(pb::execution_failure_status::MoveObjectTooBig {
        object_size: *object_size,
        max_object_size: *max_object_size,
      })
    },
    ExecutionFailureStatus::MovePackageTooBig {object_size, max_object_size} => {
      pb::execution_failure_status::ExecutionFailureStatus::MovePackageTooBig(pb::execution_failure_status::MovePackageTooBig {
        object_size: *object_size,
        max_object_size: *max_object_size,
      })
    },
    ExecutionFailureStatus::CircularObjectOwnership {object} => {
      pb::execution_failure_status::ExecutionFailureStatus::CircularObjectOwnership(pb::execution_failure_status::CircularObjectOwnership {
        object: Some(convert_sui_object(&object)),
      })
    },
    ExecutionFailureStatus::InsufficientCoinBalance => {
      pb::execution_failure_status::ExecutionFailureStatus::InsufficientCoinBalance(())
    },
    ExecutionFailureStatus::CoinBalanceOverflow => {
      pb::execution_failure_status::ExecutionFailureStatus::CoinBalanceOverflow(())
    },
    ExecutionFailureStatus::PublishErrorNonZeroAddress => {
      pb::execution_failure_status::ExecutionFailureStatus::PublishErrorNonZeroAddress(())
    },
    ExecutionFailureStatus::SuiMoveVerificationError => {
      pb::execution_failure_status::ExecutionFailureStatus::SuiMoveVerificationError(())
    },
    ExecutionFailureStatus::MovePrimitiveRuntimeError(source) => {
      pb::execution_failure_status::ExecutionFailureStatus::MovePrimitiveRuntimeError(pb::execution_failure_status::MoveLocationOpt {
        move_location: convert_move_location_opt(source),
      })
    },
    ExecutionFailureStatus::MoveAbort(loc, abort_code) => {
      pb::execution_failure_status::ExecutionFailureStatus::MoveAbort(pb::execution_failure_status::MoveAbort {
        move_location: Some(convert_move_location(loc)),
        abort_code: *abort_code,
    })
    },
    ExecutionFailureStatus::VMVerificationOrDeserializationError => {
      pb::execution_failure_status::ExecutionFailureStatus::VmVerificationOrDeserializationError(())
    },
    ExecutionFailureStatus::VMInvariantViolation => {
      pb::execution_failure_status::ExecutionFailureStatus::VmInvariantViolation(())
    },
    ExecutionFailureStatus::FunctionNotFound => {
      pb::execution_failure_status::ExecutionFailureStatus::FunctionNotFound(())
    },
    ExecutionFailureStatus::ArityMismatch => {
      pb::execution_failure_status::ExecutionFailureStatus::ArityMismatch(())
    },
    ExecutionFailureStatus::TypeArityMismatch => {
      pb::execution_failure_status::ExecutionFailureStatus::TypeArityMismatch(())
    },
    ExecutionFailureStatus::NonEntryFunctionInvoked => {
      pb::execution_failure_status::ExecutionFailureStatus::NonEntryFunctionInvoked(())
    },
    ExecutionFailureStatus::CommandArgumentError {arg_idx, kind} => {
      pb::execution_failure_status::ExecutionFailureStatus::CommandArgError(pb::execution_failure_status::CommandArgumentError{
        arg_idx: *arg_idx as u32,
        kind: Some(convert_command_arg_error(kind)),
      })
    },
    ExecutionFailureStatus::TypeArgumentError {argument_idx, kind} => {
      pb::execution_failure_status::ExecutionFailureStatus::TypeArgumentError(pb::execution_failure_status::TypeArgumentError {
        argument_idx: *argument_idx as u32,
        kind: Some(convert_type_arg_error(kind)),
    })
    },
    ExecutionFailureStatus::UnusedValueWithoutDrop {result_idx, secondary_idx} => {
      pb::execution_failure_status::ExecutionFailureStatus::UnusedValueWithoutDrop(pb::execution_failure_status::UnusedValueWithoutDrop {
        result_idx: *result_idx as u32,
        secondary_idx: *secondary_idx as u32,
      })
    },
    ExecutionFailureStatus::InvalidPublicFunctionReturnType {idx} => {
      pb::execution_failure_status::ExecutionFailureStatus::InvalidPublicFunctionReturnType(pb::execution_failure_status::InvalidPublicFunctionReturnType {
        idx: *idx as u32,
      })
    },
    ExecutionFailureStatus::InvalidTransferObject => {
      pb::execution_failure_status::ExecutionFailureStatus::InvalidTransferObject(())
    },
    ExecutionFailureStatus::EffectsTooLarge {current_size, max_size} => {
      pb::execution_failure_status::ExecutionFailureStatus::EffectsTooLarge(pb::execution_failure_status::EffectsTooLarge {
        current_size: *current_size,
        max_size: *max_size,
      })
    },
    ExecutionFailureStatus::PublishUpgradeMissingDependency => {
      pb::execution_failure_status::ExecutionFailureStatus::PublishUpgradeMissingDependency(())
    },
    ExecutionFailureStatus::PublishUpgradeDependencyDowngrade => {
      pb::execution_failure_status::ExecutionFailureStatus::PublishUpgradeDependencyDowngrade(())
    },
    ExecutionFailureStatus::PackageUpgradeError {upgrade_error} => {
      pb::execution_failure_status::ExecutionFailureStatus::PackageUpgradeError(pb::execution_failure_status::PackageUpgradeError {
        upgrade_error: Some(convert_package_upgrade_error(&upgrade_error)),
      })
    },
    ExecutionFailureStatus::WrittenObjectsTooLarge {current_size, max_size} => {
      pb::execution_failure_status::ExecutionFailureStatus::WrittenObjectsTooLarge(pb::execution_failure_status::WrittenObjectsTooLarge {
        current_size: *current_size,
        max_size: *max_size,
      })
    },
    ExecutionFailureStatus::CertificateDenied => {
      pb::execution_failure_status::ExecutionFailureStatus::CertificateDenied(())
    },
    ExecutionFailureStatus::SuiMoveVerificationTimedout => {
      pb::execution_failure_status::ExecutionFailureStatus::SuiMoveVerificationTimedout(())
    },
    ExecutionFailureStatus::SharedObjectOperationNotAllowed => {
      pb::execution_failure_status::ExecutionFailureStatus::SharedObjectOperationNotAllowed(())
    },
    ExecutionFailureStatus::InputObjectDeleted => {
      pb::execution_failure_status::ExecutionFailureStatus::InputObjectDeleted(())
    }
  };
  pb::ExecutionFailureStatus {
    execution_failure_status: Some(execution_failure_status),
  }
}

fn convert_package_upgrade_error(upgrade_error: &PackageUpgradeError) -> pb::PackageUpgradeError {
  let package_upgrade_error = match upgrade_error {
    PackageUpgradeError::UnableToFetchPackage {package_id} => {
      pb::package_upgrade_error::PackageUpgradeError::UnableToFetchPackage(pb::package_upgrade_error::UnableToFetchPackage {
        package_id: Some(convert_sui_object(package_id)),
      })
    },
    PackageUpgradeError::NotAPackage {object_id} => {
      pb::package_upgrade_error::PackageUpgradeError::NotAPackage(pb::package_upgrade_error::NotAPackage {
        object_id: Some(convert_sui_object(object_id)),
      })
    },
    PackageUpgradeError::IncompatibleUpgrade => {
      pb::package_upgrade_error::PackageUpgradeError::IncompatibleUpgrade(())
    },
    PackageUpgradeError::DigestDoesNotMatch {digest} => {
      pb::package_upgrade_error::PackageUpgradeError::DigestDoesNotMatch(pb::package_upgrade_error::DigestDoesNotMatch {
        digest: digest.clone(),
      })
    },
    PackageUpgradeError::UnknownUpgradePolicy {policy} => {
      pb::package_upgrade_error::PackageUpgradeError::UnknownUpgradePolicy(pb::package_upgrade_error::UnknownUpgradePolicy {
        policy: *policy as u32,
      })
    },
    PackageUpgradeError::PackageIDDoesNotMatch {package_id, ticket_id} => {
      pb::package_upgrade_error::PackageUpgradeError::PackageIdDoesNotMatch(pb::package_upgrade_error::PackageIdDoesNotMatch {
        package_id: Some(convert_sui_object(package_id)),
        ticket_id: Some(convert_sui_object(ticket_id)),
      })
    },
  };

  pb::PackageUpgradeError {
    package_upgrade_error: Some(package_upgrade_error),
}
}

fn convert_type_arg_error(kind: &TypeArgumentError) -> pb::TypeArgumentError {
  let type_argument_error = match kind {
    TypeArgumentError::TypeNotFound => pb::type_argument_error::TypeArgumentError::TypeNotFound(()),
    TypeArgumentError::ConstraintNotSatisfied => pb::type_argument_error::TypeArgumentError::ConstraintNotSatisfied(()),
  };

  pb::TypeArgumentError {
    type_argument_error: Some(type_argument_error),
}
}

fn convert_command_arg_error(kind: &CommandArgumentError) -> pb::CommandArgumentError {
  let command_argument_error = match kind {
    CommandArgumentError::TypeMismatch => {
      pb::command_argument_error::CommandArgumentError::TypeMismatch(())
    },
    CommandArgumentError::InvalidBCSBytes => {
      pb::command_argument_error::CommandArgumentError::InvalidBcsBytes(())
    },
    CommandArgumentError::InvalidUsageOfPureArg => {
      pb::command_argument_error::CommandArgumentError::InvalidUsageOfPureArg(())
    },
    CommandArgumentError::InvalidArgumentToPrivateEntryFunction => {
      pb::command_argument_error::CommandArgumentError::InvalidArgumentToPrivateEntryFunction(())
    },
    CommandArgumentError::IndexOutOfBounds {idx} => {
      pb::command_argument_error::CommandArgumentError::IndexOutOfBounds(pb::command_argument_error::IndexOutOfBounds {
        idx: *idx as u32,
      })
    },
    CommandArgumentError::SecondaryIndexOutOfBounds {result_idx, secondary_idx} => {
      pb::command_argument_error::CommandArgumentError::SecondaryIndexOutOfBounds(pb::command_argument_error::SecondaryIndexOutOfBounds {
        result_idx: *result_idx as u32,
        secondary_idx: *secondary_idx as u32,
    })
    },
    CommandArgumentError::InvalidResultArity {result_idx} => {
      pb::command_argument_error::CommandArgumentError::InvalidResultArity(pb::command_argument_error::InvalidResultArity {
        result_idx: *result_idx as u32,
      })
    },
    CommandArgumentError::InvalidGasCoinUsage => {
      pb::command_argument_error::CommandArgumentError::InvalidGasCoinUsage(())
    },
    CommandArgumentError::InvalidValueUsage => {
      pb::command_argument_error::CommandArgumentError::InvalidValueUsage(())
    },
    CommandArgumentError::InvalidObjectByValue => {
      pb::command_argument_error::CommandArgumentError::InvalidObjectByValue(())
    },
    CommandArgumentError::InvalidObjectByMutRef => {
      pb::command_argument_error::CommandArgumentError::InvalidObjectByMutRef(())
    },
    CommandArgumentError::SharedObjectOperationNotAllowed => {
      pb::command_argument_error::CommandArgumentError::SharedObjectOperationNotAllowed(())
    },
  };

  pb::CommandArgumentError {
    command_argument_error: Some(command_argument_error),
}
}

fn convert_move_location(source: &MoveLocation) -> execution_failure_status::MoveLocation {
  execution_failure_status::MoveLocation {
    module: Some(convert_module_id(&source.module)),
    function: source.function as u32,
    instruction: source.instruction as u32,
    function_name: source.function_name.clone(),
  }
}

fn convert_move_location_opt(source: &MoveLocationOpt) -> Option<execution_failure_status::MoveLocation> {
  source.0.as_ref().map(convert_move_location)
}

