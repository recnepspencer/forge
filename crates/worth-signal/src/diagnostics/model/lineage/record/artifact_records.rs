use super::{
    ArtifactTransitionKind, InvalidationCause, LineageArtifactId, LineageRecord, LineageRecordKind,
};
use crate::data::handle::NodeId;
use crate::logic::planner::{ExecutionRecordId, SemanticSegmentId};
use crate::logic::transaction::{
    ArtifactMergeAction, BranchMergeConflictKind, BranchMergeDivergence, BranchMergeKind,
    BranchMergeReconciliationPolicy, BranchMergeStrategy, MergeDecisionBasis,
};
use crate::state::SignalBranchId;

impl LineageRecord {
    pub fn artifact_transition(
        sequence: u64,
        emitted_on_branch_id: SignalBranchId,
        node: NodeId,
        artifact_id: LineageArtifactId,
        parent_artifact_id: Option<LineageArtifactId>,
        execution_record_id: ExecutionRecordId,
        semantic_segment_id: SemanticSegmentId,
        transition: ArtifactTransitionKind,
    ) -> Self {
        Self::new(
            sequence,
            emitted_on_branch_id,
            LineageRecordKind::ArtifactTransition {
                node,
                artifact_id,
                parent_artifact_id,
                execution_record_id,
                semantic_segment_id,
                transition,
            },
        )
    }

    pub fn artifact_merge(
        sequence: u64,
        emitted_on_branch_id: SignalBranchId,
        source_node: NodeId,
        target_node: Option<NodeId>,
        source_branch_id: SignalBranchId,
        target_branch_id: SignalBranchId,
        source_artifact_id: Option<LineageArtifactId>,
        target_artifact_id_before: Option<LineageArtifactId>,
        target_artifact_id_after: Option<LineageArtifactId>,
        merge_action: ArtifactMergeAction,
        decision_basis: MergeDecisionBasis,
        merge_kind: BranchMergeKind,
        divergence: BranchMergeDivergence,
        merge_strategy: BranchMergeStrategy,
        reconciliation_policy: BranchMergeReconciliationPolicy,
        resolved_conflict_kinds: Vec<BranchMergeConflictKind>,
    ) -> Self {
        Self::new(
            sequence,
            emitted_on_branch_id,
            LineageRecordKind::ArtifactMerge {
                source_node,
                target_node,
                source_branch_id,
                target_branch_id,
                source_artifact_id,
                target_artifact_id_before,
                target_artifact_id_after,
                merge_action,
                decision_basis,
                merge_kind,
                divergence,
                merge_strategy,
                reconciliation_policy,
                resolved_conflict_kinds,
            },
        )
    }

    pub fn invalidation(
        sequence: u64,
        emitted_on_branch_id: SignalBranchId,
        node: NodeId,
        artifact_id: LineageArtifactId,
        cause: InvalidationCause,
    ) -> Self {
        Self::new(
            sequence,
            emitted_on_branch_id,
            LineageRecordKind::Invalidation {
                node,
                artifact_id,
                cause,
            },
        )
    }
}
