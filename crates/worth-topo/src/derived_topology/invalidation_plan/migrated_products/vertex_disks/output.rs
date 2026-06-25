use serde::Serialize;

use super::input::VertexDiskExecutionInput;
use super::VertexDiskReadStageCounters;

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct VertexDiskProductRow {
    touched_vertex_identities: Vec<String>,
    touched_source_identity: String,
    source_half_edge_identity: String,
    source_edge_identity: String,
    incident_half_edge_identities: Vec<String>,
    incident_different_edge_half_edge_identities: Vec<String>,
    touched_incident_edge_identities: Vec<String>,
    touched_incident_edge_count: usize,
    branch_vertex_disk: bool,
    row_digest: String,
}

impl VertexDiskProductRow {
    pub(crate) fn from_source_row(row: &super::VertexDiskBoundarySourceRow) -> Self {
        let mut parts = vec![
            "worth-topo:vertex-disk-product-row:v1".to_string(),
            format!("touched-source:{}", row.touched_source_identity()),
            format!("source-half-edge:{}", row.source_half_edge_identity()),
            format!("source-edge:{}", row.source_edge_identity()),
            format!("incident-edge-count:{}", row.touched_incident_edge_count()),
            format!("branch-vertex-disk:{}", row.branch_vertex_disk()),
        ];
        parts.extend(
            row.touched_vertex_identities()
                .iter()
                .map(|identity| format!("touched-vertex:{identity}")),
        );
        parts.extend(
            row.incident_half_edge_identities()
                .iter()
                .map(|identity| format!("incident-half-edge:{identity}")),
        );
        parts.extend(
            row.incident_different_edge_half_edge_identities()
                .iter()
                .map(|identity| format!("incident-different-edge-half-edge:{identity}")),
        );
        parts.extend(
            row.touched_incident_edge_identities()
                .iter()
                .map(|identity| format!("touched-incident-edge:{identity}")),
        );
        let row_digest = super::super::super::catalog::catalog_digest(parts);
        Self {
            touched_vertex_identities: row.touched_vertex_identities().to_vec(),
            touched_source_identity: row.touched_source_identity().to_string(),
            source_half_edge_identity: row.source_half_edge_identity().to_string(),
            source_edge_identity: row.source_edge_identity().to_string(),
            incident_half_edge_identities: row.incident_half_edge_identities().to_vec(),
            incident_different_edge_half_edge_identities: row
                .incident_different_edge_half_edge_identities()
                .to_vec(),
            touched_incident_edge_identities: row.touched_incident_edge_identities().to_vec(),
            touched_incident_edge_count: row.touched_incident_edge_count(),
            branch_vertex_disk: row.branch_vertex_disk(),
            row_digest,
        }
    }

    pub fn touched_vertex_identities(&self) -> &[String] {
        self.touched_vertex_identities.as_slice()
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

    pub fn incident_half_edge_identities(&self) -> &[String] {
        self.incident_half_edge_identities.as_slice()
    }

    pub fn incident_different_edge_half_edge_identities(&self) -> &[String] {
        self.incident_different_edge_half_edge_identities.as_slice()
    }

    pub fn touched_incident_edge_identities(&self) -> &[String] {
        self.touched_incident_edge_identities.as_slice()
    }

    pub const fn touched_incident_edge_count(&self) -> usize {
        self.touched_incident_edge_count
    }

    pub const fn branch_vertex_disk(&self) -> bool {
        self.branch_vertex_disk
    }

    pub fn row_digest(&self) -> &str {
        &self.row_digest
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct VertexDiskDerivedProductOutput {
    rows: Vec<VertexDiskProductRow>,
    touched_closure_vertex_disk_bound: usize,
    selected_source_row_count: usize,
    available_source_row_count: usize,
    read_stage_counters: VertexDiskReadStageCounters,
    selected_plan_digest: String,
    source_rows_digest: String,
    input_digest: String,
    output_digest: String,
}

impl VertexDiskDerivedProductOutput {
    pub(crate) fn from_execution_input(input: &VertexDiskExecutionInput) -> Self {
        let rows = input
            .selected_rows()
            .iter()
            .map(VertexDiskProductRow::from_source_row)
            .collect::<Vec<_>>();
        Self::from_rows(
            rows,
            input.touched_closure_vertex_disk_bound(),
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
        rows: Vec<VertexDiskProductRow>,
        touched_closure_vertex_disk_bound: usize,
        selected_source_row_count: usize,
        available_source_row_count: usize,
        read_stage_counters: VertexDiskReadStageCounters,
        selected_plan_digest: &str,
        read_stage_receipt_digest: &str,
        source_rows_digest: &str,
        input_digest: &str,
    ) -> Self {
        let mut parts = vec![
            "worth-topo:vertex-disk-derived-product-output:v1".to_string(),
            format!("selected-plan:{selected_plan_digest}"),
            format!("read-stage:{read_stage_receipt_digest}"),
            format!("source-rows:{source_rows_digest}"),
            format!("input:{input_digest}"),
            format!("touched-bound:{touched_closure_vertex_disk_bound}"),
            format!("selected-source-rows:{selected_source_row_count}"),
            format!("available-source-rows:{available_source_row_count}"),
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
            format!(
                "read-stage-whole-view-fallbacks:{}",
                read_stage_counters.whole_view_fallback_count()
            ),
        ];
        parts.extend(rows.iter().map(|row| format!("row:{}", row.row_digest())));
        let output_digest = super::super::super::catalog::catalog_digest(parts);
        Self {
            rows,
            touched_closure_vertex_disk_bound,
            selected_source_row_count,
            available_source_row_count,
            read_stage_counters,
            selected_plan_digest: selected_plan_digest.to_string(),
            source_rows_digest: source_rows_digest.to_string(),
            input_digest: input_digest.to_string(),
            output_digest,
        }
    }

    pub fn rows(&self) -> &[VertexDiskProductRow] {
        &self.rows
    }

    pub const fn touched_closure_vertex_disk_bound(&self) -> usize {
        self.touched_closure_vertex_disk_bound
    }

    pub const fn selected_source_row_count(&self) -> usize {
        self.selected_source_row_count
    }

    pub const fn available_source_row_count(&self) -> usize {
        self.available_source_row_count
    }

    pub const fn read_stage_counters(&self) -> &VertexDiskReadStageCounters {
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
