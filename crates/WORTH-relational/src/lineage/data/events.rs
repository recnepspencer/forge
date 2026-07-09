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
    pub(crate) event_id: u64,
    pub(crate) commit: CommitReference,
    pub(crate) branch_id: BranchId,
    pub(crate) kind: LineageEventKind,
    pub(crate) sources: Vec<LineageId>,
    pub(crate) targets: Vec<LineageId>,
}

impl LineageEventRecord {
    pub fn event_id(&self) -> u64 {
        self.event_id
    }

    pub fn commit(&self) -> &CommitReference {
        &self.commit
    }

    pub fn branch_id(&self) -> &BranchId {
        &self.branch_id
    }

    pub fn kind(&self) -> LineageEventKind {
        self.kind
    }

    pub fn sources(&self) -> &[LineageId] {
        &self.sources
    }

    pub fn targets(&self) -> &[LineageId] {
        &self.targets
    }
}
