use sui_types::execution_status::{ExecutionFailureStatus, ExecutionStatus, MoveLocationOpt};
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
      pb::execution_failure_status::ExecutionFailureStatus::MovePrimitiveRuntimeError(pb::execution_failure_status::MoveLocationOpt{
        move_location: convert_move_location(source),
      })
    },
    ExecutionFailureStatus::MoveAbort(_, _) => todo!(),
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
    ExecutionFailureStatus::CommandArgumentError { arg_idx, kind } => todo!(),
    ExecutionFailureStatus::TypeArgumentError { argument_idx, kind } => todo!(),
    ExecutionFailureStatus::UnusedValueWithoutDrop { result_idx, secondary_idx } => todo!(),
    ExecutionFailureStatus::InvalidPublicFunctionReturnType { idx } => todo!(),
    ExecutionFailureStatus::InvalidTransferObject => {
      pb::execution_failure_status::ExecutionFailureStatus::InvalidTransferObject(())
    },
    ExecutionFailureStatus::EffectsTooLarge { current_size, max_size } => todo!(),
    ExecutionFailureStatus::PublishUpgradeMissingDependency => {
      pb::execution_failure_status::ExecutionFailureStatus::PublishUpgradeMissingDependency(())
    },
    ExecutionFailureStatus::PublishUpgradeDependencyDowngrade => {
      pb::execution_failure_status::ExecutionFailureStatus::PublishUpgradeDependencyDowngrade(())
    },
    ExecutionFailureStatus::PackageUpgradeError { upgrade_error } => todo!(),
    ExecutionFailureStatus::WrittenObjectsTooLarge { current_size, max_size } => todo!(),
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

fn convert_move_location(source: &MoveLocationOpt) -> Option<execution_failure_status::MoveLocation> {
  source.0.map(|source| {
    execution_failure_status::MoveLocation {
      module: Some(convert_module_id(&source.module)),
      function: source.function as u32,
      instruction: source.instruction as u32,
      function_name: source.function_name,
    }
  })
}

