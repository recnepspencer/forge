use crate::workload_platform::evidence_ledger::WorkloadEvidenceStage;

use super::chain_checkpoint::RetainedCancellationCheckpointTrigger;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum RetainedCancellationChainPredicate {
    Certified,
    UncertainAtStep(usize),
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum RetainedCancellationChainReplayPolicy {
    RetainedOnly,
    LiveExtractionRequested,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum RetainedCancellationChainTransformPosture {
    Valid,
    InvalidatedAtStep(usize),
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum RetainedCancellationChainIntegrity {
    Consistent,
    RetainedReplayMismatch { step_index: usize },
    ProjectionConsumedFactMismatch { step_index: usize },
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum RetainedCancellationChainError {
    MissingReceiptBackedStage(WorkloadEvidenceStage),
    MissingCheckpointHistory,
    InsufficientCheckpointHistory {
        required: usize,
        actual: usize,
    },
    InsufficientReplaySampling {
        expected: usize,
        actual: usize,
    },
    MissingTriggerLocalReplay {
        step_index: usize,
    },
    DuplicateCheckpointEvidence {
        step_index: usize,
    },
    CheckpointNotFromPlatformStage {
        step_index: usize,
        stage: WorkloadEvidenceStage,
    },
    PredicateUncertain {
        step_index: usize,
    },
    PolicyRequired {
        step_index: usize,
        trigger: RetainedCancellationCheckpointTrigger,
    },
    RetainedReplayMismatch {
        step_index: usize,
    },
    TransformInvalidation {
        step_index: usize,
    },
    ProjectionConsumedFactMismatch {
        step_index: usize,
    },
    LiveExtractionForbidden,
}

impl RetainedCancellationChainError {
    pub fn human_reason(&self) -> String {
        match self {
            Self::MissingReceiptBackedStage(stage) => {
                format!(
                    "retained cancellation chain requires receipt-backed {}",
                    stage.human_name()
                )
            }
            Self::MissingCheckpointHistory => {
                "retained cancellation chain requires retained checkpoint history".to_string()
            }
            Self::InsufficientCheckpointHistory { required, actual } => {
                format!(
                    "retained cancellation chain requires at least {required} retained checkpoints; only {actual} were provided"
                )
            }
            Self::InsufficientReplaySampling { expected, actual } => {
                format!(
                    "retained cancellation chain requires replay of every fourth checkpoint and trigger-local checkpoint; expected at least {expected} replayed checkpoints but found {actual}"
                )
            }
            Self::MissingTriggerLocalReplay { step_index } => {
                format!(
                    "retained cancellation chain requires retained replay at checkpoint {step_index} because that checkpoint triggered a stop condition"
                )
            }
            Self::DuplicateCheckpointEvidence { step_index } => {
                format!(
                    "retained cancellation chain requires distinct retained evidence for checkpoint {step_index}"
                )
            }
            Self::CheckpointNotFromPlatformStage { step_index, stage } => {
                format!(
                    "retained cancellation checkpoint {step_index} must come from the workload catalog {} receipt",
                    stage.human_name()
                )
            }
            Self::PredicateUncertain { step_index } => {
                format!(
                    "predicate authority became uncertain at retained checkpoint {step_index}; no automatic boolean option is available"
                )
            }
            Self::PolicyRequired {
                step_index,
                trigger,
            } => {
                format!(
                    "retained cancellation chain needs a user policy decision at checkpoint {step_index} because of {}",
                    trigger.human_name()
                )
            }
            Self::RetainedReplayMismatch { step_index } => {
                format!(
                    "retained replay mismatch at checkpoint {step_index}; replay must use retained history rather than live extraction"
                )
            }
            Self::TransformInvalidation { step_index } => {
                format!(
                    "movement or rotation invalidated the retained cancellation chain at checkpoint {step_index}"
                )
            }
            Self::ProjectionConsumedFactMismatch { step_index } => {
                format!(
                    "projection-consumed facts do not match the retained basis at checkpoint {step_index}"
                )
            }
            Self::LiveExtractionForbidden => {
                "retained cancellation chain forbids live extraction during replay; replay must consume retained artifacts"
                    .to_string()
            }
        }
    }
}
