use forge_relational::facade::identity::EntityId;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TopologySeedNeighborhoodReceipt {
    center_vertex_id: EntityId,
    incident_half_edge_ids: Vec<EntityId>,
}

impl TopologySeedNeighborhoodReceipt {
    pub(crate) fn new(center_vertex_id: EntityId, incident_half_edge_ids: Vec<EntityId>) -> Self {
        Self {
            center_vertex_id,
            incident_half_edge_ids,
        }
    }

    pub fn center_vertex_id(&self) -> EntityId {
        self.center_vertex_id
    }

    pub fn incident_half_edge_ids(&self) -> &[EntityId] {
        &self.incident_half_edge_ids
    }

    pub fn valence(&self) -> usize {
        self.incident_half_edge_ids.len()
    }
}
