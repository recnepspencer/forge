use serde::Serialize;

use crate::derived_topology::invalidation_plan::deletion_closeout::DerivedInvalidationDeletionCloseout;
use crate::derived_topology::invalidation_plan::execution::DerivedInvalidationExecutionReceipt;
use crate::derived_topology::invalidation_plan::migrated_products::CoveredDerivedProductMigrationSweepCloseout;

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct DerivedInvalidationMilestoneTenPerformanceProof {
    selected_plan_digest: String,
    execution_receipt_digest: String,
    deletion_closeout_digest: String,
    slope_cases: Vec<DerivedInvalidationMilestoneTenPerformanceSlopeCase>,
    proof_digest: String,
}

impl DerivedInvalidationMilestoneTenPerformanceProof {
    pub(crate) fn from_products(
        migration_sweep: &CoveredDerivedProductMigrationSweepCloseout,
        execution_receipt: &DerivedInvalidationExecutionReceipt,
        deletion_closeout: &DerivedInvalidationDeletionCloseout,
    ) -> Self {
        let slope_cases = vec![
            DerivedInvalidationMilestoneTenPerformanceSlopeCase::new(
                "semantic_delta_bounded_execution",
                migration_sweep.counters().selected_family_count(),
                execution_receipt.counters().executed_product_count(),
                execution_receipt.counters().unaffected_product_count(),
                0,
            ),
            DerivedInvalidationMilestoneTenPerformanceSlopeCase::new(
                "no_whole_view_fallback",
                0,
                execution_receipt.counters().whole_view_fallback_count(),
                deletion_closeout
                    .counters()
                    .ordinary_whole_view_rebuild_count(),
                0,
            ),
            DerivedInvalidationMilestoneTenPerformanceSlopeCase::new(
                "no_caller_owned_graph_work",
                0,
                execution_receipt.counters().caller_owned_graph_work_count(),
                deletion_closeout.counters().ordinary_dirty_path_count(),
                0,
            ),
            DerivedInvalidationMilestoneTenPerformanceSlopeCase::new(
                "product_catalog_closed_once",
                migration_sweep.counters().required_family_count(),
                migration_sweep
                    .counters()
                    .ordinary_consumable_family_count(),
                deletion_closeout
                    .counters()
                    .source_firewall_violation_count(),
                0,
            ),
        ];
        let proof_digest = proof_digest(
            migration_sweep,
            execution_receipt,
            deletion_closeout,
            &slope_cases,
        );
        Self {
            selected_plan_digest: migration_sweep.selected_plan_digest().to_string(),
            execution_receipt_digest: execution_receipt.execution_receipt_digest().to_string(),
            deletion_closeout_digest: deletion_closeout.closeout_digest().to_string(),
            slope_cases,
            proof_digest,
        }
    }

    pub fn selected_plan_digest(&self) -> &str {
        &self.selected_plan_digest
    }

    pub fn execution_receipt_digest(&self) -> &str {
        &self.execution_receipt_digest
    }

    pub fn deletion_closeout_digest(&self) -> &str {
        &self.deletion_closeout_digest
    }

    pub fn slope_cases(&self) -> &[DerivedInvalidationMilestoneTenPerformanceSlopeCase] {
        &self.slope_cases
    }

    pub fn proof_digest(&self) -> &str {
        &self.proof_digest
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct DerivedInvalidationMilestoneTenPerformanceSlopeCase {
    label: String,
    touched_or_declared_bound: usize,
    observed_work_count: usize,
    forbidden_global_work_count: usize,
    allowed_global_work_count: usize,
    row_digest: String,
}

impl DerivedInvalidationMilestoneTenPerformanceSlopeCase {
    fn new(
        label: &str,
        touched_or_declared_bound: usize,
        observed_work_count: usize,
        forbidden_global_work_count: usize,
        allowed_global_work_count: usize,
    ) -> Self {
        let row_digest = super::super::catalog::catalog_digest([
            "worth-topo:derived-invalidation-milestone-ten-performance-slope-case:v1".to_string(),
            format!("label:{label}"),
            format!("bound:{touched_or_declared_bound}"),
            format!("observed-work:{observed_work_count}"),
            format!("forbidden-global-work:{forbidden_global_work_count}"),
            format!("allowed-global-work:{allowed_global_work_count}"),
        ]);
        Self {
            label: label.to_string(),
            touched_or_declared_bound,
            observed_work_count,
            forbidden_global_work_count,
            allowed_global_work_count,
            row_digest,
        }
    }

    pub fn label(&self) -> &str {
        &self.label
    }

    pub const fn touched_or_declared_bound(&self) -> usize {
        self.touched_or_declared_bound
    }

    pub const fn observed_work_count(&self) -> usize {
        self.observed_work_count
    }

    pub const fn forbidden_global_work_count(&self) -> usize {
        self.forbidden_global_work_count
    }

    pub const fn allowed_global_work_count(&self) -> usize {
        self.allowed_global_work_count
    }

    pub fn row_digest(&self) -> &str {
        &self.row_digest
    }
}

fn proof_digest(
    migration_sweep: &CoveredDerivedProductMigrationSweepCloseout,
    execution_receipt: &DerivedInvalidationExecutionReceipt,
    deletion_closeout: &DerivedInvalidationDeletionCloseout,
    slope_cases: &[DerivedInvalidationMilestoneTenPerformanceSlopeCase],
) -> String {
    let mut parts = vec![
        "worth-topo:derived-invalidation-milestone-ten-performance-proof:v1".to_string(),
        format!("selected-plan:{}", migration_sweep.selected_plan_digest()),
        format!("execution:{}", execution_receipt.execution_receipt_digest()),
        format!("deletion:{}", deletion_closeout.closeout_digest()),
    ];
    parts.extend(
        slope_cases
            .iter()
            .map(|row| format!("row:{}", row.row_digest())),
    );
    super::super::catalog::catalog_digest(parts)
}
