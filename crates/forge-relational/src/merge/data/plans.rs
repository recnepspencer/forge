use std::sync::Arc;

use crate::history::data::{BranchId, CommitId, CommitReference};
use crate::identity::data::{KindId, LineageId};
use crate::merge::data::{
    CausalAnnotationSummary, IdentityBasisDeclaration, IdentityBasisKind, IdentityBasisScope,
    IdentityDiscoverySummary, IdentityMatchCandidate, LoweredMergePlanSummary,
    MergeAncestrySummary, MergePlanningDecisionLog, MergePlanningDecisionLogDigestBasis,
    MergePlanningRequest, MergePolicyResolutionSummary, ResolvedMergeBase,
};
use crate::storage::data::{EntityReadRecord, RelationReadRecord};
use crate::transactions::data::RecordRef;

#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct BranchTouchedRecordDelta {
    pub(crate) target: RecordRef,
    pub(crate) commit_ids: Arc<[CommitId]>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct BranchCommitDelta {
    pub(crate) branch_id: BranchId,
    pub(crate) commits: Arc<[CommitId]>,
    pub(crate) touched_records: Arc<[BranchTouchedRecordDelta]>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct HistoryScopedMergePlan {
    pub(crate) request: MergePlanningRequest,
    pub(crate) target_head: CommitReference,
    pub(crate) source_head: CommitReference,
    pub(crate) merge_base: ResolvedMergeBase,
    pub(crate) target_delta: BranchCommitDelta,
    pub(crate) source_delta: BranchCommitDelta,
}

#[derive(
    Debug,
    Clone,
    PartialEq,
    Eq,
    PartialOrd,
    Ord,
    serde::Serialize,
    serde::Deserialize,
)]
pub(crate) enum VisibleMergeRecordKind {
    Entity,
    Relation,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct VisibleMergeRecord {
    pub(crate) record_ref: RecordRef,
    pub(crate) record_kind: VisibleMergeRecordKind,
    pub(crate) kind_id: Option<KindId>,
    pub(crate) source_kind_id: Option<KindId>,
    pub(crate) target_kind_id: Option<KindId>,
    pub(crate) lineage_id: Option<LineageId>,
    pub(crate) source_lineage_id: Option<LineageId>,
    pub(crate) target_lineage_id: Option<LineageId>,
    pub(crate) source_entity: Option<EntityReadRecord>,
    pub(crate) target_entity: Option<EntityReadRecord>,
    pub(crate) source_relation: Option<RelationReadRecord>,
    pub(crate) target_relation: Option<RelationReadRecord>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct IdentityScopedMergePlan {
    pub(crate) request: MergePlanningRequest,
    pub(crate) target_head: CommitReference,
    pub(crate) source_head: CommitReference,
    pub(crate) merge_base: ResolvedMergeBase,
    pub(crate) ancestry: MergeAncestrySummary,
    pub(crate) target_delta: BranchCommitDelta,
    pub(crate) source_delta: BranchCommitDelta,
    pub(crate) effective_identity_declarations: Arc<[IdentityBasisDeclaration]>,
    pub(crate) source_records: Arc<[VisibleMergeRecord]>,
    pub(crate) candidates: Arc<[IdentityMatchCandidate]>,
    pub(crate) validated_schema_correspondences: Arc<[ValidatedSchemaDeclaredCorrespondence]>,
    pub(crate) identity_summary: IdentityDiscoverySummary,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct ConflictClassifiedMergePlan {
    pub(crate) request: MergePlanningRequest,
    pub(crate) target_head: CommitReference,
    pub(crate) source_head: CommitReference,
    pub(crate) merge_base: ResolvedMergeBase,
    pub(crate) ancestry: MergeAncestrySummary,
    pub(crate) target_delta: BranchCommitDelta,
    pub(crate) source_delta: BranchCommitDelta,
    pub(crate) effective_identity_declarations: Arc<[IdentityBasisDeclaration]>,
    pub(crate) source_records: Arc<[VisibleMergeRecord]>,
    pub(crate) candidates: Arc<[IdentityMatchCandidate]>,
    pub(crate) validated_schema_correspondences: Arc<[ValidatedSchemaDeclaredCorrespondence]>,
    pub(crate) identity_summary: IdentityDiscoverySummary,
    pub(crate) classifications: Arc<[crate::merge::data::MergeConflictClassification]>,
    pub(crate) conflict_summary: crate::merge::data::ConflictClassificationSummary,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct CausallyAnnotatedMergePlan {
    pub(crate) request: MergePlanningRequest,
    pub(crate) target_head: CommitReference,
    pub(crate) source_head: CommitReference,
    pub(crate) merge_base: ResolvedMergeBase,
    pub(crate) ancestry: MergeAncestrySummary,
    pub(crate) target_delta: BranchCommitDelta,
    pub(crate) source_delta: BranchCommitDelta,
    pub(crate) effective_identity_declarations: Arc<[IdentityBasisDeclaration]>,
    pub(crate) source_records: Arc<[VisibleMergeRecord]>,
    pub(crate) candidates: Arc<[IdentityMatchCandidate]>,
    pub(crate) validated_schema_correspondences: Arc<[ValidatedSchemaDeclaredCorrespondence]>,
    pub(crate) identity_summary: IdentityDiscoverySummary,
    pub(crate) classifications: Arc<[crate::merge::data::MergeConflictClassification]>,
    pub(crate) conflict_summary: crate::merge::data::ConflictClassificationSummary,
    pub(crate) causal_annotations: Arc<[crate::merge::data::MergeRecordCausalAnnotation]>,
    pub(crate) causal_summary: CausalAnnotationSummary,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct PolicyResolvedMergePlan {
    pub(crate) request: MergePlanningRequest,
    pub(crate) target_head: CommitReference,
    pub(crate) source_head: CommitReference,
    pub(crate) merge_base: ResolvedMergeBase,
    pub(crate) ancestry: MergeAncestrySummary,
    pub(crate) target_delta: BranchCommitDelta,
    pub(crate) source_delta: BranchCommitDelta,
    pub(crate) effective_identity_declarations: Arc<[IdentityBasisDeclaration]>,
    pub(crate) source_records: Arc<[VisibleMergeRecord]>,
    pub(crate) candidates: Arc<[IdentityMatchCandidate]>,
    pub(crate) validated_schema_correspondences: Arc<[ValidatedSchemaDeclaredCorrespondence]>,
    pub(crate) identity_summary: IdentityDiscoverySummary,
    pub(crate) classifications: Arc<[crate::merge::data::MergeConflictClassification]>,
    pub(crate) conflict_summary: crate::merge::data::ConflictClassificationSummary,
    pub(crate) causal_annotations: Arc<[crate::merge::data::MergeRecordCausalAnnotation]>,
    pub(crate) causal_summary: CausalAnnotationSummary,
    pub(crate) policy_records: Arc<[crate::merge::data::MergePolicyResolutionRecord]>,
    pub(crate) policy_summary: MergePolicyResolutionSummary,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct LoweredMergePlan {
    pub(crate) request: MergePlanningRequest,
    pub(crate) target_head: CommitReference,
    pub(crate) source_head: CommitReference,
    pub(crate) merge_base: ResolvedMergeBase,
    pub(crate) ancestry: MergeAncestrySummary,
    pub(crate) target_delta: BranchCommitDelta,
    pub(crate) source_delta: BranchCommitDelta,
    pub(crate) effective_identity_declarations: Arc<[IdentityBasisDeclaration]>,
    pub(crate) source_records: Arc<[VisibleMergeRecord]>,
    pub(crate) candidates: Arc<[IdentityMatchCandidate]>,
    pub(crate) validated_schema_correspondences: Arc<[ValidatedSchemaDeclaredCorrespondence]>,
    pub(crate) identity_summary: IdentityDiscoverySummary,
    pub(crate) classifications: Arc<[crate::merge::data::MergeConflictClassification]>,
    pub(crate) conflict_summary: crate::merge::data::ConflictClassificationSummary,
    pub(crate) causal_annotations: Arc<[crate::merge::data::MergeRecordCausalAnnotation]>,
    pub(crate) causal_summary: CausalAnnotationSummary,
    pub(crate) policy_records: Arc<[crate::merge::data::MergePolicyResolutionRecord]>,
    pub(crate) policy_summary: MergePolicyResolutionSummary,
    pub(crate) lowered_records: Arc<[crate::merge::data::LoweredMergePlanRecord]>,
    pub(crate) lowered_summary: LoweredMergePlanSummary,
    pub(crate) decision_log: MergePlanningDecisionLog,
    pub(crate) decision_log_digest_basis: MergePlanningDecisionLogDigestBasis,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct ValidatedSchemaDeclaredCorrespondence {
    pub(crate) scope: IdentityBasisScope,
    pub(crate) basis: IdentityBasisKind,
    pub(crate) source_record: RecordRef,
    pub(crate) target_record: RecordRef,
    pub(crate) candidate_count_for_source: usize,
    pub(crate) candidate_count_for_target: usize,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum MergePlanningError {
    MissingSourceHead { branch_id: BranchId },
    MissingTargetHead { branch_id: BranchId },
    MissingMergeBase {
        source_branch: BranchId,
        target_branch: BranchId,
    },
    MissingMergeBaseEnvelope { commit_id: CommitId },
    MissingConflictSourceRecord { record: RecordRef },
    MissingPolicySourceRecord { record: RecordRef },
    MissingPolicyTargetRecord {
        record: RecordRef,
        target_record: RecordRef,
    },
    MissingLoweringSourceRecord { record: RecordRef },
    MissingCausalAnnotation { record: RecordRef },
    MissingLoweredRecordExecutionBundle {
        classification: crate::merge::data::MergeConflictClass,
        readiness: crate::merge::data::MergeExecutionReadiness,
        lowered_action: Option<crate::merge::data::LoweredMergeAction>,
    },
    MissingLoweredRecordDenialBundle,
}
