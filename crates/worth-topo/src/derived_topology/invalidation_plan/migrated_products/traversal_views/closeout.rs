use serde::Serialize;

use super::{
    TraversalViewsDerivedProductExecutor, TraversalViewsDerivedProductOutput,
    TraversalViewsDiagnosticProjection, TraversalViewsExecutionInput,
    TraversalViewsMigrationCounters, TraversalViewsMigrationError,
    TraversalViewsOldAuthorityResidue, TraversalViewsPhaseElevenSeed,
};
use crate::derived_topology::invalidation_plan::catalog::DerivedTopologyProductFamilyIdentity;
use crate::derived_topology::invalidation_plan::execution::{
    DerivedInvalidationExecutedProductRow, DerivedInvalidationExecutionReceipt,
};
use crate::derived_topology::invalidation_plan::migrated_products::MigratedDerivedProductFamilyCloseout;
use crate::derived_topology::invalidation_plan::selection::DerivedInvalidationSelectedPlan;

pub fn close_traversal_views_migration_slice(
    selected_plan: &DerivedInvalidationSelectedPlan,
    input: TraversalViewsExecutionInput,
) -> Result<TraversalViewsMigrationCloseout, TraversalViewsMigrationError> {
    let read_stage_receipt = input.read_stage_receipt().clone();
    let executor = TraversalViewsDerivedProductExecutor::new(input);
    let receipt = DerivedInvalidationExecutionReceipt::execute_selected_plan_with_executor(
        selected_plan,
        &executor,
    )
    .map_err(|_| TraversalViewsMigrationError::ExecutionReceiptFailed)?;
    let output = executor
        .output()
        .ok_or(TraversalViewsMigrationError::ExecutionReceiptMissingTraversalViewsRow)?;
    let diagnostic_projection = TraversalViewsDiagnosticProjection::from_read_stage_and_output(
        &read_stage_receipt,
        &output,
    );
    let old_authority_residue = TraversalViewsOldAuthorityResidue::current_source_scan();
    TraversalViewsMigrationCloseout::close(
        &receipt,
        &output,
        &diagnostic_projection,
        &old_authority_residue,
    )
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct TraversalViewsMigrationCloseout {
    execution_receipt_digest: String,
    traversal_views_output_digest: String,
    traversal_views_executed_row_digest: String,
    diagnostic_projection_digest: String,
    old_authority_residue_digest: String,
    counters: TraversalViewsMigrationCounters,
    migrated_family_closeout: MigratedDerivedProductFamilyCloseout,
    phase_eleven_seed: TraversalViewsPhaseElevenSeed,
    closeout_digest: String,
}

impl TraversalViewsMigrationCloseout {
    pub(crate) fn close(
        receipt: &DerivedInvalidationExecutionReceipt,
        output: &TraversalViewsDerivedProductOutput,
        diagnostic_projection: &TraversalViewsDiagnosticProjection,
        old_authority_residue: &TraversalViewsOldAuthorityResidue,
    ) -> Result<Self, TraversalViewsMigrationError> {
        if !old_authority_residue.contains_required_caps() {
            return Err(TraversalViewsMigrationError::OldAuthorityResidueMissingRequiredCap);
        }
        let row = traversal_views_row(receipt)?;
        if row.product_output_digest() != Some(output.output_digest()) {
            return Err(TraversalViewsMigrationError::OutputDigestNotBoundToReceipt);
        }
        if receipt.selected_plan_digest() != output.selected_plan_digest() {
            return Err(TraversalViewsMigrationError::OutputSelectedPlanNotBoundToInput);
        }
        if diagnostic_projection.selected_plan_digest() != output.selected_plan_digest()
            || diagnostic_projection.read_stage_receipt_digest()
                != output.read_stage_receipt_digest()
            || diagnostic_projection.product_output_digest() != output.output_digest()
            || diagnostic_projection.touched_closure_traversal_bound()
                != output.touched_closure_traversal_bound()
            || diagnostic_projection.selected_traversal_count() != output.selected_traversal_count()
            || diagnostic_projection.available_traversal_count()
                != output.available_traversal_count()
        {
            return Err(TraversalViewsMigrationError::OutputDigestNotBoundToReceipt);
        }
        if row.execution_work_count() == 0 && output.selected_traversal_count() > 0 {
            return Err(TraversalViewsMigrationError::NoTraversalViewsExecutionObserved);
        }

        let counters = TraversalViewsMigrationCounters::new(
            output,
            row.execution_work_count(),
            row.whole_view_fallback_count(),
            non_traversal_placeholder_execution_count(receipt),
            old_authority_residue.capped_traversal_authority_count(),
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
            DerivedTopologyProductFamilyIdentity::TraversalViews,
            receipt.selected_plan_digest(),
            receipt.execution_receipt_digest(),
            row.row_digest(),
            output.output_digest(),
            old_authority_residue.residue_digest(),
            counters.counters_digest(),
        );
        let phase_eleven_seed = TraversalViewsPhaseElevenSeed::from_closeout_parts(
            &closeout_digest,
            &counters,
            old_authority_residue,
        );
        Ok(Self {
            execution_receipt_digest: receipt.execution_receipt_digest().to_string(),
            traversal_views_output_digest: output.output_digest().to_string(),
            traversal_views_executed_row_digest: row.row_digest().to_string(),
            diagnostic_projection_digest: diagnostic_projection.projection_digest().to_string(),
            old_authority_residue_digest: old_authority_residue.residue_digest().to_string(),
            counters,
            migrated_family_closeout,
            phase_eleven_seed,
            closeout_digest,
        })
    }

    pub fn execution_receipt_digest(&self) -> &str {
        &self.execution_receipt_digest
    }

    pub fn traversal_views_output_digest(&self) -> &str {
        &self.traversal_views_output_digest
    }

    pub fn old_authority_residue_digest(&self) -> &str {
        &self.old_authority_residue_digest
    }

    pub fn diagnostic_projection_digest(&self) -> &str {
        &self.diagnostic_projection_digest
    }

    pub const fn counters(&self) -> &TraversalViewsMigrationCounters {
        &self.counters
    }

    pub const fn migrated_family_closeout(&self) -> &MigratedDerivedProductFamilyCloseout {
        &self.migrated_family_closeout
    }

    pub const fn phase_eleven_seed(&self) -> &TraversalViewsPhaseElevenSeed {
        &self.phase_eleven_seed
    }

    pub fn closeout_digest(&self) -> &str {
        &self.closeout_digest
    }
}

fn non_traversal_placeholder_execution_count(
    receipt: &DerivedInvalidationExecutionReceipt,
) -> usize {
    receipt
        .executed_rows()
        .iter()
        .filter(|row| {
            row.family_identity() != DerivedTopologyProductFamilyIdentity::TraversalViews
                && row.product_output_digest().is_none()
        })
        .count()
}

fn traversal_views_row(
    receipt: &DerivedInvalidationExecutionReceipt,
) -> Result<&DerivedInvalidationExecutedProductRow, TraversalViewsMigrationError> {
    receipt
        .executed_rows()
        .iter()
        .find(|row| row.family_identity() == DerivedTopologyProductFamilyIdentity::TraversalViews)
        .ok_or(TraversalViewsMigrationError::ExecutionReceiptMissingTraversalViewsRow)
}

fn closeout_digest(
    receipt: &DerivedInvalidationExecutionReceipt,
    output: &TraversalViewsDerivedProductOutput,
    row: &DerivedInvalidationExecutedProductRow,
    diagnostic_projection: &TraversalViewsDiagnosticProjection,
    residue: &TraversalViewsOldAuthorityResidue,
    counters: &TraversalViewsMigrationCounters,
) -> String {
    super::super::super::catalog::catalog_digest([
        "worth-topo:traversal-views-migration-closeout:v1".to_string(),
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
