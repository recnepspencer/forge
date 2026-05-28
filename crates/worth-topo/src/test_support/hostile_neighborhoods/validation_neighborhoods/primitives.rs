use forge_relational::facade::identity::{EntityId, PartitionId};

use crate::brep::topology_graph::{TopologyEdge, TopologyHalfEdge, TopologyVertex};
pub(crate) fn entity(slot: u64) -> EntityId {
    EntityId::new(PartitionId::main(), slot, 1)
}

pub(crate) fn edge(label: &str, entity_id: EntityId) -> TopologyEdge {
    TopologyEdge {
        entity_id,
        label: label.into(),
    }
}

pub(crate) fn vertex(label: &str, entity_id: EntityId) -> TopologyVertex {
    TopologyVertex {
        entity_id,
        label: label.into(),
    }
}

pub(crate) fn half_edge(
    entity_id: EntityId,
    label: &str,
    wire_id: Option<EntityId>,
    edge_id: Option<EntityId>,
    origin_vertex_id: Option<EntityId>,
    target_vertex_id: Option<EntityId>,
    face_id: Option<EntityId>,
) -> TopologyHalfEdge {
    half_edge_with_links(
        entity_id,
        label,
        None,
        wire_id,
        Some(entity_id),
        Some(entity_id),
        Some(entity_id),
        edge_id,
        origin_vertex_id,
        target_vertex_id,
        face_id,
    )
}

pub(crate) fn half_edge_with_links(
    entity_id: EntityId,
    label: &str,
    loop_id: Option<EntityId>,
    wire_id: Option<EntityId>,
    next_half_edge_id: Option<EntityId>,
    prev_half_edge_id: Option<EntityId>,
    radial_next_half_edge_id: Option<EntityId>,
    edge_id: Option<EntityId>,
    origin_vertex_id: Option<EntityId>,
    target_vertex_id: Option<EntityId>,
    face_id: Option<EntityId>,
) -> TopologyHalfEdge {
    TopologyHalfEdge {
        entity_id,
        label: label.into(),
        loop_id,
        wire_id,
        next_half_edge_id,
        prev_half_edge_id,
        radial_next_half_edge_id,
        edge_id,
        origin_vertex_id,
        target_vertex_id,
        face_id,
    }
}




