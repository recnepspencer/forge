use forge_relational::facade::identity::EntityId;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct WorthTopologyBody {
    pub entity_id: EntityId,
    pub label: String,
    pub model_id: Option<EntityId>,
    pub lump_ids: Vec<EntityId>,
}
