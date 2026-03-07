use serde::{Deserialize, Serialize};

use crate::data::identity::{NamingAnchorId, SpecNodeId};
use crate::data::schema::SpecNodeKind;

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct NamingAnchor {
    pub id: NamingAnchorId,
    pub target: SpecNodeId,
    pub target_kind: SpecNodeKind,
    pub semantic_role: String,
    pub ordinal: u32,
    pub origin_feature: Option<SpecNodeId>,
    pub origin_operation: u64,
    pub retarget_history: Vec<SpecNodeId>,
}
