use serde::{Deserialize, Serialize};

use crate::history::data::BranchId;
use crate::identity::data::LineageId;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default, Serialize, Deserialize)]
pub struct HistoricalLineageResolutionMetrics {
    pub traversed_event_count: usize,
    pub branch_event_scan_count: usize,
    pub resolved_lineage_count: usize,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct HistoricalResolutionRequest {
    pub branch_id: BranchId,
    pub lineage_id: LineageId,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct RecordHistoryRequest {
    pub branch_id: BranchId,
    pub entity_id: crate::identity::data::EntityId,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct HistoricalResolutionTrace {
    pub traversed_event_ids: Vec<u64>,
    pub metrics: HistoricalLineageResolutionMetrics,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct HistoricalLineageResolution {
    pub branch_id: BranchId,
    pub start: LineageId,
    pub resolved: Vec<LineageId>,
    pub traversed_event_ids: Vec<u64>,
    pub trace: HistoricalResolutionTrace,
    pub metrics: HistoricalLineageResolutionMetrics,
}
