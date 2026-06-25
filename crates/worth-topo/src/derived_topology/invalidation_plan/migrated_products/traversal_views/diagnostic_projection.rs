use serde::Serialize;

use super::{TraversalViewsDerivedProductOutput, TraversalViewsReadStageReceipt};

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct TraversalViewsDiagnosticProjection {
    selected_plan_digest: String,
    read_stage_receipt_digest: String,
    product_output_digest: String,
    touched_closure_traversal_bound: usize,
    selected_traversal_count: usize,
    available_traversal_count: usize,
    projection_digest: String,
}

impl TraversalViewsDiagnosticProjection {
    pub(crate) fn from_read_stage_and_output(
        receipt: &TraversalViewsReadStageReceipt,
        output: &TraversalViewsDerivedProductOutput,
    ) -> Self {
        let selected_plan_digest = output.selected_plan_digest().to_string();
        let read_stage_receipt_digest = receipt.receipt_digest().to_string();
        let product_output_digest = output.output_digest().to_string();
        let touched_closure_traversal_bound = output.touched_closure_traversal_bound();
        let selected_traversal_count = output.selected_traversal_count();
        let available_traversal_count = output.available_traversal_count();
        let projection_digest = super::super::super::catalog::catalog_digest([
            "worth-topo:traversal-views-diagnostic-projection:v1".to_string(),
            format!("selected-plan:{selected_plan_digest}"),
            format!("read-stage:{read_stage_receipt_digest}"),
            format!("product-output:{product_output_digest}"),
            format!("touched-bound:{touched_closure_traversal_bound}"),
            format!("selected-traversals:{selected_traversal_count}"),
            format!("available-traversals:{available_traversal_count}"),
        ]);
        Self {
            selected_plan_digest,
            read_stage_receipt_digest,
            product_output_digest,
            touched_closure_traversal_bound,
            selected_traversal_count,
            available_traversal_count,
            projection_digest,
        }
    }

    pub fn selected_plan_digest(&self) -> &str {
        &self.selected_plan_digest
    }

    pub fn read_stage_receipt_digest(&self) -> &str {
        &self.read_stage_receipt_digest
    }

    pub fn product_output_digest(&self) -> &str {
        &self.product_output_digest
    }

    pub const fn touched_closure_traversal_bound(&self) -> usize {
        self.touched_closure_traversal_bound
    }

    pub const fn selected_traversal_count(&self) -> usize {
        self.selected_traversal_count
    }

    pub const fn available_traversal_count(&self) -> usize {
        self.available_traversal_count
    }

    pub fn projection_digest(&self) -> &str {
        &self.projection_digest
    }
}
