use forge_relational::facade::identity::EntityId;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct TopologyHalfEdge {
    pub entity_id: EntityId,
    pub label: String,
    pub loop_id: Option<EntityId>,
    pub wire_id: Option<EntityId>,
    pub next_half_edge_id: Option<EntityId>,
    pub prev_half_edge_id: Option<EntityId>,
    pub radial_next_half_edge_id: Option<EntityId>,
    pub edge_id: Option<EntityId>,
    pub origin_vertex_id: Option<EntityId>,
    pub target_vertex_id: Option<EntityId>,
    pub face_id: Option<EntityId>,
}




