use forge_query::facade::ForgeQueryEntity;
use forge_relational::facade::identity::{EntityId, RelationId};
use schema::facade::{EntityKind, RelationKind, TopologyEntityKind, TopologyRelationKind};

use super::TopologyQueryEditExecutionError;

pub(super) struct QueryEntityBinding {
    pub(super) query_identity: String,
    pub(super) kind: TopologyEntityKind,
}

pub(super) struct QueryRelationBinding {
    pub(super) query_identity: String,
    pub(super) kind: TopologyRelationKind,
    pub(super) source_query_identity: String,
    pub(super) target_query_identity: String,
}

pub(super) fn query_outgoing_relation_target_identities(
    rows: &[ForgeQueryEntity],
    source_query_identity: &str,
    expected_kind: TopologyRelationKind,
) -> Result<Vec<String>, TopologyQueryEditExecutionError> {
    let mut target_identities = Vec::new();
    for row in rows {
        let Some(kind_name) = row
            .payload
            .get("topology")
            .and_then(|value| value.get("kind"))
            .and_then(serde_json::Value::as_str)
        else {
            continue;
        };
        if kind_name != expected_kind.kind_name() {
            continue;
        }
        let Some(row_source_identity) = row
            .payload
            .get("topology")
            .and_then(|value| value.get("source_identity"))
            .and_then(serde_json::Value::as_str)
        else {
            continue;
        };
        if row_source_identity != source_query_identity {
            continue;
        }
        let target_identity = row
            .payload
            .get("topology")
            .and_then(|value| value.get("target_identity"))
            .and_then(serde_json::Value::as_str)
            .ok_or_else(|| {
                TopologyQueryEditExecutionError::MaterializedDecode(format!(
                    "query relation `{}` is missing topology.target_identity while resolving outgoing `{}` binding",
                    row.identity,
                    expected_kind.kind_name()
                ))
            })?;
        target_identities.push(target_identity.to_string());
    }
    Ok(target_identities)
}

pub(super) fn query_outgoing_relation_ids(
    rows: &[ForgeQueryEntity],
    source_query_identity: &str,
    expected_kind: TopologyRelationKind,
) -> Result<Vec<RelationId>, TopologyQueryEditExecutionError> {
    let mut relation_ids = Vec::new();
    for row in rows {
        let Some(kind_name) = row
            .payload
            .get("topology")
            .and_then(|value| value.get("kind"))
            .and_then(serde_json::Value::as_str)
        else {
            continue;
        };
        if kind_name != expected_kind.kind_name() {
            continue;
        }
        let Some(row_source_identity) = row
            .payload
            .get("topology")
            .and_then(|value| value.get("source_identity"))
            .and_then(serde_json::Value::as_str)
        else {
            continue;
        };
        if row_source_identity != source_query_identity {
            continue;
        }
        let Some(provenance) = row
            .payload
            .get("lineage")
            .and_then(|value| value.get("provenance"))
        else {
            continue;
        };
        let relation_id = serde_json::from_value::<RelationId>(provenance.clone()).map_err(
            |error| {
                TopologyQueryEditExecutionError::MaterializedDecode(format!(
                    "failed to decode query relation provenance while resolving outgoing `{}` bindings: {error}",
                    expected_kind.kind_name()
                ))
            },
        )?;
        relation_ids.push(relation_id);
    }
    Ok(relation_ids)
}

pub(super) fn query_incoming_relation_source_identities(
    rows: &[ForgeQueryEntity],
    target_query_identity: &str,
    expected_kind: TopologyRelationKind,
) -> Result<Vec<String>, TopologyQueryEditExecutionError> {
    let mut source_identities = Vec::new();
    for row in rows {
        let Some(kind_name) = row
            .payload
            .get("topology")
            .and_then(|value| value.get("kind"))
            .and_then(serde_json::Value::as_str)
        else {
            continue;
        };
        if kind_name != expected_kind.kind_name() {
            continue;
        }
        let Some(row_target_identity) = row
            .payload
            .get("topology")
            .and_then(|value| value.get("target_identity"))
            .and_then(serde_json::Value::as_str)
        else {
            continue;
        };
        if row_target_identity != target_query_identity {
            continue;
        }
        let source_identity = row
            .payload
            .get("topology")
            .and_then(|value| value.get("source_identity"))
            .and_then(serde_json::Value::as_str)
            .ok_or_else(|| {
                TopologyQueryEditExecutionError::MaterializedDecode(format!(
                    "query relation `{}` is missing topology.source_identity while resolving incoming `{}` binding",
                    row.identity,
                    expected_kind.kind_name()
                ))
            })?;
        source_identities.push(source_identity.to_string());
    }
    Ok(source_identities)
}

pub(super) fn query_incoming_relation_ids(
    rows: &[ForgeQueryEntity],
    target_query_identity: &str,
    expected_kind: TopologyRelationKind,
) -> Result<Vec<RelationId>, TopologyQueryEditExecutionError> {
    let mut relation_ids = Vec::new();
    for row in rows {
        let Some(kind_name) = row
            .payload
            .get("topology")
            .and_then(|value| value.get("kind"))
            .and_then(serde_json::Value::as_str)
        else {
            continue;
        };
        if kind_name != expected_kind.kind_name() {
            continue;
        }
        let Some(row_target_identity) = row
            .payload
            .get("topology")
            .and_then(|value| value.get("target_identity"))
            .and_then(serde_json::Value::as_str)
        else {
            continue;
        };
        if row_target_identity != target_query_identity {
            continue;
        }
        let Some(provenance) = row
            .payload
            .get("lineage")
            .and_then(|value| value.get("provenance"))
        else {
            continue;
        };
        let relation_id = serde_json::from_value::<RelationId>(provenance.clone()).map_err(
            |error| {
                TopologyQueryEditExecutionError::MaterializedDecode(format!(
                    "failed to decode query relation provenance while resolving incoming `{}` bindings: {error}",
                    expected_kind.kind_name()
                ))
            },
        )?;
        relation_ids.push(relation_id);
    }
    Ok(relation_ids)
}

pub(super) fn query_entity_id_by_identity(
    rows: &[ForgeQueryEntity],
    query_identity: &str,
) -> Result<Option<EntityId>, TopologyQueryEditExecutionError> {
    for row in rows {
        if row.identity != query_identity {
            continue;
        }
        let Some(provenance) = row
            .payload
            .get("lineage")
            .and_then(|value| value.get("provenance"))
        else {
            continue;
        };
        let entity_id = serde_json::from_value::<EntityId>(provenance.clone()).map_err(|error| {
            TopologyQueryEditExecutionError::MaterializedDecode(format!(
                "failed to decode query entity provenance while resolving query identity `{query_identity}`: {error}"
            ))
        })?;
        return Ok(Some(entity_id));
    }
    Ok(None)
}

pub(super) fn query_entity_binding(
    rows: &[ForgeQueryEntity],
    entity_id: EntityId,
) -> Result<Option<QueryEntityBinding>, TopologyQueryEditExecutionError> {
    for row in rows {
        let Some(provenance) = row
            .payload
            .get("lineage")
            .and_then(|value| value.get("provenance"))
        else {
            continue;
        };
        let row_entity_id: EntityId = serde_json::from_value::<EntityId>(provenance.clone())
            .map_err(|error| {
                TopologyQueryEditExecutionError::MaterializedDecode(format!(
                    "failed to decode query entity provenance while resolving existing binding: {error}"
                ))
            })?;
        if row_entity_id == entity_id {
            let kind_name = row
                .payload
                .get("topology")
                .and_then(|value| value.get("kind"))
                .and_then(serde_json::Value::as_str)
                .ok_or_else(|| {
                    TopologyQueryEditExecutionError::MaterializedDecode(format!(
                        "query entity `{}` is missing topology.kind while resolving existing binding",
                        row.identity
                    ))
                })?;
            let kind = EntityKind::ALL
                .into_iter()
                .find_map(|kind| match kind {
                    EntityKind::Topology(topology_kind)
                        if topology_kind.kind_name() == kind_name =>
                    {
                        Some(topology_kind)
                    }
                    _ => None,
                })
                .ok_or_else(|| {
                    TopologyQueryEditExecutionError::MaterializedDecode(format!(
                        "query entity `{}` reported unknown topology kind `{kind_name}` while resolving existing binding",
                        row.identity
                    ))
                })?;
            return Ok(Some(QueryEntityBinding {
                query_identity: row.identity.clone(),
                kind,
            }));
        }
    }
    Ok(None)
}

pub(super) fn query_relation_binding(
    rows: &[ForgeQueryEntity],
    relation_id: RelationId,
) -> Result<Option<QueryRelationBinding>, TopologyQueryEditExecutionError> {
    for row in rows {
        let Some(provenance) = row
            .payload
            .get("lineage")
            .and_then(|value| value.get("provenance"))
        else {
            continue;
        };
        let row_relation_id: RelationId =
            serde_json::from_value::<RelationId>(provenance.clone()).map_err(|error| {
                TopologyQueryEditExecutionError::MaterializedDecode(format!(
                    "failed to decode query relation provenance while resolving existing binding: {error}"
                ))
            })?;
        if row_relation_id == relation_id {
            let kind_name = row
                .payload
                .get("topology")
                .and_then(|value| value.get("kind"))
                .and_then(serde_json::Value::as_str)
                .ok_or_else(|| {
                    TopologyQueryEditExecutionError::MaterializedDecode(format!(
                        "query relation `{}` is missing topology.kind while resolving existing binding",
                        row.identity
                    ))
                })?;
            let kind = RelationKind::ALL
                .into_iter()
                .find_map(|kind| match kind {
                    RelationKind::Topology(topology_kind)
                        if topology_kind.kind_name() == kind_name =>
                    {
                        Some(topology_kind)
                    }
                    _ => None,
                })
                .ok_or_else(|| {
                    TopologyQueryEditExecutionError::MaterializedDecode(format!(
                        "query relation `{}` reported unknown topology kind `{kind_name}` while resolving existing binding",
                        row.identity
                    ))
                })?;
            let source_query_identity = row
                .payload
                .get("topology")
                .and_then(|value| value.get("source_identity"))
                .and_then(serde_json::Value::as_str)
                .ok_or_else(|| {
                    TopologyQueryEditExecutionError::MaterializedDecode(format!(
                        "query relation `{}` is missing topology.source_identity while resolving existing binding",
                        row.identity
                    ))
                })?
                .to_string();
            let target_query_identity = row
                .payload
                .get("topology")
                .and_then(|value| value.get("target_identity"))
                .and_then(serde_json::Value::as_str)
                .ok_or_else(|| {
                    TopologyQueryEditExecutionError::MaterializedDecode(format!(
                        "query relation `{}` is missing topology.target_identity while resolving existing binding",
                        row.identity
                    ))
                })?
                .to_string();
            return Ok(Some(QueryRelationBinding {
                query_identity: row.identity.clone(),
                kind,
                source_query_identity,
                target_query_identity,
            }));
        }
    }
    Ok(None)
}
