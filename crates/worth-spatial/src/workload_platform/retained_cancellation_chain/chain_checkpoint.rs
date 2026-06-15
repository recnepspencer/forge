use worth_primitives::{truth_digest_parts, TruthDigestScope};

use crate::workload_platform::retained_replay_workload::ReplayReceiptSet;
use crate::workload_platform::transform_workload::TransformReceiptSet;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum RetainedCancellationCheckpointTrigger {
    NearGrazePolicyRequired,
    PredicateUncertain,
    RetainedReplayMismatch,
    TransformInvalidation,
    ProjectionConsumedFactMismatch,
}

impl RetainedCancellationCheckpointTrigger {
    pub fn human_name(self) -> &'static str {
        match self {
            Self::NearGrazePolicyRequired => "near-graze policy requirement",
            Self::PredicateUncertain => "predicate uncertainty",
            Self::RetainedReplayMismatch => "retained replay mismatch",
            Self::TransformInvalidation => "transform invalidation",
            Self::ProjectionConsumedFactMismatch => "projection-consumed retained fact mismatch",
        }
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct RetainedCancellationCheckpoint {
    step_index: usize,
    checkpoint_identity: String,
    transform_stage_receipt_identity: String,
    transform_stage_identity: String,
    retained_replay_stage_identity: String,
    retained_artifact_capture_identity: String,
    retained_basis_identity: String,
    replay_checkpoint_identity: String,
    projection_consumed_source_identity: String,
    projection_consumed_identity: String,
    replayed_from_retained_history: bool,
    trigger: Option<RetainedCancellationCheckpointTrigger>,
}

impl RetainedCancellationCheckpoint {
    pub fn from_receipts(
        step_index: usize,
        transform_receipts: &TransformReceiptSet,
        replay_receipts: &ReplayReceiptSet,
    ) -> Self {
        let retained_basis_identity = replay_receipts.retained_basis_identity().to_string();
        let transform_stage_receipt_identity = transform_receipts
            .stage_identity()
            .receipt_identity()
            .to_string();
        let retained_replay_stage_identity = replay_receipts
            .stage_identity()
            .receipt_identity()
            .to_string();
        let transform_stage_identity = checkpoint_identity(
            "transform-step",
            step_index,
            &transform_stage_receipt_identity,
        );
        let retained_artifact_capture_identity = checkpoint_identity(
            "retained-artifact-capture",
            step_index,
            replay_receipts.retained_artifact_capture_identity(),
        );
        let replay_checkpoint_identity = checkpoint_identity(
            "retained-replay-checkpoint",
            step_index,
            replay_receipts.replay_checkpoint_identity(),
        );
        let projection_consumed_identity = checkpoint_projection_consumed_identity(
            step_index,
            replay_receipts.replay_evidence_identity(),
            &retained_basis_identity,
        );
        Self::new(CheckpointParts {
            step_index,
            transform_stage_receipt_identity,
            transform_stage_identity,
            retained_replay_stage_identity,
            retained_artifact_capture_identity,
            retained_basis_identity,
            replay_checkpoint_identity,
            projection_consumed_source_identity: replay_receipts
                .replay_evidence_identity()
                .to_string(),
            projection_consumed_identity,
            replayed_from_retained_history: false,
            trigger: None,
        })
    }

    pub fn sampled_for_replay(mut self) -> Self {
        self.replayed_from_retained_history = true;
        self.checkpoint_identity = self.derive_checkpoint_identity();
        self
    }

    pub fn with_trigger(mut self, trigger: RetainedCancellationCheckpointTrigger) -> Self {
        self.trigger = Some(trigger);
        self.checkpoint_identity = self.derive_checkpoint_identity();
        self
    }

    pub fn with_projection_consumed_identity(mut self, identity: impl Into<String>) -> Self {
        self.projection_consumed_identity = identity.into();
        self.checkpoint_identity = self.derive_checkpoint_identity();
        self
    }

    pub fn step_index(&self) -> usize {
        self.step_index
    }

    pub fn checkpoint_identity(&self) -> &str {
        &self.checkpoint_identity
    }

    pub fn transform_stage_identity(&self) -> &str {
        &self.transform_stage_identity
    }

    pub fn transform_stage_receipt_identity(&self) -> &str {
        &self.transform_stage_receipt_identity
    }

    pub fn retained_replay_stage_identity(&self) -> &str {
        &self.retained_replay_stage_identity
    }

    pub fn retained_artifact_capture_identity(&self) -> &str {
        &self.retained_artifact_capture_identity
    }

    pub fn retained_basis_identity(&self) -> &str {
        &self.retained_basis_identity
    }

    pub fn replay_checkpoint_identity(&self) -> &str {
        &self.replay_checkpoint_identity
    }

    pub fn projection_consumed_identity(&self) -> &str {
        &self.projection_consumed_identity
    }

    pub fn replayed_from_retained_history(&self) -> bool {
        self.replayed_from_retained_history
    }

    pub fn trigger(&self) -> Option<RetainedCancellationCheckpointTrigger> {
        self.trigger
    }

    pub(crate) fn projection_matches_retained_checkpoint(&self) -> bool {
        self.projection_consumed_identity
            == checkpoint_projection_consumed_identity(
                self.step_index,
                &self.projection_consumed_source_identity,
                &self.retained_basis_identity,
            )
    }

    fn new(parts: CheckpointParts) -> Self {
        let mut checkpoint = Self {
            step_index: parts.step_index,
            checkpoint_identity: String::new(),
            transform_stage_receipt_identity: parts.transform_stage_receipt_identity,
            transform_stage_identity: parts.transform_stage_identity,
            retained_replay_stage_identity: parts.retained_replay_stage_identity,
            retained_artifact_capture_identity: parts.retained_artifact_capture_identity,
            retained_basis_identity: parts.retained_basis_identity,
            replay_checkpoint_identity: parts.replay_checkpoint_identity,
            projection_consumed_source_identity: parts.projection_consumed_source_identity,
            projection_consumed_identity: parts.projection_consumed_identity,
            replayed_from_retained_history: parts.replayed_from_retained_history,
            trigger: parts.trigger,
        };
        checkpoint.checkpoint_identity = checkpoint.derive_checkpoint_identity();
        checkpoint
    }

    fn derive_checkpoint_identity(&self) -> String {
        truth_digest_parts(
            TruthDigestScope::ArtifactIdentity,
            &[
                "retained-cancellation-checkpoint".to_string(),
                format!("step:{}", self.step_index),
                format!("transform_stage:{}", self.transform_stage_receipt_identity),
                format!("transform:{}", self.transform_stage_identity),
                format!(
                    "retained_replay_stage:{}",
                    self.retained_replay_stage_identity
                ),
                format!("capture:{}", self.retained_artifact_capture_identity),
                format!("retained_basis:{}", self.retained_basis_identity),
                format!("replay_checkpoint:{}", self.replay_checkpoint_identity),
                format!(
                    "projection_consumed_source:{}",
                    self.projection_consumed_source_identity
                ),
                format!("projection_consumed:{}", self.projection_consumed_identity),
                format!("replayed:{}", self.replayed_from_retained_history),
                format!("trigger:{:?}", self.trigger),
            ],
        )
    }
}

fn checkpoint_identity(kind: &str, step_index: usize, base_identity: &str) -> String {
    format!("{kind}:checkpoint={step_index}:{base_identity}")
}

fn checkpoint_projection_consumed_identity(
    step_index: usize,
    replay_evidence_identity: &str,
    retained_basis_identity: &str,
) -> String {
    format!(
        "projection-consumed-checkpoint:checkpoint={step_index}:{replay_evidence_identity}:retained-basis={retained_basis_identity}"
    )
}

struct CheckpointParts {
    step_index: usize,
    transform_stage_receipt_identity: String,
    transform_stage_identity: String,
    retained_replay_stage_identity: String,
    retained_artifact_capture_identity: String,
    retained_basis_identity: String,
    replay_checkpoint_identity: String,
    projection_consumed_source_identity: String,
    projection_consumed_identity: String,
    replayed_from_retained_history: bool,
    trigger: Option<RetainedCancellationCheckpointTrigger>,
}
