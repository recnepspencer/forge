use std::collections::BTreeMap;

use forge_query::facade::{ForgeQueryEntity, ForgeQueryEntityIdentity};
use forge_relational::facade::identity::{EntityId, RelationId};
use forge_runtime_bridge::facade::RelationalBridgeRecordIdentityKind;
use schema::facade::platform::entities::{EntityKind, TopologyEntityKind};
use schema::facade::platform::relations::{RelationKind, TopologyRelationKind};

use crate::topology_operators::application::bindings::{QueryEntityBinding, QueryRelationBinding};
use crate::topology_operators::application::TopologyMutationApplicationError;

#[derive(Debug, Clone, Default)]
pub(crate) struct TopologyQueryBindingIndex {
    entity_bindings_by_id: BTreeMap<EntityId, QueryEntityBinding>,
    entity_ids_by_identity: BTreeMap<String, EntityId>,
    relation_bindings_by_id: BTreeMap<RelationId, QueryRelationBinding>,
    outgoing_relation_target_identities: BTreeMap<(String, String), Vec<String>>,
    outgoing_relation_ids: BTreeMap<(String, String), Vec<RelationId>>,
    incoming_relation_source_identities: BTreeMap<(String, String), Vec<String>>,
    incoming_relation_ids: BTreeMap<(String, String), Vec<RelationId>>,
}

impl TopologyQueryBindingIndex {
    pub(crate) fn from_query_rows(
        entity_rows: &[ForgeQueryEntity],
        relation_rows: &[ForgeQueryEntity],
    ) -> Result<Self, TopologyMutationApplicationError> {
        let mut bindings = Self::default();
        for row in entity_rows {
            let entity_id = decode_entity_id(row, "existing entity binding")?;
            let kind = decode_entity_kind(row, "existing entity binding")?;
            let query_identity_label = query_identity_label(row.identity())?;
            bindings
                .entity_ids_by_identity
                .insert(query_identity_label.clone(), entity_id);
            bindings.entity_bindings_by_id.insert(
                entity_id,
                QueryEntityBinding {
                    query_identity: row.identity().clone(),
                    query_identity_label,
                    kind,
                },
            );
        }
        for row in relation_rows {
            let relation_id = decode_relation_id(row, "existing relation binding")?;
            let kind = decode_relation_kind(row, "existing relation binding")?;
            let source_query_identity =
                decode_relation_endpoint(row, "source_identity", "existing relation binding")?;
            let target_query_identity =
                decode_relation_endpoint(row, "target_identity", "existing relation binding")?;
            let key = relation_key(&source_query_identity, kind);
            bindings
                .outgoing_relation_target_identities
                .entry(key.clone())
                .or_default()
                .push(target_query_identity.clone());
            bindings
                .outgoing_relation_ids
                .entry(key)
                .or_default()
                .push(relation_id);
            let incoming_key = relation_key(&target_query_identity, kind);
            bindings
                .incoming_relation_source_identities
                .entry(incoming_key.clone())
                .or_default()
                .push(source_query_identity.clone());
            bindings
                .incoming_relation_ids
                .entry(incoming_key)
                .or_default()
                .push(relation_id);
            bindings.relation_bindings_by_id.insert(
                relation_id,
                QueryRelationBinding {
                    query_identity: row.identity().clone(),
                    kind,
                    source_query_identity,
                    target_query_identity,
                },
            );
        }
        Ok(bindings)
    }

    pub(crate) fn outgoing_relation_target_identities(
        &self,
        source_query_identity: &str,
        expected_kind: TopologyRelationKind,
    ) -> Vec<String> {
        self.outgoing_relation_target_identities
            .get(&relation_key(source_query_identity, expected_kind))
            .cloned()
            .unwrap_or_default()
    }

    pub(crate) fn incoming_relation_source_identities(
        &self,
        target_query_identity: &str,
        expected_kind: TopologyRelationKind,
    ) -> Vec<String> {
        self.incoming_relation_source_identities
            .get(&relation_key(target_query_identity, expected_kind))
            .cloned()
            .unwrap_or_default()
    }

    pub(crate) fn incoming_relation_ids(
        &self,
        target_query_identity: &str,
        expected_kind: TopologyRelationKind,
    ) -> Vec<RelationId> {
        self.incoming_relation_ids
            .get(&relation_key(target_query_identity, expected_kind))
            .cloned()
            .unwrap_or_default()
    }

    pub(crate) fn entity_id_by_identity(&self, query_identity: &str) -> Option<EntityId> {
        self.entity_ids_by_identity.get(query_identity).copied()
    }

    pub(crate) fn entity_binding(&self, entity_id: EntityId) -> Option<QueryEntityBinding> {
        self.entity_bindings_by_id.get(&entity_id).cloned()
    }

    pub(crate) fn relation_binding(&self, relation_id: RelationId) -> Option<QueryRelationBinding> {
        self.relation_bindings_by_id.get(&relation_id).cloned()
    }
}

fn relation_key(identity: &str, kind: TopologyRelationKind) -> (String, String) {
    (identity.to_string(), kind.kind_name().to_string())
}

fn query_identity_label(
    identity: &ForgeQueryEntityIdentity,
) -> Result<String, TopologyMutationApplicationError> {
    let parts = identity.relational_record_parts().ok_or_else(|| {
        TopologyMutationApplicationError::MaterializedDecode(format!(
            "topology operator bindings require relational query row identities, got `{identity}`"
        ))
    })?;
    let kind = match parts.kind() {
        RelationalBridgeRecordIdentityKind::Entity => "entity",
        RelationalBridgeRecordIdentityKind::Relation => "relation",
    };
    Ok(format!(
        "{kind}:{}:{}:{}",
        parts.partition_id(),
        parts.local_slot(),
        parts.generation()
    ))
}

pub(crate) fn decode_entity_id(
    row: &ForgeQueryEntity,
    context: &str,
) -> Result<EntityId, TopologyMutationApplicationError> {
    let provenance = row
        .external_row()
        .get("lineage")
        .and_then(|value| value.get("provenance"))
        .ok_or_else(|| {
            TopologyMutationApplicationError::MaterializedDecode(format!(
                "query entity `{}` is missing lineage.provenance while resolving {context}",
                row.identity()
            ))
        })?;
    serde_json::from_value::<EntityId>(provenance.clone()).map_err(|error| {
        TopologyMutationApplicationError::MaterializedDecode(format!(
            "failed to decode query entity provenance while resolving {context}: {error}"
        ))
    })
}

pub(crate) fn decode_relation_id(
    row: &ForgeQueryEntity,
    context: &str,
) -> Result<RelationId, TopologyMutationApplicationError> {
    let provenance = row
        .external_row()
        .get("lineage")
        .and_then(|value| value.get("provenance"))
        .ok_or_else(|| {
            TopologyMutationApplicationError::MaterializedDecode(format!(
                "query relation `{}` is missing lineage.provenance while resolving {context}",
                row.identity()
            ))
        })?;
    serde_json::from_value::<RelationId>(provenance.clone()).map_err(|error| {
        TopologyMutationApplicationError::MaterializedDecode(format!(
            "failed to decode query relation provenance while resolving {context}: {error}"
        ))
    })
}

pub(crate) fn decode_entity_kind(
    row: &ForgeQueryEntity,
    context: &str,
) -> Result<TopologyEntityKind, TopologyMutationApplicationError> {
    let kind_name = row
        .external_row()
        .get("topology")
        .and_then(|value| value.get("kind"))
        .and_then(serde_json::Value::as_str)
        .ok_or_else(|| {
            TopologyMutationApplicationError::MaterializedDecode(format!(
                "query entity `{}` is missing topology.kind while resolving {context}",
                row.identity()
            ))
        })?;
    EntityKind::ALL
        .into_iter()
        .find_map(|kind| match kind {
            EntityKind::Topology(topology_kind) if topology_kind.kind_name() == kind_name => {
                Some(topology_kind)
            }
            _ => None,
        })
        .ok_or_else(|| {
            TopologyMutationApplicationError::MaterializedDecode(format!(
                "query entity `{}` reported unknown topology kind `{kind_name}` while resolving {context}",
                row.identity()
            ))
        })
}

pub(crate) fn decode_relation_kind(
    row: &ForgeQueryEntity,
    context: &str,
) -> Result<TopologyRelationKind, TopologyMutationApplicationError> {
    let kind_name = row
        .external_row()
        .get("topology")
        .and_then(|value| value.get("kind"))
        .and_then(serde_json::Value::as_str)
        .ok_or_else(|| {
            TopologyMutationApplicationError::MaterializedDecode(format!(
                "query relation `{}` is missing topology.kind while resolving {context}",
                row.identity()
            ))
        })?;
    RelationKind::ALL
        .into_iter()
        .find_map(|kind| match kind {
            RelationKind::Topology(topology_kind) if topology_kind.kind_name() == kind_name => {
                Some(topology_kind)
            }
            _ => None,
        })
        .ok_or_else(|| {
            TopologyMutationApplicationError::MaterializedDecode(format!(
                "query relation `{}` reported unknown topology kind `{kind_name}` while resolving {context}",
                row.identity()
            ))
        })
}

pub(crate) fn decode_relation_endpoint(
    row: &ForgeQueryEntity,
    endpoint: &'static str,
    context: &str,
) -> Result<String, TopologyMutationApplicationError> {
    row.external_row()
        .get("topology")
        .and_then(|value| value.get(endpoint))
        .and_then(serde_json::Value::as_str)
        .map(str::to_string)
        .ok_or_else(|| {
            TopologyMutationApplicationError::MaterializedDecode(format!(
                "query relation `{}` is missing topology.{endpoint} while resolving {context}",
                row.identity()
            ))
        })
}
