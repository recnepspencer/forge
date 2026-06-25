use serde::Serialize;

use super::{VertexDiskMigrationError, VertexDiskReadStageCounters, VertexDiskReadStageReceipt};
use crate::derived_topology::invalidation_plan::catalog::DerivedTopologyProductFamilyIdentity;
use crate::derived_topology::invalidation_plan::selection::DerivedInvalidationSelectedPlan;
use crate::projection::read_views::TopologyHalfEdgeSharedVertexNeighborhoodView;

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct VertexDiskBoundarySourceRow {
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

impl VertexDiskBoundarySourceRow {
    pub(crate) fn new(
        touched_vertex_identities: Vec<String>,
        touched_source_identity: impl Into<String>,
        source_half_edge_identity: impl Into<String>,
        source_edge_identity: impl Into<String>,
        incident_half_edge_identities: Vec<String>,
        incident_different_edge_half_edge_identities: Vec<String>,
        touched_incident_edge_identities: Vec<String>,
    ) -> Self {
        let touched_source_identity = touched_source_identity.into();
        let source_half_edge_identity = source_half_edge_identity.into();
        let source_edge_identity = source_edge_identity.into();
        let touched_incident_edge_count = touched_incident_edge_identities.len();
        let branch_vertex_disk = touched_incident_edge_count >= 3;
        let mut parts = vec![
            "worth-topo:vertex-disk-boundary-source-row:v1".to_string(),
            format!("touched-source:{touched_source_identity}"),
            format!("source-half-edge:{source_half_edge_identity}"),
            format!("source-edge:{source_edge_identity}"),
            format!("incident-edge-count:{touched_incident_edge_count}"),
            format!("branch-vertex-disk:{branch_vertex_disk}"),
        ];
        parts.extend(
            touched_vertex_identities
                .iter()
                .map(|identity| format!("touched-vertex:{identity}")),
        );
        parts.extend(
            incident_half_edge_identities
                .iter()
                .map(|identity| format!("incident-half-edge:{identity}")),
        );
        parts.extend(
            incident_different_edge_half_edge_identities
                .iter()
                .map(|identity| format!("incident-different-edge-half-edge:{identity}")),
        );
        parts.extend(
            touched_incident_edge_identities
                .iter()
                .map(|identity| format!("touched-incident-edge:{identity}")),
        );
        let row_digest = super::super::super::catalog::catalog_digest(parts);
        Self {
            touched_vertex_identities,
            touched_source_identity,
            source_half_edge_identity,
            source_edge_identity,
            incident_half_edge_identities,
            incident_different_edge_half_edge_identities,
            touched_incident_edge_identities,
            touched_incident_edge_count,
            branch_vertex_disk,
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

    pub(crate) fn from_query_shared_vertex_view(
        view: &TopologyHalfEdgeSharedVertexNeighborhoodView,
    ) -> Self {
        let touched_vertices = view.source_vertex_identities().to_vec();
        Self::new(
            touched_vertices,
            view.source_half_edge_identity(),
            view.source_half_edge_identity(),
            view.source_edge_identity(),
            view.vertex_adjacent_half_edge_identities().to_vec(),
            view.vertex_adjacent_different_edge_half_edge_identities()
                .to_vec(),
            touched_incident_edge_identities_from_query_view(view),
        )
    }
}

fn touched_incident_edge_identities_from_query_view(
    view: &TopologyHalfEdgeSharedVertexNeighborhoodView,
) -> Vec<String> {
    let mut edge_identities = view
        .vertex_adjacent_different_edge_half_edges()
        .iter()
        .map(|evidence| evidence.edge_identity().to_string())
        .collect::<Vec<_>>();
    edge_identities.push(view.source_edge_identity().to_string());
    edge_identities.sort();
    edge_identities.dedup();
    edge_identities
}

#[cfg(test)]
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct VertexDiskTouchedBoundaryRows {
    selected_rows: Vec<VertexDiskBoundarySourceRow>,
    available_source_row_count: usize,
    source_rows_digest: String,
}

#[cfg(test)]
impl VertexDiskTouchedBoundaryRows {
    pub fn from_selected_rows(rows: Vec<VertexDiskBoundarySourceRow>) -> Self {
        let available_source_row_count = rows.len();
        Self::from_selected_rows_with_available_count(rows, available_source_row_count)
            .expect("selected rows are always a valid available row set")
    }

    pub fn from_selected_rows_with_available_count(
        rows: Vec<VertexDiskBoundarySourceRow>,
        available_source_row_count: usize,
    ) -> Result<Self, VertexDiskMigrationError> {
        if rows.len() > available_source_row_count {
            return Err(VertexDiskMigrationError::SelectedRowsExceedAvailableRows);
        }
        let source_rows_digest = source_rows_digest(&rows, available_source_row_count);
        Ok(Self {
            selected_rows: rows,
            available_source_row_count,
            source_rows_digest,
        })
    }

    pub fn selected_rows(&self) -> &[VertexDiskBoundarySourceRow] {
        &self.selected_rows
    }

    pub const fn available_source_row_count(&self) -> usize {
        self.available_source_row_count
    }

    pub fn selected_row_count(&self) -> usize {
        self.selected_rows.len()
    }

    pub fn source_rows_digest(&self) -> &str {
        &self.source_rows_digest
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct VertexDiskExecutionInput {
    selected_rows: Vec<VertexDiskBoundarySourceRow>,
    available_source_row_count: usize,
    selected_plan_digest: String,
    vertex_disk_selected_row_digest: String,
    source_rows_digest: String,
    read_stage_receipt_digest: String,
    touched_closure_vertex_disk_bound: usize,
    read_stage_counters: VertexDiskReadStageCounters,
    input_digest: String,
}

impl VertexDiskExecutionInput {
    pub fn from_selected_plan_and_read_stage(
        selected_plan: &DerivedInvalidationSelectedPlan,
        read_stage_receipt: VertexDiskReadStageReceipt,
    ) -> Result<Self, VertexDiskMigrationError> {
        let vertex_disk_selected_row_digest = selected_plan
            .selected_rows()
            .iter()
            .find(|row| row.family_identity() == DerivedTopologyProductFamilyIdentity::VertexDisks)
            .map(|row| row.row_digest().to_string())
            .ok_or(VertexDiskMigrationError::SelectedPlanMissingVertexDiskRow)?;
        if selected_plan.selected_plan_digest() != read_stage_receipt.selected_plan_digest()
            || selected_plan.touched_closure_digest() != read_stage_receipt.touched_closure_digest()
            || selected_plan.query_support_digest() != read_stage_receipt.query_support_digest()
            || selected_plan.legality_support_digest()
                != read_stage_receipt.legality_support_digest()
            || vertex_disk_selected_row_digest
                != read_stage_receipt.vertex_disk_selected_row_digest()
        {
            return Err(VertexDiskMigrationError::ReadStageReceiptNotBoundToSelectedPlan);
        }

        let selected_plan_digest = selected_plan.selected_plan_digest().to_string();
        let source_rows_digest = read_stage_receipt.read_source_digest().to_string();
        let read_stage_receipt_digest = read_stage_receipt.receipt_digest().to_string();
        let input_digest = super::super::super::catalog::catalog_digest([
            "worth-topo:vertex-disk-execution-input:v2".to_string(),
            format!("selected-plan:{selected_plan_digest}"),
            format!("vertex-disk-selected-row:{vertex_disk_selected_row_digest}"),
            format!("read-stage:{read_stage_receipt_digest}"),
            format!("source-rows:{source_rows_digest}"),
        ]);
        Ok(Self {
            selected_rows: read_stage_receipt.selected_rows().to_vec(),
            available_source_row_count: read_stage_receipt.available_source_row_count(),
            selected_plan_digest,
            vertex_disk_selected_row_digest,
            source_rows_digest,
            read_stage_receipt_digest,
            touched_closure_vertex_disk_bound: read_stage_receipt
                .touched_closure_vertex_disk_bound(),
            read_stage_counters: *read_stage_receipt.read_stage_counters(),
            input_digest,
        })
    }

    #[cfg(test)]
    pub fn from_selected_plan(
        selected_plan: &DerivedInvalidationSelectedPlan,
        rows: VertexDiskTouchedBoundaryRows,
    ) -> Result<Self, VertexDiskMigrationError> {
        let vertex_disk_selected_row_digest = selected_plan
            .selected_rows()
            .iter()
            .find(|row| row.family_identity() == DerivedTopologyProductFamilyIdentity::VertexDisks)
            .map(|row| row.row_digest().to_string())
            .ok_or(VertexDiskMigrationError::SelectedPlanMissingVertexDiskRow)?;
        let selected_plan_digest = selected_plan.selected_plan_digest().to_string();
        let source_rows_digest = rows.source_rows_digest().to_string();
        let selected_row_count = rows.selected_rows.len();
        let input_digest = super::super::super::catalog::catalog_digest([
            "worth-topo:vertex-disk-execution-input:v1".to_string(),
            format!("selected-plan:{selected_plan_digest}"),
            format!("vertex-disk-selected-row:{vertex_disk_selected_row_digest}"),
            format!("source-rows:{source_rows_digest}"),
        ]);
        Ok(Self {
            selected_rows: rows.selected_rows,
            available_source_row_count: rows.available_source_row_count,
            selected_plan_digest,
            vertex_disk_selected_row_digest,
            source_rows_digest,
            read_stage_receipt_digest: "test-only-raw-vertex-disk-input".to_string(),
            touched_closure_vertex_disk_bound: selected_row_bound(selected_plan),
            read_stage_counters: VertexDiskReadStageCounters::for_selected_rows(
                selected_row_count,
                rows.available_source_row_count,
            ),
            input_digest,
        })
    }

    pub fn selected_rows(&self) -> &[VertexDiskBoundarySourceRow] {
        &self.selected_rows
    }

    pub const fn available_source_row_count(&self) -> usize {
        self.available_source_row_count
    }

    pub fn selected_row_count(&self) -> usize {
        self.selected_rows.len()
    }

    pub fn selected_plan_digest(&self) -> &str {
        &self.selected_plan_digest
    }

    pub fn source_rows_digest(&self) -> &str {
        &self.source_rows_digest
    }

    pub fn read_stage_receipt_digest(&self) -> &str {
        &self.read_stage_receipt_digest
    }

    pub const fn touched_closure_vertex_disk_bound(&self) -> usize {
        self.touched_closure_vertex_disk_bound
    }

    pub const fn read_stage_counters(&self) -> &VertexDiskReadStageCounters {
        &self.read_stage_counters
    }

    pub fn input_digest(&self) -> &str {
        &self.input_digest
    }
}

#[cfg(test)]
fn selected_row_bound(selected_plan: &DerivedInvalidationSelectedPlan) -> usize {
    let counters = selected_plan.counters();
    counters.touched_entity_count()
        + counters.touched_relation_count()
        + counters.touched_relation_kind_count()
        + counters.touched_aspect_count()
        + counters.touched_scope_count()
}

#[cfg(test)]
fn source_rows_digest(
    rows: &[VertexDiskBoundarySourceRow],
    available_source_row_count: usize,
) -> String {
    let mut parts = vec![
        "worth-topo:vertex-disk-touched-source-rows:v1".to_string(),
        format!("selected-rows:{}", rows.len()),
        format!("available-source-rows:{available_source_row_count}"),
    ];
    parts.extend(rows.iter().map(|row| format!("row:{}", row.row_digest())));
    super::super::super::catalog::catalog_digest(parts)
}
