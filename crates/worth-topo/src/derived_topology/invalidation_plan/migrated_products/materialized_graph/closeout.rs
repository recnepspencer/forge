use serde::Serialize;

use super::{
    MaterializedGraphDerivedProductExecutor, MaterializedGraphDerivedProductOutput,
    MaterializedGraphDiagnosticProjection, MaterializedGraphExecutionInput,
    MaterializedGraphMigrationCounters, MaterializedGraphMigrationError,
    MaterializedGraphOldAuthorityResidue, MaterializedGraphPhaseTenSeed,
};
use crate::derived_topology::invalidation_plan::catalog::DerivedTopologyProductFamilyIdentity;
use crate::derived_topology::invalidation_plan::execution::{
    DerivedInvalidationExecutedProductRow, DerivedInvalidationExecutionReceipt,
};
use crate::derived_topology::invalidation_plan::migrated_products::MigratedDerivedProductFamilyCloseout;
use crate::derived_topology::invalidation_plan::selection::DerivedInvalidationSelectedPlan;

pub fn close_materialized_graph_migration_slice(
    selected_plan: &DerivedInvalidationSelectedPlan,
    input: MaterializedGraphExecutionInput,
) -> Result<MaterializedGraphMigrationCloseout, MaterializedGraphMigrationError> {
    let read_stage_receipt = input.read_stage_receipt().clone();
    let executor = MaterializedGraphDerivedProductExecutor::new(input);
    let receipt = DerivedInvalidationExecutionReceipt::execute_selected_plan_with_executor(
        selected_plan,
        &executor,
    )
    .map_err(|_| MaterializedGraphMigrationError::ExecutionReceiptFailed)?;
    let output = executor
        .output()
        .ok_or(MaterializedGraphMigrationError::ExecutionReceiptMissingMaterializedGraphRow)?;
    let diagnostic_projection = MaterializedGraphDiagnosticProjection::from_read_stage_and_output(
        &read_stage_receipt,
        &output,
    );
    let old_authority_residue = MaterializedGraphOldAuthorityResidue::current_source_scan();
    MaterializedGraphMigrationCloseout::close(
        &receipt,
        &output,
        &diagnostic_projection,
        &old_authority_residue,
    )
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct MaterializedGraphMigrationCloseout {
    execution_receipt_digest: String,
    materialized_graph_output_digest: String,
    materialized_graph_executed_row_digest: String,
    diagnostic_projection_digest: String,
    old_authority_residue_digest: String,
    counters: MaterializedGraphMigrationCounters,
    migrated_family_closeout: MigratedDerivedProductFamilyCloseout,
    phase_ten_seed: MaterializedGraphPhaseTenSeed,
    closeout_digest: String,
}

impl MaterializedGraphMigrationCloseout {
    pub(crate) fn close(
        receipt: &DerivedInvalidationExecutionReceipt,
        output: &MaterializedGraphDerivedProductOutput,
        diagnostic_projection: &MaterializedGraphDiagnosticProjection,
        old_authority_residue: &MaterializedGraphOldAuthorityResidue,
    ) -> Result<Self, MaterializedGraphMigrationError> {
        if old_authority_residue.capped_whole_view_authority_count() == 0 {
            return Err(MaterializedGraphMigrationError::OldAuthorityResidueNotCapped);
        }
        if !old_authority_residue.contains_required_caps() {
            return Err(MaterializedGraphMigrationError::OldAuthorityResidueMissingRequiredCap);
        }
        let row = materialized_graph_row(receipt)?;
        if row.product_output_digest() != Some(output.output_digest()) {
            return Err(MaterializedGraphMigrationError::OutputDigestNotBoundToReceipt);
        }
        if receipt.selected_plan_digest() != output.selected_plan_digest() {
            return Err(MaterializedGraphMigrationError::OutputSelectedPlanNotBoundToInput);
        }
        if diagnostic_projection.selected_plan_digest() != output.selected_plan_digest()
            || diagnostic_projection.read_stage_receipt_digest()
                != output.read_stage_receipt_digest()
            || diagnostic_projection.product_output_digest() != output.output_digest()
        {
            return Err(MaterializedGraphMigrationError::OutputDigestNotBoundToReceipt);
        }
        if row.execution_work_count() == 0
            && (output.selected_entity_count() > 0 || output.selected_relation_count() > 0)
        {
            return Err(MaterializedGraphMigrationError::NoMaterializedGraphExecutionObserved);
        }

        let counters = MaterializedGraphMigrationCounters::new(
            output,
            row.execution_work_count(),
            row.whole_view_fallback_count(),
            non_materialized_placeholder_execution_count(receipt),
            old_authority_residue.capped_whole_view_authority_count(),
        );
        let closeout_digest = closeout_digest(
            receipt,
            output,
            row,
            diagnostic_projection,
            old_authority_residue,
            &counters,
        );
        let migrated_family_closeout = MigratedDerivedProductFamilyCloseout::new(
            DerivedTopologyProductFamilyIdentity::MaterializedGraph,
            receipt.selected_plan_digest(),
            receipt.execution_receipt_digest(),
            row.row_digest(),
            output.output_digest(),
            old_authority_residue.residue_digest(),
            counters.counters_digest(),
        );
        let phase_ten_seed = MaterializedGraphPhaseTenSeed::from_closeout_parts(
            &closeout_digest,
            &counters,
            old_authority_residue,
        );
        Ok(Self {
            execution_receipt_digest: receipt.execution_receipt_digest().to_string(),
            materialized_graph_output_digest: output.output_digest().to_string(),
            materialized_graph_executed_row_digest: row.row_digest().to_string(),
            diagnostic_projection_digest: diagnostic_projection.projection_digest().to_string(),
            old_authority_residue_digest: old_authority_residue.residue_digest().to_string(),
            counters,
            migrated_family_closeout,
            phase_ten_seed,
            closeout_digest,
        })
    }

    pub fn execution_receipt_digest(&self) -> &str {
        &self.execution_receipt_digest
    }

    pub fn materialized_graph_output_digest(&self) -> &str {
        &self.materialized_graph_output_digest
    }

    pub fn old_authority_residue_digest(&self) -> &str {
        &self.old_authority_residue_digest
    }

    pub fn diagnostic_projection_digest(&self) -> &str {
        &self.diagnostic_projection_digest
    }

    pub const fn counters(&self) -> &MaterializedGraphMigrationCounters {
        &self.counters
    }

    pub const fn migrated_family_closeout(&self) -> &MigratedDerivedProductFamilyCloseout {
        &self.migrated_family_closeout
    }

    pub const fn phase_ten_seed(&self) -> &MaterializedGraphPhaseTenSeed {
        &self.phase_ten_seed
    }

    pub fn closeout_digest(&self) -> &str {
        &self.closeout_digest
    }
}

fn non_materialized_placeholder_execution_count(
    receipt: &DerivedInvalidationExecutionReceipt,
) -> usize {
    receipt
        .executed_rows()
        .iter()
        .filter(|row| {
            row.family_identity() != DerivedTopologyProductFamilyIdentity::MaterializedGraph
                && row.product_output_digest().is_none()
        })
        .count()
}

fn materialized_graph_row(
    receipt: &DerivedInvalidationExecutionReceipt,
) -> Result<&DerivedInvalidationExecutedProductRow, MaterializedGraphMigrationError> {
    receipt
        .executed_rows()
        .iter()
        .find(|row| {
            row.family_identity() == DerivedTopologyProductFamilyIdentity::MaterializedGraph
        })
        .ok_or(MaterializedGraphMigrationError::ExecutionReceiptMissingMaterializedGraphRow)
}

fn closeout_digest(
    receipt: &DerivedInvalidationExecutionReceipt,
    output: &MaterializedGraphDerivedProductOutput,
    row: &DerivedInvalidationExecutedProductRow,
    diagnostic_projection: &MaterializedGraphDiagnosticProjection,
    residue: &MaterializedGraphOldAuthorityResidue,
    counters: &MaterializedGraphMigrationCounters,
) -> String {
    super::super::super::catalog::catalog_digest([
        "worth-topo:materialized-graph-migration-closeout:v1".to_string(),
        format!("execution-receipt:{}", receipt.execution_receipt_digest()),
        format!("selected-plan:{}", receipt.selected_plan_digest()),
        format!("query-support:{}", receipt.query_support_digest()),
        format!("legality-support:{}", receipt.legality_support_digest()),
        format!("executed-row:{}", row.row_digest()),
        format!("read-stage:{}", output.read_stage_receipt_digest()),
        format!("input:{}", output.input_digest()),
        format!("output:{}", output.output_digest()),
        format!("diagnostic:{}", diagnostic_projection.projection_digest()),
        format!("old-authority-residue:{}", residue.residue_digest()),
        format!("counters:{}", counters.counters_digest()),
    ])
}
