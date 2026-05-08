use std::collections::BTreeMap;

use forge_query::facade::ForgeQueryEntity;
use forge_relational::facade::identity::EntityId;
use forge_relational::facade::runtime::{EntityReadRecord, RelationReadRecord};
use schema::facade::{EntityKind, NamingEntityKind, NamingRelationKind, RelationKind};
use serde_json::Value;

use crate::query::materialized::topology_relation_dependency_path;

use super::binding::TopologyRuntimeBinding;

pub(super) fn topology_entity_rows(binding: &TopologyRuntimeBinding) -> Vec<ForgeQueryEntity> {
    let names = topology_entity_persistent_name_map(binding);
    let relations = topology_entity_relation_map(binding);
    let relation_identities = topology_entity_relation_identity_map(binding);
    binding
        .entity_records()
        .into_iter()
        .filter_map(|entity| entity_row(&entity, &names, &relations, &relation_identities))
        .collect()
}

pub(super) fn topology_relation_rows(binding: &TopologyRuntimeBinding) -> Vec<ForgeQueryEntity> {
    let identities = entity_identity_map(binding);
    binding
        .relation_records()
        .into_iter()
        .filter_map(|relation| relation_row(&relation, &identities))
        .collect()
}

pub(super) fn persistent_name_rows(binding: &TopologyRuntimeBinding) -> Vec<ForgeQueryEntity> {
    let targets = persistent_name_target_map(binding);
    binding
        .entity_records()
        .into_iter()
        .filter_map(|entity| persistent_name_row(&entity, &targets))
        .collect()
}

fn entity_identity_map(binding: &TopologyRuntimeBinding) -> BTreeMap<EntityId, String> {
    binding
        .entity_records()
        .into_iter()
        .filter_map(|entity| {
            EntityKind::from_kind_id(entity.kind.kind_id)
                .filter(|kind| kind.is_topological())
                .map(|_| (entity.entity_id, entity_identity(entity.entity_id)))
        })
        .collect()
}

fn persistent_name_target_map(binding: &TopologyRuntimeBinding) -> BTreeMap<EntityId, String> {
    let identities = entity_identity_map(binding);
    binding
        .relation_records()
        .into_iter()
        .filter_map(
            |relation| match RelationKind::from_kind_id(relation.kind.kind_id) {
                Some(RelationKind::Naming(NamingRelationKind::PersistentNameTargetsEntity)) => {
                    identities
                        .get(&relation.target)
                        .cloned()
                        .map(|target| (relation.source, target))
                }
                _ => None,
            },
        )
        .collect()
}

fn topology_entity_persistent_name_map(
    binding: &TopologyRuntimeBinding,
) -> BTreeMap<EntityId, String> {
    let name_entities = binding
        .entity_records()
        .into_iter()
        .filter_map(|entity| {
            (EntityKind::from_kind_id(entity.kind.kind_id)
                == Some(EntityKind::Naming(NamingEntityKind::PersistentName)))
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
            |relation| match RelationKind::from_kind_id(relation.kind.kind_id) {
                Some(RelationKind::Naming(NamingRelationKind::PersistentNameTargetsEntity)) => {
                    name_entities
                        .get(&relation.source)
                        .cloned()
                        .map(|name| (relation.target, name))
                }
                _ => None,
            },
        )
        .collect()
}

fn entity_row(
    entity: &EntityReadRecord,
    persistent_names: &BTreeMap<EntityId, String>,
    relations: &BTreeMap<EntityId, BTreeMap<String, String>>,
    relation_identities: &BTreeMap<EntityId, BTreeMap<String, String>>,
) -> Option<ForgeQueryEntity> {
    let kind = EntityKind::from_kind_id(entity.kind.kind_id)?;
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
            },
            "relations": relations
                .get(&entity.entity_id)
                .cloned()
                .map(serde_json::to_value)
                .transpose()
                .ok()
                .flatten()
                .unwrap_or_else(|| serde_json::json!({})),
            "relation_identities": relation_identities
                .get(&entity.entity_id)
                .cloned()
                .map(serde_json::to_value)
                .transpose()
                .ok()
                .flatten()
                .unwrap_or_else(|| serde_json::json!({}))
        }),
    })
}

fn topology_entity_relation_map(
    binding: &TopologyRuntimeBinding,
) -> BTreeMap<EntityId, BTreeMap<String, String>> {
    let identities = entity_identity_map(binding);
    let mut relations = BTreeMap::<EntityId, BTreeMap<String, String>>::new();
    for relation in binding.relation_records() {
        let Some(kind) = RelationKind::from_kind_id(relation.kind.kind_id) else {
            continue;
        };
        let RelationKind::Topology(topology_kind) = kind else {
            continue;
        };
        let Some(target_identity) = identities.get(&relation.target).cloned() else {
            continue;
        };
        relations
            .entry(relation.source)
            .or_default()
            .insert(topology_kind.kind_name().to_string(), target_identity);
    }
    relations
}

fn topology_entity_relation_identity_map(
    binding: &TopologyRuntimeBinding,
) -> BTreeMap<EntityId, BTreeMap<String, String>> {
    let mut relations = BTreeMap::<EntityId, BTreeMap<String, String>>::new();
    for relation in binding.relation_records() {
        let Some(kind) = RelationKind::from_kind_id(relation.kind.kind_id) else {
            continue;
        };
        let RelationKind::Topology(topology_kind) = kind else {
            continue;
        };
        relations.entry(relation.source).or_default().insert(
            topology_kind.kind_name().to_string(),
            relation_identity(relation.relation_id),
        );
    }
    relations
}

fn persistent_name_row(
    entity: &EntityReadRecord,
    targets: &BTreeMap<EntityId, String>,
) -> Option<ForgeQueryEntity> {
    let kind = EntityKind::from_kind_id(entity.kind.kind_id)?;
    if kind != EntityKind::Naming(NamingEntityKind::PersistentName) {
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
    let kind = RelationKind::from_kind_id(relation.kind.kind_id)?;
    let RelationKind::Topology(_) = kind else {
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
        identity: relation_identity(relation.relation_id),
        payload,
    })
}

fn entity_identity(entity: EntityId) -> String {
    format!(
        "entity:{}:{}:{}",
        entity.partition_id.0, entity.local_slot.0, entity.generation.0
    )
}

fn relation_identity(relation: forge_relational::facade::identity::RelationId) -> String {
    format!(
        "relation:{}:{}:{}",
        relation.partition_id.0, relation.local_slot.0, relation.generation.0
    )
}
