use super::{LineageRecord, LineageRecordKind};
use crate::logic::transaction::{
    BranchConflictResolutionPlan, BranchMergeDivergence, BranchMergeKind,
    BranchMergeReconciliationPolicy, BranchMergeStrategy,
};
use crate::state::{SignalBranchId, SignalSnapshotId};

impl LineageRecord {
    pub fn branch_fork(
        sequence: u64,
        emitted_on_branch_id: SignalBranchId,
        created_branch_id: SignalBranchId,
        parent_branch_id: SignalBranchId,
        created_branch_display_name: impl Into<String>,
        parent_branch_display_name: impl Into<String>,
    ) -> Self {
        Self::new(
            sequence,
            emitted_on_branch_id,
            LineageRecordKind::BranchFork {
                created_branch_id,
                parent_branch_id,
                created_branch_display_name: created_branch_display_name.into(),
                parent_branch_display_name: parent_branch_display_name.into(),
            },
        )
    }

    pub fn branch_switch(
        sequence: u64,
        emitted_on_branch_id: SignalBranchId,
        from_branch_id: SignalBranchId,
        to_branch_id: SignalBranchId,
        from_branch_display_name: impl Into<String>,
        to_branch_display_name: impl Into<String>,
    ) -> Self {
        Self::new(
            sequence,
            emitted_on_branch_id,
            LineageRecordKind::BranchSwitch {
                from_branch_id,
                to_branch_id,
                from_branch_display_name: from_branch_display_name.into(),
                to_branch_display_name: to_branch_display_name.into(),
            },
        )
    }

    pub fn branch_merge(
        sequence: u64,
        emitted_on_branch_id: SignalBranchId,
        source_branch_id: SignalBranchId,
        target_branch_id: SignalBranchId,
        merge_kind: BranchMergeKind,
        divergence: BranchMergeDivergence,
        merge_strategy: BranchMergeStrategy,
        reconciliation_policy: BranchMergeReconciliationPolicy,
        resolution_plan: Option<BranchConflictResolutionPlan>,
        merged_snapshot_id: Option<SignalSnapshotId>,
        source_branch_display_name: impl Into<String>,
        target_branch_display_name: impl Into<String>,
    ) -> Self {
        Self::new(
            sequence,
            emitted_on_branch_id,
            LineageRecordKind::BranchMerge {
                source_branch_id,
                target_branch_id,
                merge_kind,
                divergence,
                merge_strategy,
                reconciliation_policy,
                resolution_plan,
                merged_snapshot_id,
                source_branch_display_name: source_branch_display_name.into(),
                target_branch_display_name: target_branch_display_name.into(),
            },
        )
    }
}
