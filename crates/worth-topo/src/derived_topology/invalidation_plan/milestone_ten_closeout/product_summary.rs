use serde::Serialize;

use crate::derived_topology::invalidation_plan::catalog::{
    DerivedInvalidationFamilyCatalogCloseout, DerivedTopologyInvalidationPredicate,
    DerivedTopologyProductFamilyIdentity,
};
use crate::derived_topology::invalidation_plan::execution::DerivedInvalidationExecutionReceipt;
use crate::derived_topology::invalidation_plan::migrated_products::{
    CoveredDerivedProductMigrationStatus, CoveredDerivedProductMigrationSweepCloseout,
};
use crate::derived_topology::invalidation_plan::selection::DerivedInvalidationSelectedPlan;

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct DerivedInvalidationMilestoneTenProductSummaryReport {
    selected_plan_digest: String,
    touched_closure_digest: String,
    execution_receipt_digest: String,
    migration_sweep_digest: String,
    rows: Vec<DerivedInvalidationMilestoneTenProductSummaryRow>,
    report_digest: String,
}

impl DerivedInvalidationMilestoneTenProductSummaryReport {
    pub(crate) fn from_products(
        catalog_closeout: &DerivedInvalidationFamilyCatalogCloseout,
        selected_plan: &DerivedInvalidationSelectedPlan,
        migration_sweep: &CoveredDerivedProductMigrationSweepCloseout,
        execution_receipt: &DerivedInvalidationExecutionReceipt,
    ) -> Self {
        let rows = migration_sweep
            .status_rows()
            .iter()
            .map(|status_row| {
                DerivedInvalidationMilestoneTenProductSummaryRow::from_products(
                    catalog_closeout,
                    selected_plan,
                    status_row.family_identity(),
                    status_row.status(),
                    status_row.ordinary_invalidation_consumable(),
                    status_row.proof_digest(),
                    execution_receipt,
                )
            })
            .collect::<Vec<_>>();
        let report_digest = report_digest(selected_plan, migration_sweep, execution_receipt, &rows);
        Self {
            selected_plan_digest: migration_sweep.selected_plan_digest().to_string(),
            touched_closure_digest: selected_plan.touched_closure_digest().to_string(),
            execution_receipt_digest: execution_receipt.execution_receipt_digest().to_string(),
            migration_sweep_digest: migration_sweep.closeout_digest().to_string(),
            rows,
            report_digest,
        }
    }

    pub fn selected_plan_digest(&self) -> &str {
        &self.selected_plan_digest
    }

    pub fn touched_closure_digest(&self) -> &str {
        &self.touched_closure_digest
    }

    pub fn execution_receipt_digest(&self) -> &str {
        &self.execution_receipt_digest
    }

    pub fn migration_sweep_digest(&self) -> &str {
        &self.migration_sweep_digest
    }

    pub fn rows(&self) -> &[DerivedInvalidationMilestoneTenProductSummaryRow] {
        &self.rows
    }

    pub fn report_digest(&self) -> &str {
        &self.report_digest
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct DerivedInvalidationMilestoneTenProductSummaryRow {
    family_identity: DerivedTopologyProductFamilyIdentity,
    migration_status: CoveredDerivedProductMigrationStatus,
    ordinary_invalidation_consumable: bool,
    migration_proof_digest: String,
    selected_row_digest: Option<String>,
    invalidation_predicate: DerivedTopologyInvalidationPredicate,
    consumed_graph_facts_digest: String,
    consumed_relation_kind_count: usize,
    consumed_aspect_count: usize,
    selected_by_touched_closure: bool,
    executed_count: usize,
    unaffected_count: usize,
    denied_count: usize,
    query_receipt_bound_count: usize,
    legality_receipt_bound_count: usize,
    product_output_bound_count: usize,
    execution_work_count: usize,
    whole_view_fallback_count: usize,
    caller_owned_graph_work_count: usize,
    row_digest: String,
}

impl DerivedInvalidationMilestoneTenProductSummaryRow {
    fn from_products(
        catalog_closeout: &DerivedInvalidationFamilyCatalogCloseout,
        selected_plan: &DerivedInvalidationSelectedPlan,
        family_identity: DerivedTopologyProductFamilyIdentity,
        migration_status: CoveredDerivedProductMigrationStatus,
        ordinary_invalidation_consumable: bool,
        migration_proof_digest: &str,
        execution_receipt: &DerivedInvalidationExecutionReceipt,
    ) -> Self {
        let family_record = catalog_closeout
            .catalog()
            .family(family_identity)
            .expect("family closeout already validated required product families");
        let selected_row = selected_plan
            .selected_rows()
            .iter()
            .find(|row| row.family_identity() == family_identity);
        let selected_row_digest = selected_row.map(|row| row.row_digest().to_string());
        let selected_by_touched_closure = selected_row.is_some();
        let consumed_graph_facts = family_record.consumed_graph_facts();
        let consumed_graph_facts_digest =
            super::super::catalog::catalog_digest(consumed_graph_facts.digest_parts());
        let consumed_relation_kind_count = consumed_graph_facts.relation_kinds().len();
        let consumed_aspect_count = consumed_graph_facts.aspects().len();
        let invalidation_predicate = family_record.invalidation_predicate();
        let executed_rows = execution_receipt
            .executed_rows()
            .iter()
            .filter(|row| row.family_identity() == family_identity)
            .collect::<Vec<_>>();
        let executed_count = executed_rows.len();
        let unaffected_count = execution_receipt
            .unaffected_rows()
            .iter()
            .filter(|row| row.family_identity() == family_identity)
            .count();
        let denied_count = execution_receipt
            .denied_rows()
            .iter()
            .filter(|row| row.family_identity() == family_identity)
            .count();
        let query_receipt_bound_count = executed_rows
            .iter()
            .filter(|row| row.query_receipt_digest().is_some())
            .count();
        let legality_receipt_bound_count = executed_rows
            .iter()
            .filter(|row| row.legality_receipt_digest().is_some())
            .count();
        let product_output_bound_count = executed_rows
            .iter()
            .filter(|row| row.product_output_digest().is_some())
            .count();
        let execution_work_count = executed_rows
            .iter()
            .map(|row| row.execution_work_count())
            .sum::<usize>();
        let whole_view_fallback_count = executed_rows
            .iter()
            .map(|row| row.whole_view_fallback_count())
            .sum::<usize>();
        let caller_owned_graph_work_count = executed_rows
            .iter()
            .map(|row| row.caller_owned_graph_work_count())
            .sum::<usize>();
        let row_digest = super::super::catalog::catalog_digest([
            "worth-topo:derived-invalidation-milestone-ten-product-summary-row:v1".to_string(),
            format!("family:{}", family_identity.as_str()),
            format!("migration-status:{}", migration_status.as_str()),
            format!("ordinary-consumable:{ordinary_invalidation_consumable}"),
            format!("migration-proof:{migration_proof_digest}"),
            format!(
                "selected-row:{}",
                selected_row_digest.as_deref().unwrap_or("not-selected")
            ),
            format!("predicate:{}", invalidation_predicate.as_str()),
            format!("consumed-facts:{consumed_graph_facts_digest}"),
            format!("consumed-relations:{consumed_relation_kind_count}"),
            format!("consumed-aspects:{consumed_aspect_count}"),
            format!("selected-by-touched-closure:{selected_by_touched_closure}"),
            format!("executed:{executed_count}"),
            format!("unaffected:{unaffected_count}"),
            format!("denied:{denied_count}"),
            format!("query-receipts:{query_receipt_bound_count}"),
            format!("legality-receipts:{legality_receipt_bound_count}"),
            format!("product-outputs:{product_output_bound_count}"),
            format!("execution-work:{execution_work_count}"),
            format!("whole-view-fallbacks:{whole_view_fallback_count}"),
            format!("caller-owned-graph-work:{caller_owned_graph_work_count}"),
        ]);
        Self {
            family_identity,
            migration_status,
            ordinary_invalidation_consumable,
            migration_proof_digest: migration_proof_digest.to_string(),
            selected_row_digest,
            invalidation_predicate,
            consumed_graph_facts_digest,
            consumed_relation_kind_count,
            consumed_aspect_count,
            selected_by_touched_closure,
            executed_count,
            unaffected_count,
            denied_count,
            query_receipt_bound_count,
            legality_receipt_bound_count,
            product_output_bound_count,
            execution_work_count,
            whole_view_fallback_count,
            caller_owned_graph_work_count,
            row_digest,
        }
    }

    pub const fn family_identity(&self) -> DerivedTopologyProductFamilyIdentity {
        self.family_identity
    }

    pub const fn migration_status(&self) -> CoveredDerivedProductMigrationStatus {
        self.migration_status
    }

    pub const fn ordinary_invalidation_consumable(&self) -> bool {
        self.ordinary_invalidation_consumable
    }

    pub fn migration_proof_digest(&self) -> &str {
        &self.migration_proof_digest
    }

    pub fn selected_row_digest(&self) -> Option<&str> {
        self.selected_row_digest.as_deref()
    }

    pub const fn invalidation_predicate(&self) -> DerivedTopologyInvalidationPredicate {
        self.invalidation_predicate
    }

    pub fn consumed_graph_facts_digest(&self) -> &str {
        &self.consumed_graph_facts_digest
    }

    pub const fn consumed_relation_kind_count(&self) -> usize {
        self.consumed_relation_kind_count
    }

    pub const fn consumed_aspect_count(&self) -> usize {
        self.consumed_aspect_count
    }

    pub const fn selected_by_touched_closure(&self) -> bool {
        self.selected_by_touched_closure
    }

    pub const fn executed_count(&self) -> usize {
        self.executed_count
    }

    pub const fn unaffected_count(&self) -> usize {
        self.unaffected_count
    }

    pub const fn denied_count(&self) -> usize {
        self.denied_count
    }

    pub const fn query_receipt_bound_count(&self) -> usize {
        self.query_receipt_bound_count
    }

    pub const fn legality_receipt_bound_count(&self) -> usize {
        self.legality_receipt_bound_count
    }

    pub const fn product_output_bound_count(&self) -> usize {
        self.product_output_bound_count
    }

    pub const fn execution_work_count(&self) -> usize {
        self.execution_work_count
    }

    pub const fn whole_view_fallback_count(&self) -> usize {
        self.whole_view_fallback_count
    }

    pub const fn caller_owned_graph_work_count(&self) -> usize {
        self.caller_owned_graph_work_count
    }

    pub fn row_digest(&self) -> &str {
        &self.row_digest
    }
}

fn report_digest(
    selected_plan: &DerivedInvalidationSelectedPlan,
    migration_sweep: &CoveredDerivedProductMigrationSweepCloseout,
    execution_receipt: &DerivedInvalidationExecutionReceipt,
    rows: &[DerivedInvalidationMilestoneTenProductSummaryRow],
) -> String {
    let mut parts = vec![
        "worth-topo:derived-invalidation-milestone-ten-product-summary:v1".to_string(),
        format!("selected-plan:{}", migration_sweep.selected_plan_digest()),
        format!("touched-closure:{}", selected_plan.touched_closure_digest()),
        format!("migration-sweep:{}", migration_sweep.closeout_digest()),
        format!("execution:{}", execution_receipt.execution_receipt_digest()),
    ];
    parts.extend(rows.iter().map(|row| format!("row:{}", row.row_digest())));
    super::super::catalog::catalog_digest(parts)
}
