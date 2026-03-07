use serde::{Deserialize, Serialize};

use crate::data::identity::{SpecNodeId, SpecRelationId};

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct SpecReplayRecord {
    pub operation_id: u64,
    pub operation_name: String,
    pub schema_version: u32,
    pub parameters: Vec<u8>,
    pub pre_hash: u128,
    pub post_hash: u128,
    pub touched_nodes: Vec<SpecNodeId>,
    pub touched_relations: Vec<SpecRelationId>,
    pub mutation_trace: Vec<String>,
    pub projection_refresh_trace: Vec<String>,
    pub decision_summary: Option<String>,
}
