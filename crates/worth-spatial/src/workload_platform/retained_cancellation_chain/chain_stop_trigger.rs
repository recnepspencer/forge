use super::chain_checkpoint::RetainedCancellationCheckpointTrigger;
use super::chain_policy::RetainedCancellationChainError;

pub(super) fn stop_trigger_error(
    step_index: usize,
    trigger: RetainedCancellationCheckpointTrigger,
) -> RetainedCancellationChainError {
    match trigger {
        RetainedCancellationCheckpointTrigger::NearGrazePolicyRequired => {
            RetainedCancellationChainError::PolicyRequired {
                step_index,
                trigger,
            }
        }
        RetainedCancellationCheckpointTrigger::PredicateUncertain => {
            RetainedCancellationChainError::PredicateUncertain { step_index }
        }
        RetainedCancellationCheckpointTrigger::RetainedReplayMismatch => {
            RetainedCancellationChainError::RetainedReplayMismatch { step_index }
        }
        RetainedCancellationCheckpointTrigger::TransformInvalidation => {
            RetainedCancellationChainError::TransformInvalidation { step_index }
        }
        RetainedCancellationCheckpointTrigger::ProjectionConsumedFactMismatch => {
            RetainedCancellationChainError::ProjectionConsumedFactMismatch { step_index }
        }
    }
}
