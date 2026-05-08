use forge_relational::facade::identity::EntityId;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct TopologyLump {
    pub entity_id: EntityId,
    pub label: String,
    pub body_id: Option<EntityId>,
    pub region_ids: Vec<EntityId>,
}
