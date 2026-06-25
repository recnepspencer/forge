use serde::Serialize;

use super::super::{VertexDiskBoundarySourceRow, VertexDiskMigrationError};
use super::counters::VertexDiskReadStageCounters;
use super::query_proof::query_report_digests_for_shared_vertex_views;
#[cfg(test)]
use super::touched_topology_selection::VertexDiskTouchedTopologySelection;
#[cfg(test)]
use crate::brep::topology_graph::TopologyView;
use crate::derived_topology::invalidation_plan::selection::{
    DerivedInvalidationSelectedPlan, DerivedInvalidationTouchedClosure,
};
use crate::projection::read_views::TopologyHalfEdgeSharedVertexNeighborhoodView;

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct VertexDiskReadSource {
    selected_rows: Vec<VertexDiskBoundarySourceRow>,
    available_source_row_count: usize,
    counters: VertexDiskReadStageCounters,
    query_report_digests: Vec<String>,
    read_source_digest: String,
}

impl VertexDiskReadSource {
    pub fn from_query_shared_vertex_neighborhood_views(
        selected_plan: &DerivedInvalidationSelectedPlan,
        touched_closure: &DerivedInvalidationTouchedClosure,
        shared_vertex_views: &[TopologyHalfEdgeSharedVertexNeighborhoodView],
    ) -> Result<Self, VertexDiskMigrationError> {
        if selected_plan.touched_closure_digest() != touched_closure.closure_digest() {
            return Err(VertexDiskMigrationError::ReadStageTouchedClosureNotBoundToSelectedPlan);
        }

        let query_report_digests =
            query_report_digests_for_shared_vertex_views(shared_vertex_views)?;
        let rows = shared_vertex_views
            .iter()
            .map(VertexDiskBoundarySourceRow::from_query_shared_vertex_view)
            .collect::<Vec<_>>();
        let touched_vertex_count = rows
            .iter()
            .map(|row| row.touched_vertex_identities().len())
            .sum();
        let touched_incident_half_edge_count = rows
            .iter()
            .map(|row| row.incident_half_edge_identities().len())
            .sum();
        let touched_incident_edge_count = rows
            .iter()
            .map(VertexDiskBoundarySourceRow::touched_incident_edge_count)
            .sum();
        let counters = VertexDiskReadStageCounters::new(
            touched_vertex_count,
            shared_vertex_views.len(),
            rows.len(),
            rows.len(),
            rows.len(),
            touched_incident_half_edge_count,
            touched_incident_edge_count,
            0,
            0,
        );
        Self::from_rows_with_counters_and_query_reports(
            rows,
            shared_vertex_views.len(),
            counters,
            query_report_digests,
        )
    }

    #[cfg(test)]
    pub fn select_from_touched_closure(
        selected_plan: &DerivedInvalidationSelectedPlan,
        touched_closure: &DerivedInvalidationTouchedClosure,
        topology: &TopologyView,
    ) -> Result<Self, VertexDiskMigrationError> {
        if selected_plan.touched_closure_digest() != touched_closure.closure_digest() {
            return Err(VertexDiskMigrationError::ReadStageTouchedClosureNotBoundToSelectedPlan);
        }

        let selected = VertexDiskTouchedTopologySelection::from_touched_closure_and_topology(
            touched_closure,
            topology,
        );
        let (selected_rows, counters) = selected.into_rows_and_counters();
        let available_source_row_count = selected_rows.len();
        if selected_rows.is_empty() {
            return Err(VertexDiskMigrationError::ReadStageTouchedClosureSelectedNoVertexDiskRows);
        }

        let query_report_digests =
            selected_plan_query_report_digests(selected_plan).unwrap_or_default();
        Self::from_rows_with_counters_and_query_reports(
            selected_rows,
            available_source_row_count,
            counters,
            query_report_digests,
        )
    }

    #[cfg(test)]
    pub(crate) fn from_rows(
        selected_rows: Vec<VertexDiskBoundarySourceRow>,
        available_source_row_count: usize,
    ) -> Result<Self, VertexDiskMigrationError> {
        let selected_source_row_count = selected_rows.len();
        Self::from_rows_with_counters_and_query_reports(
            selected_rows,
            available_source_row_count,
            VertexDiskReadStageCounters::for_selected_rows(
                selected_source_row_count,
                available_source_row_count,
            ),
            vec!["query.native.read.receipt".to_string()],
        )
    }

    #[cfg(test)]
    pub(crate) fn from_rows_with_counters(
        selected_rows: Vec<VertexDiskBoundarySourceRow>,
        available_source_row_count: usize,
        counters: VertexDiskReadStageCounters,
    ) -> Result<Self, VertexDiskMigrationError> {
        Self::from_rows_with_counters_and_query_reports(
            selected_rows,
            available_source_row_count,
            counters,
            Vec::new(),
        )
    }

    fn from_rows_with_counters_and_query_reports(
        selected_rows: Vec<VertexDiskBoundarySourceRow>,
        available_source_row_count: usize,
        counters: VertexDiskReadStageCounters,
        query_report_digests: Vec<String>,
    ) -> Result<Self, VertexDiskMigrationError> {
        if selected_rows.len() > available_source_row_count {
            return Err(VertexDiskMigrationError::SelectedRowsExceedAvailableRows);
        }
        if selected_rows.iter().any(|row| {
            row.touched_vertex_identities().is_empty()
                || row.touched_source_identity().trim().is_empty()
                || row.source_half_edge_identity().trim().is_empty()
        }) {
            return Err(VertexDiskMigrationError::ReadStageQueryProofInvalid);
        }
        if counters.selected_source_row_count() != selected_rows.len()
            || counters.available_source_row_count() != available_source_row_count
        {
            return Err(VertexDiskMigrationError::ReadStageCountersNotBoundToRows);
        }
        Ok(Self {
            read_source_digest: vertex_disk_read_source_digest(
                &selected_rows,
                available_source_row_count,
                &counters,
                &query_report_digests,
            ),
            selected_rows,
            available_source_row_count,
            counters,
            query_report_digests,
        })
    }

    pub fn selected_rows(&self) -> &[VertexDiskBoundarySourceRow] {
        &self.selected_rows
    }

    pub const fn available_source_row_count(&self) -> usize {
        self.available_source_row_count
    }

    pub fn read_source_digest(&self) -> &str {
        &self.read_source_digest
    }

    pub fn query_report_digests(&self) -> &[String] {
        self.query_report_digests.as_slice()
    }

    pub const fn counters(&self) -> &VertexDiskReadStageCounters {
        &self.counters
    }
}

#[cfg(test)]
fn selected_plan_query_report_digests(
    selected_plan: &DerivedInvalidationSelectedPlan,
) -> Option<Vec<String>> {
    selected_plan
        .selected_rows()
        .iter()
        .find(|row| row.family_identity() == crate::derived_topology::invalidation_plan::catalog::DerivedTopologyProductFamilyIdentity::VertexDisks)
        .and_then(|row| row.query_receipt_digest())
        .map(|digest| vec![digest.to_string()])
}

fn vertex_disk_read_source_digest(
    rows: &[VertexDiskBoundarySourceRow],
    available_source_row_count: usize,
    counters: &VertexDiskReadStageCounters,
    query_report_digests: &[String],
) -> String {
    let mut parts = vec![
        "worth-topo:vertex-disk-read-source:v1".to_string(),
        format!("selected-rows:{}", rows.len()),
        format!("available-source-rows:{available_source_row_count}"),
        format!("touched-vertices:{}", counters.touched_vertex_count()),
        format!(
            "touched-half-edge-lookups:{}",
            counters.touched_half_edge_lookup_count()
        ),
        format!(
            "selected-vertex-disk-roots:{}",
            counters.selected_vertex_disk_root_count()
        ),
        format!(
            "touched-incident-half-edges:{}",
            counters.touched_incident_half_edge_count()
        ),
        format!(
            "touched-incident-edges:{}",
            counters.touched_incident_edge_count()
        ),
        format!(
            "unrelated-vertex-disk-breadth:{}",
            counters.unrelated_vertex_disk_breadth_count()
        ),
        format!(
            "whole-view-fallbacks:{}",
            counters.whole_view_fallback_count()
        ),
    ];
    parts.extend(
        query_report_digests
            .iter()
            .map(|digest| format!("query-report:{digest}")),
    );
    parts.extend(rows.iter().map(|row| {
        format!(
            "row:{}:{}:{}:{}:{}:{}",
            row.touched_source_identity(),
            row.source_half_edge_identity(),
            row.source_edge_identity(),
            row.touched_vertex_identities().len(),
            row.incident_half_edge_identities().len(),
            row.touched_incident_edge_count()
        )
    }));
    super::super::super::super::catalog::catalog_digest(parts)
}
