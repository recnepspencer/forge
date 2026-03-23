use serde::{Deserialize, Serialize};

use crate::history::data::BranchId;
use crate::identity::data::{EntityId, LineageId};
use crate::lineage::data::{CorrespondenceCandidate, LineageEventRecord};

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct LineageNode {
    pub lineage_id: LineageId,
    pub entity_id: EntityId,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct LineageGraphSnapshot {
    pub branch_id: BranchId,
    pub nodes: Vec<LineageNode>,
    pub events: Vec<LineageEventRecord>,
    pub correspondence_candidates: Vec<CorrespondenceCandidate>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct LineageGraphRequest {
    pub branch_id: BranchId,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default, Serialize, Deserialize)]
pub struct LineageDivergenceMetrics {
    pub left_event_count: usize,
    pub right_event_count: usize,
    pub left_node_count: usize,
    pub right_node_count: usize,
    pub shared_lineage_count: usize,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct LineageDivergenceSummary {
    pub left_branch: BranchId,
    pub right_branch: BranchId,
    pub left_only_event_ids: Vec<u64>,
    pub right_only_event_ids: Vec<u64>,
    pub shared_lineage_ids: Vec<LineageId>,
    pub metrics: LineageDivergenceMetrics,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct LineageDivergenceRequest {
    pub left_branch: BranchId,
    pub right_branch: BranchId,
}
