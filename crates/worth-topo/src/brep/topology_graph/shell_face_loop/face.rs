use forge_relational::facade::identity::EntityId;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct TopologyFace {
    pub entity_id: EntityId,
    pub label: String,
    pub shell_id: Option<EntityId>,
    pub outer_loop_id: Option<EntityId>,
    pub inner_loop_ids: Vec<EntityId>,
    pub boundary_half_edge_ids: Vec<EntityId>,
}




