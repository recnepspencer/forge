use serde::Serialize;

use super::super::LoopCycleBoundarySourceRow;
#[cfg(test)]
use super::super::LoopCycleMigrationError;
use super::counters::LoopCycleReadStageCounters;
#[cfg(test)]
use super::touched_topology_selection::LoopCycleTouchedTopologySelection;
#[cfg(test)]
use crate::brep::topology_graph::TopologyView;
#[cfg(test)]
use crate::derived_topology::invalidation_plan::selection::{
    DerivedInvalidationSelectedPlan, DerivedInvalidationTouchedClosure,
};

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct LoopCycleReadSource {
    selected_rows: Vec<LoopCycleBoundarySourceRow>,
    available_source_row_count: usize,
    counters: LoopCycleReadStageCounters,
    read_source_digest: String,
}

impl LoopCycleReadSource {
    #[cfg(test)]
    pub(crate) fn select_from_touched_closure(
        selected_plan: &DerivedInvalidationSelectedPlan,
        touched_closure: &DerivedInvalidationTouchedClosure,
        topology: &TopologyView,
    ) -> Result<Self, LoopCycleMigrationError> {
        if selected_plan.touched_closure_digest() != touched_closure.closure_digest() {
            return Err(LoopCycleMigrationError::ReadStageTouchedClosureNotBoundToSelectedPlan);
        }

        let selected = LoopCycleTouchedTopologySelection::from_touched_closure_and_topology(
            touched_closure,
            topology,
        );
        let (selected_rows, counters) = selected.into_rows_and_counters();
        let available_source_row_count = selected_rows.len();
        if selected_rows.is_empty() {
            return Err(LoopCycleMigrationError::ReadStageTouchedClosureSelectedNoLoopCycleRows);
        }

        Self::from_rows_with_counters(selected_rows, available_source_row_count, counters)
    }

    #[cfg(test)]
    pub(crate) fn from_rows(
        selected_rows: Vec<LoopCycleBoundarySourceRow>,
        available_source_row_count: usize,
    ) -> Result<Self, LoopCycleMigrationError> {
        let selected_source_row_count = selected_rows.len();
        Self::from_rows_with_counters(
            selected_rows,
            available_source_row_count,
            LoopCycleReadStageCounters::for_selected_rows(
                selected_source_row_count,
                available_source_row_count,
            ),
        )
    }

    #[cfg(test)]
    pub(crate) fn from_rows_with_counters(
        selected_rows: Vec<LoopCycleBoundarySourceRow>,
        available_source_row_count: usize,
        counters: LoopCycleReadStageCounters,
    ) -> Result<Self, LoopCycleMigrationError> {
        if selected_rows.len() > available_source_row_count {
            return Err(LoopCycleMigrationError::SelectedRowsExceedAvailableRows);
        }
        if counters.selected_source_row_count() != selected_rows.len()
            || counters.available_source_row_count() != available_source_row_count
        {
            return Err(LoopCycleMigrationError::ReadStageCountersNotBoundToRows);
        }
        Ok(Self {
            read_source_digest: loop_cycle_read_source_digest(
                &selected_rows,
                available_source_row_count,
                &counters,
            ),
            selected_rows,
            available_source_row_count,
            counters,
        })
    }

    pub fn selected_rows(&self) -> &[LoopCycleBoundarySourceRow] {
        &self.selected_rows
    }

    pub const fn available_source_row_count(&self) -> usize {
        self.available_source_row_count
    }

    pub fn read_source_digest(&self) -> &str {
        &self.read_source_digest
    }

    pub const fn counters(&self) -> &LoopCycleReadStageCounters {
        &self.counters
    }
}

#[cfg(test)]
fn loop_cycle_read_source_digest(
    rows: &[LoopCycleBoundarySourceRow],
    available_source_row_count: usize,
    counters: &LoopCycleReadStageCounters,
) -> String {
    let mut parts = vec![
        "worth-topo:loop-cycle-read-source:v1".to_string(),
        format!("selected-rows:{}", rows.len()),
        format!("available-source-rows:{available_source_row_count}"),
        format!("touched-anchors:{}", counters.touched_anchor_count()),
        format!("shell-lookups:{}", counters.shell_lookup_count()),
        format!("face-lookups:{}", counters.face_lookup_count()),
        format!(
            "unrelated-source-breadth:{}",
            counters.unrelated_source_breadth_count()
        ),
        format!(
            "whole-view-fallbacks:{}",
            counters.whole_view_fallback_count()
        ),
    ];
    parts.extend(rows.iter().map(|row| {
        format!(
            "row:{:?}:{}:{}",
            row.shell_id(),
            row.boundary_component_count(),
            row.boundary_half_edge_count()
        )
    }));
    super::super::super::super::catalog::catalog_digest(parts)
}
