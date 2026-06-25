use serde::Serialize;

use super::VertexDiskDerivedProductOutput;

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct VertexDiskMigrationCounters {
    touched_closure_vertex_disk_bound: usize,
    selected_source_row_count: usize,
    available_source_row_count: usize,
    output_row_count: usize,
    execution_work_count: usize,
    whole_view_fallback_count: usize,
    read_stage_touched_vertex_count: usize,
    read_stage_touched_half_edge_lookup_count: usize,
    read_stage_selected_vertex_disk_root_count: usize,
    read_stage_touched_incident_half_edge_count: usize,
    read_stage_touched_incident_edge_count: usize,
    read_stage_unrelated_vertex_disk_breadth_count: usize,
    non_loop_placeholder_execution_count: usize,
    old_authority_residue_count: usize,
    counters_digest: String,
}

impl VertexDiskMigrationCounters {
    pub(crate) fn new(
        output: &VertexDiskDerivedProductOutput,
        execution_work_count: usize,
        whole_view_fallback_count: usize,
        non_loop_placeholder_execution_count: usize,
        old_authority_residue_count: usize,
    ) -> Self {
        let output_row_count = output.rows().len();
        let touched_closure_vertex_disk_bound = output.touched_closure_vertex_disk_bound();
        let selected_source_row_count = output.selected_source_row_count();
        let available_source_row_count = output.available_source_row_count();
        let read_stage_counters = output.read_stage_counters();
        let counters_digest = super::super::super::catalog::catalog_digest([
            "worth-topo:vertex-disk-migration-counters:v1".to_string(),
            format!("touched-bound:{touched_closure_vertex_disk_bound}"),
            format!("selected-source-rows:{selected_source_row_count}"),
            format!("available-source-rows:{available_source_row_count}"),
            format!("output-rows:{output_row_count}"),
            format!("execution-work:{execution_work_count}"),
            format!("whole-view-fallbacks:{whole_view_fallback_count}"),
            format!(
                "read-stage-touched-vertices:{}",
                read_stage_counters.touched_vertex_count()
            ),
            format!(
                "read-stage-touched-half-edge-lookups:{}",
                read_stage_counters.touched_half_edge_lookup_count()
            ),
            format!(
                "read-stage-selected-vertex-disk-roots:{}",
                read_stage_counters.selected_vertex_disk_root_count()
            ),
            format!(
                "read-stage-touched-incident-half-edges:{}",
                read_stage_counters.touched_incident_half_edge_count()
            ),
            format!(
                "read-stage-touched-incident-edges:{}",
                read_stage_counters.touched_incident_edge_count()
            ),
            format!(
                "read-stage-unrelated-vertex-disk-breadth:{}",
                read_stage_counters.unrelated_vertex_disk_breadth_count()
            ),
            format!("non-loop-placeholders:{non_loop_placeholder_execution_count}"),
            format!("old-authority-residue:{old_authority_residue_count}"),
        ]);
        Self {
            touched_closure_vertex_disk_bound,
            selected_source_row_count,
            available_source_row_count,
            output_row_count,
            execution_work_count,
            whole_view_fallback_count,
            read_stage_touched_vertex_count: read_stage_counters.touched_vertex_count(),
            read_stage_touched_half_edge_lookup_count: read_stage_counters
                .touched_half_edge_lookup_count(),
            read_stage_selected_vertex_disk_root_count: read_stage_counters
                .selected_vertex_disk_root_count(),
            read_stage_touched_incident_half_edge_count: read_stage_counters
                .touched_incident_half_edge_count(),
            read_stage_touched_incident_edge_count: read_stage_counters
                .touched_incident_edge_count(),
            read_stage_unrelated_vertex_disk_breadth_count: read_stage_counters
                .unrelated_vertex_disk_breadth_count(),
            non_loop_placeholder_execution_count,
            old_authority_residue_count,
            counters_digest,
        }
    }

    pub const fn selected_source_row_count(&self) -> usize {
        self.selected_source_row_count
    }

    pub const fn touched_closure_vertex_disk_bound(&self) -> usize {
        self.touched_closure_vertex_disk_bound
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

    pub const fn read_stage_touched_vertex_count(&self) -> usize {
        self.read_stage_touched_vertex_count
    }

    pub const fn read_stage_touched_half_edge_lookup_count(&self) -> usize {
        self.read_stage_touched_half_edge_lookup_count
    }

    pub const fn read_stage_selected_vertex_disk_root_count(&self) -> usize {
        self.read_stage_selected_vertex_disk_root_count
    }

    pub const fn read_stage_touched_incident_half_edge_count(&self) -> usize {
        self.read_stage_touched_incident_half_edge_count
    }

    pub const fn read_stage_touched_incident_edge_count(&self) -> usize {
        self.read_stage_touched_incident_edge_count
    }

    pub const fn read_stage_unrelated_vertex_disk_breadth_count(&self) -> usize {
        self.read_stage_unrelated_vertex_disk_breadth_count
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
