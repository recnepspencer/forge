use serde::Serialize;

use super::super::WireViewMigrationError;
use super::super::WireViewSourceRow;
use super::counters::WireViewReadStageCounters;
use super::query_proof::query_report_digests_for_wire_views;
use super::query_read::WireViewQueryReadRow;
use crate::derived_topology::invalidation_plan::selection::{
    DerivedInvalidationSelectedPlan, DerivedInvalidationTouchedClosure,
};

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct WireViewReadSource {
    selected_rows: Vec<WireViewSourceRow>,
    available_source_row_count: usize,
    counters: WireViewReadStageCounters,
    query_report_digests: Vec<String>,
    read_source_digest: String,
}

impl WireViewReadSource {
    pub fn from_query_wire_views(
        selected_plan: &DerivedInvalidationSelectedPlan,
        touched_closure: &DerivedInvalidationTouchedClosure,
        wire_views: &[WireViewQueryReadRow],
    ) -> Result<Self, WireViewMigrationError> {
        if selected_plan.touched_closure_digest() != touched_closure.closure_digest() {
            return Err(WireViewMigrationError::ReadStageTouchedClosureNotBoundToSelectedPlan);
        }
        let query_report_digests = query_report_digests_for_wire_views(wire_views)?;
        let selected_rows: Vec<_> = wire_views.iter().map(source_row_from_query_view).collect();
        let counters = counters_from_query_wire_views(&selected_rows, wire_views);
        Self::from_validated_rows_with_query_reports(
            selected_rows,
            wire_views.len(),
            counters,
            query_report_digests,
        )
    }

    #[cfg(test)]
    pub fn from_rows_with_query_reports(
        selected_plan: &DerivedInvalidationSelectedPlan,
        touched_closure: &DerivedInvalidationTouchedClosure,
        selected_rows: Vec<WireViewSourceRow>,
        available_source_row_count: usize,
        counters: WireViewReadStageCounters,
        query_report_digests: Vec<String>,
    ) -> Result<Self, WireViewMigrationError> {
        if selected_plan.touched_closure_digest() != touched_closure.closure_digest() {
            return Err(WireViewMigrationError::ReadStageTouchedClosureNotBoundToSelectedPlan);
        }
        Self::from_validated_rows_with_query_reports(
            selected_rows,
            available_source_row_count,
            counters,
            query_report_digests,
        )
    }

    fn from_validated_rows_with_query_reports(
        selected_rows: Vec<WireViewSourceRow>,
        available_source_row_count: usize,
        counters: WireViewReadStageCounters,
        query_report_digests: Vec<String>,
    ) -> Result<Self, WireViewMigrationError> {
        if selected_rows.len() > available_source_row_count {
            return Err(WireViewMigrationError::SelectedRowsExceedAvailableRows);
        }
        if selected_rows.iter().any(|row| row.half_edge_count() == 0) {
            return Err(WireViewMigrationError::ReadStageQueryProofInvalid);
        }
        if counters.selected_source_row_count() != selected_rows.len()
            || counters.available_source_row_count() != available_source_row_count
        {
            return Err(WireViewMigrationError::ReadStageCountersNotBoundToRows);
        }
        Ok(Self {
            read_source_digest: wire_view_read_source_digest(
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

    pub fn selected_rows(&self) -> &[WireViewSourceRow] {
        &self.selected_rows
    }

    pub const fn available_source_row_count(&self) -> usize {
        self.available_source_row_count
    }

    pub const fn counters(&self) -> &WireViewReadStageCounters {
        &self.counters
    }

    pub fn query_report_digests(&self) -> &[String] {
        self.query_report_digests.as_slice()
    }

    pub fn read_source_digest(&self) -> &str {
        &self.read_source_digest
    }
}

fn wire_view_read_source_digest(
    rows: &[WireViewSourceRow],
    available_source_row_count: usize,
    counters: &WireViewReadStageCounters,
    query_report_digests: &[String],
) -> String {
    let mut parts = vec![
        "worth-topo:wire-view-read-source:v1".to_string(),
        format!("selected-rows:{}", rows.len()),
        format!("available-source-rows:{available_source_row_count}"),
        format!("touched-wires:{}", counters.touched_wire_count()),
        format!(
            "touched-half-edge-lookups:{}",
            counters.touched_half_edge_lookup_count()
        ),
        format!(
            "selected-wire-roots:{}",
            counters.selected_wire_root_count()
        ),
        format!(
            "touched-terminal-vertices:{}",
            counters.touched_terminal_vertex_count()
        ),
        format!(
            "touched-branch-vertices:{}",
            counters.touched_branch_vertex_count()
        ),
        format!(
            "unrelated-wire-breadth:{}",
            counters.unrelated_wire_breadth_count()
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
            "row:{:?}:{:?}:{}:{}:{}:{}",
            row.wire_id(),
            row.class(),
            row.connected_component_count(),
            row.half_edge_count(),
            row.terminal_vertex_ids().len(),
            row.branch_vertex_ids().len()
        )
    }));
    super::super::super::super::catalog::catalog_digest(parts)
}

fn source_row_from_query_view(view: &WireViewQueryReadRow) -> WireViewSourceRow {
    WireViewSourceRow::new(
        view.wire_id(),
        view.class(),
        view.connected_component_count(),
        view.half_edge_ids().len(),
        view.terminal_vertex_ids().to_vec(),
        view.branch_vertex_ids().to_vec(),
    )
}

fn counters_from_query_wire_views(
    rows: &[WireViewSourceRow],
    views: &[WireViewQueryReadRow],
) -> WireViewReadStageCounters {
    let touched_half_edge_lookup_count = views
        .iter()
        .map(|view| view.half_edge_ids().len())
        .sum::<usize>();
    let touched_terminal_vertex_count = views
        .iter()
        .map(|view| view.terminal_vertex_ids().len())
        .sum::<usize>();
    let touched_branch_vertex_count = views
        .iter()
        .map(|view| view.branch_vertex_ids().len())
        .sum::<usize>();
    WireViewReadStageCounters::new(
        views.len(),
        touched_half_edge_lookup_count,
        rows.len(),
        rows.len(),
        views.len(),
        touched_terminal_vertex_count,
        touched_branch_vertex_count,
        0,
        0,
    )
}
