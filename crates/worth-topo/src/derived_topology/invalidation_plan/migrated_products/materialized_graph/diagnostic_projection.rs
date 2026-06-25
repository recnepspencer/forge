use serde::Serialize;

use super::{MaterializedGraphDerivedProductOutput, MaterializedGraphReadStageReceipt};

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct MaterializedGraphDiagnosticProjection {
    selected_plan_digest: String,
    read_stage_receipt_digest: String,
    product_output_digest: String,
    selected_entity_count: usize,
    selected_relation_count: usize,
    available_entity_count: usize,
    available_relation_count: usize,
    projection_digest: String,
}

impl MaterializedGraphDiagnosticProjection {
    pub(crate) fn from_read_stage_and_output(
        receipt: &MaterializedGraphReadStageReceipt,
        output: &MaterializedGraphDerivedProductOutput,
    ) -> Self {
        let selected_plan_digest = output.selected_plan_digest().to_string();
        let read_stage_receipt_digest = receipt.receipt_digest().to_string();
        let product_output_digest = output.output_digest().to_string();
        let selected_entity_count = output.selected_entity_count();
        let selected_relation_count = output.selected_relation_count();
        let available_entity_count = output.available_entity_count();
        let available_relation_count = output.available_relation_count();
        let projection_digest = super::super::super::catalog::catalog_digest([
            "worth-topo:materialized-graph-diagnostic-projection:v1".to_string(),
            format!("selected-plan:{selected_plan_digest}"),
            format!("read-stage:{read_stage_receipt_digest}"),
            format!("product-output:{product_output_digest}"),
            format!("selected-entities:{selected_entity_count}"),
            format!("selected-relations:{selected_relation_count}"),
            format!("available-entities:{available_entity_count}"),
            format!("available-relations:{available_relation_count}"),
        ]);
        Self {
            selected_plan_digest,
            read_stage_receipt_digest,
            product_output_digest,
            selected_entity_count,
            selected_relation_count,
            available_entity_count,
            available_relation_count,
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

    pub const fn selected_entity_count(&self) -> usize {
        self.selected_entity_count
    }

    pub const fn selected_relation_count(&self) -> usize {
        self.selected_relation_count
    }

    pub const fn available_entity_count(&self) -> usize {
        self.available_entity_count
    }

    pub const fn available_relation_count(&self) -> usize {
        self.available_relation_count
    }

    pub fn projection_digest(&self) -> &str {
        &self.projection_digest
    }
}
