use forge_relational::facade::identity::EntityId;
use serde::Serialize;

use super::input::LoopCycleExecutionInput;
use super::LoopCycleReadStageCounters;

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct LoopCycleProductRow {
    shell_id: EntityId,
    boundary_component_count: usize,
    boundary_half_edge_count: usize,
    closed_boundary: bool,
    row_digest: String,
}

impl LoopCycleProductRow {
    pub(crate) fn from_source_row(row: &super::LoopCycleBoundarySourceRow) -> Self {
        let closed_boundary = row.boundary_half_edge_count() == 0;
        let row_digest = super::super::super::catalog::catalog_digest([
            "worth-topo:loop-cycle-product-row:v1".to_string(),
            format!("shell:{:?}", row.shell_id()),
            format!("boundary-components:{}", row.boundary_component_count()),
            format!("boundary-half-edges:{}", row.boundary_half_edge_count()),
            format!("closed-boundary:{closed_boundary}"),
        ]);
        Self {
            shell_id: row.shell_id(),
            boundary_component_count: row.boundary_component_count(),
            boundary_half_edge_count: row.boundary_half_edge_count(),
            closed_boundary,
            row_digest,
        }
    }

    pub const fn shell_id(&self) -> EntityId {
        self.shell_id
    }

    pub const fn boundary_component_count(&self) -> usize {
        self.boundary_component_count
    }

    pub const fn boundary_half_edge_count(&self) -> usize {
        self.boundary_half_edge_count
    }

    pub const fn closed_boundary(&self) -> bool {
        self.closed_boundary
    }

    pub fn row_digest(&self) -> &str {
        &self.row_digest
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct LoopCycleDerivedProductOutput {
    rows: Vec<LoopCycleProductRow>,
    touched_closure_loop_cycle_bound: usize,
    selected_source_row_count: usize,
    available_source_row_count: usize,
    read_stage_counters: LoopCycleReadStageCounters,
    selected_plan_digest: String,
    source_rows_digest: String,
    input_digest: String,
    output_digest: String,
}

impl LoopCycleDerivedProductOutput {
    pub(crate) fn from_execution_input(input: &LoopCycleExecutionInput) -> Self {
        let rows = input
            .selected_rows()
            .iter()
            .map(LoopCycleProductRow::from_source_row)
            .collect::<Vec<_>>();
        Self::from_rows(
            rows,
            input.touched_closure_loop_cycle_bound(),
            input.selected_row_count(),
            input.available_source_row_count(),
            *input.read_stage_counters(),
            input.selected_plan_digest(),
            input.read_stage_receipt_digest(),
            input.source_rows_digest(),
            input.input_digest(),
        )
    }

    pub(crate) fn from_rows(
        rows: Vec<LoopCycleProductRow>,
        touched_closure_loop_cycle_bound: usize,
        selected_source_row_count: usize,
        available_source_row_count: usize,
        read_stage_counters: LoopCycleReadStageCounters,
        selected_plan_digest: &str,
        read_stage_receipt_digest: &str,
        source_rows_digest: &str,
        input_digest: &str,
    ) -> Self {
        let mut parts = vec![
            "worth-topo:loop-cycle-derived-product-output:v1".to_string(),
            format!("selected-plan:{selected_plan_digest}"),
            format!("read-stage:{read_stage_receipt_digest}"),
            format!("source-rows:{source_rows_digest}"),
            format!("input:{input_digest}"),
            format!("touched-bound:{touched_closure_loop_cycle_bound}"),
            format!("selected-source-rows:{selected_source_row_count}"),
            format!("available-source-rows:{available_source_row_count}"),
            format!(
                "read-stage-touched-anchors:{}",
                read_stage_counters.touched_anchor_count()
            ),
            format!(
                "read-stage-shell-lookups:{}",
                read_stage_counters.shell_lookup_count()
            ),
            format!(
                "read-stage-face-lookups:{}",
                read_stage_counters.face_lookup_count()
            ),
            format!(
                "read-stage-unrelated-breadth:{}",
                read_stage_counters.unrelated_source_breadth_count()
            ),
            format!(
                "read-stage-whole-view-fallbacks:{}",
                read_stage_counters.whole_view_fallback_count()
            ),
        ];
        parts.extend(rows.iter().map(|row| format!("row:{}", row.row_digest())));
        let output_digest = super::super::super::catalog::catalog_digest(parts);
        Self {
            rows,
            touched_closure_loop_cycle_bound,
            selected_source_row_count,
            available_source_row_count,
            read_stage_counters,
            selected_plan_digest: selected_plan_digest.to_string(),
            source_rows_digest: source_rows_digest.to_string(),
            input_digest: input_digest.to_string(),
            output_digest,
        }
    }

    pub fn rows(&self) -> &[LoopCycleProductRow] {
        &self.rows
    }

    pub const fn touched_closure_loop_cycle_bound(&self) -> usize {
        self.touched_closure_loop_cycle_bound
    }

    pub const fn selected_source_row_count(&self) -> usize {
        self.selected_source_row_count
    }

    pub const fn available_source_row_count(&self) -> usize {
        self.available_source_row_count
    }

    pub const fn read_stage_counters(&self) -> &LoopCycleReadStageCounters {
        &self.read_stage_counters
    }

    pub fn selected_plan_digest(&self) -> &str {
        &self.selected_plan_digest
    }

    pub fn source_rows_digest(&self) -> &str {
        &self.source_rows_digest
    }

    pub fn input_digest(&self) -> &str {
        &self.input_digest
    }

    pub fn output_digest(&self) -> &str {
        &self.output_digest
    }
}
