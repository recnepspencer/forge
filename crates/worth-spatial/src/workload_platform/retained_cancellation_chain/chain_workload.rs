use super::{
    chain_checkpoint::{RetainedCancellationCheckpoint, RetainedCancellationCheckpointTrigger},
    chain_counters::{RetainedCancellationChainCounterInput, RetainedCancellationChainCounters},
    chain_digest::retained_cancellation_chain_digest,
    chain_evidence_guard,
    chain_policy::{
        RetainedCancellationChainError, RetainedCancellationChainIntegrity,
        RetainedCancellationChainPredicate, RetainedCancellationChainReplayPolicy,
        RetainedCancellationChainTransformPosture,
    },
    chain_receipt::RetainedCancellationChainReceipt,
};
use crate::workload_platform::evidence_ledger::{
    CompleteWorkloadEvidenceLedger, WorkloadEvidenceStage,
};

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct RetainedReplaySampling {
    checkpoint_stride: usize,
}

impl RetainedReplaySampling {
    pub fn every_fourth_checkpoint_plus_trigger_steps() -> Self {
        Self {
            checkpoint_stride: 4,
        }
    }

    pub fn checkpoint_stride(self) -> usize {
        self.checkpoint_stride
    }
}

pub struct RetainedCancellationChainWorkload<'a> {
    evidence_ledger: &'a CompleteWorkloadEvidenceLedger,
    declaration: String,
    required_checkpoints: usize,
    replay_sampling: RetainedReplaySampling,
    checkpoints: Vec<RetainedCancellationCheckpoint>,
    predicate: RetainedCancellationChainPredicate,
    replay_policy: RetainedCancellationChainReplayPolicy,
    transform_posture: RetainedCancellationChainTransformPosture,
    integrity: RetainedCancellationChainIntegrity,
}

impl<'a> RetainedCancellationChainWorkload<'a> {
    pub fn from_platform_evidence(evidence_ledger: &'a CompleteWorkloadEvidenceLedger) -> Self {
        Self {
            evidence_ledger,
            declaration: "retained cancellation chain workload".to_string(),
            required_checkpoints: 32,
            replay_sampling: RetainedReplaySampling::every_fourth_checkpoint_plus_trigger_steps(),
            checkpoints: Vec::new(),
            predicate: RetainedCancellationChainPredicate::Certified,
            replay_policy: RetainedCancellationChainReplayPolicy::RetainedOnly,
            transform_posture: RetainedCancellationChainTransformPosture::Valid,
            integrity: RetainedCancellationChainIntegrity::Consistent,
        }
    }

    pub fn declared(mut self, declaration: impl Into<String>) -> Self {
        self.declaration = declaration.into();
        self
    }

    pub fn with_required_checkpoints(mut self, required_checkpoints: usize) -> Self {
        self.required_checkpoints = required_checkpoints;
        self
    }

    pub fn with_replay_sampling(mut self, replay_sampling: RetainedReplaySampling) -> Self {
        self.replay_sampling = replay_sampling;
        self
    }

    pub fn with_checkpoints(mut self, checkpoints: Vec<RetainedCancellationCheckpoint>) -> Self {
        self.checkpoints = checkpoints;
        self
    }

    pub fn requiring_predicate(mut self, predicate: RetainedCancellationChainPredicate) -> Self {
        self.predicate = predicate;
        self
    }

    pub fn requiring_replay_policy(
        mut self,
        replay_policy: RetainedCancellationChainReplayPolicy,
    ) -> Self {
        self.replay_policy = replay_policy;
        self
    }

    pub fn requiring_transform_posture(
        mut self,
        transform_posture: RetainedCancellationChainTransformPosture,
    ) -> Self {
        self.transform_posture = transform_posture;
        self
    }

    pub fn requiring_integrity(mut self, integrity: RetainedCancellationChainIntegrity) -> Self {
        self.integrity = integrity;
        self
    }

    pub fn certify(
        self,
    ) -> Result<RetainedCancellationChainReceipt, RetainedCancellationChainError> {
        self.require_retained_only_replay()?;
        self.require_platform_evidence()?;
        self.require_checkpoint_breadth()?;
        chain_evidence_guard::require_distinct_checkpoint_evidence(&self.checkpoints)?;
        self.require_replay_sampling()?;
        let retained_basis_identity = self.retained_basis_identity()?;
        chain_evidence_guard::require_projection_consumed_checkpoint_match(
            &self.checkpoints,
            &retained_basis_identity,
        )?;
        self.require_predicate_certification()?;
        self.require_transform_posture()?;
        self.require_integrity()?;

        let counters = self.counters()?;
        let workload_identity = self.workload_identity()?;
        let projection_consumed_identity = self.projection_consumed_identity()?;
        let chain_digest = retained_cancellation_chain_digest(
            &workload_identity,
            &retained_basis_identity,
            &projection_consumed_identity,
            &self.checkpoints,
            counters,
        );

        Ok(RetainedCancellationChainReceipt::new(
            chain_digest,
            workload_identity,
            retained_basis_identity,
            projection_consumed_identity,
            self.checkpoints,
            counters,
        ))
    }

    fn require_retained_only_replay(&self) -> Result<(), RetainedCancellationChainError> {
        if self.replay_policy == RetainedCancellationChainReplayPolicy::LiveExtractionRequested {
            Err(RetainedCancellationChainError::LiveExtractionForbidden)
        } else {
            Ok(())
        }
    }

    fn require_platform_evidence(&self) -> Result<(), RetainedCancellationChainError> {
        self.require_stage(WorkloadEvidenceStage::Topology)?;
        self.require_stage(WorkloadEvidenceStage::GeometryBinding)?;
        self.require_stage(WorkloadEvidenceStage::SurfaceSupport)?;
        self.require_stage(WorkloadEvidenceStage::Projection)?;
        self.require_stage(WorkloadEvidenceStage::Transform)?;
        self.require_stage(WorkloadEvidenceStage::RetainedReplay)?;
        self.require_stage(WorkloadEvidenceStage::Diagnostics)?;
        self.require_stage(WorkloadEvidenceStage::Response)?;
        Ok(())
    }

    fn require_stage(
        &self,
        stage: WorkloadEvidenceStage,
    ) -> Result<(), RetainedCancellationChainError> {
        let row = self.evidence_ledger.row_for_stage(stage).ok_or(
            RetainedCancellationChainError::MissingReceiptBackedStage(stage),
        )?;
        if row.is_receipt_backed() && row.is_admitted() {
            Ok(())
        } else {
            Err(RetainedCancellationChainError::MissingReceiptBackedStage(
                stage,
            ))
        }
    }

    fn require_checkpoint_breadth(&self) -> Result<(), RetainedCancellationChainError> {
        if self.checkpoints.is_empty() {
            return Err(RetainedCancellationChainError::MissingCheckpointHistory);
        }
        if self.checkpoints.len() < self.required_checkpoints {
            return Err(
                RetainedCancellationChainError::InsufficientCheckpointHistory {
                    required: self.required_checkpoints,
                    actual: self.checkpoints.len(),
                },
            );
        }
        Ok(())
    }

    fn require_replay_sampling(&self) -> Result<(), RetainedCancellationChainError> {
        if let Some(checkpoint) = self.checkpoints.iter().find(|checkpoint| {
            checkpoint.trigger().is_some() && !checkpoint.replayed_from_retained_history()
        }) {
            return Err(RetainedCancellationChainError::MissingTriggerLocalReplay {
                step_index: checkpoint.step_index(),
            });
        }
        let expected = self.expected_sampled_checkpoint_count();
        let actual = self.replayed_checkpoint_count();
        if actual < expected {
            return Err(RetainedCancellationChainError::InsufficientReplaySampling {
                expected,
                actual,
            });
        }
        Ok(())
    }

    fn require_predicate_certification(&self) -> Result<(), RetainedCancellationChainError> {
        match self.predicate {
            RetainedCancellationChainPredicate::Certified => Ok(()),
            RetainedCancellationChainPredicate::UncertainAtStep(step_index) => {
                Err(RetainedCancellationChainError::PredicateUncertain { step_index })
            }
        }
    }

    fn require_transform_posture(&self) -> Result<(), RetainedCancellationChainError> {
        match self.transform_posture {
            RetainedCancellationChainTransformPosture::Valid => Ok(()),
            RetainedCancellationChainTransformPosture::InvalidatedAtStep(step_index) => {
                Err(RetainedCancellationChainError::TransformInvalidation { step_index })
            }
        }
    }

    fn require_integrity(&self) -> Result<(), RetainedCancellationChainError> {
        match self.integrity {
            RetainedCancellationChainIntegrity::Consistent => self.require_no_stop_trigger(),
            RetainedCancellationChainIntegrity::RetainedReplayMismatch { step_index } => {
                Err(RetainedCancellationChainError::RetainedReplayMismatch { step_index })
            }
            RetainedCancellationChainIntegrity::ProjectionConsumedFactMismatch { step_index } => {
                Err(RetainedCancellationChainError::ProjectionConsumedFactMismatch { step_index })
            }
        }
    }

    fn require_no_stop_trigger(&self) -> Result<(), RetainedCancellationChainError> {
        if let Some(checkpoint) = self.checkpoints.iter().find_map(|checkpoint| {
            checkpoint
                .trigger()
                .map(|trigger| (checkpoint.step_index(), trigger))
        }) {
            return Err(stop_trigger_error(checkpoint.0, checkpoint.1));
        }
        Ok(())
    }

    fn expected_sampled_checkpoint_count(&self) -> usize {
        let stride = self.replay_sampling.checkpoint_stride().max(1);
        self.checkpoints
            .iter()
            .filter(|checkpoint| {
                checkpoint.step_index() % stride == 0 || checkpoint.trigger().is_some()
            })
            .count()
    }

    fn replayed_checkpoint_count(&self) -> usize {
        self.checkpoints
            .iter()
            .filter(|checkpoint| checkpoint.replayed_from_retained_history())
            .count()
    }

    fn counters(
        &self,
    ) -> Result<RetainedCancellationChainCounters, RetainedCancellationChainError> {
        let transform = self.stage_counters(WorkloadEvidenceStage::Transform)?;
        let retained_replay = self.stage_counters(WorkloadEvidenceStage::RetainedReplay)?;
        let projection = self.stage_counters(WorkloadEvidenceStage::Projection)?;
        let diagnostics = self.stage_counters(WorkloadEvidenceStage::Diagnostics)?;
        let response = self.stage_counters(WorkloadEvidenceStage::Response)?;
        Ok(RetainedCancellationChainCounters::new(
            RetainedCancellationChainCounterInput {
                checkpoint_count: self.checkpoints.len(),
                transform_step_count: self.checkpoints.len().max(transform.transform_step_count()),
                replayed_checkpoint_count: self.replayed_checkpoint_count(),
                trigger_local_replay_count: self
                    .checkpoints
                    .iter()
                    .filter(|checkpoint| {
                        checkpoint.trigger().is_some()
                            && checkpoint.replayed_from_retained_history()
                    })
                    .count(),
                retained_artifact_count: self.checkpoints.len()
                    * retained_replay.retained_artifact_count().max(1),
                projection_consumed_fact_count: self
                    .checkpoints
                    .len()
                    .max(projection.projected_entity_count()),
                diagnostic_trigger_count: diagnostics.diagnostic_count(),
                user_outcome_count: response.user_outcome_count(),
            },
        ))
    }

    fn stage_counters(
        &self,
        stage: WorkloadEvidenceStage,
    ) -> Result<
        crate::workload_platform::evidence_ledger::WorkloadEvidenceStageCounters,
        RetainedCancellationChainError,
    > {
        self.evidence_ledger
            .row_for_stage(stage)
            .map(|row| row.counters())
            .ok_or(RetainedCancellationChainError::MissingReceiptBackedStage(
                stage,
            ))
    }

    fn workload_identity(&self) -> Result<String, RetainedCancellationChainError> {
        Ok(format!(
            "retained-cancellation-chain:{}:{}",
            self.declaration,
            self.evidence_ledger
                .evidence_for_stage(WorkloadEvidenceStage::RetainedReplay)
                .ok_or(RetainedCancellationChainError::MissingReceiptBackedStage(
                    WorkloadEvidenceStage::RetainedReplay
                ))?
        ))
    }

    fn retained_basis_identity(&self) -> Result<String, RetainedCancellationChainError> {
        self.checkpoints
            .first()
            .map(|checkpoint| checkpoint.retained_basis_identity().to_string())
            .ok_or(RetainedCancellationChainError::MissingCheckpointHistory)
    }

    fn projection_consumed_identity(&self) -> Result<String, RetainedCancellationChainError> {
        self.checkpoints
            .first()
            .map(|checkpoint| checkpoint.projection_consumed_identity().to_string())
            .ok_or(RetainedCancellationChainError::MissingCheckpointHistory)
    }
}

fn stop_trigger_error(
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
