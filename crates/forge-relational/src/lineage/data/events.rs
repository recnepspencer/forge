use serde::{Deserialize, Serialize};

use crate::history::data::{BranchId, CommitReference};
use crate::identity::data::LineageId;

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
pub struct LineageEventRecord {
    pub event_id: u64,
    pub commit: CommitReference,
    pub branch_id: BranchId,
    pub kind: LineageEventKind,
    pub sources: Vec<LineageId>,
    pub targets: Vec<LineageId>,
}
