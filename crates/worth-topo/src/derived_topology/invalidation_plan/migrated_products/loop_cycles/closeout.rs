use serde::Serialize;

use super::{
    LoopCycleDerivedProductExecutor, LoopCycleDerivedProductOutput, LoopCycleExecutionInput,
    LoopCycleMigrationCounters, LoopCycleMigrationError, LoopCycleOldAuthorityResidue,
    LoopCyclePhaseSixSeed,
};
use crate::derived_topology::invalidation_plan::catalog::DerivedTopologyProductFamilyIdentity;
use crate::derived_topology::invalidation_plan::execution::{
    DerivedInvalidationExecutedProductRow, DerivedInvalidationExecutionReceipt,
};
use crate::derived_topology::invalidation_plan::migrated_products::MigratedDerivedProductFamilyCloseout;
use crate::derived_topology::invalidation_plan::selection::DerivedInvalidationSelectedPlan;

pub fn close_loop_cycle_migration_slice(
    selected_plan: &DerivedInvalidationSelectedPlan,
    input: LoopCycleExecutionInput,
) -> Result<LoopCycleMigrationCloseout, LoopCycleMigrationError> {
    let executor = LoopCycleDerivedProductExecutor::new(input);
    let receipt = DerivedInvalidationExecutionReceipt::execute_selected_plan_with_executor(
        selected_plan,
        &executor,
    )
    .map_err(|_| LoopCycleMigrationError::ExecutionReceiptFailed)?;
    let output = executor
        .output()
        .ok_or(LoopCycleMigrationError::ExecutionReceiptMissingLoopCycleRow)?;
    let old_authority_residue = LoopCycleOldAuthorityResidue::current_source_scan();
    LoopCycleMigrationCloseout::close(&receipt, &output, &old_authority_residue)
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct LoopCycleMigrationCloseout {
    execution_receipt_digest: String,
    loop_cycle_output_digest: String,
    loop_cycle_executed_row_digest: String,
    old_authority_residue_digest: String,
    counters: LoopCycleMigrationCounters,
    migrated_family_closeout: MigratedDerivedProductFamilyCloseout,
    phase_six_seed: LoopCyclePhaseSixSeed,
    closeout_digest: String,
}

impl LoopCycleMigrationCloseout {
    pub(crate) fn close(
        receipt: &DerivedInvalidationExecutionReceipt,
        output: &LoopCycleDerivedProductOutput,
        old_authority_residue: &LoopCycleOldAuthorityResidue,
    ) -> Result<Self, LoopCycleMigrationError> {
        if old_authority_residue.capped_direct_interpreter_count()
            != LoopCycleOldAuthorityResidue::required_capped_callers().len()
            || !old_authority_residue.contains_required_caps()
        {
            return Err(LoopCycleMigrationError::OldAuthorityResidueMissingRequiredCap);
        }
        let row = loop_cycle_row(receipt)?;
        if row.product_output_digest() != Some(output.output_digest()) {
            return Err(LoopCycleMigrationError::OutputDigestNotBoundToReceipt);
        }
        if receipt.selected_plan_digest() != output.selected_plan_digest() {
            return Err(LoopCycleMigrationError::OutputSelectedPlanNotBoundToInput);
        }
        if row.execution_work_count() == 0 && !output.rows().is_empty() {
            return Err(LoopCycleMigrationError::NoLoopCycleExecutionObserved);
        }
        if row.whole_view_fallback_count() != 0 {
            return Err(LoopCycleMigrationError::WholeViewFallbackNotAllowed);
        }

        let counters = LoopCycleMigrationCounters::new(
            output,
            row.execution_work_count(),
            row.whole_view_fallback_count(),
            non_loop_placeholder_execution_count(receipt),
            old_authority_residue.capped_direct_interpreter_count(),
        );
        let closeout_digest =
            closeout_digest(receipt, output, row, old_authority_residue, &counters);
        let migrated_family_closeout = MigratedDerivedProductFamilyCloseout::new(
            DerivedTopologyProductFamilyIdentity::LoopCycles,
            receipt.selected_plan_digest(),
            receipt.execution_receipt_digest(),
            row.row_digest(),
            output.output_digest(),
            old_authority_residue.residue_digest(),
            counters.counters_digest(),
        );
        let phase_six_seed = LoopCyclePhaseSixSeed::from_closeout_parts(
            &closeout_digest,
            &counters,
            old_authority_residue,
        );
        Ok(Self {
            execution_receipt_digest: receipt.execution_receipt_digest().to_string(),
            loop_cycle_output_digest: output.output_digest().to_string(),
            loop_cycle_executed_row_digest: row.row_digest().to_string(),
            old_authority_residue_digest: old_authority_residue.residue_digest().to_string(),
            counters,
            migrated_family_closeout,
            phase_six_seed,
            closeout_digest,
        })
    }

    pub fn execution_receipt_digest(&self) -> &str {
        &self.execution_receipt_digest
    }

    pub fn loop_cycle_output_digest(&self) -> &str {
        &self.loop_cycle_output_digest
    }

    pub fn loop_cycle_executed_row_digest(&self) -> &str {
        &self.loop_cycle_executed_row_digest
    }

    pub fn old_authority_residue_digest(&self) -> &str {
        &self.old_authority_residue_digest
    }

    pub const fn counters(&self) -> &LoopCycleMigrationCounters {
        &self.counters
    }

    pub const fn migrated_family_closeout(&self) -> &MigratedDerivedProductFamilyCloseout {
        &self.migrated_family_closeout
    }

    pub const fn phase_six_seed(&self) -> &LoopCyclePhaseSixSeed {
        &self.phase_six_seed
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
            row.family_identity() != DerivedTopologyProductFamilyIdentity::LoopCycles
                && row.product_output_digest().is_none()
        })
        .count()
}

fn loop_cycle_row(
    receipt: &DerivedInvalidationExecutionReceipt,
) -> Result<&DerivedInvalidationExecutedProductRow, LoopCycleMigrationError> {
    receipt
        .executed_rows()
        .iter()
        .find(|row| row.family_identity() == DerivedTopologyProductFamilyIdentity::LoopCycles)
        .ok_or(LoopCycleMigrationError::ExecutionReceiptMissingLoopCycleRow)
}

fn closeout_digest(
    receipt: &DerivedInvalidationExecutionReceipt,
    output: &LoopCycleDerivedProductOutput,
    row: &DerivedInvalidationExecutedProductRow,
    residue: &LoopCycleOldAuthorityResidue,
    counters: &LoopCycleMigrationCounters,
) -> String {
    super::super::super::catalog::catalog_digest([
        "worth-topo:loop-cycle-migration-closeout:v1".to_string(),
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
