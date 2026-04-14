use forge_relational::facade::identity::EntityId;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct WorthTopologyLoop {
    pub entity_id: EntityId,
    pub label: String,
    pub face_ids: Vec<EntityId>,
    pub half_edge_ids: Vec<EntityId>,
}
