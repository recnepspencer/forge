use std::collections::BTreeMap;

use forge_query::facade::{
    forge_query_materialized_relation_field_key, ForgeQueryEntity, RelationName,
};
use forge_relational::facade::identity::EntityId;
use forge_relational::facade::runtime::{EntityReadRecord, RelationReadRecord};
use schema::facade::platform::aspects::{Aspect, NamingAspect};
use schema::facade::platform::entities::{EntityKind, NamingEntityKind};
use schema::facade::platform::relations::{NamingRelationKind, RelationKind};

use crate::query_native_runtime_boundary::{
    native_entity_row, native_field_path, native_i64, native_null, native_string,
};
use crate::relational_aspect_boundary::{entity_record_domain_label, entity_record_string_aspect};
use crate::topology_operators::topology_relation_dependency_path;

use super::binding::TopologyRuntimeBinding;
use super::write_support::{
    entity_identity, entity_identity_label, relation_identity, relation_identity_label,
};

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
                .map(|_| (entity.entity_id, entity_identity_label(entity.entity_id)))
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
                let name = entity_record_string_aspect(
                    &entity,
                    &Aspect::Naming(NamingAspect::PersistentName),
                    "persistent_name",
                )?;
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
    let structure =
        entity_record_domain_label(entity).unwrap_or_else(|| kind.kind_name().to_string());
    let mut fields = vec![
        row_field(
            ["identity", "id"],
            native_string(entity_identity_label(entity.entity_id)),
        )?,
        row_field(["topology", "kind"], native_string(kind.kind_name()))?,
        row_field(["topology", "structure"], native_string(structure))?,
        row_field(
            ["lineage", "provenance_partition"],
            native_i64(i64::from(entity.entity_id.partition_value())),
        )?,
        row_field(
            ["lineage", "provenance_local_slot"],
            native_i64(entity.entity_id.local_slot_value() as i64),
        )?,
        row_field(
            ["lineage", "provenance_generation"],
            native_i64(i64::from(entity.entity_id.generation_value())),
        )?,
        row_field(
            ["naming", "persistent_name"],
            persistent_names
                .get(&entity.entity_id)
                .cloned()
                .map(native_string)
                .unwrap_or_else(native_null),
        )?,
    ];
    if let Some(relation_map) = relations.get(&entity.entity_id) {
        fields.extend(relation_map.iter().filter_map(|(relation_kind, target)| {
            row_field(
                ["relations", relation_kind.as_str()],
                native_string(target.clone()),
            )
        }));
    }
    if let Some(relation_identity_map) = relation_identities.get(&entity.entity_id) {
        fields.extend(
            relation_identity_map
                .iter()
                .filter_map(|(relation_kind, identity)| {
                    row_field(
                        ["relation_identities", relation_kind.as_str()],
                        native_string(identity.clone()),
                    )
                }),
        );
    }
    Some(native_entity_row(entity_identity(entity.entity_id), fields))
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
        relations.entry(relation.source).or_default().insert(
            topology_relation_slot(topology_kind.kind_name()),
            target_identity,
        );
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
            topology_relation_slot(topology_kind.kind_name()),
            relation_identity_label(relation.relation_id),
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
    let mut fields = vec![
        row_field(["topology", "kind"], native_string(kind.kind_name()))?,
        row_field(
            ["lineage", "provenance_partition"],
            native_i64(i64::from(entity.entity_id.partition_value())),
        )?,
        row_field(
            ["lineage", "provenance_local_slot"],
            native_i64(entity.entity_id.local_slot_value() as i64),
        )?,
        row_field(
            ["lineage", "provenance_generation"],
            native_i64(i64::from(entity.entity_id.generation_value())),
        )?,
        row_field(
            ["naming", "persistent_name"],
            entity_record_string_aspect(
                entity,
                &Aspect::Naming(NamingAspect::PersistentName),
                "persistent_name",
            )
            .map(native_string)
            .unwrap_or_else(native_null),
        )?,
    ];
    if let Some(target_identity) = targets.get(&entity.entity_id) {
        fields.push(row_field(
            ["naming", "target_identity"],
            native_string(target_identity.clone()),
        )?);
    }
    Some(native_entity_row(entity_identity(entity.entity_id), fields))
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
    let mut fields = vec![
        row_field(
            ["identity", "id"],
            native_string(relation_identity_label(relation.relation_id)),
        )?,
        row_field(["topology", "kind"], native_string(kind.kind_name()))?,
        row_field(
            ["topology", "source_identity"],
            native_string(source_identity),
        )?,
        row_field(
            ["topology", "target_identity"],
            native_string(target_identity),
        )?,
        row_field(
            ["lineage", "provenance_partition"],
            native_i64(i64::from(relation.relation_id.partition_value())),
        )?,
        row_field(
            ["lineage", "provenance_local_slot"],
            native_i64(relation.relation_id.local_slot_value() as i64),
        )?,
        row_field(
            ["lineage", "provenance_generation"],
            native_i64(i64::from(relation.relation_id.generation_value())),
        )?,
    ];
    if let Some(path) = topology_relation_dependency_path(kind) {
        fields.push(row_field(
            [path.section(), path.field()],
            native_string(kind.kind_name()),
        )?);
    }
    Some(native_entity_row(
        relation_identity(relation.relation_id),
        fields,
    ))
}

fn row_field(
    path: impl IntoIterator<Item = impl Into<String>>,
    value: forge_foundational::facade::AspectValue,
) -> Option<(
    forge_foundational::facade::CanonicalFieldPath,
    forge_foundational::facade::AspectValue,
)> {
    Some((native_field_path(path).ok()?, value))
}

fn topology_relation_slot(kind_name: &str) -> String {
    let relation =
        RelationName::new(kind_name).expect("schema topology relation kind names must admit");
    forge_query_materialized_relation_field_key(&relation)
        .as_str()
        .to_string()
}
