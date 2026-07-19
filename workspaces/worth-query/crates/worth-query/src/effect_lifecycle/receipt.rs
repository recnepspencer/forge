use crate::basis_lifecycle::BasisFamily;
use crate::{WorthQueryEvidenceIdentity, WorthQueryEvidenceScope, WorthQueryEvidenceTag};

use super::batch_execution::ExecutedEffectBatchPlan;
use super::counters::EffectLifecycleCounters;
use super::diagnostics::{EffectDiagnosticsMaterialization, EffectDiagnosticsRequest};
use super::envelope::SelfDescribingEffectEnvelope;
use super::execution::{ExecutedEffectAuthorityArtifact, ExecutedEffectPlan};
use super::execution_artifacts::{
    executed_authority_artifact_identity, writeback_bridge_evidence_identity,
    writeback_bridge_execution_receipt_evidence_identity,
    writeback_bridge_receipt_evidence_identity,
};
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
        outcome_identity: WorthQueryEvidenceIdentity,
        authority_receipt_identity: WorthQueryEvidenceIdentity,
        execution_receipt_identity: WorthQueryEvidenceIdentity,
    },
    BatchMutation {
        commit_id: u64,
        version_id: u64,
        component_count: usize,
    },
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct EffectReceiptDecisionTrace {
    admitted_or_batch_identity: WorthQueryEvidenceIdentity,
    lowered_identity: WorthQueryEvidenceIdentity,
    authority_owner: EffectAuthorityOwner,
    decision_trace_identity: WorthQueryEvidenceIdentity,
}

impl EffectReceiptDecisionTrace {
    fn scalar(executed: &ExecutedEffectPlan) -> Self {
        let admitted_or_batch_identity = executed
            .lowered()
            .authority_scoped_plan()
            .admitted()
            .admitted_identity()
            .clone();
        let lowered_identity = executed
            .lowered()
            .lowered_effect_execution_plan_identity()
            .clone();
        let authority_owner = executed.authority_owner();
        let decision_trace_identity =
            WorthQueryEvidenceIdentity::compose(WorthQueryEvidenceScope::EffectIntentReceipt)
                .field_shape(
                    WorthQueryEvidenceTag::new("identity_family"),
                    "effect_receipt_decision_trace_v1",
                )
                .field_evidence_identity(
                    WorthQueryEvidenceTag::new("admitted"),
                    &admitted_or_batch_identity,
                )
                .field_evidence_identity(WorthQueryEvidenceTag::new("lowered"), &lowered_identity)
                .field_shape(
                    WorthQueryEvidenceTag::new("authority_owner"),
                    authority_owner.as_str(),
                )
                .seal();
        Self {
            admitted_or_batch_identity,
            lowered_identity,
            authority_owner,
            decision_trace_identity,
        }
    }

    fn batch(executed: &ExecutedEffectBatchPlan) -> Self {
        let admitted_or_batch_identity = executed.lowered().admitted_batch_identity().clone();
        let lowered_identity = executed.lowered().batch_identity().clone();
        let authority_owner = executed.authority_owner();
        let decision_trace_identity =
            WorthQueryEvidenceIdentity::compose(WorthQueryEvidenceScope::EffectIntentReceipt)
                .field_shape(
                    WorthQueryEvidenceTag::new("identity_family"),
                    "effect_receipt_decision_trace_v1",
                )
                .field_evidence_identity(
                    WorthQueryEvidenceTag::new("admitted_batch"),
                    &admitted_or_batch_identity,
                )
                .field_evidence_identity(WorthQueryEvidenceTag::new("lowered"), &lowered_identity)
                .field_shape(
                    WorthQueryEvidenceTag::new("authority_owner"),
                    authority_owner.as_str(),
                )
                .seal();
        Self {
            admitted_or_batch_identity,
            lowered_identity,
            authority_owner,
            decision_trace_identity,
        }
    }

    pub fn admitted_or_batch_identity(&self) -> &WorthQueryEvidenceIdentity {
        &self.admitted_or_batch_identity
    }

    pub fn admitted_or_batch_for_reporting(&self) -> &str {
        self.admitted_or_batch_identity.as_str()
    }

    pub fn lowered_identity(&self) -> &WorthQueryEvidenceIdentity {
        &self.lowered_identity
    }

    pub fn lowered_for_reporting(&self) -> &str {
        self.lowered_identity.as_str()
    }

    pub fn authority_owner(&self) -> EffectAuthorityOwner {
        self.authority_owner
    }

    pub fn decision_trace_identity(&self) -> &WorthQueryEvidenceIdentity {
        &self.decision_trace_identity
    }

    pub fn decision_trace_for_reporting(&self) -> &str {
        self.decision_trace_identity.as_str()
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct EffectReceiptIntegrityMarkers {
    authority_artifact_identity: WorthQueryEvidenceIdentity,
    counter_snapshot_identity: WorthQueryEvidenceIdentity,
    integrity_identity: WorthQueryEvidenceIdentity,
}

impl EffectReceiptIntegrityMarkers {
    fn new(
        authority_artifact: &ExecutedEffectAuthorityArtifact,
        counters: &EffectLifecycleCounters,
        receipt_identity: &WorthQueryEvidenceIdentity,
    ) -> Self {
        let authority_artifact_identity = executed_authority_artifact_identity(authority_artifact);
        let counter_snapshot_identity = counters.evidence_identity();
        let integrity_identity =
            WorthQueryEvidenceIdentity::compose(WorthQueryEvidenceScope::EffectIntentReceipt)
                .field_shape(
                    WorthQueryEvidenceTag::new("identity_family"),
                    "effect_receipt_integrity_markers_v1",
                )
                .field_evidence_identity(
                    WorthQueryEvidenceTag::new("authority_artifact"),
                    &authority_artifact_identity,
                )
                .field_evidence_identity(
                    WorthQueryEvidenceTag::new("counters"),
                    &counter_snapshot_identity,
                )
                .field_evidence_identity(WorthQueryEvidenceTag::new("receipt"), receipt_identity)
                .seal();
        Self {
            authority_artifact_identity,
            counter_snapshot_identity,
            integrity_identity,
        }
    }

    pub fn authority_artifact_identity(&self) -> &WorthQueryEvidenceIdentity {
        &self.authority_artifact_identity
    }

    pub fn authority_artifact_for_reporting(&self) -> &str {
        self.authority_artifact_identity.as_str()
    }

    pub fn counter_snapshot_identity(&self) -> &WorthQueryEvidenceIdentity {
        &self.counter_snapshot_identity
    }

    pub fn counter_snapshot_for_reporting(&self) -> &str {
        self.counter_snapshot_identity.as_str()
    }

    pub fn integrity_identity(&self) -> &WorthQueryEvidenceIdentity {
        &self.integrity_identity
    }

    pub fn integrity_for_reporting(&self) -> &str {
        self.integrity_identity.as_str()
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
    receipt_identity: WorthQueryEvidenceIdentity,
    decision_trace: EffectReceiptDecisionTrace,
    integrity_markers: EffectReceiptIntegrityMarkers,
    counters: EffectLifecycleCounters,
}

impl EffectExecutionReceipt {
    pub(super) fn from_scalar(executed: ExecutedEffectPlan) -> Self {
        let receipt_family = match executed.artifact() {
            ExecutedEffectAuthorityArtifact::Writeback { .. } => {
                EffectReceiptArtifactKind::WorthQueryWriteReceipt
            }
            ExecutedEffectAuthorityArtifact::Mutation(_)
            | ExecutedEffectAuthorityArtifact::Merge(_) => {
                EffectReceiptArtifactKind::WorthQueryIntentExecution
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
        let execution_identity = scalar_execution_receipt_identity(&executed, receipt_family);
        let receipt_identity =
            WorthQueryEvidenceIdentity::compose(WorthQueryEvidenceScope::EffectIntentReceipt)
                .field_shape(
                    WorthQueryEvidenceTag::new("identity_family"),
                    "effect_execution_receipt_v1",
                )
                .field_shape(
                    WorthQueryEvidenceTag::new("family"),
                    receipt_family.as_str(),
                )
                .field_evidence_identity(
                    WorthQueryEvidenceTag::new("execution"),
                    &execution_identity,
                )
                .seal();
        let decision_trace = EffectReceiptDecisionTrace::scalar(&executed);
        let integrity_markers = EffectReceiptIntegrityMarkers::new(
            executed.artifact(),
            executed.counters(),
            &receipt_identity,
        );
        let counters = executed.counters().clone();
        Self {
            artifact: EffectExecutionReceiptArtifact::Scalar(executed),
            receipt_family,
            declared_effect_family,
            authority_lane,
            basis_family,
            receipt_identity,
            decision_trace,
            integrity_markers,
            counters,
        }
    }

    pub(super) fn from_batch(executed: ExecutedEffectBatchPlan) -> Self {
        let receipt_family = EffectReceiptArtifactKind::WorthQueryBatchWriteReceipt;
        let declared_effect_family = EffectFamily::Mutation;
        let authority_lane = executed.authority_lane();
        let basis_family = executed.basis_family();
        let execution_identity = batch_execution_receipt_identity(&executed);
        let receipt_identity =
            WorthQueryEvidenceIdentity::compose(WorthQueryEvidenceScope::EffectIntentReceipt)
                .field_shape(
                    WorthQueryEvidenceTag::new("identity_family"),
                    "effect_execution_receipt_v1",
                )
                .field_shape(
                    WorthQueryEvidenceTag::new("family"),
                    receipt_family.as_str(),
                )
                .field_evidence_identity(
                    WorthQueryEvidenceTag::new("execution"),
                    &execution_identity,
                )
                .seal();
        let decision_trace = EffectReceiptDecisionTrace::batch(&executed);
        let integrity_markers = EffectReceiptIntegrityMarkers::new(
            executed.aggregate_artifact(),
            executed.counters(),
            &receipt_identity,
        );
        let counters = executed.counters().clone();
        Self {
            artifact: EffectExecutionReceiptArtifact::Batch(executed),
            receipt_family,
            declared_effect_family,
            authority_lane,
            basis_family,
            receipt_identity,
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

    pub fn receipt_identity(&self) -> &WorthQueryEvidenceIdentity {
        &self.receipt_identity
    }

    pub fn receipt_for_reporting(&self) -> &str {
        self.receipt_identity.as_str()
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
                        outcome_identity: writeback_bridge_evidence_identity(
                            "outcome",
                            execution.outcome(),
                        ),
                        authority_receipt_identity: writeback_bridge_receipt_evidence_identity(
                            "authority_receipt",
                            execution.authority_receipt(),
                        ),
                        execution_receipt_identity:
                            writeback_bridge_execution_receipt_evidence_identity(
                                "execution_receipt",
                                execution.execution_receipt(),
                            ),
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

    pub fn lowered_for_reporting(&self) -> &str {
        self.decision_trace.lowered_for_reporting()
    }
}

impl EffectReceiptTargetEvidence {
    pub fn writeback_outcome_for_reporting(&self) -> Option<&str> {
        match self {
            Self::Writeback {
                outcome_identity, ..
            } => Some(outcome_identity.as_str()),
            _ => None,
        }
    }

    pub fn writeback_authority_receipt_for_reporting(&self) -> Option<&str> {
        match self {
            Self::Writeback {
                authority_receipt_identity,
                ..
            } => Some(authority_receipt_identity.as_str()),
            _ => None,
        }
    }

    pub fn writeback_execution_receipt_for_reporting(&self) -> Option<&str> {
        match self {
            Self::Writeback {
                execution_receipt_identity,
                ..
            } => Some(execution_receipt_identity.as_str()),
            _ => None,
        }
    }
}

fn scalar_execution_receipt_identity(
    executed: &ExecutedEffectPlan,
    receipt_family: EffectReceiptArtifactKind,
) -> WorthQueryEvidenceIdentity {
    WorthQueryEvidenceIdentity::compose(WorthQueryEvidenceScope::EffectIntentReceipt)
        .field_shape(
            WorthQueryEvidenceTag::new("identity_family"),
            "effect_execution_receipt_execution_v1",
        )
        .field_shape(
            WorthQueryEvidenceTag::new("family"),
            receipt_family.as_str(),
        )
        .field_evidence_identity(
            WorthQueryEvidenceTag::new("lowered"),
            executed.lowered().lowered_effect_execution_plan_identity(),
        )
        .field_evidence_identity(
            WorthQueryEvidenceTag::new("authority_artifact"),
            &executed_authority_artifact_identity(executed.artifact()),
        )
        .seal()
}

fn batch_execution_receipt_identity(
    executed: &ExecutedEffectBatchPlan,
) -> WorthQueryEvidenceIdentity {
    WorthQueryEvidenceIdentity::compose(WorthQueryEvidenceScope::EffectIntentReceipt)
        .field_shape(
            WorthQueryEvidenceTag::new("identity_family"),
            "effect_execution_receipt_execution_v1",
        )
        .field_shape(
            WorthQueryEvidenceTag::new("family"),
            EffectReceiptArtifactKind::WorthQueryBatchWriteReceipt.as_str(),
        )
        .field_evidence_identity(
            WorthQueryEvidenceTag::new("lowered"),
            executed.lowered().batch_identity(),
        )
        .field_evidence_identity(
            WorthQueryEvidenceTag::new("authority_artifact"),
            &executed_authority_artifact_identity(executed.aggregate_artifact()),
        )
        .seal()
}
