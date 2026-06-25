use forge_query::facade::ForgeQueryEntity;
#[cfg(test)]
use forge_relational::facade::identity::{EntityId, RelationId};

use crate::query_native_runtime_boundary::row_text_at;
#[cfg(test)]
use crate::query_native_runtime_boundary::{
    query_entity_id_from_identity, query_relation_id_from_identity,
};

#[cfg(test)]
#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct TopologyQueryRowError {
    detail: String,
}

#[cfg(test)]
impl TopologyQueryRowError {
    pub(crate) fn new(detail: impl Into<String>) -> Self {
        Self {
            detail: detail.into(),
        }
    }
}

#[cfg(test)]
impl std::fmt::Display for TopologyQueryRowError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(&self.detail)
    }
}

#[cfg(test)]
impl std::error::Error for TopologyQueryRowError {}

pub(crate) fn relation_kind_name(row: &ForgeQueryEntity) -> &str {
    row_text_at(row, ["topology", "kind"]).unwrap_or("")
}

pub(crate) fn topology_source_identity(row: &ForgeQueryEntity) -> Option<&str> {
    row_text_at(row, ["topology", "source_identity"])
}

pub(crate) fn topology_target_identity(row: &ForgeQueryEntity) -> Option<&str> {
    row_text_at(row, ["topology", "target_identity"])
}

#[cfg(test)]
pub(crate) fn query_relation_id_from_row(
    row: &ForgeQueryEntity,
) -> Result<RelationId, TopologyQueryRowError> {
    query_relation_id_from_identity(row.identity()).map_err(|error| {
        TopologyQueryRowError::new(format!("query relation provenance should decode: {error}"))
    })
}

#[cfg(test)]
pub(crate) fn query_entity_id_from_row(
    row: &ForgeQueryEntity,
) -> Result<EntityId, TopologyQueryRowError> {
    query_entity_id_from_identity(row.identity()).map_err(|error| {
        TopologyQueryRowError::new(format!("query entity provenance should decode: {error}"))
    })
}
