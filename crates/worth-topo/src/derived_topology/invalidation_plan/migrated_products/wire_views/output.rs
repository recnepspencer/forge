use forge_relational::facade::identity::EntityId;
use schema::facade::platform::authority::WireInterpretationClass;
use serde::Serialize;

use super::{WireViewExecutionInput, WireViewReadStageCounters};

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct WireViewProductRow {
    wire_id: EntityId,
    class: WireInterpretationClass,
    connected_component_count: usize,
    half_edge_count: usize,
    terminal_vertex_ids: Vec<EntityId>,
    branch_vertex_ids: Vec<EntityId>,
    row_digest: String,
}

impl WireViewProductRow {
    pub(crate) fn from_source_row(row: &super::WireViewSourceRow) -> Self {
        let row_digest = super::super::super::catalog::catalog_digest([
            "worth-topo:wire-view-product-row:v1".to_string(),
            format!("wire:{:?}", row.wire_id()),
            format!("class:{:?}", row.class()),
            format!("connected-components:{}", row.connected_component_count()),
            format!("half-edges:{}", row.half_edge_count()),
            format!("terminal-vertices:{:?}", row.terminal_vertex_ids()),
            format!("branch-vertices:{:?}", row.branch_vertex_ids()),
        ]);
        Self {
            wire_id: row.wire_id(),
            class: row.class(),
            connected_component_count: row.connected_component_count(),
            half_edge_count: row.half_edge_count(),
            terminal_vertex_ids: row.terminal_vertex_ids().to_vec(),
            branch_vertex_ids: row.branch_vertex_ids().to_vec(),
            row_digest,
        }
    }

    pub const fn wire_id(&self) -> EntityId {
        self.wire_id
    }

    pub const fn class(&self) -> WireInterpretationClass {
        self.class
    }

    pub const fn connected_component_count(&self) -> usize {
        self.connected_component_count
    }

    pub const fn half_edge_count(&self) -> usize {
        self.half_edge_count
    }

    pub fn terminal_vertex_ids(&self) -> &[EntityId] {
        &self.terminal_vertex_ids
    }

    pub fn branch_vertex_ids(&self) -> &[EntityId] {
        &self.branch_vertex_ids
    }

    pub fn row_digest(&self) -> &str {
        &self.row_digest
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct WireViewDerivedProductOutput {
    rows: Vec<WireViewProductRow>,
    touched_closure_wire_view_bound: usize,
    selected_source_row_count: usize,
    available_source_row_count: usize,
    read_stage_counters: WireViewReadStageCounters,
    selected_plan_digest: String,
    source_rows_digest: String,
    input_digest: String,
    output_digest: String,
}

impl WireViewDerivedProductOutput {
    pub(crate) fn from_execution_input(input: &WireViewExecutionInput) -> Self {
        let rows = input
            .selected_rows()
            .iter()
            .map(WireViewProductRow::from_source_row)
            .collect::<Vec<_>>();
        Self::from_rows(
            rows,
            input.touched_closure_wire_view_bound(),
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
        rows: Vec<WireViewProductRow>,
        touched_closure_wire_view_bound: usize,
        selected_source_row_count: usize,
        available_source_row_count: usize,
        read_stage_counters: WireViewReadStageCounters,
        selected_plan_digest: &str,
        read_stage_receipt_digest: &str,
        source_rows_digest: &str,
        input_digest: &str,
    ) -> Self {
        let mut parts = vec![
            "worth-topo:wire-view-derived-product-output:v1".to_string(),
            format!("selected-plan:{selected_plan_digest}"),
            format!("read-stage:{read_stage_receipt_digest}"),
            format!("source-rows:{source_rows_digest}"),
            format!("input:{input_digest}"),
            format!("touched-bound:{touched_closure_wire_view_bound}"),
            format!("selected-source-rows:{selected_source_row_count}"),
            format!("available-source-rows:{available_source_row_count}"),
            format!(
                "read-stage-touched-wires:{}",
                read_stage_counters.touched_wire_count()
            ),
            format!(
                "read-stage-touched-half-edge-lookups:{}",
                read_stage_counters.touched_half_edge_lookup_count()
            ),
            format!(
                "read-stage-selected-wire-roots:{}",
                read_stage_counters.selected_wire_root_count()
            ),
            format!(
                "read-stage-unrelated-wire-breadth:{}",
                read_stage_counters.unrelated_wire_breadth_count()
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
            touched_closure_wire_view_bound,
            selected_source_row_count,
            available_source_row_count,
            read_stage_counters,
            selected_plan_digest: selected_plan_digest.to_string(),
            source_rows_digest: source_rows_digest.to_string(),
            input_digest: input_digest.to_string(),
            output_digest,
        }
    }

    pub fn rows(&self) -> &[WireViewProductRow] {
        &self.rows
    }

    pub const fn selected_source_row_count(&self) -> usize {
        self.selected_source_row_count
    }

    pub const fn touched_closure_wire_view_bound(&self) -> usize {
        self.touched_closure_wire_view_bound
    }

    pub const fn available_source_row_count(&self) -> usize {
        self.available_source_row_count
    }

    pub const fn read_stage_counters(&self) -> &WireViewReadStageCounters {
        &self.read_stage_counters
    }

    pub fn selected_plan_digest(&self) -> &str {
        &self.selected_plan_digest
    }

    pub fn output_digest(&self) -> &str {
        &self.output_digest
    }
}
