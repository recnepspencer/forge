use std::collections::BTreeMap;

use forge_query::facade::ForgeQueryEntity;
use forge_relational::facade::identity::EntityId;
use forge_relational::facade::runtime::{EntityReadRecord, RelationReadRecord};
use serde_json::Value;
use worth_schema::facade::{
    WorthEntityKind, WorthNamingEntityKind, WorthNamingRelationKind, WorthRelationKind,
};

use crate::query::materialized::topology_relation_dependency_path;

use super::binding::WorthTopologyRuntimeBinding;

pub(super) fn topology_entity_rows(binding: &WorthTopologyRuntimeBinding) -> Vec<ForgeQueryEntity> {
    let names = topology_entity_persistent_name_map(binding);
    binding
        .entity_records()
        .into_iter()
        .filter_map(|entity| entity_row(&entity, &names))
        .collect()
}

pub(super) fn topology_relation_rows(
    binding: &WorthTopologyRuntimeBinding,
) -> Vec<ForgeQueryEntity> {
    let identities = entity_identity_map(binding);
    binding
        .relation_records()
        .into_iter()
        .filter_map(|relation| relation_row(&relation, &identities))
        .collect()
}

pub(super) fn persistent_name_rows(binding: &WorthTopologyRuntimeBinding) -> Vec<ForgeQueryEntity> {
    let targets = persistent_name_target_map(binding);
    binding
        .entity_records()
        .into_iter()
        .filter_map(|entity| persistent_name_row(&entity, &targets))
        .collect()
}

fn entity_identity_map(binding: &WorthTopologyRuntimeBinding) -> BTreeMap<EntityId, String> {
    binding
        .entity_records()
        .into_iter()
        .filter_map(|entity| {
            WorthEntityKind::from_kind_id(entity.kind.kind_id)
                .filter(|kind| kind.is_topological())
                .map(|_| (entity.entity_id, entity_identity(entity.entity_id)))
        })
        .collect()
}

fn persistent_name_target_map(binding: &WorthTopologyRuntimeBinding) -> BTreeMap<EntityId, String> {
    let identities = entity_identity_map(binding);
    binding
        .relation_records()
        .into_iter()
        .filter_map(
            |relation| match WorthRelationKind::from_kind_id(relation.kind.kind_id) {
                Some(WorthRelationKind::Naming(
                    WorthNamingRelationKind::PersistentNameTargetsEntity,
                )) => identities
                    .get(&relation.target)
                    .cloned()
                    .map(|target| (relation.source, target)),
                _ => None,
            },
        )
        .collect()
}

fn topology_entity_persistent_name_map(
    binding: &WorthTopologyRuntimeBinding,
) -> BTreeMap<EntityId, String> {
    let name_entities = binding
        .entity_records()
        .into_iter()
        .filter_map(|entity| {
            (WorthEntityKind::from_kind_id(entity.kind.kind_id)
                == Some(WorthEntityKind::Naming(
                    WorthNamingEntityKind::PersistentName,
                )))
            .then(|| {
                let name = entity
                    .payload
                    .as_json()
                    .and_then(|value| value.get("naming"))
                    .and_then(|value| value.get("persistent_name"))
                    .and_then(|value| value.as_str())
                    .map(str::to_string)?;
                Some((entity.entity_id, name))
            })?
        })
        .collect::<BTreeMap<_, _>>();
    binding
        .relation_records()
        .into_iter()
        .filter_map(
            |relation| match WorthRelationKind::from_kind_id(relation.kind.kind_id) {
                Some(WorthRelationKind::Naming(
                    WorthNamingRelationKind::PersistentNameTargetsEntity,
                )) => name_entities
                    .get(&relation.source)
                    .cloned()
                    .map(|name| (relation.target, name)),
                _ => None,
            },
        )
        .collect()
}

fn entity_row(
    entity: &EntityReadRecord,
    persistent_names: &BTreeMap<EntityId, String>,
) -> Option<ForgeQueryEntity> {
    let kind = WorthEntityKind::from_kind_id(entity.kind.kind_id)?;
    if !kind.is_topological() {
        return None;
    }
    let payload = entity.payload.as_json();
    Some(ForgeQueryEntity {
        identity: entity_identity(entity.entity_id),
        payload: serde_json::json!({
            "topology": {
                "kind": kind.kind_name(),
                "structure": payload
                    .and_then(|value| value.get("topology"))
                    .and_then(|value| value.get("structure"))
                    .cloned()
                    .unwrap_or_else(|| Value::String(kind.kind_name().to_string()))
            },
            "lineage": { "provenance": entity.entity_id },
            "naming": {
                "persistent_name": persistent_names
                    .get(&entity.entity_id)
                    .cloned()
                    .map(Value::String)
                    .unwrap_or(Value::Null)
            }
        }),
    })
}

fn persistent_name_row(
    entity: &EntityReadRecord,
    targets: &BTreeMap<EntityId, String>,
) -> Option<ForgeQueryEntity> {
    let kind = WorthEntityKind::from_kind_id(entity.kind.kind_id)?;
    if kind != WorthEntityKind::Naming(WorthNamingEntityKind::PersistentName) {
        return None;
    }
    let payload = entity.payload.as_json();
    let mut row = serde_json::json!({
        "topology": { "kind": kind.kind_name() },
        "lineage": { "provenance": entity.entity_id },
        "naming": {
            "persistent_name": payload
                .and_then(|value| value.get("naming"))
                .and_then(|value| value.get("persistent_name"))
                .cloned()
                .unwrap_or(Value::Null)
        }
    });
    if let Some(target_identity) = targets.get(&entity.entity_id) {
        row["naming"]["target_identity"] = Value::String(target_identity.clone());
    }
    Some(ForgeQueryEntity {
        identity: entity_identity(entity.entity_id),
        payload: row,
    })
}

fn relation_row(
    relation: &RelationReadRecord,
    identities: &BTreeMap<EntityId, String>,
) -> Option<ForgeQueryEntity> {
    let kind = WorthRelationKind::from_kind_id(relation.kind.kind_id)?;
    let WorthRelationKind::Topology(_) = kind else {
        return None;
    };
    let source_identity = identities.get(&relation.source)?.clone();
    let target_identity = identities.get(&relation.target)?.clone();
    let mut payload = serde_json::json!({
        "topology": {
            "kind": kind.kind_name(),
            "source_identity": source_identity,
            "target_identity": target_identity
        },
        "lineage": { "provenance": relation.relation_id }
    });
    if let Some(path) = topology_relation_dependency_path(kind) {
        let (section, field) = path.split_once('.').expect("topology dependency path");
        payload[section][field] = Value::String(kind.kind_name().to_string());
    }
    Some(ForgeQueryEntity {
        identity: format!(
            "relation:{}:{}:{}",
            relation.relation_id.partition_id.0,
            relation.relation_id.local_slot.0,
            relation.relation_id.generation.0
        ),
        payload,
    })
}

fn entity_identity(entity: EntityId) -> String {
    format!(
        "entity:{}:{}:{}",
        entity.partition_id.0, entity.local_slot.0, entity.generation.0
    )
}
