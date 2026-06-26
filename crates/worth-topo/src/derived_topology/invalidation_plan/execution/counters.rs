use serde::Serialize;

use super::rows::{
    DerivedInvalidationDeniedProductExecutionRow, DerivedInvalidationExecutedProductRow,
    DerivedInvalidationResidueExecutionRow, DerivedInvalidationUnaffectedProductExecutionRow,
};
use super::DerivedInvalidationExecutionOutcome;

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct DerivedInvalidationExecutionCounters {
    executed_product_count: usize,
    incremental_update_count: usize,
    bounded_rebuild_count: usize,
    unaffected_product_count: usize,
    denied_product_count: usize,
    residue_product_count: usize,
    whole_view_fallback_count: usize,
    caller_owned_graph_work_count: usize,
    diagnostic_row_count: usize,
    counters_digest: String,
}

impl DerivedInvalidationExecutionCounters {
    pub(super) fn from_rows(
        executed_rows: &[DerivedInvalidationExecutedProductRow],
        unaffected_rows: &[DerivedInvalidationUnaffectedProductExecutionRow],
        denied_rows: &[DerivedInvalidationDeniedProductExecutionRow],
        residue_rows: &[DerivedInvalidationResidueExecutionRow],
    ) -> Self {
        let incremental_update_count = executed_rows
            .iter()
            .filter(|row| row.outcome() == DerivedInvalidationExecutionOutcome::IncrementalUpdated)
            .count();
        let bounded_rebuild_count = executed_rows
            .iter()
            .filter(|row| row.outcome() == DerivedInvalidationExecutionOutcome::BoundedRebuilt)
            .count();
        let whole_view_fallback_count = executed_rows
            .iter()
            .map(DerivedInvalidationExecutedProductRow::whole_view_fallback_count)
            .sum::<usize>();
        let caller_owned_graph_work_count = executed_rows
            .iter()
            .map(DerivedInvalidationExecutedProductRow::caller_owned_graph_work_count)
            .sum::<usize>();
        let diagnostic_row_count =
            executed_rows.len() + unaffected_rows.len() + denied_rows.len() + residue_rows.len();
        let counters_digest = super::super::catalog::catalog_digest([
            "worth-topo:derived-invalidation-execution-counters:v1".to_string(),
            format!("executed-products:{}", executed_rows.len()),
            format!("incremental-updates:{incremental_update_count}"),
            format!("bounded-rebuilds:{bounded_rebuild_count}"),
            format!("unaffected-products:{}", unaffected_rows.len()),
            format!("denied-products:{}", denied_rows.len()),
            format!("residue-products:{}", residue_rows.len()),
            format!("whole-view-fallbacks:{whole_view_fallback_count}"),
            format!("caller-owned-graph-work:{caller_owned_graph_work_count}"),
            format!("diagnostic-rows:{diagnostic_row_count}"),
        ]);
        Self {
            executed_product_count: executed_rows.len(),
            incremental_update_count,
            bounded_rebuild_count,
            unaffected_product_count: unaffected_rows.len(),
            denied_product_count: denied_rows.len(),
            residue_product_count: residue_rows.len(),
            whole_view_fallback_count,
            caller_owned_graph_work_count,
            diagnostic_row_count,
            counters_digest,
        }
    }

    pub const fn executed_product_count(&self) -> usize {
        self.executed_product_count
    }

    pub const fn incremental_update_count(&self) -> usize {
        self.incremental_update_count
    }

    pub const fn bounded_rebuild_count(&self) -> usize {
        self.bounded_rebuild_count
    }

    pub const fn unaffected_product_count(&self) -> usize {
        self.unaffected_product_count
    }

    pub const fn denied_product_count(&self) -> usize {
        self.denied_product_count
    }

    pub const fn residue_product_count(&self) -> usize {
        self.residue_product_count
    }

    pub const fn whole_view_fallback_count(&self) -> usize {
        self.whole_view_fallback_count
    }

    pub const fn caller_owned_graph_work_count(&self) -> usize {
        self.caller_owned_graph_work_count
    }

    pub const fn diagnostic_row_count(&self) -> usize {
        self.diagnostic_row_count
    }

    pub fn counters_digest(&self) -> &str {
        &self.counters_digest
    }
}
