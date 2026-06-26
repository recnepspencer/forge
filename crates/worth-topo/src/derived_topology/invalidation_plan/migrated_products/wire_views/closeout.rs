use serde::Serialize;

use super::{
    WireViewDerivedProductExecutor, WireViewDerivedProductOutput, WireViewExecutionInput,
    WireViewFamilyCloseoutSeed, WireViewMigrationCounters, WireViewMigrationError,
    WireViewOldAuthorityResidue,
};
use crate::derived_topology::invalidation_plan::catalog::DerivedTopologyProductFamilyIdentity;
use crate::derived_topology::invalidation_plan::execution::{
    DerivedInvalidationExecutedProductRow, DerivedInvalidationExecutionReceipt,
};
use crate::derived_topology::invalidation_plan::migrated_products::MigratedDerivedProductFamilyCloseout;
use crate::derived_topology::invalidation_plan::selection::DerivedInvalidationSelectedPlan;

pub fn close_wire_view_migration_slice(
    selected_plan: &DerivedInvalidationSelectedPlan,
    input: WireViewExecutionInput,
) -> Result<WireViewMigrationCloseout, WireViewMigrationError> {
    let executor = WireViewDerivedProductExecutor::new(input);
    let receipt = DerivedInvalidationExecutionReceipt::execute_selected_plan_with_executor(
        selected_plan,
        &executor,
    )
    .map_err(|_| WireViewMigrationError::ExecutionReceiptFailed)?;
    let output = executor
        .output()
        .ok_or(WireViewMigrationError::ExecutionReceiptMissingWireViewRow)?;
    let old_authority_residue = WireViewOldAuthorityResidue::current_source_scan();
    WireViewMigrationCloseout::close(&receipt, &output, &old_authority_residue)
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct WireViewMigrationCloseout {
    execution_receipt_digest: String,
    wire_view_output_digest: String,
    wire_view_executed_row_digest: String,
    old_authority_residue_digest: String,
    counters: WireViewMigrationCounters,
    migrated_family_closeout: MigratedDerivedProductFamilyCloseout,
    family_closeout_seed: WireViewFamilyCloseoutSeed,
    closeout_digest: String,
}

impl WireViewMigrationCloseout {
    pub(crate) fn close(
        receipt: &DerivedInvalidationExecutionReceipt,
        output: &WireViewDerivedProductOutput,
        old_authority_residue: &WireViewOldAuthorityResidue,
    ) -> Result<Self, WireViewMigrationError> {
        if !old_authority_residue.contains_required_caps() {
            return Err(WireViewMigrationError::OldAuthorityResidueMissingRequiredCap);
        }
        let row = wire_view_row(receipt)?;
        if row.product_output_digest() != Some(output.output_digest()) {
            return Err(WireViewMigrationError::OutputDigestNotBoundToReceipt);
        }
        if receipt.selected_plan_digest() != output.selected_plan_digest() {
            return Err(WireViewMigrationError::OutputSelectedPlanNotBoundToInput);
        }
        if row.execution_work_count() == 0 && !output.rows().is_empty() {
            return Err(WireViewMigrationError::NoWireViewExecutionObserved);
        }
        if row.whole_view_fallback_count() != 0 {
            return Err(WireViewMigrationError::WholeViewFallbackNotAllowed);
        }

        let counters = WireViewMigrationCounters::new(
            output,
            row.execution_work_count(),
            row.whole_view_fallback_count(),
            non_wire_placeholder_execution_count(receipt),
            old_authority_residue.capped_direct_interpreter_count(),
        );
        let closeout_digest =
            closeout_digest(receipt, output, row, old_authority_residue, &counters);
        let migrated_family_closeout = MigratedDerivedProductFamilyCloseout::new(
            DerivedTopologyProductFamilyIdentity::WireViews,
            receipt.selected_plan_digest(),
            receipt.execution_receipt_digest(),
            row.row_digest(),
            output.output_digest(),
            old_authority_residue.residue_digest(),
            counters.counters_digest(),
        );
        let family_closeout_seed = WireViewFamilyCloseoutSeed::from_closeout_parts(
            &closeout_digest,
            &counters,
            old_authority_residue,
        );
        Ok(Self {
            execution_receipt_digest: receipt.execution_receipt_digest().to_string(),
            wire_view_output_digest: output.output_digest().to_string(),
            wire_view_executed_row_digest: row.row_digest().to_string(),
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

    pub fn wire_view_output_digest(&self) -> &str {
        &self.wire_view_output_digest
    }

    pub fn old_authority_residue_digest(&self) -> &str {
        &self.old_authority_residue_digest
    }

    pub const fn counters(&self) -> &WireViewMigrationCounters {
        &self.counters
    }

    pub const fn migrated_family_closeout(&self) -> &MigratedDerivedProductFamilyCloseout {
        &self.migrated_family_closeout
    }

    pub const fn family_closeout_seed(&self) -> &WireViewFamilyCloseoutSeed {
        &self.family_closeout_seed
    }

    pub fn closeout_digest(&self) -> &str {
        &self.closeout_digest
    }
}

fn non_wire_placeholder_execution_count(receipt: &DerivedInvalidationExecutionReceipt) -> usize {
    receipt
        .executed_rows()
        .iter()
        .filter(|row| {
            row.family_identity() != DerivedTopologyProductFamilyIdentity::WireViews
                && row.product_output_digest().is_none()
        })
        .count()
}

fn wire_view_row(
    receipt: &DerivedInvalidationExecutionReceipt,
) -> Result<&DerivedInvalidationExecutedProductRow, WireViewMigrationError> {
    receipt
        .executed_rows()
        .iter()
        .find(|row| row.family_identity() == DerivedTopologyProductFamilyIdentity::WireViews)
        .ok_or(WireViewMigrationError::ExecutionReceiptMissingWireViewRow)
}

fn closeout_digest(
    receipt: &DerivedInvalidationExecutionReceipt,
    output: &WireViewDerivedProductOutput,
    row: &DerivedInvalidationExecutedProductRow,
    residue: &WireViewOldAuthorityResidue,
    counters: &WireViewMigrationCounters,
) -> String {
    super::super::super::catalog::catalog_digest([
        "worth-topo:wire-view-migration-closeout:v1".to_string(),
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
