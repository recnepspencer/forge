use serde::{Deserialize, Serialize};

use crate::data::identity::SpecNodeId;

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct LineageRecord {
    pub node: SpecNodeId,
    pub producing_feature: Option<SpecNodeId>,
    pub creation_operation: u64,
    pub parent_nodes: Vec<SpecNodeId>,
    pub ancestry_hash: u128,
    pub derivation_role: Option<String>,
}
