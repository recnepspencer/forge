use serde::Serialize;

use super::{
    RadialRingDerivedProductExecutor, RadialRingDerivedProductOutput, RadialRingExecutionInput,
    RadialRingFamilyCloseoutSeed, RadialRingMigrationCounters, RadialRingMigrationError,
    RadialRingOldAuthorityResidue,
};
use crate::derived_topology::invalidation_plan::catalog::DerivedTopologyProductFamilyIdentity;
use crate::derived_topology::invalidation_plan::execution::{
    DerivedInvalidationExecutedProductRow, DerivedInvalidationExecutionReceipt,
};
use crate::derived_topology::invalidation_plan::migrated_products::MigratedDerivedProductFamilyCloseout;
use crate::derived_topology::invalidation_plan::selection::DerivedInvalidationSelectedPlan;

pub fn close_radial_ring_migration_slice(
    selected_plan: &DerivedInvalidationSelectedPlan,
    input: RadialRingExecutionInput,
) -> Result<RadialRingMigrationCloseout, RadialRingMigrationError> {
    let executor = RadialRingDerivedProductExecutor::new(input);
    let receipt = DerivedInvalidationExecutionReceipt::execute_selected_plan_with_executor(
        selected_plan,
        &executor,
    )
    .map_err(|_| RadialRingMigrationError::ExecutionReceiptFailed)?;
    let output = executor
        .output()
        .ok_or(RadialRingMigrationError::ExecutionReceiptMissingRadialRingRow)?;
    let old_authority_residue = RadialRingOldAuthorityResidue::current_source_scan();
    RadialRingMigrationCloseout::close(&receipt, &output, &old_authority_residue)
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct RadialRingMigrationCloseout {
    execution_receipt_digest: String,
    radial_ring_output_digest: String,
    radial_ring_executed_row_digest: String,
    old_authority_residue_digest: String,
    counters: RadialRingMigrationCounters,
    migrated_family_closeout: MigratedDerivedProductFamilyCloseout,
    family_closeout_seed: RadialRingFamilyCloseoutSeed,
    closeout_digest: String,
}

impl RadialRingMigrationCloseout {
    pub(crate) fn close(
        receipt: &DerivedInvalidationExecutionReceipt,
        output: &RadialRingDerivedProductOutput,
        old_authority_residue: &RadialRingOldAuthorityResidue,
    ) -> Result<Self, RadialRingMigrationError> {
        if old_authority_residue.capped_direct_interpreter_count()
            != RadialRingOldAuthorityResidue::required_capped_callers().len()
            || !old_authority_residue.contains_required_caps()
        {
            return Err(RadialRingMigrationError::OldAuthorityResidueMissingRequiredCap);
        }
        let row = radial_ring_row(receipt)?;
        if row.product_output_digest() != Some(output.output_digest()) {
            return Err(RadialRingMigrationError::OutputDigestNotBoundToReceipt);
        }
        if receipt.selected_plan_digest() != output.selected_plan_digest() {
            return Err(RadialRingMigrationError::OutputSelectedPlanNotBoundToInput);
        }
        if row.execution_work_count() == 0 && !output.rows().is_empty() {
            return Err(RadialRingMigrationError::NoRadialRingExecutionObserved);
        }
        if row.whole_view_fallback_count() != 0 {
            return Err(RadialRingMigrationError::WholeViewFallbackNotAllowed);
        }

        let counters = RadialRingMigrationCounters::new(
            output,
            row.execution_work_count(),
            row.whole_view_fallback_count(),
            non_loop_placeholder_execution_count(receipt),
            old_authority_residue.capped_direct_interpreter_count(),
        );
        let closeout_digest =
            closeout_digest(receipt, output, row, old_authority_residue, &counters);
        let migrated_family_closeout = MigratedDerivedProductFamilyCloseout::new(
            DerivedTopologyProductFamilyIdentity::RadialRings,
            receipt.selected_plan_digest(),
            receipt.execution_receipt_digest(),
            row.row_digest(),
            output.output_digest(),
            old_authority_residue.residue_digest(),
            counters.counters_digest(),
        );
        let family_closeout_seed = RadialRingFamilyCloseoutSeed::from_closeout_parts(
            &closeout_digest,
            &counters,
            old_authority_residue,
        );
        Ok(Self {
            execution_receipt_digest: receipt.execution_receipt_digest().to_string(),
            radial_ring_output_digest: output.output_digest().to_string(),
            radial_ring_executed_row_digest: row.row_digest().to_string(),
            old_authority_residue_digest: old_authority_residue.residue_digest().to_string(),
            counters,
            migrated_family_closeout,
            family_closeout_seed,
            closeout_digest,
        })
    }

    pub fn execution_receipt_digest(&self) -> &str {
        &self.execution_receipt_digest
    }

    pub fn radial_ring_output_digest(&self) -> &str {
        &self.radial_ring_output_digest
    }

    pub fn radial_ring_executed_row_digest(&self) -> &str {
        &self.radial_ring_executed_row_digest
    }

    pub fn old_authority_residue_digest(&self) -> &str {
        &self.old_authority_residue_digest
    }

    pub const fn counters(&self) -> &RadialRingMigrationCounters {
        &self.counters
    }

    pub const fn migrated_family_closeout(&self) -> &MigratedDerivedProductFamilyCloseout {
        &self.migrated_family_closeout
    }

    pub const fn family_closeout_seed(&self) -> &RadialRingFamilyCloseoutSeed {
        &self.family_closeout_seed
    }

    pub fn closeout_digest(&self) -> &str {
        &self.closeout_digest
    }
}

fn non_loop_placeholder_execution_count(receipt: &DerivedInvalidationExecutionReceipt) -> usize {
    receipt
        .executed_rows()
        .iter()
        .filter(|row| {
            row.family_identity() != DerivedTopologyProductFamilyIdentity::RadialRings
                && row.product_output_digest().is_none()
        })
        .count()
}

fn radial_ring_row(
    receipt: &DerivedInvalidationExecutionReceipt,
) -> Result<&DerivedInvalidationExecutedProductRow, RadialRingMigrationError> {
    receipt
        .executed_rows()
        .iter()
        .find(|row| row.family_identity() == DerivedTopologyProductFamilyIdentity::RadialRings)
        .ok_or(RadialRingMigrationError::ExecutionReceiptMissingRadialRingRow)
}

fn closeout_digest(
    receipt: &DerivedInvalidationExecutionReceipt,
    output: &RadialRingDerivedProductOutput,
    row: &DerivedInvalidationExecutedProductRow,
    residue: &RadialRingOldAuthorityResidue,
    counters: &RadialRingMigrationCounters,
) -> String {
    super::super::super::catalog::catalog_digest([
        "worth-topo:radial-ring-migration-closeout:v1".to_string(),
        format!("execution-receipt:{}", receipt.execution_receipt_digest()),
        format!("selected-plan:{}", receipt.selected_plan_digest()),
        format!("query-support:{}", receipt.query_support_digest()),
        format!("legality-support:{}", receipt.legality_support_digest()),
        format!("executed-row:{}", row.row_digest()),
        format!("output:{}", output.output_digest()),
        format!("old-authority-residue:{}", residue.residue_digest()),
        format!("counters:{}", counters.counters_digest()),
    ])
}
