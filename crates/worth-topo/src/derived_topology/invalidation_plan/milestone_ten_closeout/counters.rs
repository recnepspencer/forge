use serde::Serialize;

use super::performance_proof::DerivedInvalidationMilestoneTenPerformanceProof;
use super::product_summary::DerivedInvalidationMilestoneTenProductSummaryReport;
use crate::derived_topology::invalidation_plan::deletion_closeout::DerivedInvalidationDeletionCloseout;
use crate::derived_topology::invalidation_plan::execution::DerivedInvalidationExecutionReceipt;
use crate::derived_topology::invalidation_plan::migrated_products::CoveredDerivedProductMigrationSweepCloseout;

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct DerivedInvalidationMilestoneTenCounters {
    required_family_count: usize,
    summary_row_count: usize,
    executed_product_count: usize,
    unaffected_product_count: usize,
    denied_product_count: usize,
    residue_product_count: usize,
    ordinary_consumable_family_count: usize,
    source_firewall_violation_count: usize,
    old_authority_residue_count: usize,
    whole_view_fallback_count: usize,
    caller_owned_graph_work_count: usize,
    slope_case_count: usize,
    counters_digest: String,
}

impl DerivedInvalidationMilestoneTenCounters {
    pub(crate) fn from_products(
        migration_sweep: &CoveredDerivedProductMigrationSweepCloseout,
        execution_receipt: &DerivedInvalidationExecutionReceipt,
        deletion_closeout: &DerivedInvalidationDeletionCloseout,
        summary: &DerivedInvalidationMilestoneTenProductSummaryReport,
        performance: &DerivedInvalidationMilestoneTenPerformanceProof,
    ) -> Self {
        let required_family_count = migration_sweep.counters().required_family_count();
        let summary_row_count = summary.rows().len();
        let executed_product_count = execution_receipt.counters().executed_product_count();
        let unaffected_product_count = execution_receipt.counters().unaffected_product_count();
        let denied_product_count = execution_receipt.counters().denied_product_count();
        let residue_product_count = execution_receipt.counters().residue_product_count();
        let ordinary_consumable_family_count = migration_sweep
            .counters()
            .ordinary_consumable_family_count();
        let source_firewall_violation_count = deletion_closeout
            .counters()
            .source_firewall_violation_count();
        let old_authority_residue_count = deletion_closeout.counters().ordinary_dirty_path_count()
            + deletion_closeout
                .counters()
                .ordinary_whole_view_rebuild_count();
        let whole_view_fallback_count = execution_receipt.counters().whole_view_fallback_count();
        let caller_owned_graph_work_count =
            execution_receipt.counters().caller_owned_graph_work_count();
        let slope_case_count = performance.slope_cases().len();
        let counters_digest = super::super::catalog::catalog_digest([
            "worth-topo:derived-invalidation-milestone-ten-counters:v1".to_string(),
            format!("required-families:{required_family_count}"),
            format!("summary-rows:{summary_row_count}"),
            format!("executed-products:{executed_product_count}"),
            format!("unaffected-products:{unaffected_product_count}"),
            format!("denied-products:{denied_product_count}"),
            format!("residue-products:{residue_product_count}"),
            format!("ordinary-consumable:{ordinary_consumable_family_count}"),
            format!("source-firewall-violations:{source_firewall_violation_count}"),
            format!("old-authority-residue:{old_authority_residue_count}"),
            format!("whole-view-fallbacks:{whole_view_fallback_count}"),
            format!("caller-owned-graph-work:{caller_owned_graph_work_count}"),
            format!("slope-cases:{slope_case_count}"),
        ]);
        Self {
            required_family_count,
            summary_row_count,
            executed_product_count,
            unaffected_product_count,
            denied_product_count,
            residue_product_count,
            ordinary_consumable_family_count,
            source_firewall_violation_count,
            old_authority_residue_count,
            whole_view_fallback_count,
            caller_owned_graph_work_count,
            slope_case_count,
            counters_digest,
        }
    }

    pub const fn required_family_count(&self) -> usize {
        self.required_family_count
    }

    pub const fn summary_row_count(&self) -> usize {
        self.summary_row_count
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

    pub const fn ordinary_consumable_family_count(&self) -> usize {
        self.ordinary_consumable_family_count
    }

    pub const fn source_firewall_violation_count(&self) -> usize {
        self.source_firewall_violation_count
    }

    pub const fn old_authority_residue_count(&self) -> usize {
        self.old_authority_residue_count
    }

    pub const fn whole_view_fallback_count(&self) -> usize {
        self.whole_view_fallback_count
    }

    pub const fn caller_owned_graph_work_count(&self) -> usize {
        self.caller_owned_graph_work_count
    }

    pub const fn slope_case_count(&self) -> usize {
        self.slope_case_count
    }

    pub fn counters_digest(&self) -> &str {
        &self.counters_digest
    }
}
