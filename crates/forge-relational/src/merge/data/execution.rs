use std::sync::Arc;

use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};

use crate::history::data::{BranchId, CommitId, CommitReference};
use crate::merge::data::{
    BranchTouchedRecordDelta, LoweredMergePlanRecord, LoweredRecordDecision,
    LoweredRecordDecisionKind, MergeConflictClass, MergePlanningArtifactCore, MergePlanningError,
    MergePlanningRequest, MergeSchemaSnapshotDigestBasis, ResolvedAspectMergePolicy,
    ResolvedMergeBase, VisibleMergeRecord, VisibleMergeRecordKind,
};
use crate::storage::data::{EntityReadRecord, RelationReadRecord};
use crate::transactions::data::RecordRef;

use super::plans::LoweredMergePlan;

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct MergeExecutionRequest {
    pub target_branch: crate::history::data::BranchId,
    pub source_branch: crate::history::data::BranchId,
    pub merge_intent: crate::merge::data::MergeIntent,
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
    InvalidVisibleAspectReference {
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
    pub(crate) fn execution_ready_plan_mut_for_test(&mut self) -> &mut ExecutionReadyLoweredMergePlan {
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
            .filter_map(|record| match record.record_decision {
                LoweredRecordDecision::Execute(_) => None,
                LoweredRecordDecision::Block(_) => Some(MergeExecutionDeniedRecord {
                    record: record.record.clone(),
                    decision: LoweredRecordDecisionKind::Block,
                }),
                LoweredRecordDecision::Reject(_) => Some(MergeExecutionDeniedRecord {
                    record: record.record.clone(),
                    decision: LoweredRecordDecisionKind::Reject,
                }),
            })
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

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum MergeValueMaterialization {
    EqualityWitnessDigest,
    SnapshotPinnedRead,
    InternedCanonicalValueHandle,
    EagerInlineCanonicalValue,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum MergeValueSourceSide {
    Source,
    Target,
    Base,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct MaterializedAspectValue {
    pub policy: MergeValueMaterialization,
    pub payload: MaterializedAspectValuePayload,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum MaterializedAspectValuePayload {
    EqualityWitnessDigest(String),
    VisibleAspectReference {
        side: MergeValueSourceSide,
        record: RecordRef,
        aspect_key: crate::publication::patch::data::AspectKey,
    },
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum VisibleMergeRecordSnapshot {
    Entity(EntityReadRecord),
    Relation(RelationReadRecord),
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct SharedTruthWitness {
    pub witness_digest: String,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ReconciledIdentityBasis {
    pub source_record: RecordRef,
    pub target_record: RecordRef,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum ExecutableAspectPlan {
    AdoptSourceValue {
        aspect_key: crate::publication::patch::data::AspectKey,
        source_value: MaterializedAspectValue,
    },
    PreserveSharedValue {
        aspect_key: crate::publication::patch::data::AspectKey,
        shared_value: MaterializedAspectValue,
    },
    ReconcileValue {
        aspect_key: crate::publication::patch::data::AspectKey,
        source_value: Option<MaterializedAspectValue>,
        target_value: Option<MaterializedAspectValue>,
        base_value: Option<MaterializedAspectValue>,
        resolved_value: Option<MaterializedAspectValue>,
    },
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct MergeExecutableRecordProvenance {
    pub classification: MergeConflictClass,
    pub resolution_class: crate::merge::data::MergeResolutionClass,
    pub executable_class: crate::merge::data::MergeExecutableClass,
    pub causal_disposition: crate::merge::data::MergeRecordCausalDisposition,
    pub policy_proof_boundary: crate::merge::data::MergePolicyProofBoundary,
    pub applied_policies: Arc<[ResolvedAspectMergePolicy]>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct AdoptSourceRecordPlan {
    pub source_record: RecordRef,
    pub(crate) record_kind: VisibleMergeRecordKind,
    pub source_visible_snapshot: VisibleMergeRecordSnapshot,
    pub provenance: MergeExecutableRecordProvenance,
    pub aspect_plan: Arc<[ExecutableAspectPlan]>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct PreserveSharedRecordPlan {
    pub record: RecordRef,
    pub target_record: Option<RecordRef>,
    pub equality_witness: SharedTruthWitness,
    pub provenance: MergeExecutableRecordProvenance,
    pub aspect_plan: Arc<[ExecutableAspectPlan]>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ReconcileRecordPlan {
    pub source_record: RecordRef,
    pub target_record: RecordRef,
    pub source_visible_snapshot: VisibleMergeRecordSnapshot,
    pub identity_basis: ReconciledIdentityBasis,
    pub causal_disposition: crate::merge::data::MergeRecordCausalDisposition,
    pub provenance: MergeExecutableRecordProvenance,
    pub aspect_plan: Arc<[ExecutableAspectPlan]>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ConvergeDeletedOnBothSidesRecordPlan {
    pub source_record: RecordRef,
    pub target_record: Option<RecordRef>,
    pub equality_witness: SharedTruthWitness,
    pub provenance: MergeExecutableRecordProvenance,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum BoundExecutableMergeRecordPlan {
    AdoptSource(AdoptSourceRecordPlan),
    PreserveShared(PreserveSharedRecordPlan),
    Reconcile(ReconcileRecordPlan),
    ConvergeDeletedOnBothSides(ConvergeDeletedOnBothSidesRecordPlan),
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct BoundExecutableMergePlan {
    pub authority_binding: MergeExecutionAuthorityBinding,
    pub parent_order: Arc<[CommitId]>,
    pub record_plans: Arc<[BoundExecutableMergeRecordPlan]>,
    pub diagnostics_plan: crate::merge::data::MergeExecutionDiagnosticsPlan,
}

pub(crate) fn compiled_executable_plan_digest(
    target_branch: &BranchId,
    source_branch: &BranchId,
    merge_intent: crate::merge::data::MergeIntent,
    parent_order: &[CommitId],
    record_plans: &[BoundExecutableMergeRecordPlan],
) -> String {
    let bytes = serde_json::to_vec(&(
        target_branch,
        source_branch,
        merge_intent,
        parent_order,
        executable_record_plan_digest_rows(record_plans),
    ))
    .expect("compiled executable merge plan serialization");
    let digest = Sha256::digest(bytes);
    digest.iter().map(|byte| format!("{byte:02x}")).collect()
}

pub(crate) fn schema_snapshot_digest(schema_snapshot: &MergeSchemaSnapshotDigestBasis) -> String {
    let bytes = serde_json::to_vec(schema_snapshot).expect("merge schema snapshot serialization");
    let digest = Sha256::digest(bytes);
    digest.iter().map(|byte| format!("{byte:02x}")).collect()
}

pub(crate) fn bound_parent_order(
    execution_ready: &ExecutionReadyLoweredMergePlan,
) -> Arc<[CommitId]> {
    Arc::from([
        execution_ready.target_head.commit_id,
        execution_ready.source_head.commit_id,
    ])
}

pub(crate) fn visible_record_snapshot(
    record: &VisibleMergeRecord,
) -> Option<VisibleMergeRecordSnapshot> {
    match record.record_kind {
        VisibleMergeRecordKind::Entity => record
            .source_entity
            .clone()
            .map(VisibleMergeRecordSnapshot::Entity),
        VisibleMergeRecordKind::Relation => record
            .source_relation
            .clone()
            .map(VisibleMergeRecordSnapshot::Relation),
    }
}

pub(crate) fn equality_witness_digest(record: &VisibleMergeRecord) -> String {
    let bytes = serde_json::to_vec(&(
        record.record_ref.clone(),
        record.source_entity.as_ref(),
        record.target_entity.as_ref(),
        record.source_relation.as_ref(),
        record.target_relation.as_ref(),
    ))
    .expect("visible merge record serialization");
    let digest = Sha256::digest(bytes);
    digest.iter().map(|byte| format!("{byte:02x}")).collect()
}

pub(crate) fn aspect_reference(
    side: MergeValueSourceSide,
    record: RecordRef,
    aspect_key: crate::publication::patch::data::AspectKey,
) -> MaterializedAspectValue {
    MaterializedAspectValue {
        policy: MergeValueMaterialization::SnapshotPinnedRead,
        payload: MaterializedAspectValuePayload::VisibleAspectReference {
            side,
            record,
            aspect_key,
        },
    }
}

fn executable_record_plan_digest_rows(
    record_plans: &[BoundExecutableMergeRecordPlan],
) -> Vec<ExecutableRecordPlanDigestRow<'_>> {
    record_plans
        .iter()
        .map(|plan| match plan {
            BoundExecutableMergeRecordPlan::AdoptSource(plan) => ExecutableRecordPlanDigestRow {
                variant: "adopt_source",
                source_record: Some(&plan.source_record),
                target_record: None,
                record: None,
                record_kind: Some(match plan.record_kind {
                    VisibleMergeRecordKind::Entity => "entity",
                    VisibleMergeRecordKind::Relation => "relation",
                }),
                source_visible_snapshot: Some(&plan.source_visible_snapshot),
                equality_witness: None,
                identity_basis: None,
                provenance: Some(&plan.provenance),
                aspect_plan: executable_aspect_plan_digest_rows(&plan.aspect_plan),
            },
            BoundExecutableMergeRecordPlan::PreserveShared(plan) => ExecutableRecordPlanDigestRow {
                variant: "preserve_shared",
                source_record: None,
                target_record: plan.target_record.as_ref(),
                record: Some(&plan.record),
                record_kind: None,
                source_visible_snapshot: None,
                equality_witness: Some(&plan.equality_witness),
                identity_basis: None,
                provenance: Some(&plan.provenance),
                aspect_plan: executable_aspect_plan_digest_rows(&plan.aspect_plan),
            },
            BoundExecutableMergeRecordPlan::Reconcile(plan) => ExecutableRecordPlanDigestRow {
                variant: "reconcile",
                source_record: Some(&plan.source_record),
                target_record: Some(&plan.target_record),
                record: None,
                record_kind: None,
                source_visible_snapshot: Some(&plan.source_visible_snapshot),
                equality_witness: None,
                identity_basis: Some(&plan.identity_basis),
                provenance: Some(&plan.provenance),
                aspect_plan: executable_aspect_plan_digest_rows(&plan.aspect_plan),
            },
            BoundExecutableMergeRecordPlan::ConvergeDeletedOnBothSides(plan) => {
                ExecutableRecordPlanDigestRow {
                    variant: "converge_deleted_on_both_sides",
                    source_record: Some(&plan.source_record),
                    target_record: plan.target_record.as_ref(),
                    record: None,
                    record_kind: None,
                    source_visible_snapshot: None,
                    equality_witness: Some(&plan.equality_witness),
                    identity_basis: None,
                    provenance: Some(&plan.provenance),
                    aspect_plan: Vec::new(),
                }
            }
        })
        .collect()
}

fn executable_aspect_plan_digest_rows(
    aspect_plans: &[ExecutableAspectPlan],
) -> Vec<ExecutableAspectPlanDigestRow<'_>> {
    aspect_plans
        .iter()
        .map(|plan| match plan {
            ExecutableAspectPlan::AdoptSourceValue {
                aspect_key,
                source_value,
            } => ExecutableAspectPlanDigestRow {
                variant: "adopt_source",
                aspect_key,
                source_value: Some(source_value),
                target_value: None,
                base_value: None,
                shared_value: None,
                resolved_value: None,
            },
            ExecutableAspectPlan::PreserveSharedValue {
                aspect_key,
                shared_value,
            } => ExecutableAspectPlanDigestRow {
                variant: "preserve_shared",
                aspect_key,
                source_value: None,
                target_value: None,
                base_value: None,
                shared_value: Some(shared_value),
                resolved_value: None,
            },
            ExecutableAspectPlan::ReconcileValue {
                aspect_key,
                source_value,
                target_value,
                base_value,
                resolved_value,
            } => ExecutableAspectPlanDigestRow {
                variant: "reconcile",
                aspect_key,
                source_value: source_value.as_ref(),
                target_value: target_value.as_ref(),
                base_value: base_value.as_ref(),
                shared_value: None,
                resolved_value: resolved_value.as_ref(),
            },
        })
        .collect()
}

#[derive(Serialize)]
struct ExecutableRecordPlanDigestRow<'a> {
    variant: &'static str,
    source_record: Option<&'a RecordRef>,
    target_record: Option<&'a RecordRef>,
    record: Option<&'a RecordRef>,
    record_kind: Option<&'static str>,
    source_visible_snapshot: Option<&'a VisibleMergeRecordSnapshot>,
    equality_witness: Option<&'a SharedTruthWitness>,
    identity_basis: Option<&'a ReconciledIdentityBasis>,
    provenance: Option<&'a MergeExecutableRecordProvenance>,
    aspect_plan: Vec<ExecutableAspectPlanDigestRow<'a>>,
}

#[derive(Serialize)]
struct ExecutableAspectPlanDigestRow<'a> {
    variant: &'static str,
    aspect_key: &'a crate::publication::patch::data::AspectKey,
    source_value: Option<&'a MaterializedAspectValue>,
    target_value: Option<&'a MaterializedAspectValue>,
    base_value: Option<&'a MaterializedAspectValue>,
    shared_value: Option<&'a MaterializedAspectValue>,
    resolved_value: Option<&'a MaterializedAspectValue>,
}
