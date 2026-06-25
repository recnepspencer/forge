use serde::Serialize;

use super::input::ShellViewExecutionInput;
use super::ShellViewReadStageCounters;

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct ShellViewProductRow {
    touched_shell_identity: String,
    touched_source_identity: String,
    source_half_edge_identity: String,
    source_edge_identity: String,
    radial_target_half_edge_identity: String,
    current_target_edge_identity: String,
    source_radial_next_relation_identity: String,
    ring_half_edge_count: usize,
    boundary_half_edge: bool,
    non_manifold_edge: bool,
    row_digest: String,
}

impl ShellViewProductRow {
    pub(crate) fn from_source_row(row: &super::ShellViewBoundarySourceRow) -> Self {
        let row_digest = super::super::super::catalog::catalog_digest([
            "worth-topo:shell-view-product-row:v1".to_string(),
            format!("touched-shell:{}", row.touched_shell_identity()),
            format!("touched-source:{}", row.touched_source_identity()),
            format!("source-half-edge:{}", row.source_half_edge_identity()),
            format!("source-edge:{}", row.source_edge_identity()),
            format!("radial-target:{}", row.radial_target_half_edge_identity()),
            format!("current-target-edge:{}", row.current_target_edge_identity()),
            format!(
                "radial-relation:{}",
                row.source_radial_next_relation_identity()
            ),
            format!("ring-half-edges:{}", row.ring_half_edge_count()),
            format!("boundary-half-edge:{}", row.boundary_half_edge()),
            format!("non-manifold-edge:{}", row.non_manifold_edge()),
        ]);
        Self {
            touched_shell_identity: row.touched_shell_identity().to_string(),
            touched_source_identity: row.touched_source_identity().to_string(),
            source_half_edge_identity: row.source_half_edge_identity().to_string(),
            source_edge_identity: row.source_edge_identity().to_string(),
            radial_target_half_edge_identity: row.radial_target_half_edge_identity().to_string(),
            current_target_edge_identity: row.current_target_edge_identity().to_string(),
            source_radial_next_relation_identity: row
                .source_radial_next_relation_identity()
                .to_string(),
            ring_half_edge_count: row.ring_half_edge_count(),
            boundary_half_edge: row.boundary_half_edge(),
            non_manifold_edge: row.non_manifold_edge(),
            row_digest,
        }
    }

    pub fn touched_shell_identity(&self) -> &str {
        &self.touched_shell_identity
    }

    pub fn touched_source_identity(&self) -> &str {
        &self.touched_source_identity
    }

    pub fn source_half_edge_identity(&self) -> &str {
        &self.source_half_edge_identity
    }

    pub fn source_edge_identity(&self) -> &str {
        &self.source_edge_identity
    }

    pub fn radial_target_half_edge_identity(&self) -> &str {
        &self.radial_target_half_edge_identity
    }

    pub fn current_target_edge_identity(&self) -> &str {
        &self.current_target_edge_identity
    }

    pub fn source_radial_next_relation_identity(&self) -> &str {
        &self.source_radial_next_relation_identity
    }

    pub const fn ring_half_edge_count(&self) -> usize {
        self.ring_half_edge_count
    }

    pub const fn boundary_half_edge(&self) -> bool {
        self.boundary_half_edge
    }

    pub const fn non_manifold_edge(&self) -> bool {
        self.non_manifold_edge
    }

    pub fn row_digest(&self) -> &str {
        &self.row_digest
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct ShellViewDerivedProductOutput {
    rows: Vec<ShellViewProductRow>,
    touched_closure_shell_view_bound: usize,
    selected_source_row_count: usize,
    available_source_row_count: usize,
    read_stage_counters: ShellViewReadStageCounters,
    selected_plan_digest: String,
    source_rows_digest: String,
    input_digest: String,
    output_digest: String,
}

impl ShellViewDerivedProductOutput {
    pub(crate) fn from_execution_input(input: &ShellViewExecutionInput) -> Self {
        let rows = input
            .selected_rows()
            .iter()
            .map(ShellViewProductRow::from_source_row)
            .collect::<Vec<_>>();
        Self::from_rows(
            rows,
            input.touched_closure_shell_view_bound(),
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
        rows: Vec<ShellViewProductRow>,
        touched_closure_shell_view_bound: usize,
        selected_source_row_count: usize,
        available_source_row_count: usize,
        read_stage_counters: ShellViewReadStageCounters,
        selected_plan_digest: &str,
        read_stage_receipt_digest: &str,
        source_rows_digest: &str,
        input_digest: &str,
    ) -> Self {
        let mut parts = vec![
            "worth-topo:shell-view-derived-product-output:v1".to_string(),
            format!("selected-plan:{selected_plan_digest}"),
            format!("read-stage:{read_stage_receipt_digest}"),
            format!("source-rows:{source_rows_digest}"),
            format!("input:{input_digest}"),
            format!("touched-bound:{touched_closure_shell_view_bound}"),
            format!("selected-source-rows:{selected_source_row_count}"),
            format!("available-source-rows:{available_source_row_count}"),
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
            format!(
                "read-stage-whole-view-fallbacks:{}",
                read_stage_counters.whole_view_fallback_count()
            ),
        ];
        parts.extend(rows.iter().map(|row| format!("row:{}", row.row_digest())));
        let output_digest = super::super::super::catalog::catalog_digest(parts);
        Self {
            rows,
            touched_closure_shell_view_bound,
            selected_source_row_count,
            available_source_row_count,
            read_stage_counters,
            selected_plan_digest: selected_plan_digest.to_string(),
            source_rows_digest: source_rows_digest.to_string(),
            input_digest: input_digest.to_string(),
            output_digest,
        }
    }

    pub fn rows(&self) -> &[ShellViewProductRow] {
        &self.rows
    }

    pub const fn touched_closure_shell_view_bound(&self) -> usize {
        self.touched_closure_shell_view_bound
    }

    pub const fn selected_source_row_count(&self) -> usize {
        self.selected_source_row_count
    }

    pub const fn available_source_row_count(&self) -> usize {
        self.available_source_row_count
    }

    pub const fn read_stage_counters(&self) -> &ShellViewReadStageCounters {
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
