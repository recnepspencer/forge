use serde::Serialize;

use super::{MaterializedGraphMigrationError, MaterializedGraphReadStageReceipt};
use crate::derived_topology::invalidation_plan::selection::DerivedInvalidationSelectedPlan;

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct MaterializedGraphExecutionInput {
    read_stage_receipt: MaterializedGraphReadStageReceipt,
    input_digest: String,
}

impl MaterializedGraphExecutionInput {
    pub fn from_selected_plan_and_read_stage(
        selected_plan: &DerivedInvalidationSelectedPlan,
        read_stage_receipt: MaterializedGraphReadStageReceipt,
    ) -> Result<Self, MaterializedGraphMigrationError> {
        if read_stage_receipt.selected_plan_digest() != selected_plan.selected_plan_digest()
            || read_stage_receipt.touched_closure_digest() != selected_plan.touched_closure_digest()
            || read_stage_receipt.query_support_digest() != selected_plan.query_support_digest()
            || read_stage_receipt.legality_support_digest()
                != selected_plan.legality_support_digest()
        {
            return Err(MaterializedGraphMigrationError::ReadStageReceiptNotBoundToSelectedPlan);
        }
        let selected_row = selected_plan
            .selected_rows()
            .iter()
            .find(|row| {
                row.family_identity()
                    == crate::derived_topology::invalidation_plan::catalog::DerivedTopologyProductFamilyIdentity::MaterializedGraph
            })
            .ok_or(MaterializedGraphMigrationError::SelectedPlanMissingMaterializedGraphRow)?;
        if selected_row.row_digest() != read_stage_receipt.materialized_graph_selected_row_digest()
            || selected_row.query_receipt_digest()
                != Some(read_stage_receipt.native_query_read_receipt_digest())
            || selected_row.legality_receipt_digest()
                != Some(read_stage_receipt.selected_legality_receipt_digest())
        {
            return Err(MaterializedGraphMigrationError::ReadStageReceiptNotBoundToSelectedPlan);
        }
        let input_digest = super::super::super::catalog::catalog_digest([
            "worth-topo:materialized-graph-execution-input:v1".to_string(),
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
                "materialized-row:{}",
                read_stage_receipt.materialized_graph_selected_row_digest()
            ),
        ]);
        Ok(Self {
            read_stage_receipt,
            input_digest,
        })
    }

    pub const fn read_stage_receipt(&self) -> &MaterializedGraphReadStageReceipt {
        &self.read_stage_receipt
    }

    pub fn selected_plan_digest(&self) -> &str {
        self.read_stage_receipt.selected_plan_digest()
    }

    pub const fn selected_entity_count(&self) -> usize {
        self.read_stage_receipt.selected_entity_count()
    }

    pub const fn selected_relation_count(&self) -> usize {
        self.read_stage_receipt.selected_relation_count()
    }

    pub fn input_digest(&self) -> &str {
        &self.input_digest
    }
}
