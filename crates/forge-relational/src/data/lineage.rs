use serde::{Deserialize, Serialize};

use crate::data::identity::{EntityId, LineageId};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum LineageEventKind {
    Replace,
    Split,
    Merge,
    Correspond,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct LineageEvent {
    pub lineage_id: LineageId,
    pub kind: LineageEventKind,
    pub sources: Vec<EntityId>,
    pub targets: Vec<EntityId>,
}
