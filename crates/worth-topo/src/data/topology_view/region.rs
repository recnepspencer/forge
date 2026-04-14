use forge_relational::facade::identity::EntityId;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct WorthTopologyRegion {
    pub entity_id: EntityId,
    pub label: String,
    pub lump_id: Option<EntityId>,
    pub shell_ids: Vec<EntityId>,
}
