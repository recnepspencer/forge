use crate::basis_lifecycle::BasisFamily;
use crate::{
    ForgeQueryEvidenceIdentity, ForgeQueryEvidenceScope, ForgeQueryEvidenceTag,
};

use super::batch_execution::ExecutedEffectBatchPlan;
use super::counters::EffectLifecycleCounters;
use super::diagnostics::{EffectDiagnosticsMaterialization, EffectDiagnosticsRequest};
use super::envelope::SelfDescribingEffectEnvelope;
use super::execution::{ExecutedEffectAuthorityArtifact, ExecutedEffectPlan};
use super::inventory::EffectReceiptArtifactKind;
use super::planning::EffectAuthorityOwner;
use super::receipt_transitions::EffectReceiptTransitionRules;
use super::taxonomy::{EffectAuthorityLane, EffectFamily};

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum EffectReceiptTargetEvidence {
    MutationCommit {
        commit_id: u64,
        version_id: u64,
    },
    MergeCommit {
        commit_id: u64,
        version_id: u64,
    },
    Writeback {
        outcome_digest: String,
        receipt_digest: String,
        execution_receipt_digest: String,
    },
    BatchMutation {
        commit_id: u64,
        version_id: u64,
        component_count: usize,
    },
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct EffectReceiptDecisionTrace {
    admitted_or_batch_digest: String,
    lowered_digest: String,
    authority_owner: EffectAuthorityOwner,
    decision_trace_digest: String,
}

impl EffectReceiptDecisionTrace {
    fn scalar(executed: &ExecutedEffectPlan) -> Self {
        let admitted_or_batch_digest = executed
            .lowered()
            .authority_scoped_plan()
            .admitted()
            .admitted_digest()
            .to_string();
        let lowered_digest = executed
            .lowered()
            .lowered_effect_execution_plan_digest()
            .to_string();
        let authority_owner = executed.authority_owner();
        let decision_trace_digest = ForgeQueryEvidenceIdentity::compose(
            ForgeQueryEvidenceScope::EffectIntentReceipt,
        )
        .field_shape(
            ForgeQueryEvidenceTag::new("identity_family"),
            "effect_receipt_decision_trace_v1",
        )
        .field_identity(ForgeQueryEvidenceTag::new("admitted"), &admitted_or_batch_digest)
        .field_identity(ForgeQueryEvidenceTag::new("lowered"), &lowered_digest)
        .field_shape(
            ForgeQueryEvidenceTag::new("authority_owner"),
            authority_owner.as_str(),
        )
        .seal()
        .as_str()
        .to_string();
        Self {
            admitted_or_batch_digest,
            lowered_digest,
            authority_owner,
            decision_trace_digest,
        }
    }

    fn batch(executed: &ExecutedEffectBatchPlan) -> Self {
        let admitted_or_batch_digest = executed.lowered().admitted_batch_digest().to_string();
        let lowered_digest = executed.lowered().batch_digest().to_string();
        let authority_owner = executed.authority_owner();
        let decision_trace_digest = ForgeQueryEvidenceIdentity::compose(
            ForgeQueryEvidenceScope::EffectIntentReceipt,
        )
        .field_shape(
            ForgeQueryEvidenceTag::new("identity_family"),
            "effect_receipt_decision_trace_v1",
        )
        .field_identity(
            ForgeQueryEvidenceTag::new("admitted_batch"),
            &admitted_or_batch_digest,
        )
        .field_identity(ForgeQueryEvidenceTag::new("lowered"), &lowered_digest)
        .field_shape(
            ForgeQueryEvidenceTag::new("authority_owner"),
            authority_owner.as_str(),
        )
        .seal()
        .as_str()
        .to_string();
        Self {
            admitted_or_batch_digest,
            lowered_digest,
            authority_owner,
            decision_trace_digest,
        }
    }

    pub fn admitted_or_batch_digest(&self) -> &str {
        &self.admitted_or_batch_digest
    }

    pub fn lowered_digest(&self) -> &str {
        &self.lowered_digest
    }

    pub fn authority_owner(&self) -> EffectAuthorityOwner {
        self.authority_owner
    }

    pub fn decision_trace_digest(&self) -> &str {
        &self.decision_trace_digest
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct EffectReceiptIntegrityMarkers {
    authority_artifact_digest: String,
    counter_snapshot_digest: String,
    integrity_digest: String,
}

impl EffectReceiptIntegrityMarkers {
    fn new(
        authority_artifact_digest: String,
        counters: &EffectLifecycleCounters,
        receipt_digest: &str,
    ) -> Self {
        let counter_snapshot_digest = counters.digest();
        let integrity_digest = ForgeQueryEvidenceIdentity::compose(
            ForgeQueryEvidenceScope::EffectIntentReceipt,
        )
        .field_shape(
            ForgeQueryEvidenceTag::new("identity_family"),
            "effect_receipt_integrity_markers_v1",
        )
        .field_identity(
            ForgeQueryEvidenceTag::new("authority_artifact"),
            &authority_artifact_digest,
        )
        .field_identity(ForgeQueryEvidenceTag::new("counters"), &counter_snapshot_digest)
        .field_identity(ForgeQueryEvidenceTag::new("receipt"), receipt_digest)
        .seal()
        .as_str()
        .to_string();
        Self {
            authority_artifact_digest,
            counter_snapshot_digest,
            integrity_digest,
        }
    }

    pub fn authority_artifact_digest(&self) -> &str {
        &self.authority_artifact_digest
    }

    pub fn counter_snapshot_digest(&self) -> &str {
        &self.counter_snapshot_digest
    }

    pub fn integrity_digest(&self) -> &str {
        &self.integrity_digest
    }
}

#[derive(Clone, Debug, PartialEq)]
enum EffectExecutionReceiptArtifact {
    Scalar(ExecutedEffectPlan),
    Batch(ExecutedEffectBatchPlan),
}

#[derive(Clone, Debug, PartialEq)]
pub struct EffectExecutionReceipt {
    artifact: EffectExecutionReceiptArtifact,
    receipt_family: EffectReceiptArtifactKind,
    declared_effect_family: EffectFamily,
    authority_lane: EffectAuthorityLane,
    basis_family: BasisFamily,
    receipt_digest: String,
    decision_trace: EffectReceiptDecisionTrace,
    integrity_markers: EffectReceiptIntegrityMarkers,
    counters: EffectLifecycleCounters,
}

impl EffectExecutionReceipt {
    pub(super) fn from_scalar(executed: ExecutedEffectPlan) -> Self {
        let receipt_family = match executed.artifact() {
            ExecutedEffectAuthorityArtifact::Writeback { .. } => {
                EffectReceiptArtifactKind::ForgeQueryWriteReceipt
            }
            ExecutedEffectAuthorityArtifact::Mutation(_)
            | ExecutedEffectAuthorityArtifact::Merge(_) => {
                EffectReceiptArtifactKind::ForgeQueryIntentExecution
            }
        };
        let declared_effect_family = executed.lowered().family();
        let authority_lane = executed.lowered().authority_lane();
        let basis_family = executed
            .lowered()
            .authority_scoped_plan()
            .admitted()
            .normalized()
            .basis_family();
        let receipt_digest = ForgeQueryEvidenceIdentity::compose(
            ForgeQueryEvidenceScope::EffectIntentReceipt,
        )
        .field_shape(
            ForgeQueryEvidenceTag::new("identity_family"),
            "effect_execution_receipt_v1",
        )
        .field_shape(ForgeQueryEvidenceTag::new("family"), receipt_family.as_str())
        .field_identity(
            ForgeQueryEvidenceTag::new("execution"),
            executed.effect_execution_digest(),
        )
        .seal()
        .as_str()
        .to_string();
        let decision_trace = EffectReceiptDecisionTrace::scalar(&executed);
        let integrity_markers = EffectReceiptIntegrityMarkers::new(
            authority_artifact_digest(executed.artifact()),
            executed.counters(),
            &receipt_digest,
        );
        let counters = executed.counters().clone();
        Self {
            artifact: EffectExecutionReceiptArtifact::Scalar(executed),
            receipt_family,
            declared_effect_family,
            authority_lane,
            basis_family,
            receipt_digest,
            decision_trace,
            integrity_markers,
            counters,
        }
    }

    pub(super) fn from_batch(executed: ExecutedEffectBatchPlan) -> Self {
        let receipt_family = EffectReceiptArtifactKind::ForgeQueryBatchWriteReceipt;
        let declared_effect_family = EffectFamily::Mutation;
        let authority_lane = executed.authority_lane();
        let basis_family = executed.basis_family();
        let receipt_digest = ForgeQueryEvidenceIdentity::compose(
            ForgeQueryEvidenceScope::EffectIntentReceipt,
        )
        .field_shape(
            ForgeQueryEvidenceTag::new("identity_family"),
            "effect_execution_receipt_v1",
        )
        .field_shape(ForgeQueryEvidenceTag::new("family"), receipt_family.as_str())
        .field_identity(ForgeQueryEvidenceTag::new("execution"), executed.batch_digest())
        .seal()
        .as_str()
        .to_string();
        let decision_trace = EffectReceiptDecisionTrace::batch(&executed);
        let integrity_markers = EffectReceiptIntegrityMarkers::new(
            authority_artifact_digest(executed.aggregate_artifact()),
            executed.counters(),
            &receipt_digest,
        );
        let counters = executed.counters().clone();
        Self {
            artifact: EffectExecutionReceiptArtifact::Batch(executed),
            receipt_family,
            declared_effect_family,
            authority_lane,
            basis_family,
            receipt_digest,
            decision_trace,
            integrity_markers,
            counters,
        }
    }

    pub fn receipt_family(&self) -> EffectReceiptArtifactKind {
        self.receipt_family
    }

    pub fn declared_effect_family(&self) -> EffectFamily {
        self.declared_effect_family
    }

    pub fn authority_lane(&self) -> EffectAuthorityLane {
        self.authority_lane
    }

    pub fn basis_lane(&self) -> BasisFamily {
        self.basis_family
    }

    pub fn authority_owner(&self) -> EffectAuthorityOwner {
        self.decision_trace.authority_owner()
    }

    pub fn receipt_digest(&self) -> &str {
        &self.receipt_digest
    }

    pub fn decision_trace(&self) -> &EffectReceiptDecisionTrace {
        &self.decision_trace
    }

    pub fn integrity_markers(&self) -> &EffectReceiptIntegrityMarkers {
        &self.integrity_markers
    }

    pub fn delivery_counters(&self) -> &EffectLifecycleCounters {
        &self.counters
    }

    pub fn write_count(&self) -> usize {
        match &self.artifact {
            EffectExecutionReceiptArtifact::Scalar(_) => 1,
            EffectExecutionReceiptArtifact::Batch(executed) => executed.components().len(),
        }
    }

    pub fn effect_envelope(&self) -> SelfDescribingEffectEnvelope {
        SelfDescribingEffectEnvelope::from_receipt(self)
    }

    pub fn materialize_diagnostics(
        &self,
        request: EffectDiagnosticsRequest,
    ) -> EffectDiagnosticsMaterialization {
        let envelope = self.effect_envelope();
        EffectDiagnosticsMaterialization::from_receipt(self, &envelope, request)
    }

    pub fn transition_rules(&self) -> EffectReceiptTransitionRules {
        EffectReceiptTransitionRules::for_receipt_family(self.receipt_family)
    }

    pub fn target_evidence(&self) -> EffectReceiptTargetEvidence {
        match &self.artifact {
            EffectExecutionReceiptArtifact::Scalar(executed) => match executed.artifact() {
                ExecutedEffectAuthorityArtifact::Mutation(result) => {
                    EffectReceiptTargetEvidence::MutationCommit {
                        commit_id: result.outcome.commit.commit_id.0,
                        version_id: result.outcome.commit.version_id.0,
                    }
                }
                ExecutedEffectAuthorityArtifact::Merge(result) => {
                    EffectReceiptTargetEvidence::MergeCommit {
                        commit_id: result.commit.outcome.commit.commit_id.0,
                        version_id: result.commit.outcome.commit.version_id.0,
                    }
                }
                ExecutedEffectAuthorityArtifact::Writeback { execution } => {
                    EffectReceiptTargetEvidence::Writeback {
                        outcome_digest: execution.outcome().digest().to_string(),
                        receipt_digest: execution.authority_receipt().digest().to_string(),
                        execution_receipt_digest: execution
                            .execution_receipt()
                            .digest()
                            .to_string(),
                    }
                }
            },
            EffectExecutionReceiptArtifact::Batch(executed) => {
                let aggregate = executed
                    .aggregate_mutation()
                    .expect("phase 5 batch receipts remain mutation-only");
                EffectReceiptTargetEvidence::BatchMutation {
                    commit_id: aggregate.outcome.commit.commit_id.0,
                    version_id: aggregate.outcome.commit.version_id.0,
                    component_count: executed.components().len(),
                }
            }
        }
    }

    pub fn lowered_digest(&self) -> &str {
        self.decision_trace.lowered_digest()
    }
}

fn authority_artifact_digest(artifact: &ExecutedEffectAuthorityArtifact) -> String {
    match artifact {
        ExecutedEffectAuthorityArtifact::Mutation(result) => {
            format!(
                "commit:{}:{}",
                result.outcome.commit.commit_id.0, result.outcome.commit.version_id.0
            )
        }
        ExecutedEffectAuthorityArtifact::Merge(result) => {
            format!(
                "merge:{}:{}",
                result.commit.outcome.commit.commit_id.0, result.commit.outcome.commit.version_id.0
            )
        }
        ExecutedEffectAuthorityArtifact::Writeback { execution } => {
            format!("writeback:{}", execution.digest())
        }
    }
}
