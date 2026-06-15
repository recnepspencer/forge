use forge_relational::facade::identity::EntityId;

use crate::brep::topology_graph::TopologyView;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TopologySeedEntityIdentities {
    model_ids: Vec<EntityId>,
    body_ids: Vec<EntityId>,
    lump_ids: Vec<EntityId>,
    region_ids: Vec<EntityId>,
    shell_ids: Vec<EntityId>,
    face_ids: Vec<EntityId>,
    loop_ids: Vec<EntityId>,
    wire_ids: Vec<EntityId>,
    half_edge_ids: Vec<EntityId>,
    edge_ids: Vec<EntityId>,
    vertex_ids: Vec<EntityId>,
}

impl TopologySeedEntityIdentities {
    pub(crate) fn from_view(view: &TopologyView) -> Self {
        Self {
            model_ids: view.models.iter().map(|entity| entity.entity_id).collect(),
            body_ids: view.bodies.iter().map(|entity| entity.entity_id).collect(),
            lump_ids: view.lumps.iter().map(|entity| entity.entity_id).collect(),
            region_ids: view.regions.iter().map(|entity| entity.entity_id).collect(),
            shell_ids: view.shells.iter().map(|entity| entity.entity_id).collect(),
            face_ids: view.faces.iter().map(|entity| entity.entity_id).collect(),
            loop_ids: view.loops.iter().map(|entity| entity.entity_id).collect(),
            wire_ids: view.wires.iter().map(|entity| entity.entity_id).collect(),
            half_edge_ids: view
                .half_edges
                .iter()
                .map(|entity| entity.entity_id)
                .collect(),
            edge_ids: view.edges.iter().map(|entity| entity.entity_id).collect(),
            vertex_ids: view
                .vertices
                .iter()
                .map(|entity| entity.entity_id)
                .collect(),
        }
    }

    pub fn model_ids(&self) -> &[EntityId] {
        &self.model_ids
    }

    pub fn shell_ids(&self) -> &[EntityId] {
        &self.shell_ids
    }

    pub fn face_ids(&self) -> &[EntityId] {
        &self.face_ids
    }

    pub fn face_identity_tokens(&self) -> Vec<String> {
        entity_identity_tokens(&self.face_ids)
    }

    pub fn loop_ids(&self) -> &[EntityId] {
        &self.loop_ids
    }

    pub fn loop_identity_tokens(&self) -> Vec<String> {
        entity_identity_tokens(&self.loop_ids)
    }

    pub fn wire_ids(&self) -> &[EntityId] {
        &self.wire_ids
    }

    pub fn half_edge_ids(&self) -> &[EntityId] {
        &self.half_edge_ids
    }

    pub fn edge_ids(&self) -> &[EntityId] {
        &self.edge_ids
    }

    pub fn edge_identity_tokens(&self) -> Vec<String> {
        entity_identity_tokens(&self.edge_ids)
    }

    pub fn vertex_ids(&self) -> &[EntityId] {
        &self.vertex_ids
    }
}

fn entity_identity_tokens(ids: &[EntityId]) -> Vec<String> {
    ids.iter().copied().map(entity_identity_token).collect()
}

fn entity_identity_token(entity: EntityId) -> String {
    format!(
        "entity:{}:{}:{}",
        entity.partition_value(),
        entity.local_slot_value(),
        entity.generation_value()
    )
}
