use serde::Serialize;

use super::super::execution::DerivedInvalidationExecutionReceipt;
use super::super::migrated_products::CoveredDerivedProductMigrationSweepCloseout;
use super::super::selection::DerivedInvalidationSelectedPlan;

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct DerivedInvalidationOperatorCutoverCounters {
    selected_product_count: usize,
    executed_product_count: usize,
    unaffected_product_count: usize,
    denied_product_count: usize,
    residue_product_count: usize,
    migrated_ordinary_product_count: usize,
    projection_dirty_expansion_count: usize,
    whole_view_fallback_count: usize,
    caller_owned_graph_work_count: usize,
    counters_digest: String,
}

#[cfg_attr(not(test), allow(dead_code))]
impl DerivedInvalidationOperatorCutoverCounters {
    pub(crate) fn from_proofs(
        selected_plan: &DerivedInvalidationSelectedPlan,
        execution_receipt: &DerivedInvalidationExecutionReceipt,
        phase_six_closeout: &CoveredDerivedProductMigrationSweepCloseout,
        projection_dirty_expansion_count: usize,
    ) -> Self {
        let execution_counters = execution_receipt.counters();
        let selected_product_count = selected_plan.selected_rows().len();
        let executed_product_count = execution_counters.executed_product_count();
        let unaffected_product_count = execution_counters.unaffected_product_count();
        let denied_product_count = execution_counters.denied_product_count();
        let residue_product_count = execution_counters.residue_product_count();
        let migrated_ordinary_product_count = phase_six_closeout
            .counters()
            .ordinary_consumable_family_count();
        let whole_view_fallback_count = execution_counters.whole_view_fallback_count();
        let caller_owned_graph_work_count = execution_counters.caller_owned_graph_work_count();
        let counters_digest = super::super::catalog::catalog_digest([
            "worth-topo:derived-invalidation-operator-cutover-counters:v1".to_string(),
            format!("selected-products:{selected_product_count}"),
            format!("executed-products:{executed_product_count}"),
            format!("unaffected-products:{unaffected_product_count}"),
            format!("denied-products:{denied_product_count}"),
            format!("residue-products:{residue_product_count}"),
            format!("migrated-ordinary-products:{migrated_ordinary_product_count}"),
            format!("projection-dirty-expansion:{projection_dirty_expansion_count}"),
            format!("whole-view-fallback:{whole_view_fallback_count}"),
            format!("caller-owned-graph-work:{caller_owned_graph_work_count}"),
        ]);
        Self {
            selected_product_count,
            executed_product_count,
            unaffected_product_count,
            denied_product_count,
            residue_product_count,
            migrated_ordinary_product_count,
            projection_dirty_expansion_count,
            whole_view_fallback_count,
            caller_owned_graph_work_count,
            counters_digest,
        }
    }

    pub const fn selected_product_count(&self) -> usize {
        self.selected_product_count
    }

    pub const fn executed_product_count(&self) -> usize {
        self.executed_product_count
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

    pub const fn migrated_ordinary_product_count(&self) -> usize {
        self.migrated_ordinary_product_count
    }

    pub const fn projection_dirty_expansion_count(&self) -> usize {
        self.projection_dirty_expansion_count
    }

    pub const fn whole_view_fallback_count(&self) -> usize {
        self.whole_view_fallback_count
    }

    pub const fn caller_owned_graph_work_count(&self) -> usize {
        self.caller_owned_graph_work_count
    }

    pub fn counters_digest(&self) -> &str {
        &self.counters_digest
    }
}
