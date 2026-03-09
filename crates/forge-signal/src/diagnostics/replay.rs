use serde::{Deserialize, Serialize};

use crate::data::handle::NodeId;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
pub enum ReplayEventKind {
    TaskApplied,
    TransactionCommitted,
    TransactionRolledBack,
    FailureRecorded,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ReplayEvent {
    pub sequence: u64,
    pub kind: ReplayEventKind,
    pub node: Option<NodeId>,
    pub execution_record_id: Option<u64>,
    pub semantic_segment_id: Option<u64>,
    pub detail: Option<String>,
}

impl ReplayEvent {
    pub fn new(
        sequence: u64,
        kind: ReplayEventKind,
        node: Option<NodeId>,
        execution_record_id: Option<u64>,
        semantic_segment_id: Option<u64>,
        detail: Option<String>,
    ) -> Self {
        Self {
            sequence,
            kind,
            node,
            execution_record_id,
            semantic_segment_id,
            detail,
        }
    }
}
