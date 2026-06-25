use super::identity_slots::entity_id;
use super::selected_plans::selected_radial_rings_plan;
use super::touched_closures::selected_radial_ring_touched_closure;
use crate::brep::topology_graph::{TopologyHalfEdge, TopologyView};
use crate::derived_topology::invalidation_plan::migrated_products::radial_rings::{
    RadialRingBoundarySourceRow, RadialRingReadSource,
};

pub(crate) fn selected_radial_ring_read_source() -> RadialRingReadSource {
    let plan = selected_radial_rings_plan("radial-touch");
    let touched_closure = selected_radial_ring_touched_closure("radial-touch");
    let topology = selected_radial_ring_topology_with_unrelated_shells();
    RadialRingReadSource::select_from_touched_closure(&plan, &touched_closure, &topology).unwrap()
}

pub(crate) fn selected_radial_ring_topology_with_unrelated_shells() -> TopologyView {
    TopologyView {
        half_edges: vec![
            radial_half_edge(24, 240, 25),
            radial_half_edge(25, 240, 24),
            radial_half_edge(99, 990, 100),
            radial_half_edge(100, 990, 101),
            radial_half_edge(101, 990, 99),
            radial_half_edge(1_000, 1_000, 1_000),
        ],
        ..TopologyView::default()
    }
}

pub(crate) fn selected_radial_ring_topology_with_many_unrelated_shells(
    unrelated_ring_count: usize,
) -> TopologyView {
    let mut topology = selected_radial_ring_topology_with_unrelated_shells();
    for index in 0..unrelated_ring_count {
        let slot = 2_000 + index as u64;
        topology
            .half_edges
            .push(radial_half_edge(slot, slot + 10_000, slot));
    }
    topology
}

pub(crate) fn source_row(
    half_edge_slot: u64,
    edge_slot: u64,
    radial_target_slot: u64,
    ring_half_edge_count: usize,
    boundary_half_edge: bool,
    non_manifold_edge: bool,
) -> RadialRingBoundarySourceRow {
    RadialRingBoundarySourceRow::new(
        entity_label(half_edge_slot),
        entity_label(edge_slot),
        entity_label(radial_target_slot),
        entity_label(edge_slot),
        format!("relation:0:{}:1", half_edge_slot + 50_000),
        ring_half_edge_count,
        boundary_half_edge,
        non_manifold_edge,
    )
}

pub(crate) fn entity_label(slot: u64) -> String {
    format!("entity:0:{slot}:1")
}

fn radial_half_edge(slot: u64, edge_slot: u64, radial_next_slot: u64) -> TopologyHalfEdge {
    TopologyHalfEdge {
        entity_id: entity_id(slot),
        label: format!("half-edge-{slot}"),
        loop_id: None,
        wire_id: None,
        next_half_edge_id: None,
        prev_half_edge_id: None,
        radial_next_half_edge_id: Some(entity_id(radial_next_slot)),
        edge_id: Some(entity_id(edge_slot)),
        origin_vertex_id: None,
        target_vertex_id: None,
        face_id: None,
    }
}
