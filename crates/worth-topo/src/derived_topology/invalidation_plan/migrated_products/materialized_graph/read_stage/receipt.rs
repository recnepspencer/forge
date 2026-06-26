use serde::Serialize;

use super::source::{
    MaterializedGraphReadEntityRow, MaterializedGraphReadRelationRow, MaterializedGraphReadSource,
};
use crate::derived_topology::invalidation_plan::catalog::DerivedTopologyProductFamilyIdentity;
use crate::derived_topology::invalidation_plan::migrated_products::materialized_graph::MaterializedGraphMigrationError;
use crate::derived_topology::invalidation_plan::selection::DerivedInvalidationSelectedPlan;

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct MaterializedGraphReadStageReceipt {
    selected_plan_digest: String,
    touched_closure_digest: String,
    query_support_digest: String,
    legality_support_digest: String,
    materialized_graph_selected_row_digest: String,
    native_query_read_receipt_digest: String,
    selected_legality_receipt_digest: String,
    read_source_digest: String,
    selected_entity_count: usize,
    selected_relation_count: usize,
    available_entity_count: usize,
    available_relation_count: usize,
    topology_entity_count: usize,
    topology_relation_count: usize,
    selected_entity_rows: Vec<MaterializedGraphReadEntityRow>,
    selected_relation_rows: Vec<MaterializedGraphReadRelationRow>,
    receipt_digest: String,
}

impl MaterializedGraphReadStageReceipt {
    pub fn from_selected_plan_and_read_source(
        selected_plan: &DerivedInvalidationSelectedPlan,
        read_source: MaterializedGraphReadSource,
    ) -> Result<Self, MaterializedGraphMigrationError> {
        let selected_materialized_graph_row = selected_plan
            .selected_rows()
            .iter()
            .find(|row| {
                row.family_identity() == DerivedTopologyProductFamilyIdentity::MaterializedGraph
            })
            .ok_or(MaterializedGraphMigrationError::SelectedPlanMissingMaterializedGraphRow)?;
        let native_query_read_receipt_digest = selected_materialized_graph_row
            .query_receipt_digest()
            .ok_or(MaterializedGraphMigrationError::ReadStageReceiptMissingQueryReceipt)?
            .to_string();
        let selected_legality_receipt_digest = selected_materialized_graph_row
            .legality_receipt_digest()
            .ok_or(MaterializedGraphMigrationError::ReadStageReceiptMissingLegalityReceipt)?
            .to_string();
        let materialized_graph_selected_row_digest =
            selected_materialized_graph_row.row_digest().to_string();
        let selected_entity_count = read_source.selected_entities().len();
        let selected_relation_count = read_source.selected_relations().len();
        let available_entity_count = read_source.available_entity_count();
        let available_relation_count = read_source.available_relation_count();
        let topology_entity_count = available_entity_count;
        let topology_relation_count = available_relation_count;
        let read_source_digest = read_source.source_digest().to_string();
        let selected_entity_rows = read_source.selected_entities().to_vec();
        let selected_relation_rows = read_source.selected_relations().to_vec();
        let mut receipt_parts = vec![
            "worth-topo:materialized-graph-read-stage-receipt:v1".to_string(),
            format!("selected-plan:{}", selected_plan.selected_plan_digest()),
            format!("touched-closure:{}", selected_plan.touched_closure_digest()),
            format!("query-support:{}", selected_plan.query_support_digest()),
            format!(
                "legality-support:{}",
                selected_plan.legality_support_digest()
            ),
            format!("selected-row:{materialized_graph_selected_row_digest}"),
            format!("native-query-read:{native_query_read_receipt_digest}"),
            format!("selected-legality:{selected_legality_receipt_digest}"),
            format!("read-source:{read_source_digest}"),
            format!("selected-entities:{selected_entity_count}"),
            format!("selected-relations:{selected_relation_count}"),
            format!("available-entities:{available_entity_count}"),
            format!("available-relations:{available_relation_count}"),
            format!("topology-entities:{topology_entity_count}"),
            format!("topology-relations:{topology_relation_count}"),
        ];
        receipt_parts.extend(
            selected_entity_rows
                .iter()
                .map(|row| format!("entity-row:{}", row.row_digest())),
        );
        receipt_parts.extend(
            selected_relation_rows
                .iter()
                .map(|row| format!("relation-row:{}", row.row_digest())),
        );
        let receipt_digest = super::super::super::super::catalog::catalog_digest(receipt_parts);
        Ok(Self {
            selected_plan_digest: selected_plan.selected_plan_digest().to_string(),
            touched_closure_digest: selected_plan.touched_closure_digest().to_string(),
            query_support_digest: selected_plan.query_support_digest().to_string(),
            legality_support_digest: selected_plan.legality_support_digest().to_string(),
            materialized_graph_selected_row_digest,
            native_query_read_receipt_digest,
            selected_legality_receipt_digest,
            read_source_digest,
            selected_entity_count,
            selected_relation_count,
            available_entity_count,
            available_relation_count,
            topology_entity_count,
            topology_relation_count,
            selected_entity_rows,
            selected_relation_rows,
            receipt_digest,
        })
    }

    pub fn selected_plan_digest(&self) -> &str {
        &self.selected_plan_digest
    }

    pub fn touched_closure_digest(&self) -> &str {
        &self.touched_closure_digest
    }

    pub fn query_support_digest(&self) -> &str {
        &self.query_support_digest
    }

    pub fn legality_support_digest(&self) -> &str {
        &self.legality_support_digest
    }

    pub fn materialized_graph_selected_row_digest(&self) -> &str {
        &self.materialized_graph_selected_row_digest
    }

    pub fn native_query_read_receipt_digest(&self) -> &str {
        &self.native_query_read_receipt_digest
    }

    pub fn selected_legality_receipt_digest(&self) -> &str {
        &self.selected_legality_receipt_digest
    }

    pub fn read_source_digest(&self) -> &str {
        &self.read_source_digest
    }

    pub fn selected_entity_rows(&self) -> &[MaterializedGraphReadEntityRow] {
        &self.selected_entity_rows
    }

    pub fn selected_relation_rows(&self) -> &[MaterializedGraphReadRelationRow] {
        &self.selected_relation_rows
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

    pub const fn topology_entity_count(&self) -> usize {
        self.topology_entity_count
    }

    pub const fn topology_relation_count(&self) -> usize {
        self.topology_relation_count
    }

    pub fn receipt_digest(&self) -> &str {
        &self.receipt_digest
    }

    #[cfg(test)]
    pub(crate) fn with_selected_plan_digest_for_tests(
        mut self,
        selected_plan_digest: &'static str,
    ) -> Self {
        self.selected_plan_digest = selected_plan_digest.to_string();
        self
    }

    #[cfg(test)]
    pub(crate) fn with_native_query_read_receipt_digest_for_tests(
        mut self,
        digest: &'static str,
    ) -> Self {
        self.native_query_read_receipt_digest = digest.to_string();
        self
    }
}
