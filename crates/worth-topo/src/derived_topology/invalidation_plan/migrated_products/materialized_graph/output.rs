use serde::Serialize;

use super::{
    MaterializedGraphExecutionInput, MaterializedGraphReadEntityRow,
    MaterializedGraphReadRelationRow,
};

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct MaterializedGraphProductEntityRow {
    source_entity_id: forge_relational::facade::identity::EntityId,
    topology_kind: &'static str,
    source_row_digest: String,
    row_digest: String,
}

impl MaterializedGraphProductEntityRow {
    fn from_read_row(row: &MaterializedGraphReadEntityRow) -> Self {
        let source_entity_id = row.entity_id();
        let topology_kind = row.topology_kind();
        let source_row_digest = row.row_digest().to_string();
        let row_digest = super::super::super::catalog::catalog_digest([
            "worth-topo:materialized-graph-product-entity-row:v1".to_string(),
            format!("source:{source_entity_id:?}"),
            format!("kind:{topology_kind}"),
            format!("source-row:{source_row_digest}"),
        ]);
        Self {
            source_entity_id,
            topology_kind,
            source_row_digest,
            row_digest,
        }
    }

    pub const fn source_entity_id(&self) -> forge_relational::facade::identity::EntityId {
        self.source_entity_id
    }

    pub const fn topology_kind(&self) -> &'static str {
        self.topology_kind
    }

    pub fn row_digest(&self) -> &str {
        &self.row_digest
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct MaterializedGraphProductRelationRow {
    relation_kind: &'static str,
    source_entity_id: forge_relational::facade::identity::EntityId,
    target_entity_id: forge_relational::facade::identity::EntityId,
    source_row_digest: String,
    row_digest: String,
}

impl MaterializedGraphProductRelationRow {
    fn from_read_row(row: &MaterializedGraphReadRelationRow) -> Self {
        let relation_kind = row.relation_kind();
        let source_entity_id = row.source_entity_id();
        let target_entity_id = row.target_entity_id();
        let source_row_digest = row.row_digest().to_string();
        let row_digest = super::super::super::catalog::catalog_digest([
            "worth-topo:materialized-graph-product-relation-row:v1".to_string(),
            format!("kind:{relation_kind}"),
            format!("source:{source_entity_id:?}"),
            format!("target:{target_entity_id:?}"),
            format!("source-row:{source_row_digest}"),
        ]);
        Self {
            relation_kind,
            source_entity_id,
            target_entity_id,
            source_row_digest,
            row_digest,
        }
    }

    pub const fn relation_kind(&self) -> &'static str {
        self.relation_kind
    }

    pub const fn source_entity_id(&self) -> forge_relational::facade::identity::EntityId {
        self.source_entity_id
    }

    pub const fn target_entity_id(&self) -> forge_relational::facade::identity::EntityId {
        self.target_entity_id
    }

    pub fn row_digest(&self) -> &str {
        &self.row_digest
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct MaterializedGraphDerivedProductOutput {
    entity_rows: Vec<MaterializedGraphProductEntityRow>,
    relation_rows: Vec<MaterializedGraphProductRelationRow>,
    selected_entity_count: usize,
    selected_relation_count: usize,
    available_entity_count: usize,
    available_relation_count: usize,
    topology_entity_count: usize,
    topology_relation_count: usize,
    selected_plan_digest: String,
    read_stage_receipt_digest: String,
    input_digest: String,
    output_digest: String,
}

impl MaterializedGraphDerivedProductOutput {
    pub(crate) fn from_execution_input(input: &MaterializedGraphExecutionInput) -> Self {
        let receipt = input.read_stage_receipt();
        let entity_rows = receipt
            .selected_entity_rows()
            .iter()
            .map(MaterializedGraphProductEntityRow::from_read_row)
            .collect::<Vec<_>>();
        let relation_rows = receipt
            .selected_relation_rows()
            .iter()
            .map(MaterializedGraphProductRelationRow::from_read_row)
            .collect::<Vec<_>>();
        let selected_entity_count = receipt.selected_entity_count();
        let selected_relation_count = receipt.selected_relation_count();
        let available_entity_count = receipt.available_entity_count();
        let available_relation_count = receipt.available_relation_count();
        let topology_entity_count = receipt.topology_entity_count();
        let topology_relation_count = receipt.topology_relation_count();
        let selected_plan_digest = input.selected_plan_digest().to_string();
        let read_stage_receipt_digest = receipt.receipt_digest().to_string();
        let input_digest = input.input_digest().to_string();
        let output_digest = super::super::super::catalog::catalog_digest([
            "worth-topo:materialized-graph-derived-product-output:v1".to_string(),
            format!("selected-plan:{selected_plan_digest}"),
            format!("read-stage:{read_stage_receipt_digest}"),
            format!("input:{input_digest}"),
            format!("selected-entities:{selected_entity_count}"),
            format!("selected-relations:{selected_relation_count}"),
            format!("available-entities:{available_entity_count}"),
            format!("available-relations:{available_relation_count}"),
            format!("topology-entities:{topology_entity_count}"),
            format!("topology-relations:{topology_relation_count}"),
            format!(
                "entity-rows:{:?}",
                entity_rows
                    .iter()
                    .map(|row| row.row_digest())
                    .collect::<Vec<_>>()
            ),
            format!(
                "relation-rows:{:?}",
                relation_rows
                    .iter()
                    .map(|row| row.row_digest())
                    .collect::<Vec<_>>()
            ),
        ]);
        Self {
            entity_rows,
            relation_rows,
            selected_entity_count,
            selected_relation_count,
            available_entity_count,
            available_relation_count,
            topology_entity_count,
            topology_relation_count,
            selected_plan_digest,
            read_stage_receipt_digest,
            input_digest,
            output_digest,
        }
    }

    pub const fn selected_entity_count(&self) -> usize {
        self.selected_entity_count
    }

    pub fn entity_rows(&self) -> &[MaterializedGraphProductEntityRow] {
        &self.entity_rows
    }

    pub fn relation_rows(&self) -> &[MaterializedGraphProductRelationRow] {
        &self.relation_rows
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

    pub const fn topology_entity_count(&self) -> usize {
        self.topology_entity_count
    }

    pub const fn topology_relation_count(&self) -> usize {
        self.topology_relation_count
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
