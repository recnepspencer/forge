use forge_relational::facade::identity::EntityId;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct TopologyWire {
    pub entity_id: EntityId,
    pub label: String,
    pub half_edge_ids: Vec<EntityId>,
}




