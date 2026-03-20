use serde::{Deserialize, Serialize};

use crate::state::{SignalBranchHandle, SignalBranchId, SignalSnapshotId};

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct BranchMergeRequest {
    pub source_branch: SignalBranchHandle,
    pub target_branch: SignalBranchHandle,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum BranchMergeKind {
    FastForward,
    Applied,
    ConflictResolved,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum BranchMergeStrategy {
    AdoptSourceHead,
    AdoptSourceSubset,
    ReplaySourceDeltaOntoTarget,
    RebaseSourceOntoTarget,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum BranchMergeDivergence {
    None,
    TargetAdvanced,
    SharedStateConflict,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum BranchMergeFailureKind {
    SelfMergeRejected,
    UnknownSourceBranch,
    UnknownTargetBranch,
    MissingMergeBase,
    DivergenceRequiresConflictResolution,
    UnsupportedMergeStrategy,
    UnresolvedDependencyRemap,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum MergeCandidateScope {
    WholeLiveAuthoritySurface,
    CandidateNodeSet(Vec<crate::data::handle::NodeId>),
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct BranchMergeBase {
    pub source_branch_id: SignalBranchId,
    pub target_branch_id: SignalBranchId,
    pub forked_from_snapshot_id: Option<SignalSnapshotId>,
    pub source_snapshot_id: Option<SignalSnapshotId>,
    pub target_snapshot_id_before: Option<SignalSnapshotId>,
}
