use serde::{Deserialize, Serialize};

use crate::data::handle::NodeId;
use crate::logic::planner::{ExecutionRecordId, SemanticSegmentId};
use crate::state::{SignalBranchId, SignalSnapshotId};

#[derive(
    Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize, Default,
)]
pub struct LineageArtifactId(pub u64);

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum LineageEvent {
    Refreshed,
    Replaced,
    Restored,
    BranchedFrom,
    BranchSwitched,
    MergedFrom,
    MemoizedFrom,
    InvalidatedWithoutReplacement,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct LineageRecord {
    pub sequence: u64,
    pub branch_id: SignalBranchId,
    pub node: Option<NodeId>,
    pub artifact_id: Option<LineageArtifactId>,
    pub parent_artifact_id: Option<LineageArtifactId>,
    pub event: LineageEvent,
    pub execution_record_id: Option<ExecutionRecordId>,
    pub semantic_segment_id: Option<SemanticSegmentId>,
    pub snapshot_id: Option<SignalSnapshotId>,
    pub detail: Option<String>,
}

impl LineageRecord {
    #[allow(clippy::too_many_arguments)]
    pub fn new(
        sequence: u64,
        branch_id: SignalBranchId,
        node: Option<NodeId>,
        artifact_id: Option<LineageArtifactId>,
        parent_artifact_id: Option<LineageArtifactId>,
        event: LineageEvent,
        execution_record_id: Option<ExecutionRecordId>,
        semantic_segment_id: Option<SemanticSegmentId>,
        snapshot_id: Option<SignalSnapshotId>,
        detail: Option<String>,
    ) -> Self {
        Self {
            sequence,
            branch_id,
            node,
            artifact_id,
            parent_artifact_id,
            event,
            execution_record_id,
            semantic_segment_id,
            snapshot_id,
            detail,
        }
    }
}
