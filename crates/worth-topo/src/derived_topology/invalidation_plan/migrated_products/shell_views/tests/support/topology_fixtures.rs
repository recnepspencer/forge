use super::identity_slots::entity_id;
use super::selected_plans::selected_shell_views_plan;
use super::touched_closures::selected_shell_view_touched_closure;
use crate::brep::topology_graph::{TopologyFace, TopologyHalfEdge, TopologyShell, TopologyView};
use crate::derived_topology::invalidation_plan::migrated_products::shell_views::{
    ShellViewBoundarySourceRow, ShellViewReadSource,
};

pub(crate) fn selected_shell_view_read_source() -> ShellViewReadSource {
    let plan = selected_shell_views_plan("shell-boundary-touch");
    let touched_closure = selected_shell_view_touched_closure("shell-boundary-touch");
    let topology = selected_shell_view_topology_with_unrelated_shells();
    ShellViewReadSource::select_from_touched_closure(&plan, &touched_closure, &topology).unwrap()
}

pub(crate) fn selected_shell_view_topology_with_unrelated_shells() -> TopologyView {
    TopologyView {
        half_edges: vec![
            radial_half_edge(24, 240, 25, 500),
            radial_half_edge(25, 240, 24, 500),
            radial_half_edge(99, 990, 100, 900),
            radial_half_edge(100, 990, 101, 900),
            radial_half_edge(101, 990, 99, 900),
            radial_half_edge(1_000, 1_000, 1_000, 901),
        ],
        faces: vec![
            face(500, 500, &[24, 25]),
            face(900, 900, &[99, 100, 101]),
            face(901, 901, &[1_000]),
        ],
        shells: vec![shell(500, &[500]), shell(900, &[900]), shell(901, &[901])],
        ..TopologyView::default()
    }
}

pub(crate) fn selected_shell_view_topology_with_many_unrelated_shells(
    unrelated_ring_count: usize,
) -> TopologyView {
    let mut topology = selected_shell_view_topology_with_unrelated_shells();
    for index in 0..unrelated_ring_count {
        let slot = 2_000 + index as u64;
        topology
            .half_edges
            .push(radial_half_edge(slot, slot + 10_000, slot, slot + 20_000));
        topology
            .faces
            .push(face(slot + 20_000, slot + 20_000, &[slot]));
        topology.shells.push(shell(slot + 20_000, &[slot + 20_000]));
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
) -> ShellViewBoundarySourceRow {
    ShellViewBoundarySourceRow::new(
        entity_label(500),
        entity_label(half_edge_slot),
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

fn shell(shell_slot: u64, face_slots: &[u64]) -> TopologyShell {
    TopologyShell {
        entity_id: entity_id(shell_slot),
        label: format!("shell-{shell_slot}"),
        region_id: None,
        face_ids: face_slots.iter().copied().map(entity_id).collect(),
    }
}

fn face(shell_slot: u64, face_slot: u64, boundary_half_edge_slots: &[u64]) -> TopologyFace {
    TopologyFace {
        entity_id: entity_id(face_slot),
        label: format!("face-{face_slot}"),
        shell_id: Some(entity_id(shell_slot)),
        outer_loop_id: None,
        inner_loop_ids: Vec::new(),
        boundary_half_edge_ids: boundary_half_edge_slots
            .iter()
            .copied()
            .map(entity_id)
            .collect(),
    }
}

fn radial_half_edge(
    slot: u64,
    edge_slot: u64,
    radial_next_slot: u64,
    shell_slot: u64,
) -> TopologyHalfEdge {
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
        face_id: Some(entity_id(shell_slot)),
    }
}
