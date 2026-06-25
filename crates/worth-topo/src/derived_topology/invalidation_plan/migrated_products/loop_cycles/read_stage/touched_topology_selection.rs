use std::collections::BTreeSet;

use forge_relational::facade::identity::EntityId;

use super::LoopCycleReadStageCounters;
use crate::brep::topology_graph::TopologyView;
use crate::derived_topology::invalidation_plan::migrated_products::loop_cycles::LoopCycleBoundarySourceRow;
use crate::derived_topology::invalidation_plan::selection::DerivedInvalidationTouchedClosure;

pub(super) struct LoopCycleTouchedTopologySelection {
    rows: Vec<LoopCycleBoundarySourceRow>,
    counters: LoopCycleReadStageCounters,
}

impl LoopCycleTouchedTopologySelection {
    pub(super) fn from_touched_closure_and_topology(
        touched_closure: &DerivedInvalidationTouchedClosure,
        topology: &TopologyView,
    ) -> Self {
        let touched_entities = touched_loop_cycle_anchor_entity_set(touched_closure);
        let selected =
            selected_loop_cycle_boundary_rows_from_touched_topology(topology, &touched_entities);
        let selected_source_row_count = selected.rows.len();
        let counters = LoopCycleReadStageCounters::new(
            touched_entities.len(),
            selected.shell_lookup_count,
            selected.face_lookup_count,
            selected_source_row_count,
            selected_source_row_count,
            topology
                .shells
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
    ) -> (Vec<LoopCycleBoundarySourceRow>, LoopCycleReadStageCounters) {
        (self.rows, self.counters)
    }
}

struct SelectedLoopCycleBoundaryRows {
    rows: Vec<LoopCycleBoundarySourceRow>,
    shell_lookup_count: usize,
    face_lookup_count: usize,
}

fn selected_loop_cycle_boundary_rows_from_touched_topology(
    topology: &TopologyView,
    touched_entities: &BTreeSet<EntityId>,
) -> SelectedLoopCycleBoundaryRows {
    let mut rows = Vec::new();
    let mut shell_lookup_count = 0;
    let mut face_lookup_count = 0;
    for touched_entity in touched_entities {
        shell_lookup_count += 1;
        let Some(shell) = topology
            .shells
            .iter()
            .find(|shell| shell.entity_id == *touched_entity)
        else {
            continue;
        };
        let mut boundary_half_edge_count = 0;
        for face_id in &shell.face_ids {
            face_lookup_count += 1;
            if let Some(face) = topology
                .faces
                .iter()
                .find(|face| face.entity_id == *face_id)
            {
                boundary_half_edge_count += face.boundary_half_edge_ids.len();
            }
        }
        let boundary_component_count = usize::from(boundary_half_edge_count > 0);
        rows.push(LoopCycleBoundarySourceRow::new(
            shell.entity_id,
            boundary_component_count,
            boundary_half_edge_count,
        ));
    }
    SelectedLoopCycleBoundaryRows {
        rows,
        shell_lookup_count,
        face_lookup_count,
    }
}

fn touched_loop_cycle_anchor_entity_set(
    touched_closure: &DerivedInvalidationTouchedClosure,
) -> BTreeSet<EntityId> {
    touched_closure
        .basis()
        .entities()
        .iter()
        .map(|entity| entity.entity_id())
        .collect()
}
