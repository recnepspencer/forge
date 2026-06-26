use serde::Serialize;

use super::super::{ShellViewBoundarySourceRow, ShellViewMigrationError};
use super::counters::ShellViewReadStageCounters;
use super::query_proof::query_report_digests_for_shell_boundary_views;
#[cfg(test)]
use super::touched_topology_selection::ShellViewTouchedTopologySelection;
#[cfg(test)]
use crate::brep::topology_graph::TopologyView;
use crate::derived_topology::invalidation_plan::selection::{
    DerivedInvalidationSelectedPlan, DerivedInvalidationTouchedClosure,
};
use crate::projection::read_views::TopologyShellBoundaryNeighborhoodView;

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct ShellViewReadSource {
    selected_rows: Vec<ShellViewBoundarySourceRow>,
    available_source_row_count: usize,
    counters: ShellViewReadStageCounters,
    query_report_digests: Vec<String>,
    read_source_digest: String,
}

impl ShellViewReadSource {
    pub fn from_query_shell_boundary_views(
        selected_plan: &DerivedInvalidationSelectedPlan,
        touched_closure: &DerivedInvalidationTouchedClosure,
        shell_boundary_views: &[TopologyShellBoundaryNeighborhoodView],
    ) -> Result<Self, ShellViewMigrationError> {
        if selected_plan.touched_closure_digest() != touched_closure.closure_digest() {
            return Err(ShellViewMigrationError::ReadStageTouchedClosureNotBoundToSelectedPlan);
        }

        let query_report_digests =
            query_report_digests_for_shell_boundary_views(shell_boundary_views)?;
        let rows = shell_boundary_views
            .iter()
            .map(ShellViewBoundarySourceRow::from_query_shell_boundary_view)
            .collect::<Vec<_>>();
        let counters = ShellViewReadStageCounters::new(
            touched_closure.basis().entities().len() + touched_closure.basis().relations().len(),
            shell_boundary_views.len(),
            shell_boundary_views.len(),
            rows.len(),
            rows.len(),
            rows.len(),
            shell_boundary_views
                .iter()
                .map(|view| view.same_edge_half_edge_identities().len())
                .sum(),
            0,
            0,
        );
        Self::from_rows_with_counters_and_query_reports(
            rows,
            shell_boundary_views.len(),
            counters,
            query_report_digests,
        )
    }

    #[cfg(test)]
    pub fn select_from_touched_closure(
        selected_plan: &DerivedInvalidationSelectedPlan,
        touched_closure: &DerivedInvalidationTouchedClosure,
        topology: &TopologyView,
    ) -> Result<Self, ShellViewMigrationError> {
        if selected_plan.touched_closure_digest() != touched_closure.closure_digest() {
            return Err(ShellViewMigrationError::ReadStageTouchedClosureNotBoundToSelectedPlan);
        }

        let selected = ShellViewTouchedTopologySelection::from_touched_closure_and_topology(
            touched_closure,
            topology,
        );
        let (selected_rows, counters) = selected.into_rows_and_counters();
        let available_source_row_count = selected_rows.len();
        if selected_rows.is_empty() {
            return Err(ShellViewMigrationError::ReadStageTouchedClosureSelectedNoShellViewRows);
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
        selected_rows: Vec<ShellViewBoundarySourceRow>,
        available_source_row_count: usize,
    ) -> Result<Self, ShellViewMigrationError> {
        let selected_source_row_count = selected_rows.len();
        Self::from_rows_with_counters_and_query_reports(
            selected_rows,
            available_source_row_count,
            ShellViewReadStageCounters::for_selected_rows(
                selected_source_row_count,
                available_source_row_count,
            ),
            vec!["query.native.read.receipt".to_string()],
        )
    }

    pub(crate) fn from_rows_with_counters(
        selected_rows: Vec<ShellViewBoundarySourceRow>,
        available_source_row_count: usize,
        counters: ShellViewReadStageCounters,
    ) -> Result<Self, ShellViewMigrationError> {
        Self::from_rows_with_counters_and_query_reports(
            selected_rows,
            available_source_row_count,
            counters,
            Vec::new(),
        )
    }

    pub(crate) fn from_rows_with_counters_and_query_reports(
        selected_rows: Vec<ShellViewBoundarySourceRow>,
        available_source_row_count: usize,
        counters: ShellViewReadStageCounters,
        query_report_digests: Vec<String>,
    ) -> Result<Self, ShellViewMigrationError> {
        if selected_rows.len() > available_source_row_count {
            return Err(ShellViewMigrationError::SelectedRowsExceedAvailableRows);
        }
        if selected_rows.iter().any(|row| {
            row.touched_shell_identity().trim().is_empty()
                || row.touched_source_identity().trim().is_empty()
        }) {
            return Err(ShellViewMigrationError::ReadStageQueryProofInvalid);
        }
        if counters.selected_source_row_count() != selected_rows.len()
            || counters.available_source_row_count() != available_source_row_count
        {
            return Err(ShellViewMigrationError::ReadStageCountersNotBoundToRows);
        }
        Ok(Self {
            read_source_digest: shell_view_read_source_digest(
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

    pub fn selected_rows(&self) -> &[ShellViewBoundarySourceRow] {
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

    pub const fn counters(&self) -> &ShellViewReadStageCounters {
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
        .find(|row| row.family_identity() == crate::derived_topology::invalidation_plan::catalog::DerivedTopologyProductFamilyIdentity::ShellViews)
        .and_then(|row| row.query_receipt_digest())
        .map(|digest| vec![digest.to_string()])
}

fn shell_view_read_source_digest(
    rows: &[ShellViewBoundarySourceRow],
    available_source_row_count: usize,
    counters: &ShellViewReadStageCounters,
    query_report_digests: &[String],
) -> String {
    let mut parts = vec![
        "worth-topo:shell-view-read-source:v1".to_string(),
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
            "row:{}:{}:{}:{}:{}:{}:{}:{}",
            row.touched_shell_identity(),
            row.touched_source_identity(),
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
