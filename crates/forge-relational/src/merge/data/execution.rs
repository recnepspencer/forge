mod canonical_digest;
mod executable_plan;
mod materialized_aspect_values;

use std::sync::Arc;

use serde::{Deserialize, Serialize};

pub(crate) use canonical_digest::{
    compiled_executable_plan_digest, equality_witness_digest, merge_execution_diagnostics_digest,
    schema_snapshot_digest,
};
pub(crate) use executable_plan::{bound_parent_order, visible_record_snapshot};
pub use executable_plan::{
    AdoptSourceRecordPlan, BoundExecutableMergePlan, BoundExecutableMergeRecordPlan,
    ConvergeDeletedOnBothSidesRecordPlan, DeletedOnBothSidesSemantics, ExecutableAspectPlan,
    MergeExecutableRecordProvenance, MergeLineageContinuityVerdict, PreserveSharedRecordPlan,
    ReconcileRecordPlan, ReconciledIdentityBasis, SharedTruthWitness, VisibleMergeRecordSnapshot,
};
pub(crate) use materialized_aspect_values::{aspect_reference, materialized_value_aspect_key};
pub use materialized_aspect_values::{
    MaterializedAspectValue, MaterializedAspectValueEvidence, MergeValueMaterialization,
    MergeValueSourceSide,
};

use crate::history::data::{BranchId, CommitId, CommitReference};
use crate::merge::data::{
    BranchTouchedRecordDelta, LoweredMergePlanRecord, LoweredRecordDecision,
    LoweredRecordDecisionKind, MergePlanningArtifactCore, MergePlanningError, MergePlanningRequest,
    MergeSchemaSnapshotDigestBasis, ResolvedMergeBase, VisibleMergeRecord,
};
use crate::transactions::data::RecordRef;

use super::plans::LoweredMergePlan;

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct MergeExecutionRequest {
    pub target_branch: crate::history::data::BranchId,
    pub source_branch: crate::history::data::BranchId,
    pub merge_intent: crate::merge::data::MergeIntent,
}

impl MergeExecutionRequest {
    pub fn target_branch(&self) -> &crate::history::data::BranchId {
        &self.target_branch
    }

    pub fn source_branch(&self) -> &crate::history::data::BranchId {
        &self.source_branch
    }

    pub fn merge_intent(&self) -> &crate::merge::data::MergeIntent {
        &self.merge_intent
    }
}

impl From<MergeExecutionRequest> for MergePlanningRequest {
    fn from(value: MergeExecutionRequest) -> Self {
        Self {
            target_branch: value.target_branch,
            source_branch: value.source_branch,
            merge_intent: value.merge_intent,
        }
    }
}

impl From<MergePlanningRequest> for MergeExecutionRequest {
    fn from(value: MergePlanningRequest) -> Self {
        Self {
            target_branch: value.target_branch,
            source_branch: value.source_branch,
            merge_intent: value.merge_intent,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum MergeExecutionFreshnessPolicy {
    ExactAuthorityParity,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
pub struct RuntimeInstanceId(pub u64);

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct MergeExecutionAuthorityBinding {
    pub target_branch: BranchId,
    pub source_branch: BranchId,
    pub merge_intent: crate::merge::data::MergeIntent,
    pub runtime_instance_id: RuntimeInstanceId,
    pub target_head_commit_id: CommitId,
    pub source_head_commit_id: CommitId,
    pub merge_base_commit_id: CommitId,
    pub schema_snapshot_digest: String,
    pub freshness_policy: MergeExecutionFreshnessPolicy,
    pub executable_plan_digest: String,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct MergeExecutionDeniedRecord {
    pub record: RecordRef,
    pub decision: LoweredRecordDecisionKind,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct MergeExecutionReadinessReport {
    pub record_count: usize,
    pub blocked_count: usize,
    pub rejected_count: usize,
    pub denied_records: Arc<[MergeExecutionDeniedRecord]>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum MergeExecutionPreparationError {
    Planning(MergePlanningError),
    NotExecutionReady(MergeExecutionReadinessReport),
    Compilation(MergeExecutionCompilationError),
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum MergeExecutionError {
    RuntimeInstanceMismatch {
        planned: RuntimeInstanceId,
        current: RuntimeInstanceId,
    },
    StaleBranchHead {
        branch: crate::history::data::BranchId,
        planned: CommitId,
        current: Option<CommitId>,
    },
    MergeBaseDrift {
        planned: CommitId,
        current: Option<CommitId>,
    },
    SchemaSemanticDrift {
        planned_digest: String,
        current_digest: String,
    },
    Compilation(MergeExecutionCompilationError),
    MutationPlan(MergeExecutionMutationPlanError),
    Commit(crate::transactions::data::TransactionCommitError),
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum MergeExecutionCompilationError {
    MissingSourceRecord {
        record: RecordRef,
    },
    MissingTargetRecord {
        record: RecordRef,
    },
    MissingSourceSnapshot {
        record: RecordRef,
        record_kind: &'static str,
    },
    MissingExecutableClass {
        record: RecordRef,
        resolution_class: crate::merge::data::MergeResolutionClass,
    },
    ExecutableClassDecisionMismatch {
        record: RecordRef,
        executable_class: crate::merge::data::MergeExecutableClass,
        decision: LoweredRecordDecisionKind,
    },
    UnsupportedRecordDecision {
        record: RecordRef,
        decision: LoweredRecordDecisionKind,
    },
    MissingAspectExecutionIntent {
        record: RecordRef,
        aspect_key: crate::publication::patch::data::AspectKey,
    },
    MissingAuthorizedAspectValues {
        record: RecordRef,
        aspect_key: crate::publication::patch::data::AspectKey,
    },
    MissingAspectValueWitness {
        record: RecordRef,
        aspect_key: crate::publication::patch::data::AspectKey,
    },
    UnsupportedAspectValueWitness {
        record: RecordRef,
        aspect_key: crate::publication::patch::data::AspectKey,
        detail: String,
    },
    MissingAspectBinding {
        record: RecordRef,
        aspect_key: crate::publication::patch::data::AspectKey,
    },
    PreparedAuthorityBindingMismatch {
        detail: &'static str,
    },
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum MergeExecutionMutationPlanError {
    MissingSourceHeadEnvelope,
    MissingTargetEntitySnapshot {
        record: RecordRef,
    },
    MissingTargetRelationSnapshot {
        record: RecordRef,
    },
    UnsupportedReconcileRecordKind {
        record: RecordRef,
        detail: &'static str,
    },
    UnsupportedAspectMutationMaterialization {
        record: RecordRef,
        aspect_key: crate::publication::patch::data::AspectKey,
        detail: &'static str,
    },
    MissingResolvedAspectValue {
        record: RecordRef,
        aspect_key: crate::publication::patch::data::AspectKey,
    },
    InvalidPinnedVisibleAspect {
        record: RecordRef,
        aspect_key: crate::publication::patch::data::AspectKey,
        detail: &'static str,
    },
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PreparedMergeExecution {
    artifact: MergePlanningArtifactCore,
    request: MergeExecutionRequest,
    execution_ready_plan: ExecutionReadyLoweredMergePlan,
    bound_executable_plan: BoundExecutableMergePlan,
    execution_token: PreparedMergeExecutionToken,
}

impl From<MergeExecutionMutationPlanError> for MergeExecutionError {
    fn from(value: MergeExecutionMutationPlanError) -> Self {
        Self::MutationPlan(value)
    }
}

impl From<crate::transactions::data::TransactionCommitError> for MergeExecutionError {
    fn from(value: crate::transactions::data::TransactionCommitError) -> Self {
        Self::Commit(value)
    }
}

impl PreparedMergeExecution {
    pub(crate) fn new(
        request: MergeExecutionRequest,
        artifact: MergePlanningArtifactCore,
        execution_ready_plan: ExecutionReadyLoweredMergePlan,
        bound_executable_plan: BoundExecutableMergePlan,
    ) -> Self {
        Self {
            request,
            artifact,
            execution_ready_plan,
            bound_executable_plan,
            execution_token: PreparedMergeExecutionToken,
        }
    }

    pub fn request(&self) -> &MergeExecutionRequest {
        &self.request
    }

    pub fn artifact(&self) -> &MergePlanningArtifactCore {
        &self.artifact
    }

    #[allow(dead_code)]
    pub(crate) fn execution_ready_plan(&self) -> &ExecutionReadyLoweredMergePlan {
        &self.execution_ready_plan
    }

    #[allow(dead_code)]
    pub(crate) fn bound_executable_plan(&self) -> &BoundExecutableMergePlan {
        &self.bound_executable_plan
    }

    #[cfg(test)]
    pub(crate) fn execution_ready_plan_mut_for_test(
        &mut self,
    ) -> &mut ExecutionReadyLoweredMergePlan {
        &mut self.execution_ready_plan
    }

    #[cfg(test)]
    pub(crate) fn authority_binding_mut_for_test(&mut self) -> &mut MergeExecutionAuthorityBinding {
        &mut self.bound_executable_plan.authority_binding
    }

    #[cfg(test)]
    pub(crate) fn bound_executable_plan_mut_for_test(&mut self) -> &mut BoundExecutableMergePlan {
        &mut self.bound_executable_plan
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct PreparedMergeExecutionToken;

#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct ExecutionReadyLoweredMergePlan {
    pub(crate) target_head: CommitReference,
    pub(crate) source_head: CommitReference,
    pub(crate) merge_base: ResolvedMergeBase,
    pub(crate) schema_snapshot: MergeSchemaSnapshotDigestBasis,
    pub(crate) source_records: Arc<[VisibleMergeRecord]>,
    pub(crate) target_touched_records: Arc<[BranchTouchedRecordDelta]>,
    pub(crate) lowered_records: Arc<[LoweredMergePlanRecord]>,
    pub(crate) freshness_policy: MergeExecutionFreshnessPolicy,
}

impl ExecutionReadyLoweredMergePlan {
    pub(crate) fn try_from_lowered(
        plan: LoweredMergePlan,
        schema_snapshot: MergeSchemaSnapshotDigestBasis,
    ) -> Result<Self, MergeExecutionReadinessReport> {
        let denied_records = plan
            .lowered_records
            .iter()
            .filter_map(readiness_denial_for_lowered_record)
            .collect::<Vec<_>>();

        if !denied_records.is_empty() {
            return Err(MergeExecutionReadinessReport {
                record_count: plan.lowered_summary.record_count,
                blocked_count: plan.lowered_summary.blocked_count,
                rejected_count: plan.lowered_summary.rejected_count,
                denied_records: Arc::from(denied_records),
            });
        }

        Ok(Self {
            target_head: plan.target_head,
            source_head: plan.source_head,
            merge_base: plan.merge_base,
            schema_snapshot,
            source_records: plan.source_records,
            target_touched_records: plan.target_delta.touched_records,
            lowered_records: plan.lowered_records,
            freshness_policy: MergeExecutionFreshnessPolicy::ExactAuthorityParity,
        })
    }
}

fn readiness_denial_for_lowered_record(
    record: &LoweredMergePlanRecord,
) -> Option<MergeExecutionDeniedRecord> {
    match record.record_decision {
        LoweredRecordDecision::Execute(_) => None,
        LoweredRecordDecision::Block(_) => Some(MergeExecutionDeniedRecord {
            record: record.record.clone(),
            decision: LoweredRecordDecisionKind::Block,
        }),
        LoweredRecordDecision::Reject(_) => Some(MergeExecutionDeniedRecord {
            record: record.record.clone(),
            decision: LoweredRecordDecisionKind::Reject,
        }),
    }
}
