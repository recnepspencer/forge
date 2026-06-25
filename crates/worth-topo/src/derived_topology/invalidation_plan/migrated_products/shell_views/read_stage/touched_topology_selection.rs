use std::collections::BTreeSet;

use forge_relational::facade::identity::EntityId;

use super::ShellViewReadStageCounters;
use crate::brep::topology_graph::{TopologyHalfEdge, TopologyView};
use crate::derived_topology::invalidation_plan::migrated_products::shell_views::ShellViewBoundarySourceRow;
use crate::derived_topology::invalidation_plan::selection::DerivedInvalidationTouchedClosure;

pub(super) struct ShellViewTouchedTopologySelection {
    rows: Vec<ShellViewBoundarySourceRow>,
    counters: ShellViewReadStageCounters,
}

impl ShellViewTouchedTopologySelection {
    pub(super) fn from_touched_closure_and_topology(
        touched_closure: &DerivedInvalidationTouchedClosure,
        topology: &TopologyView,
    ) -> Self {
        let touched_entities = touched_shell_view_anchor_entity_set(touched_closure);
        let selected = selected_shell_view_rows_from_touched_topology(topology, &touched_entities);
        let selected_source_row_count = selected.rows.len();
        let counters = ShellViewReadStageCounters::new(
            touched_entities.len(),
            selected.half_edge_lookup_count,
            selected.radial_relation_lookup_count,
            selected.selected_radial_root_count,
            selected_source_row_count,
            selected_source_row_count,
            selected.touched_neighborhood_breadth_count,
            topology
                .half_edges
                .len()
                .saturating_sub(selected_source_row_count),
            0,
        );
        Self {
            rows: selected.rows,
            counters,
        }
    }

    pub(super) fn into_rows_and_counters(
        self,
    ) -> (Vec<ShellViewBoundarySourceRow>, ShellViewReadStageCounters) {
        (self.rows, self.counters)
    }
}

struct SelectedShellViewRows {
    rows: Vec<ShellViewBoundarySourceRow>,
    half_edge_lookup_count: usize,
    radial_relation_lookup_count: usize,
    selected_radial_root_count: usize,
    touched_neighborhood_breadth_count: usize,
}

fn selected_shell_view_rows_from_touched_topology(
    topology: &TopologyView,
    touched_entities: &BTreeSet<EntityId>,
) -> SelectedShellViewRows {
    let mut rows = Vec::new();
    let mut half_edge_lookup_count = 0;
    let mut radial_relation_lookup_count = 0;
    let mut touched_neighborhood_breadth_count = 0;
    for touched_entity in touched_entities {
        half_edge_lookup_count += 1;
        let Some(half_edge) = topology
            .half_edges
            .iter()
            .find(|half_edge| half_edge.entity_id == *touched_entity)
        else {
            continue;
        };
        radial_relation_lookup_count += 1;
        let ring_half_edge_count = walk_selected_shell_view_len(half_edge, &topology.half_edges);
        touched_neighborhood_breadth_count += ring_half_edge_count;
        let boundary_half_edge = half_edge.radial_next_half_edge_id == Some(half_edge.entity_id);
        let non_manifold_edge = half_edge.edge_id.is_some() && ring_half_edge_count > 2;
        let touched_shell_identity = shell_identity_for_half_edge(topology, half_edge)
            .map(entity_identity)
            .unwrap_or_else(|| "shell:unknown".to_string());
        let touched_source_identity = entity_identity(half_edge.entity_id);
        rows.push(ShellViewBoundarySourceRow::new(
            touched_shell_identity,
            touched_source_identity,
            entity_identity(half_edge.entity_id),
            half_edge.edge_id.map(entity_identity).unwrap_or_default(),
            half_edge
                .radial_next_half_edge_id
                .map(entity_identity)
                .unwrap_or_default(),
            half_edge.edge_id.map(entity_identity).unwrap_or_default(),
            format!("relation:0:{}:1", half_edge.entity_id.local_slot.0 + 50_000),
            ring_half_edge_count,
            boundary_half_edge,
            non_manifold_edge,
        ));
    }
    SelectedShellViewRows {
        selected_radial_root_count: rows.len(),
        rows,
        half_edge_lookup_count,
        radial_relation_lookup_count,
        touched_neighborhood_breadth_count,
    }
}

fn shell_identity_for_half_edge(
    topology: &TopologyView,
    half_edge: &TopologyHalfEdge,
) -> Option<EntityId> {
    if let Some(face_id) = half_edge.face_id {
        if let Some(shell_id) = topology
            .faces
            .iter()
            .find(|face| face.entity_id == face_id)
            .and_then(|face| face.shell_id)
        {
            return Some(shell_id);
        }
        return topology
            .shells
            .iter()
            .find(|shell| shell.face_ids.contains(&face_id))
            .map(|shell| shell.entity_id);
    }
    None
}

fn entity_identity(entity_id: EntityId) -> String {
    format!(
        "entity:{}:{}:{}",
        entity_id.partition_id.0, entity_id.local_slot.0, entity_id.generation.0
    )
}

fn walk_selected_shell_view_len(
    start: &TopologyHalfEdge,
    half_edges: &[TopologyHalfEdge],
) -> usize {
    let mut current_id = start.entity_id;
    let mut seen = BTreeSet::new();
    loop {
        if !seen.insert(current_id) {
            return seen.len();
        }
        let Some(current) = half_edges
            .iter()
            .find(|half_edge| half_edge.entity_id == current_id)
        else {
            return seen.len();
        };
        let Some(next_id) = current.radial_next_half_edge_id else {
            return seen.len();
        };
        current_id = next_id;
    }
}

fn touched_shell_view_anchor_entity_set(
    touched_closure: &DerivedInvalidationTouchedClosure,
) -> BTreeSet<EntityId> {
    touched_closure
        .basis()
        .entities()
        .iter()
        .map(|entity| entity.entity_id())
        .collect()
}
