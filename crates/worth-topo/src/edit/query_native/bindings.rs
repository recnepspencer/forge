use forge_query::facade::ForgeQueryEntity;
use forge_relational::facade::identity::{EntityId, RelationId};
use worth_schema::facade::{
    WorthEntityKind, WorthRelationKind, WorthTopologyEntityKind, WorthTopologyRelationKind,
};

use super::WorthTopologyQueryEditExecutionError;

pub(super) struct QueryEntityBinding {
    pub(super) query_identity: String,
    pub(super) kind: WorthTopologyEntityKind,
}

pub(super) struct QueryRelationBinding {
    pub(super) query_identity: String,
    pub(super) kind: WorthTopologyRelationKind,
}

pub(super) fn query_entity_binding(
    rows: &[ForgeQueryEntity],
    entity_id: EntityId,
) -> Result<Option<QueryEntityBinding>, WorthTopologyQueryEditExecutionError> {
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
                WorthTopologyQueryEditExecutionError::MaterializedDecode(format!(
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
                    WorthTopologyQueryEditExecutionError::MaterializedDecode(format!(
                        "query entity `{}` is missing topology.kind while resolving existing binding",
                        row.identity
                    ))
                })?;
            let kind = WorthEntityKind::ALL
                .into_iter()
                .find_map(|kind| match kind {
                    WorthEntityKind::Topology(topology_kind)
                        if topology_kind.kind_name() == kind_name =>
                    {
                        Some(topology_kind)
                    }
                    _ => None,
                })
                .ok_or_else(|| {
                    WorthTopologyQueryEditExecutionError::MaterializedDecode(format!(
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
) -> Result<Option<QueryRelationBinding>, WorthTopologyQueryEditExecutionError> {
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
                WorthTopologyQueryEditExecutionError::MaterializedDecode(format!(
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
                    WorthTopologyQueryEditExecutionError::MaterializedDecode(format!(
                        "query relation `{}` is missing topology.kind while resolving existing binding",
                        row.identity
                    ))
                })?;
            let kind = WorthRelationKind::ALL
                .into_iter()
                .find_map(|kind| match kind {
                    WorthRelationKind::Topology(topology_kind)
                        if topology_kind.kind_name() == kind_name =>
                    {
                        Some(topology_kind)
                    }
                    _ => None,
                })
                .ok_or_else(|| {
                    WorthTopologyQueryEditExecutionError::MaterializedDecode(format!(
                        "query relation `{}` reported unknown topology kind `{kind_name}` while resolving existing binding",
                        row.identity
                    ))
                })?;
            return Ok(Some(QueryRelationBinding {
                query_identity: row.identity.clone(),
                kind,
            }));
        }
    }
    Ok(None)
}
