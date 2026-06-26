use super::identity_slots::entity_id;
use super::selected_plans::selected_loop_cycles_plan;
use super::touched_closures::selected_loop_cycle_touched_closure;
use crate::brep::topology_graph::{TopologyFace, TopologyShell, TopologyView};
use crate::derived_topology::invalidation_plan::migrated_products::loop_cycles::{
    LoopCycleBoundarySourceRow, LoopCycleReadSource,
};

pub(crate) fn selected_loop_cycle_read_source() -> LoopCycleReadSource {
    let plan = selected_loop_cycles_plan("loop-touch");
    let touched_closure = selected_loop_cycle_touched_closure("loop-touch");
    let topology =
        crate::test_support::hostile_neighborhoods::interpretation_neighborhoods::open_shell_nmt_fan_view(4);
    LoopCycleReadSource::select_from_touched_closure(&plan, &touched_closure, &topology).unwrap()
}

pub(crate) fn selected_loop_cycle_topology_with_unrelated_shells() -> TopologyView {
    TopologyView {
        shells: vec![
            topology_shell_with_faces(24, &[240, 241]),
            topology_shell_with_faces(99, &[990, 991]),
            topology_shell_with_faces(100, &[1000, 1001]),
        ],
        faces: vec![
            topology_face_with_boundary(240, 24, 3),
            topology_face_with_boundary(241, 24, 2),
            topology_face_with_boundary(990, 99, 9),
            topology_face_with_boundary(991, 99, 9),
            topology_face_with_boundary(1000, 100, 9),
            topology_face_with_boundary(1001, 100, 9),
        ],
        ..TopologyView::default()
    }
}

pub(crate) fn selected_loop_cycle_topology_with_many_unrelated_shells(
    unrelated_shell_count: usize,
) -> TopologyView {
    let mut topology = selected_loop_cycle_topology_with_unrelated_shells();
    for index in 0..unrelated_shell_count {
        let shell_slot = 1_000 + index as u64;
        let face_slot = 20_000 + index as u64;
        topology
            .shells
            .push(topology_shell_with_faces(shell_slot, &[face_slot]));
        topology
            .faces
            .push(topology_face_with_boundary(face_slot, shell_slot, 4));
    }
    topology
}

pub(crate) fn source_row(
    slot: u64,
    boundary_component_count: usize,
    boundary_half_edge_count: usize,
) -> LoopCycleBoundarySourceRow {
    LoopCycleBoundarySourceRow::new(
        entity_id(slot),
        boundary_component_count,
        boundary_half_edge_count,
    )
}

fn topology_shell_with_faces(slot: u64, face_slots: &[u64]) -> TopologyShell {
    TopologyShell {
        entity_id: entity_id(slot),
        label: format!("shell-{slot}"),
        region_id: None,
        face_ids: face_slots
            .iter()
            .copied()
            .map(entity_id)
            .collect::<Vec<_>>(),
    }
}

fn topology_face_with_boundary(
    slot: u64,
    shell_slot: u64,
    boundary_half_edges: usize,
) -> TopologyFace {
    TopologyFace {
        entity_id: entity_id(slot),
        label: format!("face-{slot}"),
        shell_id: Some(entity_id(shell_slot)),
        outer_loop_id: None,
        inner_loop_ids: Vec::new(),
        boundary_half_edge_ids: (0..boundary_half_edges)
            .map(|index| entity_id(slot * 100 + index as u64))
            .collect(),
    }
}
