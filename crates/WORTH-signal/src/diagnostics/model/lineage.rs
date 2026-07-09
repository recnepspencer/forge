use std::collections::VecDeque;

use serde::{Deserialize, Serialize};

use crate::data::handle::NodeId;
use crate::data::output::OutputChange;
use crate::data::reuse::PersistentCorrespondenceKind;
use crate::logic::planner::{ExecutionRecordId, SemanticSegmentId};
use crate::logic::transaction::{
    ArtifactMergeAction, BranchConflictResolutionPlan, BranchMergeConflictKind,
    BranchMergeDivergence, BranchMergeKind, BranchMergeReconciliationPolicy, BranchMergeStrategy,
    MergeDecisionBasis,
};
use crate::state::{SignalBranchId, SignalSnapshotId};

#[derive(
    Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize, Default,
)]
pub struct LineageArtifactId(pub u64);

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum ArtifactTransitionKind {
    Replaced,
    Refreshed {
        output_change: OutputChange,
    },
    MemoizedReuse,
    SnapshotRestoreReuse,
    ReconciliationAdoption,
    CrossIdentityPersistentReuse {
        correspondence_kind: PersistentCorrespondenceKind,
    },
    PartialArtifactSplice {
        composition_region_count: u32,
        recomputed_region_count: u32,
    },
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum SnapshotRestoreKind {
    CompactGlobal,
    PerNodeArtifact,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum InvalidationCause {
    SourceAspectChanged {
        aspect_index: usize,
    },
    DirectDependencyChanged {
        dependency: NodeId,
        aspect_index: usize,
    },
    TransitiveDependencyChanged {
        aspect_index: usize,
    },
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum LineageRecordKind {
    ArtifactTransition {
        node: NodeId,
        artifact_id: LineageArtifactId,
        parent_artifact_id: Option<LineageArtifactId>,
        execution_record_id: ExecutionRecordId,
        semantic_segment_id: SemanticSegmentId,
        transition: ArtifactTransitionKind,
    },
    BranchFork {
        created_branch_id: SignalBranchId,
        parent_branch_id: SignalBranchId,
        created_branch_display_name: String,
        parent_branch_display_name: String,
    },
    BranchSwitch {
        from_branch_id: SignalBranchId,
        to_branch_id: SignalBranchId,
        from_branch_display_name: String,
        to_branch_display_name: String,
    },
    BranchMerge {
        source_branch_id: SignalBranchId,
        target_branch_id: SignalBranchId,
        merge_kind: BranchMergeKind,
        divergence: BranchMergeDivergence,
        merge_strategy: BranchMergeStrategy,
        reconciliation_policy: BranchMergeReconciliationPolicy,
        resolution_plan: Option<BranchConflictResolutionPlan>,
        merged_snapshot_id: Option<SignalSnapshotId>,
        source_branch_display_name: String,
        target_branch_display_name: String,
    },
    ArtifactMerge {
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
    },
    SnapshotRestore {
        snapshot_id: SignalSnapshotId,
        node: Option<NodeId>,
        artifact_id: Option<LineageArtifactId>,
        restore_kind: SnapshotRestoreKind,
    },
    Invalidation {
        node: NodeId,
        artifact_id: LineageArtifactId,
        cause: InvalidationCause,
    },
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct LineageRecord {
    /// Monotonic lineage event order within a live runtime history.
    ///
    /// This is a runtime-local ordering clock used for canonical lineage
    /// ordering and replay/history stitching. It must stay unique and
    /// increasing within a live branched runtime, but it is not a claim about
    /// wall-clock order outside that runtime.
    pub sequence: u64,
    /// Branch context on which the lineage record was emitted.
    ///
    /// For branch-topology records, this is the active branch timeline that
    /// observed/emitted the event, not necessarily the same as the branch ids
    /// named inside the record kind payload.
    pub emitted_on_branch_id: SignalBranchId,
    pub kind: LineageRecordKind,
}

impl LineageRecord {
    pub fn new(
        sequence: u64,
        emitted_on_branch_id: SignalBranchId,
        kind: LineageRecordKind,
    ) -> Self {
        Self {
            sequence,
            emitted_on_branch_id,
            kind,
        }
    }

    #[allow(clippy::too_many_arguments)]
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

    #[allow(clippy::too_many_arguments)]
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

    pub fn snapshot_restore(
        sequence: u64,
        emitted_on_branch_id: SignalBranchId,
        snapshot_id: SignalSnapshotId,
        node: Option<NodeId>,
        artifact_id: Option<LineageArtifactId>,
        restore_kind: SnapshotRestoreKind,
    ) -> Self {
        Self::new(
            sequence,
            emitted_on_branch_id,
            LineageRecordKind::SnapshotRestore {
                snapshot_id,
                node,
                artifact_id,
                restore_kind,
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

    pub fn node(&self) -> Option<NodeId> {
        match &self.kind {
            LineageRecordKind::ArtifactTransition { node, .. }
            | LineageRecordKind::Invalidation { node, .. } => Some(*node),
            LineageRecordKind::ArtifactMerge { target_node, .. } => *target_node,
            LineageRecordKind::SnapshotRestore { node, .. } => *node,
            LineageRecordKind::BranchFork { .. }
            | LineageRecordKind::BranchSwitch { .. }
            | LineageRecordKind::BranchMerge { .. } => None,
        }
    }

    pub fn emitted_on_branch_id(&self) -> SignalBranchId {
        self.emitted_on_branch_id
    }

    pub fn subject_artifact_id(&self) -> Option<LineageArtifactId> {
        match &self.kind {
            LineageRecordKind::ArtifactTransition { artifact_id, .. }
            | LineageRecordKind::Invalidation { artifact_id, .. } => Some(*artifact_id),
            LineageRecordKind::ArtifactMerge {
                target_artifact_id_after,
                ..
            } => *target_artifact_id_after,
            LineageRecordKind::SnapshotRestore { artifact_id, .. } => *artifact_id,
            LineageRecordKind::BranchFork { .. }
            | LineageRecordKind::BranchSwitch { .. }
            | LineageRecordKind::BranchMerge { .. } => None,
        }
    }

    pub fn parent_artifact_id(&self) -> Option<LineageArtifactId> {
        match &self.kind {
            LineageRecordKind::ArtifactTransition {
                parent_artifact_id, ..
            } => *parent_artifact_id,
            LineageRecordKind::ArtifactMerge { .. } => None,
            LineageRecordKind::SnapshotRestore { .. } | LineageRecordKind::Invalidation { .. } => {
                None
            }
            LineageRecordKind::BranchFork { .. }
            | LineageRecordKind::BranchSwitch { .. }
            | LineageRecordKind::BranchMerge { .. } => None,
        }
    }

    pub fn restored_artifact_id(&self) -> Option<LineageArtifactId> {
        match &self.kind {
            LineageRecordKind::SnapshotRestore { artifact_id, .. } => *artifact_id,
            _ => None,
        }
    }

    pub fn invalidated_artifact_id(&self) -> Option<LineageArtifactId> {
        match &self.kind {
            LineageRecordKind::Invalidation { artifact_id, .. } => Some(*artifact_id),
            LineageRecordKind::ArtifactMerge {
                target_artifact_id_after,
                ..
            } => *target_artifact_id_after,
            _ => None,
        }
    }

    pub fn invalidation_cause(&self) -> Option<&InvalidationCause> {
        match &self.kind {
            LineageRecordKind::Invalidation { cause, .. } => Some(cause),
            _ => None,
        }
    }

    pub fn execution_record_id(&self) -> Option<ExecutionRecordId> {
        match &self.kind {
            LineageRecordKind::ArtifactTransition {
                execution_record_id,
                ..
            } => Some(*execution_record_id),
            _ => None,
        }
    }

    pub fn semantic_segment_id(&self) -> Option<SemanticSegmentId> {
        match &self.kind {
            LineageRecordKind::ArtifactTransition {
                semantic_segment_id,
                ..
            } => Some(*semantic_segment_id),
            _ => None,
        }
    }

    pub fn snapshot_id(&self) -> Option<SignalSnapshotId> {
        match &self.kind {
            LineageRecordKind::SnapshotRestore { snapshot_id, .. } => Some(*snapshot_id),
            _ => None,
        }
    }

    pub fn label(&self) -> &'static str {
        match self.kind {
            LineageRecordKind::ArtifactTransition {
                transition: ArtifactTransitionKind::Replaced,
                ..
            } => "Replaced",
            LineageRecordKind::ArtifactTransition {
                transition: ArtifactTransitionKind::Refreshed { .. },
                ..
            } => "Refreshed",
            LineageRecordKind::ArtifactTransition {
                transition: ArtifactTransitionKind::MemoizedReuse,
                ..
            } => "MemoizedReuse",
            LineageRecordKind::ArtifactTransition {
                transition: ArtifactTransitionKind::SnapshotRestoreReuse,
                ..
            } => "SnapshotRestoreReuse",
            LineageRecordKind::ArtifactTransition {
                transition: ArtifactTransitionKind::ReconciliationAdoption,
                ..
            } => "ReconciliationAdoption",
            LineageRecordKind::ArtifactTransition {
                transition: ArtifactTransitionKind::CrossIdentityPersistentReuse { .. },
                ..
            } => "CrossIdentityPersistentReuse",
            LineageRecordKind::ArtifactTransition {
                transition: ArtifactTransitionKind::PartialArtifactSplice { .. },
                ..
            } => "PartialArtifactSplice",
            LineageRecordKind::BranchFork { .. } => "BranchedFrom",
            LineageRecordKind::BranchSwitch { .. } => "BranchSwitched",
            LineageRecordKind::BranchMerge { .. } => "MergedFrom",
            LineageRecordKind::ArtifactMerge { .. } => "MergedArtifact",
            LineageRecordKind::SnapshotRestore { .. } => "Restored",
            LineageRecordKind::Invalidation { .. } => "InvalidatedWithoutReplacement",
        }
    }
}

#[derive(Debug, Clone, Copy, Default)]
pub struct RetainedLineageView<'a> {
    records: Option<&'a VecDeque<LineageRecord>>,
    offset: usize,
    len: usize,
}

impl<'a> RetainedLineageView<'a> {
    pub fn new(records: &'a VecDeque<LineageRecord>, offset: usize, len: usize) -> Self {
        Self {
            records: Some(records),
            offset,
            len,
        }
    }

    pub fn empty() -> Self {
        Self {
            records: None,
            offset: 0,
            len: 0,
        }
    }

    pub fn len(&self) -> usize {
        self.len
    }

    pub fn is_empty(&self) -> bool {
        self.len == 0
    }

    pub fn iter(&self) -> Box<dyn Iterator<Item = &'a LineageRecord> + 'a> {
        match self.records {
            Some(records) => Box::new(records.iter().skip(self.offset).take(self.len)),
            None => Box::new(std::iter::empty()),
        }
    }

    pub fn first(&self) -> Option<&'a LineageRecord> {
        self.iter().next()
    }

    pub fn last(&self) -> Option<&'a LineageRecord> {
        self.iter().last()
    }

    pub fn to_owned_records(&self) -> Vec<LineageRecord> {
        self.iter().cloned().collect()
    }
}

impl<'a> PartialEq for RetainedLineageView<'a> {
    fn eq(&self, other: &Self) -> bool {
        self.iter().eq(other.iter())
    }
}

impl<'a> Eq for RetainedLineageView<'a> {}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Default)]
pub struct SynthesizedLineageChain {
    records: Vec<LineageRecord>,
}

impl SynthesizedLineageChain {
    pub fn new(records: Vec<LineageRecord>) -> Self {
        Self { records }
    }

    pub fn len(&self) -> usize {
        self.records.len()
    }

    pub fn is_empty(&self) -> bool {
        self.records.is_empty()
    }

    pub fn iter(&self) -> impl Iterator<Item = &LineageRecord> {
        self.records.iter()
    }

    pub fn first(&self) -> Option<&LineageRecord> {
        self.records.first()
    }

    pub fn last(&self) -> Option<&LineageRecord> {
        self.records.last()
    }

    pub fn to_owned_records(&self) -> Vec<LineageRecord> {
        self.records.clone()
    }
}
