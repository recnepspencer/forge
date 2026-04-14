use forge_relational::facade::identity::EntityId;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct WorthTopologyShell {
    pub entity_id: EntityId,
    pub label: String,
    pub region_id: Option<EntityId>,
    pub face_ids: Vec<EntityId>,
}
