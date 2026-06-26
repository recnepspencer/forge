use serde::Serialize;

use super::{
    VertexDiskDerivedProductExecutor, VertexDiskDerivedProductOutput, VertexDiskExecutionInput,
    VertexDiskFamilyCloseoutSeed, VertexDiskMigrationCounters, VertexDiskMigrationError,
    VertexDiskOldAuthorityResidue,
};
use crate::derived_topology::invalidation_plan::catalog::DerivedTopologyProductFamilyIdentity;
use crate::derived_topology::invalidation_plan::execution::{
    DerivedInvalidationExecutedProductRow, DerivedInvalidationExecutionReceipt,
};
use crate::derived_topology::invalidation_plan::migrated_products::MigratedDerivedProductFamilyCloseout;
use crate::derived_topology::invalidation_plan::selection::DerivedInvalidationSelectedPlan;

pub fn close_vertex_disk_migration_slice(
    selected_plan: &DerivedInvalidationSelectedPlan,
    input: VertexDiskExecutionInput,
) -> Result<VertexDiskMigrationCloseout, VertexDiskMigrationError> {
    let executor = VertexDiskDerivedProductExecutor::new(input);
    let receipt = DerivedInvalidationExecutionReceipt::execute_selected_plan_with_executor(
        selected_plan,
        &executor,
    )
    .map_err(|_| VertexDiskMigrationError::ExecutionReceiptFailed)?;
    let output = executor
        .output()
        .ok_or(VertexDiskMigrationError::ExecutionReceiptMissingVertexDiskRow)?;
    let old_authority_residue = VertexDiskOldAuthorityResidue::current_source_scan();
    VertexDiskMigrationCloseout::close(&receipt, &output, &old_authority_residue)
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct VertexDiskMigrationCloseout {
    execution_receipt_digest: String,
    vertex_disk_output_digest: String,
    vertex_disk_executed_row_digest: String,
    old_authority_residue_digest: String,
    counters: VertexDiskMigrationCounters,
    migrated_family_closeout: MigratedDerivedProductFamilyCloseout,
    family_closeout_seed: VertexDiskFamilyCloseoutSeed,
    closeout_digest: String,
}

impl VertexDiskMigrationCloseout {
    pub(crate) fn close(
        receipt: &DerivedInvalidationExecutionReceipt,
        output: &VertexDiskDerivedProductOutput,
        old_authority_residue: &VertexDiskOldAuthorityResidue,
    ) -> Result<Self, VertexDiskMigrationError> {
        if old_authority_residue.capped_direct_interpreter_count()
            != VertexDiskOldAuthorityResidue::required_capped_callers().len()
            || !old_authority_residue.contains_required_caps()
        {
            return Err(VertexDiskMigrationError::OldAuthorityResidueMissingRequiredCap);
        }
        let row = vertex_disk_row(receipt)?;
        if row.product_output_digest() != Some(output.output_digest()) {
            return Err(VertexDiskMigrationError::OutputDigestNotBoundToReceipt);
        }
        if receipt.selected_plan_digest() != output.selected_plan_digest() {
            return Err(VertexDiskMigrationError::OutputSelectedPlanNotBoundToInput);
        }
        if row.execution_work_count() == 0 && !output.rows().is_empty() {
            return Err(VertexDiskMigrationError::NoVertexDiskExecutionObserved);
        }
        if row.whole_view_fallback_count() != 0 {
            return Err(VertexDiskMigrationError::WholeViewFallbackNotAllowed);
        }

        let counters = VertexDiskMigrationCounters::new(
            output,
            row.execution_work_count(),
            row.whole_view_fallback_count(),
            non_loop_placeholder_execution_count(receipt),
            old_authority_residue.capped_direct_interpreter_count(),
        );
        let closeout_digest =
            closeout_digest(receipt, output, row, old_authority_residue, &counters);
        let migrated_family_closeout = MigratedDerivedProductFamilyCloseout::new(
            DerivedTopologyProductFamilyIdentity::VertexDisks,
            receipt.selected_plan_digest(),
            receipt.execution_receipt_digest(),
            row.row_digest(),
            output.output_digest(),
            old_authority_residue.residue_digest(),
            counters.counters_digest(),
        );
        let family_closeout_seed = VertexDiskFamilyCloseoutSeed::from_closeout_parts(
            &closeout_digest,
            &counters,
            old_authority_residue,
        );
        Ok(Self {
            execution_receipt_digest: receipt.execution_receipt_digest().to_string(),
            vertex_disk_output_digest: output.output_digest().to_string(),
            vertex_disk_executed_row_digest: row.row_digest().to_string(),
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

    pub fn vertex_disk_output_digest(&self) -> &str {
        &self.vertex_disk_output_digest
    }

    pub fn vertex_disk_executed_row_digest(&self) -> &str {
        &self.vertex_disk_executed_row_digest
    }

    pub fn old_authority_residue_digest(&self) -> &str {
        &self.old_authority_residue_digest
    }

    pub const fn counters(&self) -> &VertexDiskMigrationCounters {
        &self.counters
    }

    pub const fn migrated_family_closeout(&self) -> &MigratedDerivedProductFamilyCloseout {
        &self.migrated_family_closeout
    }

    pub const fn family_closeout_seed(&self) -> &VertexDiskFamilyCloseoutSeed {
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
            row.family_identity() != DerivedTopologyProductFamilyIdentity::VertexDisks
                && row.product_output_digest().is_none()
        })
        .count()
}

fn vertex_disk_row(
    receipt: &DerivedInvalidationExecutionReceipt,
) -> Result<&DerivedInvalidationExecutedProductRow, VertexDiskMigrationError> {
    receipt
        .executed_rows()
        .iter()
        .find(|row| row.family_identity() == DerivedTopologyProductFamilyIdentity::VertexDisks)
        .ok_or(VertexDiskMigrationError::ExecutionReceiptMissingVertexDiskRow)
}

fn closeout_digest(
    receipt: &DerivedInvalidationExecutionReceipt,
    output: &VertexDiskDerivedProductOutput,
    row: &DerivedInvalidationExecutedProductRow,
    residue: &VertexDiskOldAuthorityResidue,
    counters: &VertexDiskMigrationCounters,
) -> String {
    super::super::super::catalog::catalog_digest([
        "worth-topo:vertex-disk-migration-closeout:v1".to_string(),
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
