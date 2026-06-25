use serde::Serialize;

use super::counters::DerivedInvalidationMilestoneTenCounters;
use super::error::{
    DerivedInvalidationMilestoneTenError, DerivedInvalidationMilestoneTenErrorKind,
};
use super::milestone_eleven_seed::DerivedInvalidationMilestoneElevenSeed;
use super::performance_proof::DerivedInvalidationMilestoneTenPerformanceProof;
use super::product_summary::DerivedInvalidationMilestoneTenProductSummaryReport;
use crate::derived_topology::invalidation_plan::catalog::DerivedInvalidationFamilyCatalogCloseout;
use crate::derived_topology::invalidation_plan::deletion_closeout::DerivedInvalidationDeletionCloseout;
use crate::derived_topology::invalidation_plan::execution::DerivedInvalidationExecutionReceipt;
use crate::derived_topology::invalidation_plan::migrated_products::CoveredDerivedProductMigrationSweepCloseout;
use crate::derived_topology::invalidation_plan::operator_cutover::DerivedInvalidationOperatorCutoverCloseout;
use crate::derived_topology::invalidation_plan::selection::DerivedInvalidationSelectedPlan;

pub fn close_derived_invalidation_milestone_ten(
    catalog_closeout: &DerivedInvalidationFamilyCatalogCloseout,
    selected_plan: &DerivedInvalidationSelectedPlan,
    execution_receipt: &DerivedInvalidationExecutionReceipt,
    migration_sweep: &CoveredDerivedProductMigrationSweepCloseout,
    operator_cutover: &DerivedInvalidationOperatorCutoverCloseout,
    deletion_closeout: &DerivedInvalidationDeletionCloseout,
) -> Result<DerivedInvalidationMilestoneTenCloseout, DerivedInvalidationMilestoneTenError> {
    DerivedInvalidationMilestoneTenCloseout::close(
        catalog_closeout,
        selected_plan,
        execution_receipt,
        migration_sweep,
        operator_cutover,
        deletion_closeout,
    )
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct DerivedInvalidationMilestoneTenCloseout {
    catalog_digest: String,
    selected_plan_digest: String,
    execution_receipt_digest: String,
    migration_sweep_digest: String,
    operator_cutover_digest: String,
    deletion_closeout_digest: String,
    product_summary: DerivedInvalidationMilestoneTenProductSummaryReport,
    performance_proof: DerivedInvalidationMilestoneTenPerformanceProof,
    counters: DerivedInvalidationMilestoneTenCounters,
    milestone_eleven_seed: DerivedInvalidationMilestoneElevenSeed,
    closeout_digest: String,
}

impl DerivedInvalidationMilestoneTenCloseout {
    fn close(
        catalog_closeout: &DerivedInvalidationFamilyCatalogCloseout,
        selected_plan: &DerivedInvalidationSelectedPlan,
        execution_receipt: &DerivedInvalidationExecutionReceipt,
        migration_sweep: &CoveredDerivedProductMigrationSweepCloseout,
        operator_cutover: &DerivedInvalidationOperatorCutoverCloseout,
        deletion_closeout: &DerivedInvalidationDeletionCloseout,
    ) -> Result<Self, DerivedInvalidationMilestoneTenError> {
        require_matching_catalog(catalog_closeout, selected_plan)?;
        require_matching_execution(selected_plan, execution_receipt)?;
        require_matching_migration(selected_plan, migration_sweep)?;
        require_matching_operator_cutover(selected_plan, execution_receipt, operator_cutover)?;
        require_matching_deletion(operator_cutover, migration_sweep, deletion_closeout)?;
        require_complete_migration(migration_sweep)?;
        require_hard_deletion(deletion_closeout)?;
        require_bounded_execution(execution_receipt)?;

        let product_summary = DerivedInvalidationMilestoneTenProductSummaryReport::from_products(
            catalog_closeout,
            selected_plan,
            migration_sweep,
            execution_receipt,
        );
        let performance_proof = DerivedInvalidationMilestoneTenPerformanceProof::from_products(
            migration_sweep,
            execution_receipt,
            deletion_closeout,
        );
        let counters = DerivedInvalidationMilestoneTenCounters::from_products(
            migration_sweep,
            execution_receipt,
            deletion_closeout,
            &product_summary,
            &performance_proof,
        );
        let closeout_digest = closeout_digest(
            catalog_closeout,
            selected_plan,
            execution_receipt,
            migration_sweep,
            operator_cutover,
            deletion_closeout,
            &product_summary,
            &performance_proof,
            &counters,
        );
        let milestone_eleven_seed = DerivedInvalidationMilestoneElevenSeed::from_closeout_parts(
            &closeout_digest,
            selected_plan,
            execution_receipt,
            deletion_closeout,
            &product_summary,
            &performance_proof,
            &counters,
        );
        Ok(Self {
            catalog_digest: catalog_closeout
                .phase_three_seed()
                .catalog_digest()
                .to_string(),
            selected_plan_digest: selected_plan.selected_plan_digest().to_string(),
            execution_receipt_digest: execution_receipt.execution_receipt_digest().to_string(),
            migration_sweep_digest: migration_sweep.closeout_digest().to_string(),
            operator_cutover_digest: operator_cutover.closeout_digest().to_string(),
            deletion_closeout_digest: deletion_closeout.closeout_digest().to_string(),
            product_summary,
            performance_proof,
            counters,
            milestone_eleven_seed,
            closeout_digest,
        })
    }

    pub fn catalog_digest(&self) -> &str {
        &self.catalog_digest
    }

    pub fn selected_plan_digest(&self) -> &str {
        &self.selected_plan_digest
    }

    pub fn execution_receipt_digest(&self) -> &str {
        &self.execution_receipt_digest
    }

    pub fn migration_sweep_digest(&self) -> &str {
        &self.migration_sweep_digest
    }

    pub fn operator_cutover_digest(&self) -> &str {
        &self.operator_cutover_digest
    }

    pub fn deletion_closeout_digest(&self) -> &str {
        &self.deletion_closeout_digest
    }

    pub const fn product_summary(&self) -> &DerivedInvalidationMilestoneTenProductSummaryReport {
        &self.product_summary
    }

    pub const fn performance_proof(&self) -> &DerivedInvalidationMilestoneTenPerformanceProof {
        &self.performance_proof
    }

    pub const fn counters(&self) -> &DerivedInvalidationMilestoneTenCounters {
        &self.counters
    }

    pub const fn milestone_eleven_seed(&self) -> &DerivedInvalidationMilestoneElevenSeed {
        &self.milestone_eleven_seed
    }

    pub fn closeout_digest(&self) -> &str {
        &self.closeout_digest
    }
}

fn require_matching_catalog(
    catalog_closeout: &DerivedInvalidationFamilyCatalogCloseout,
    selected_plan: &DerivedInvalidationSelectedPlan,
) -> Result<(), DerivedInvalidationMilestoneTenError> {
    if catalog_closeout.phase_three_seed().catalog_digest() != selected_plan.catalog_digest() {
        return Err(error(
            DerivedInvalidationMilestoneTenErrorKind::CatalogMismatch,
            "Milestone 10 cannot close over a selected plan from a different family catalog",
        ));
    }
    if catalog_closeout.phase_three_seed().seed_digest() != selected_plan.phase_three_seed_digest()
    {
        return Err(error(
            DerivedInvalidationMilestoneTenErrorKind::CatalogMismatch,
            "Milestone 10 selected plan must preserve the Phase 3 family catalog seed",
        ));
    }
    Ok(())
}

fn require_matching_execution(
    selected_plan: &DerivedInvalidationSelectedPlan,
    execution_receipt: &DerivedInvalidationExecutionReceipt,
) -> Result<(), DerivedInvalidationMilestoneTenError> {
    if execution_receipt.selected_plan_digest() != selected_plan.selected_plan_digest() {
        return Err(error(
            DerivedInvalidationMilestoneTenErrorKind::SelectedPlanMismatch,
            "Milestone 10 execution receipt must bind the selected plan",
        ));
    }
    if execution_receipt.touched_closure_digest() != selected_plan.touched_closure_digest() {
        return Err(error(
            DerivedInvalidationMilestoneTenErrorKind::TouchedClosureMismatch,
            "Milestone 10 execution receipt must preserve touched closure identity",
        ));
    }
    if execution_receipt.query_support_digest() != selected_plan.query_support_digest() {
        return Err(error(
            DerivedInvalidationMilestoneTenErrorKind::QuerySupportMismatch,
            "Milestone 10 execution receipt must preserve Query support identity",
        ));
    }
    if execution_receipt.legality_support_digest() != selected_plan.legality_support_digest() {
        return Err(error(
            DerivedInvalidationMilestoneTenErrorKind::LegalitySupportMismatch,
            "Milestone 10 execution receipt must preserve legality support identity",
        ));
    }
    Ok(())
}

fn require_matching_migration(
    selected_plan: &DerivedInvalidationSelectedPlan,
    migration_sweep: &CoveredDerivedProductMigrationSweepCloseout,
) -> Result<(), DerivedInvalidationMilestoneTenError> {
    if migration_sweep.selected_plan_digest() != selected_plan.selected_plan_digest() {
        return Err(error(
            DerivedInvalidationMilestoneTenErrorKind::SelectedPlanMismatch,
            "Milestone 10 migration sweep must bind the selected plan",
        ));
    }
    if !migration_sweep_rows_carry_family_execution_receipts(migration_sweep) {
        return Err(error(
            DerivedInvalidationMilestoneTenErrorKind::ExecutionReceiptMismatch,
            "Milestone 10 migration sweep must be built from family-specific closeouts carrying per-family execution receipts",
        ));
    }
    Ok(())
}

fn migration_sweep_rows_carry_family_execution_receipts(
    migration_sweep: &CoveredDerivedProductMigrationSweepCloseout,
) -> bool {
    migration_sweep.status_rows().iter().all(|row| {
        row.ordinary_invalidation_consumable()
            && row
                .execution_receipt_digest()
                .is_some_and(|digest| !digest.is_empty())
    })
}

fn require_matching_operator_cutover(
    selected_plan: &DerivedInvalidationSelectedPlan,
    execution_receipt: &DerivedInvalidationExecutionReceipt,
    operator_cutover: &DerivedInvalidationOperatorCutoverCloseout,
) -> Result<(), DerivedInvalidationMilestoneTenError> {
    let phase_eight_seed = operator_cutover.phase_eight_seed();
    if phase_eight_seed.selected_plan_digest() != selected_plan.selected_plan_digest() {
        return Err(error(
            DerivedInvalidationMilestoneTenErrorKind::OperatorCutoverMismatch,
            "Milestone 10 operator cutover must bind the selected plan",
        ));
    }
    if phase_eight_seed.execution_receipt_digest() != execution_receipt.execution_receipt_digest() {
        return Err(error(
            DerivedInvalidationMilestoneTenErrorKind::ExecutionReceiptMismatch,
            "Milestone 10 operator cutover must consume the execution receipt",
        ));
    }
    Ok(())
}

fn require_matching_deletion(
    operator_cutover: &DerivedInvalidationOperatorCutoverCloseout,
    migration_sweep: &CoveredDerivedProductMigrationSweepCloseout,
    deletion_closeout: &DerivedInvalidationDeletionCloseout,
) -> Result<(), DerivedInvalidationMilestoneTenError> {
    if deletion_closeout.phase_eight_seed_digest()
        != operator_cutover.phase_eight_seed().seed_digest()
    {
        return Err(error(
            DerivedInvalidationMilestoneTenErrorKind::DeletionCloseoutMismatch,
            "Milestone 10 deletion closeout must bind the operator cutover seed",
        ));
    }
    if deletion_closeout.migration_sweep_digest() != migration_sweep.closeout_digest() {
        return Err(error(
            DerivedInvalidationMilestoneTenErrorKind::DeletionCloseoutMismatch,
            "Milestone 10 deletion closeout must bind the migration sweep",
        ));
    }
    Ok(())
}

fn require_complete_migration(
    migration_sweep: &CoveredDerivedProductMigrationSweepCloseout,
) -> Result<(), DerivedInvalidationMilestoneTenError> {
    let counters = migration_sweep.counters();
    if counters.required_family_count() != counters.ordinary_consumable_family_count() {
        return Err(error(
            DerivedInvalidationMilestoneTenErrorKind::IncompleteProductMigration,
            "Milestone 10 requires every covered derived product to be ordinary-consumable",
        ));
    }
    Ok(())
}

fn require_hard_deletion(
    deletion_closeout: &DerivedInvalidationDeletionCloseout,
) -> Result<(), DerivedInvalidationMilestoneTenError> {
    let counters = deletion_closeout.counters();
    if counters.source_firewall_violation_count() != 0 {
        return Err(error(
            DerivedInvalidationMilestoneTenErrorKind::SourceFirewallViolation,
            "Milestone 10 cannot close while old authority appears in ordinary source",
        ));
    }
    if counters.ordinary_dirty_path_count() != 0
        || counters.ordinary_whole_view_rebuild_count() != 0
    {
        return Err(error(
            DerivedInvalidationMilestoneTenErrorKind::OldAuthorityResidue,
            "Milestone 10 cannot close while ordinary paths retain old dirty choreography",
        ));
    }
    Ok(())
}

fn require_bounded_execution(
    execution_receipt: &DerivedInvalidationExecutionReceipt,
) -> Result<(), DerivedInvalidationMilestoneTenError> {
    if execution_receipt.counters().whole_view_fallback_count() != 0 {
        return Err(error(
            DerivedInvalidationMilestoneTenErrorKind::WholeViewFallback,
            "Milestone 10 requires semantic-delta-bounded execution with no whole-view fallback",
        ));
    }
    if execution_receipt.counters().caller_owned_graph_work_count() != 0 {
        return Err(error(
            DerivedInvalidationMilestoneTenErrorKind::CallerOwnedGraphWork,
            "Milestone 10 requires operators to stop authoring dirty graph work",
        ));
    }
    Ok(())
}

fn closeout_digest(
    catalog_closeout: &DerivedInvalidationFamilyCatalogCloseout,
    selected_plan: &DerivedInvalidationSelectedPlan,
    execution_receipt: &DerivedInvalidationExecutionReceipt,
    migration_sweep: &CoveredDerivedProductMigrationSweepCloseout,
    operator_cutover: &DerivedInvalidationOperatorCutoverCloseout,
    deletion_closeout: &DerivedInvalidationDeletionCloseout,
    product_summary: &DerivedInvalidationMilestoneTenProductSummaryReport,
    performance_proof: &DerivedInvalidationMilestoneTenPerformanceProof,
    counters: &DerivedInvalidationMilestoneTenCounters,
) -> String {
    super::super::catalog::catalog_digest([
        "worth-topo:derived-invalidation-milestone-ten-closeout:v1".to_string(),
        format!(
            "catalog:{}",
            catalog_closeout.phase_three_seed().catalog_digest()
        ),
        format!("selected-plan:{}", selected_plan.selected_plan_digest()),
        format!("execution:{}", execution_receipt.execution_receipt_digest()),
        format!("migration-sweep:{}", migration_sweep.closeout_digest()),
        format!("operator-cutover:{}", operator_cutover.closeout_digest()),
        format!("deletion:{}", deletion_closeout.closeout_digest()),
        format!("product-summary:{}", product_summary.report_digest()),
        format!("performance:{}", performance_proof.proof_digest()),
        format!("counters:{}", counters.counters_digest()),
    ])
}

fn error(
    kind: DerivedInvalidationMilestoneTenErrorKind,
    message: &'static str,
) -> DerivedInvalidationMilestoneTenError {
    DerivedInvalidationMilestoneTenError::new(kind, message)
}
