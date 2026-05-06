use forge_query::facade::ForgeQueryEntity;
use forge_relational::facade::identity::{EntityId, RelationId};

use super::snapshot_index::WorthTopologyQuerySnapshotIndexError;

pub(crate) fn relation_kind_name(row: &ForgeQueryEntity) -> &str {
    row.payload
        .get("topology")
        .and_then(|value| value.get("kind"))
        .and_then(|value| value.as_str())
        .unwrap_or("")
}

pub(crate) fn topology_source_identity(row: &ForgeQueryEntity) -> Option<&str> {
    row.payload
        .get("topology")
        .and_then(|value| value.get("source_identity"))
        .and_then(|value| value.as_str())
}

pub(crate) fn topology_target_identity(row: &ForgeQueryEntity) -> Option<&str> {
    row.payload
        .get("topology")
        .and_then(|value| value.get("target_identity"))
        .and_then(|value| value.as_str())
}

pub(crate) fn query_relation_id_from_row(
    row: &ForgeQueryEntity,
) -> Result<RelationId, WorthTopologyQuerySnapshotIndexError> {
    serde_json::from_value(row.payload["lineage"]["provenance"].clone()).map_err(|error| {
        WorthTopologyQuerySnapshotIndexError::new(format!(
            "query relation provenance should decode: {error}"
        ))
    })
}

pub(crate) fn query_entity_id_from_row(
    row: &ForgeQueryEntity,
) -> Result<EntityId, WorthTopologyQuerySnapshotIndexError> {
    serde_json::from_value(row.payload["lineage"]["provenance"].clone()).map_err(|error| {
        WorthTopologyQuerySnapshotIndexError::new(format!(
            "query entity provenance should decode: {error}"
        ))
    })
}
