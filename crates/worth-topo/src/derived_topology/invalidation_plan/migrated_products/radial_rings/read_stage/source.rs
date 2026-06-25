use serde::Serialize;

use super::super::{RadialRingBoundarySourceRow, RadialRingMigrationError};
use super::counters::RadialRingReadStageCounters;
use super::query_proof::query_report_digests_for_radial_views;
#[cfg(test)]
use super::touched_topology_selection::RadialRingTouchedTopologySelection;
#[cfg(test)]
use crate::brep::topology_graph::TopologyView;
use crate::derived_topology::invalidation_plan::selection::{
    DerivedInvalidationSelectedPlan, DerivedInvalidationTouchedClosure,
};
use crate::projection::read_views::TopologyHalfEdgeRadialNeighborhoodView;

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct RadialRingReadSource {
    selected_rows: Vec<RadialRingBoundarySourceRow>,
    available_source_row_count: usize,
    counters: RadialRingReadStageCounters,
    query_report_digests: Vec<String>,
    read_source_digest: String,
}

impl RadialRingReadSource {
    pub fn from_query_radial_neighborhood_views(
        selected_plan: &DerivedInvalidationSelectedPlan,
        touched_closure: &DerivedInvalidationTouchedClosure,
        radial_views: &[TopologyHalfEdgeRadialNeighborhoodView],
    ) -> Result<Self, RadialRingMigrationError> {
        if selected_plan.touched_closure_digest() != touched_closure.closure_digest() {
            return Err(RadialRingMigrationError::ReadStageTouchedClosureNotBoundToSelectedPlan);
        }

        let query_report_digests = query_report_digests_for_radial_views(radial_views)?;
        let rows = radial_views
            .iter()
            .map(RadialRingBoundarySourceRow::from_query_radial_neighborhood_view)
            .collect::<Vec<_>>();
        let counters = RadialRingReadStageCounters::new(
            touched_closure.basis().entities().len() + touched_closure.basis().relations().len(),
            radial_views.len(),
            radial_views.len(),
            rows.len(),
            rows.len(),
            rows.len(),
            radial_views
                .iter()
                .map(|view| view.same_edge_half_edge_identities().len())
                .sum(),
            0,
            0,
        );
        Self::from_rows_with_counters_and_query_reports(
            rows,
            radial_views.len(),
            counters,
            query_report_digests,
        )
    }

    #[cfg(test)]
    pub fn select_from_touched_closure(
        selected_plan: &DerivedInvalidationSelectedPlan,
        touched_closure: &DerivedInvalidationTouchedClosure,
        topology: &TopologyView,
    ) -> Result<Self, RadialRingMigrationError> {
        if selected_plan.touched_closure_digest() != touched_closure.closure_digest() {
            return Err(RadialRingMigrationError::ReadStageTouchedClosureNotBoundToSelectedPlan);
        }

        let selected = RadialRingTouchedTopologySelection::from_touched_closure_and_topology(
            touched_closure,
            topology,
        );
        let (selected_rows, counters) = selected.into_rows_and_counters();
        let available_source_row_count = selected_rows.len();
        if selected_rows.is_empty() {
            return Err(RadialRingMigrationError::ReadStageTouchedClosureSelectedNoRadialRingRows);
        }

        Self::from_rows_with_counters(selected_rows, available_source_row_count, counters)
    }

    #[cfg(test)]
    pub(crate) fn from_rows(
        selected_rows: Vec<RadialRingBoundarySourceRow>,
        available_source_row_count: usize,
    ) -> Result<Self, RadialRingMigrationError> {
        let selected_source_row_count = selected_rows.len();
        Self::from_rows_with_counters(
            selected_rows,
            available_source_row_count,
            RadialRingReadStageCounters::for_selected_rows(
                selected_source_row_count,
                available_source_row_count,
            ),
        )
    }

    pub(crate) fn from_rows_with_counters(
        selected_rows: Vec<RadialRingBoundarySourceRow>,
        available_source_row_count: usize,
        counters: RadialRingReadStageCounters,
    ) -> Result<Self, RadialRingMigrationError> {
        Self::from_rows_with_counters_and_query_reports(
            selected_rows,
            available_source_row_count,
            counters,
            Vec::new(),
        )
    }

    pub(crate) fn from_rows_with_counters_and_query_reports(
        selected_rows: Vec<RadialRingBoundarySourceRow>,
        available_source_row_count: usize,
        counters: RadialRingReadStageCounters,
        query_report_digests: Vec<String>,
    ) -> Result<Self, RadialRingMigrationError> {
        if selected_rows.len() > available_source_row_count {
            return Err(RadialRingMigrationError::SelectedRowsExceedAvailableRows);
        }
        if counters.selected_source_row_count() != selected_rows.len()
            || counters.available_source_row_count() != available_source_row_count
        {
            return Err(RadialRingMigrationError::ReadStageCountersNotBoundToRows);
        }
        Ok(Self {
            read_source_digest: radial_ring_read_source_digest(
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

    pub fn selected_rows(&self) -> &[RadialRingBoundarySourceRow] {
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

    pub const fn counters(&self) -> &RadialRingReadStageCounters {
        &self.counters
    }
}

fn radial_ring_read_source_digest(
    rows: &[RadialRingBoundarySourceRow],
    available_source_row_count: usize,
    counters: &RadialRingReadStageCounters,
    query_report_digests: &[String],
) -> String {
    let mut parts = vec![
        "worth-topo:radial-ring-read-source:v1".to_string(),
        format!("selected-rows:{}", rows.len()),
        format!("available-source-rows:{available_source_row_count}"),
        format!("touched-anchors:{}", counters.touched_anchor_count()),
        format!("half-edge-lookups:{}", counters.half_edge_lookup_count()),
        format!(
            "radial-relation-lookups:{}",
            counters.radial_relation_lookup_count()
        ),
        format!(
            "selected-radial-roots:{}",
            counters.selected_radial_root_count()
        ),
        format!(
            "touched-neighborhood-breadth:{}",
            counters.touched_neighborhood_breadth_count()
        ),
        format!(
            "unrelated-source-breadth:{}",
            counters.unrelated_source_breadth_count()
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
            row.source_half_edge_identity(),
            row.source_edge_identity(),
            row.radial_target_half_edge_identity(),
            row.ring_half_edge_count(),
            row.boundary_half_edge(),
            row.non_manifold_edge()
        )
    }));
    super::super::super::super::catalog::catalog_digest(parts)
}
