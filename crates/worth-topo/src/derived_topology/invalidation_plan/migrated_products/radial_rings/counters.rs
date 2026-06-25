use serde::Serialize;

use super::RadialRingDerivedProductOutput;

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct RadialRingMigrationCounters {
    touched_closure_radial_ring_bound: usize,
    selected_source_row_count: usize,
    available_source_row_count: usize,
    output_row_count: usize,
    execution_work_count: usize,
    whole_view_fallback_count: usize,
    read_stage_touched_anchor_count: usize,
    read_stage_half_edge_lookup_count: usize,
    read_stage_radial_relation_lookup_count: usize,
    read_stage_selected_radial_root_count: usize,
    read_stage_touched_neighborhood_breadth_count: usize,
    read_stage_unrelated_source_breadth_count: usize,
    non_loop_placeholder_execution_count: usize,
    old_authority_residue_count: usize,
    counters_digest: String,
}

impl RadialRingMigrationCounters {
    pub(crate) fn new(
        output: &RadialRingDerivedProductOutput,
        execution_work_count: usize,
        whole_view_fallback_count: usize,
        non_loop_placeholder_execution_count: usize,
        old_authority_residue_count: usize,
    ) -> Self {
        let output_row_count = output.rows().len();
        let touched_closure_radial_ring_bound = output.touched_closure_radial_ring_bound();
        let selected_source_row_count = output.selected_source_row_count();
        let available_source_row_count = output.available_source_row_count();
        let read_stage_counters = output.read_stage_counters();
        let counters_digest = super::super::super::catalog::catalog_digest([
            "worth-topo:radial-ring-migration-counters:v1".to_string(),
            format!("touched-bound:{touched_closure_radial_ring_bound}"),
            format!("selected-source-rows:{selected_source_row_count}"),
            format!("available-source-rows:{available_source_row_count}"),
            format!("output-rows:{output_row_count}"),
            format!("execution-work:{execution_work_count}"),
            format!("whole-view-fallbacks:{whole_view_fallback_count}"),
            format!(
                "read-stage-touched-anchors:{}",
                read_stage_counters.touched_anchor_count()
            ),
            format!(
                "read-stage-half-edge-lookups:{}",
                read_stage_counters.half_edge_lookup_count()
            ),
            format!(
                "read-stage-radial-relation-lookups:{}",
                read_stage_counters.radial_relation_lookup_count()
            ),
            format!(
                "read-stage-selected-radial-roots:{}",
                read_stage_counters.selected_radial_root_count()
            ),
            format!(
                "read-stage-touched-neighborhood-breadth:{}",
                read_stage_counters.touched_neighborhood_breadth_count()
            ),
            format!(
                "read-stage-unrelated-breadth:{}",
                read_stage_counters.unrelated_source_breadth_count()
            ),
            format!("non-loop-placeholders:{non_loop_placeholder_execution_count}"),
            format!("old-authority-residue:{old_authority_residue_count}"),
        ]);
        Self {
            touched_closure_radial_ring_bound,
            selected_source_row_count,
            available_source_row_count,
            output_row_count,
            execution_work_count,
            whole_view_fallback_count,
            read_stage_touched_anchor_count: read_stage_counters.touched_anchor_count(),
            read_stage_half_edge_lookup_count: read_stage_counters.half_edge_lookup_count(),
            read_stage_radial_relation_lookup_count: read_stage_counters
                .radial_relation_lookup_count(),
            read_stage_selected_radial_root_count: read_stage_counters.selected_radial_root_count(),
            read_stage_touched_neighborhood_breadth_count: read_stage_counters
                .touched_neighborhood_breadth_count(),
            read_stage_unrelated_source_breadth_count: read_stage_counters
                .unrelated_source_breadth_count(),
            non_loop_placeholder_execution_count,
            old_authority_residue_count,
            counters_digest,
        }
    }

    pub const fn selected_source_row_count(&self) -> usize {
        self.selected_source_row_count
    }

    pub const fn touched_closure_radial_ring_bound(&self) -> usize {
        self.touched_closure_radial_ring_bound
    }

    pub const fn available_source_row_count(&self) -> usize {
        self.available_source_row_count
    }

    pub const fn output_row_count(&self) -> usize {
        self.output_row_count
    }

    pub const fn execution_work_count(&self) -> usize {
        self.execution_work_count
    }

    pub const fn whole_view_fallback_count(&self) -> usize {
        self.whole_view_fallback_count
    }

    pub const fn read_stage_touched_anchor_count(&self) -> usize {
        self.read_stage_touched_anchor_count
    }

    pub const fn read_stage_half_edge_lookup_count(&self) -> usize {
        self.read_stage_half_edge_lookup_count
    }

    pub const fn read_stage_radial_relation_lookup_count(&self) -> usize {
        self.read_stage_radial_relation_lookup_count
    }

    pub const fn read_stage_selected_radial_root_count(&self) -> usize {
        self.read_stage_selected_radial_root_count
    }

    pub const fn read_stage_touched_neighborhood_breadth_count(&self) -> usize {
        self.read_stage_touched_neighborhood_breadth_count
    }

    pub const fn read_stage_unrelated_source_breadth_count(&self) -> usize {
        self.read_stage_unrelated_source_breadth_count
    }

    pub const fn non_loop_placeholder_execution_count(&self) -> usize {
        self.non_loop_placeholder_execution_count
    }

    pub const fn old_authority_residue_count(&self) -> usize {
        self.old_authority_residue_count
    }

    pub fn counters_digest(&self) -> &str {
        &self.counters_digest
    }
}
