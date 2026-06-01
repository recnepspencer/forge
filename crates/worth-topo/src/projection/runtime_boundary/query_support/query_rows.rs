use forge_query::facade::ForgeQueryEntity;
use forge_relational::facade::identity::{EntityId, RelationId};

#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct TopologyQueryRowError {
    detail: String,
}

impl TopologyQueryRowError {
    pub(crate) fn new(detail: impl Into<String>) -> Self {
        Self {
            detail: detail.into(),
        }
    }
}

impl std::fmt::Display for TopologyQueryRowError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(&self.detail)
    }
}

impl std::error::Error for TopologyQueryRowError {}

pub(crate) fn relation_kind_name(row: &ForgeQueryEntity) -> &str {
    row.external_row()
        .get("topology")
        .and_then(|value| value.get("kind"))
        .and_then(|value| value.as_str())
        .unwrap_or("")
}

pub(crate) fn topology_source_identity(row: &ForgeQueryEntity) -> Option<&str> {
    row.external_row()
        .get("topology")
        .and_then(|value| value.get("source_identity"))
        .and_then(|value| value.as_str())
}

pub(crate) fn topology_target_identity(row: &ForgeQueryEntity) -> Option<&str> {
    row.external_row()
        .get("topology")
        .and_then(|value| value.get("target_identity"))
        .and_then(|value| value.as_str())
}

pub(crate) fn query_relation_id_from_row(
    row: &ForgeQueryEntity,
) -> Result<RelationId, TopologyQueryRowError> {
    serde_json::from_value(row.external_row()["lineage"]["provenance"].clone()).map_err(|error| {
        TopologyQueryRowError::new(format!("query relation provenance should decode: {error}"))
    })
}

pub(crate) fn query_entity_id_from_row(
    row: &ForgeQueryEntity,
) -> Result<EntityId, TopologyQueryRowError> {
    serde_json::from_value(row.external_row()["lineage"]["provenance"].clone()).map_err(|error| {
        TopologyQueryRowError::new(format!("query entity provenance should decode: {error}"))
    })
}
