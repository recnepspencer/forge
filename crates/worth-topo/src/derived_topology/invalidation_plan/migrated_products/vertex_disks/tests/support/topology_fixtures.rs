use super::identity_slots::entity_id;
use super::selected_plans::selected_vertex_disks_plan;
use super::touched_closures::selected_vertex_disk_touched_closure;
use crate::brep::topology_graph::{TopologyHalfEdge, TopologyView};
use crate::derived_topology::invalidation_plan::migrated_products::vertex_disks::{
    VertexDiskBoundarySourceRow, VertexDiskReadSource,
};

pub(crate) fn selected_vertex_disk_read_source() -> VertexDiskReadSource {
    let plan = selected_vertex_disks_plan("vertex-disk-local-touch");
    let touched_closure = selected_vertex_disk_touched_closure("vertex-disk-local-touch");
    let topology = selected_vertex_disk_topology_with_unrelated_disks();
    VertexDiskReadSource::select_from_touched_closure(&plan, &touched_closure, &topology).unwrap()
}

pub(crate) fn selected_vertex_disk_topology_with_unrelated_disks() -> TopologyView {
    TopologyView {
        half_edges: vec![
            half_edge(24, 10, 11, 100),
            half_edge(25, 10, 12, 101),
            half_edge(26, 10, 13, 102),
            half_edge(90, 90, 91, 900),
            half_edge(91, 91, 92, 901),
            half_edge(92, 92, 93, 902),
        ],
        ..TopologyView::default()
    }
}

pub(crate) fn vertex_disk_source_row(
    touched_vertex_slots: &[u64],
    source_half_edge_slot: u64,
    source_edge_slot: u64,
    incident_half_edge_slots: &[u64],
    different_edge_half_edge_slots: &[u64],
    touched_incident_edge_slots: &[u64],
) -> VertexDiskBoundarySourceRow {
    VertexDiskBoundarySourceRow::new(
        touched_vertex_slots
            .iter()
            .copied()
            .map(entity_label)
            .collect(),
        entity_label(source_half_edge_slot),
        entity_label(source_half_edge_slot),
        entity_label(source_edge_slot),
        incident_half_edge_slots
            .iter()
            .copied()
            .map(entity_label)
            .collect(),
        different_edge_half_edge_slots
            .iter()
            .copied()
            .map(entity_label)
            .collect(),
        touched_incident_edge_slots
            .iter()
            .copied()
            .map(entity_label)
            .collect(),
    )
}

fn half_edge(
    half_edge_slot: u64,
    origin_vertex_slot: u64,
    target_vertex_slot: u64,
    edge_slot: u64,
) -> TopologyHalfEdge {
    TopologyHalfEdge {
        entity_id: entity_id(half_edge_slot),
        label: format!("half-edge-{half_edge_slot}"),
        loop_id: None,
        wire_id: None,
        next_half_edge_id: None,
        prev_half_edge_id: None,
        radial_next_half_edge_id: None,
        edge_id: Some(entity_id(edge_slot)),
        origin_vertex_id: Some(entity_id(origin_vertex_slot)),
        target_vertex_id: Some(entity_id(target_vertex_slot)),
        face_id: None,
    }
}

fn entity_label(slot: u64) -> String {
    format!("entity:0:{slot}:1")
}
