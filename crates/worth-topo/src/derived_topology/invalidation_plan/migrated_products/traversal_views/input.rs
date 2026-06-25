use serde::Serialize;

use super::{TraversalViewsMigrationError, TraversalViewsReadStageReceipt};
use crate::derived_topology::invalidation_plan::catalog::DerivedTopologyProductFamilyIdentity;
use crate::derived_topology::invalidation_plan::selection::DerivedInvalidationSelectedPlan;

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct TraversalViewsExecutionInput {
    read_stage_receipt: TraversalViewsReadStageReceipt,
    input_digest: String,
}

impl TraversalViewsExecutionInput {
    pub fn from_selected_plan_and_read_stage(
        selected_plan: &DerivedInvalidationSelectedPlan,
        read_stage_receipt: TraversalViewsReadStageReceipt,
    ) -> Result<Self, TraversalViewsMigrationError> {
        if read_stage_receipt.selected_plan_digest() != selected_plan.selected_plan_digest()
            || read_stage_receipt.touched_closure_digest() != selected_plan.touched_closure_digest()
            || read_stage_receipt.query_support_digest() != selected_plan.query_support_digest()
            || read_stage_receipt.legality_support_digest()
                != selected_plan.legality_support_digest()
        {
            return Err(TraversalViewsMigrationError::ReadStageReceiptNotBoundToSelectedPlan);
        }
        let selected_row = selected_plan
            .selected_rows()
            .iter()
            .find(|row| {
                row.family_identity() == DerivedTopologyProductFamilyIdentity::TraversalViews
            })
            .ok_or(TraversalViewsMigrationError::SelectedPlanMissingTraversalViewsRow)?;
        if selected_row.row_digest() != read_stage_receipt.traversal_views_selected_row_digest()
            || selected_row.query_receipt_digest()
                != Some(read_stage_receipt.native_query_read_receipt_digest())
            || selected_row.legality_receipt_digest()
                != Some(read_stage_receipt.selected_legality_receipt_digest())
        {
            return Err(TraversalViewsMigrationError::ReadStageReceiptNotBoundToSelectedPlan);
        }
        let input_digest = super::super::super::catalog::catalog_digest([
            "worth-topo:traversal-views-execution-input:v1".to_string(),
            format!("selected-plan:{}", selected_plan.selected_plan_digest()),
            format!("read-stage:{}", read_stage_receipt.receipt_digest()),
            format!(
                "native-query-read:{}",
                read_stage_receipt.native_query_read_receipt_digest()
            ),
            format!(
                "selected-legality:{}",
                read_stage_receipt.selected_legality_receipt_digest()
            ),
            format!(
                "traversal-row:{}",
                read_stage_receipt.traversal_views_selected_row_digest()
            ),
        ]);
        Ok(Self {
            read_stage_receipt,
            input_digest,
        })
    }

    pub const fn read_stage_receipt(&self) -> &TraversalViewsReadStageReceipt {
        &self.read_stage_receipt
    }

    pub fn selected_plan_digest(&self) -> &str {
        self.read_stage_receipt.selected_plan_digest()
    }

    pub const fn selected_traversal_count(&self) -> usize {
        self.read_stage_receipt.selected_traversal_count()
    }

    pub fn input_digest(&self) -> &str {
        &self.input_digest
    }
}
