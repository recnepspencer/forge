use serde::Serialize;

use super::{TraversalViewsExecutionInput, TraversalViewsSourceRow};

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct TraversalViewsProductRow {
    traversal_kind: &'static str,
    anchor_entity_id: forge_relational::facade::identity::EntityId,
    reached_entity_count: usize,
    source_row_digest: String,
    row_digest: String,
}

impl TraversalViewsProductRow {
    fn from_source_row(row: &TraversalViewsSourceRow) -> Self {
        let traversal_kind = row.traversal_kind();
        let anchor_entity_id = row.anchor_entity_id();
        let reached_entity_count = row.reached_entity_count();
        let source_row_digest = row.row_digest().to_string();
        let row_digest = super::super::super::catalog::catalog_digest([
            "worth-topo:traversal-views-product-row:v1".to_string(),
            format!("kind:{traversal_kind}"),
            format!("anchor:{anchor_entity_id:?}"),
            format!("reached:{reached_entity_count}"),
            format!("source-row:{source_row_digest}"),
        ]);
        Self {
            traversal_kind,
            anchor_entity_id,
            reached_entity_count,
            source_row_digest,
            row_digest,
        }
    }

    pub const fn traversal_kind(&self) -> &'static str {
        self.traversal_kind
    }

    pub const fn anchor_entity_id(&self) -> forge_relational::facade::identity::EntityId {
        self.anchor_entity_id
    }

    pub const fn reached_entity_count(&self) -> usize {
        self.reached_entity_count
    }

    pub fn row_digest(&self) -> &str {
        &self.row_digest
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct TraversalViewsDerivedProductOutput {
    rows: Vec<TraversalViewsProductRow>,
    touched_closure_traversal_bound: usize,
    selected_traversal_count: usize,
    available_traversal_count: usize,
    selected_plan_digest: String,
    read_stage_receipt_digest: String,
    input_digest: String,
    output_digest: String,
}

impl TraversalViewsDerivedProductOutput {
    pub(crate) fn from_execution_input(input: &TraversalViewsExecutionInput) -> Self {
        let receipt = input.read_stage_receipt();
        let rows = receipt
            .selected_rows()
            .iter()
            .map(TraversalViewsProductRow::from_source_row)
            .collect::<Vec<_>>();
        let touched_closure_traversal_bound = receipt.touched_closure_traversal_bound();
        let selected_traversal_count = receipt.selected_traversal_count();
        let available_traversal_count = receipt.available_traversal_count();
        let selected_plan_digest = input.selected_plan_digest().to_string();
        let read_stage_receipt_digest = receipt.receipt_digest().to_string();
        let input_digest = input.input_digest().to_string();
        let output_digest = super::super::super::catalog::catalog_digest([
            "worth-topo:traversal-views-derived-product-output:v1".to_string(),
            format!("selected-plan:{selected_plan_digest}"),
            format!("read-stage:{read_stage_receipt_digest}"),
            format!("input:{input_digest}"),
            format!("touched-bound:{touched_closure_traversal_bound}"),
            format!("selected-traversals:{selected_traversal_count}"),
            format!("available-traversals:{available_traversal_count}"),
            format!(
                "rows:{:?}",
                rows.iter().map(|row| row.row_digest()).collect::<Vec<_>>()
            ),
        ]);
        Self {
            rows,
            touched_closure_traversal_bound,
            selected_traversal_count,
            available_traversal_count,
            selected_plan_digest,
            read_stage_receipt_digest,
            input_digest,
            output_digest,
        }
    }

    pub fn rows(&self) -> &[TraversalViewsProductRow] {
        &self.rows
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

    pub fn selected_plan_digest(&self) -> &str {
        &self.selected_plan_digest
    }

    pub fn read_stage_receipt_digest(&self) -> &str {
        &self.read_stage_receipt_digest
    }

    pub fn input_digest(&self) -> &str {
        &self.input_digest
    }

    pub fn output_digest(&self) -> &str {
        &self.output_digest
    }
}
