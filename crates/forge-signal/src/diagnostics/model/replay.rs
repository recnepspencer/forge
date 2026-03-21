use serde::{Deserialize, Serialize};

use crate::data::handle::NodeId;
use crate::data::reuse::{PersistentCorrespondenceKind, ReuseOrigin};
use crate::diagnostics::lineage::LineageArtifactId;
use crate::state::{SignalBranchId, SignalSnapshotId};

#[derive(
    Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize, Default,
)]
pub struct ReplayCursor(pub u64);

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
pub enum ReplayEventKind {
    TaskApplied,
    TransactionCommitted,
    TransactionRolledBack,
    FailureRecorded,
    SnapshotCaptured,
    SnapshotRestored,
    BranchCreated,
    BranchSwitched,
    BranchMerged,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ReplayEvent {
    pub cursor: ReplayCursor,
    pub kind: ReplayEventKind,
    pub branch_id: SignalBranchId,
    pub snapshot_id: Option<SignalSnapshotId>,
    pub node: Option<NodeId>,
    pub execution_record_id: Option<u64>,
    pub semantic_segment_id: Option<u64>,
    pub lineage_artifact_id: Option<LineageArtifactId>,
    pub reuse_origin: Option<ReuseOrigin>,
    pub persistent_correspondence_kind: Option<PersistentCorrespondenceKind>,
    pub detail: Option<String>,
}

impl ReplayEvent {
    #[allow(clippy::too_many_arguments)]
    pub fn new(
        cursor: ReplayCursor,
        kind: ReplayEventKind,
        branch_id: SignalBranchId,
        snapshot_id: Option<SignalSnapshotId>,
        node: Option<NodeId>,
        execution_record_id: Option<u64>,
        semantic_segment_id: Option<u64>,
        lineage_artifact_id: Option<LineageArtifactId>,
        reuse_origin: Option<ReuseOrigin>,
        persistent_correspondence_kind: Option<PersistentCorrespondenceKind>,
        detail: Option<String>,
    ) -> Self {
        Self {
            cursor,
            kind,
            branch_id,
            snapshot_id,
            node,
            execution_record_id,
            semantic_segment_id,
            lineage_artifact_id,
            reuse_origin,
            persistent_correspondence_kind,
            detail,
        }
    }
}

pub type ReplayFrame = ReplayEvent;

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Default)]
pub struct ReplaySlice {
    pub start: Option<ReplayCursor>,
    pub end: Option<ReplayCursor>,
    pub frames: Vec<ReplayFrame>,
}
