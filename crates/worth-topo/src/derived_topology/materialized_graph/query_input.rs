use forge_query::facade::ForgeQueryEntity;

use super::input_rows::{MaterializationEntityRow, MaterializationRelationRow};
use super::TopologyMaterializationError;

#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct TopologyQueryMaterializationInput {
    entity_rows: Vec<MaterializationEntityRow>,
    relation_rows: Vec<MaterializationRelationRow>,
    entity_count: usize,
    relation_count: usize,
}

impl TopologyQueryMaterializationInput {
    pub(crate) fn decode(
        entity_rows: &[ForgeQueryEntity],
        relation_rows: &[ForgeQueryEntity],
    ) -> Result<Self, TopologyMaterializationError> {
        Ok(Self {
            entity_rows: entity_rows
                .iter()
                .map(MaterializationEntityRow::from_query_row)
                .collect::<Result<Vec<_>, _>>()?,
            relation_rows: relation_rows
                .iter()
                .map(MaterializationRelationRow::from_query_row)
                .collect::<Result<Vec<_>, _>>()?,
            entity_count: entity_rows.len(),
            relation_count: relation_rows.len(),
        })
    }

    pub(crate) fn entities(&self) -> &[MaterializationEntityRow] {
        &self.entity_rows
    }

    pub(crate) fn relations(&self) -> &[MaterializationRelationRow] {
        &self.relation_rows
    }

    pub(crate) fn entity_count(&self) -> usize {
        self.entity_count
    }

    pub(crate) fn relation_count(&self) -> usize {
        self.relation_count
    }
}
