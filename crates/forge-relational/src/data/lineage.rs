use serde::{Deserialize, Serialize};

use crate::data::history::{BranchId, CommitReference};
use crate::data::identity::{EntityId, LineageId};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum LineageEventKind {
    Create,
    Replace,
    Split,
    Merge,
    Retire,
    Correspond,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct LineageNode {
    pub lineage_id: LineageId,
    pub entity_id: EntityId,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct LineageEventRecord {
    pub event_id: u64,
    pub commit: CommitReference,
    pub branch_id: BranchId,
    pub kind: LineageEventKind,
    pub sources: Vec<LineageId>,
    pub targets: Vec<LineageId>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum LineageInvariant {
    ValidReferencesOnly,
    NoSelfCycle,
    AdvisoryCorrespondenceNotPromoted,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum LineageResolutionStatus {
    Advisory,
    Promoted,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct CorrespondenceCandidate {
    pub candidate_id: u64,
    pub branch_id: BranchId,
    pub sources: Vec<LineageId>,
    pub targets: Vec<LineageId>,
    pub note: String,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct CorrespondenceResolution {
    pub candidate_id: u64,
    pub status: LineageResolutionStatus,
    pub promoted_event_id: Option<u64>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct LineageGraphSnapshot {
    pub branch_id: BranchId,
    pub nodes: Vec<LineageNode>,
    pub events: Vec<LineageEventRecord>,
    pub correspondence_candidates: Vec<CorrespondenceCandidate>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct LineageDivergenceSummary {
    pub left_branch: BranchId,
    pub right_branch: BranchId,
    pub left_only_event_ids: Vec<u64>,
    pub right_only_event_ids: Vec<u64>,
    pub shared_lineage_ids: Vec<LineageId>,
}
