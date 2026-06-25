use std::collections::BTreeMap;
use std::sync::{Arc, RwLock};

use forge_foundational::facade::{AspectValue, InternedString};
use forge_query::facade::{
    ForgeQueryAdmittedAspectValue, ForgeQueryAspectTouch, ForgeQueryEntityIdentity,
    ForgeQueryMutationDelta, ForgeQueryMutationKind, ForgeQueryWorkspaceError,
};
use forge_relational::facade::identity::{EntityId, PartitionId, RelationId};
use forge_relational::facade::publication::{
    PublishedAuthoritativeRecordPatch, RecordStructuralChange,
};
use forge_relational::facade::runtime::RelationalRuntime;
use forge_relational::facade::transactions::{CommitResult, RecordRef};
use forge_runtime_bridge::facade::{
    RelationalBridgeRecordIdentityKind, RelationalBridgeRecordIdentityParts,
};
use schema::facade::platform::entities::{EntityKind, NamingEntityKind};
use schema::facade::platform::relations::RelationKind;

use crate::relational_aspect_boundary::entity_record_domain_label;

pub(super) fn mutation_deltas_from_commit(
    runtime: &Arc<RwLock<RelationalRuntime>>,
    commit: &CommitResult,
    declared_aspect_touches: &[ForgeQueryAspectTouch],
    declared_target_collection: Option<&str>,
) -> Result<Vec<ForgeQueryMutationDelta>, ForgeQueryWorkspaceError> {
    mutation_deltas_from_patch_records(
        runtime,
        commit.envelope().commit.version_id,
        commit.patch(),
        declared_aspect_touches,
        declared_target_collection,
    )
}

pub(super) fn mutation_deltas_from_patch_records(
    runtime: &Arc<RwLock<RelationalRuntime>>,
    version_id: forge_relational::facade::identity::VersionId,
    patch_records: &[PublishedAuthoritativeRecordPatch],
    declared_aspect_touches: &[ForgeQueryAspectTouch],
    declared_target_collection: Option<&str>,
) -> Result<Vec<ForgeQueryMutationDelta>, ForgeQueryWorkspaceError> {
    let runtime = runtime
        .read()
        .expect("topology runtime write authority lock poisoned");
    let mut deltas = Vec::new();
    for record in patch_records {
        let Some(collection) = target_collection_for_patch(&runtime, version_id, &record.target)
            .or_else(|| declared_target_collection.map(ToString::to_string))
        else {
            continue;
        };
        deltas.push(ForgeQueryMutationDelta::from_touched_aspects(
            collection,
            record_identity(&record.target),
            mutation_kind(record.structural_change),
            declared_aspect_touches.to_vec(),
        ));
    }
    if deltas.is_empty() {
        return Err(ForgeQueryWorkspaceError::new(
            "topology production runtime write produced no observable query deltas",
        ));
    }
    Ok(deltas)
}

pub(super) fn aspect_map(
    aspects: &[ForgeQueryAdmittedAspectValue],
) -> Result<BTreeMap<String, AspectValue>, ForgeQueryWorkspaceError> {
    aspects
        .iter()
        .map(|aspect| {
            let value = aspect.foundational_value().cloned().ok_or_else(|| {
                ForgeQueryWorkspaceError::new(format!(
                    "topology production runtime requires set value for admitted aspect `{}`",
                    aspect_touch_key(&aspect.aspect_touch())
                ))
            })?;
            Ok((aspect_touch_key(&aspect.aspect_touch()), value))
        })
        .collect()
}

pub(super) fn required_text(
    aspects: &BTreeMap<String, AspectValue>,
    key: &str,
) -> Result<String, ForgeQueryWorkspaceError> {
    aspects
        .get(key)
        .and_then(aspect_string_value)
        .ok_or_else(|| {
            ForgeQueryWorkspaceError::new(format!(
                "topology production runtime requires string aspect `{key}`"
            ))
        })
}

pub(super) fn optional_text(aspects: &BTreeMap<String, AspectValue>, key: &str) -> Option<String> {
    aspects.get(key).and_then(aspect_string_value)
}

fn aspect_string_value(value: &AspectValue) -> Option<String> {
    match value {
        AspectValue::String(InternedString::Raw(value)) => Some(value.clone()),
        AspectValue::String(InternedString::Symbol(symbol)) => Some(format!("symbol:{}", symbol.0)),
        _ => None,
    }
}

pub(super) fn aspect_touch_key(touch: &ForgeQueryAspectTouch) -> String {
    match touch.native_field_path() {
        Some(path) => std::iter::once(touch.native_aspect_key().as_str())
            .chain(path.fields().iter().map(|field| field.as_str()))
            .collect::<Vec<_>>()
            .join("."),
        None => touch.native_aspect_key().as_str().to_string(),
    }
}

pub(super) fn entity_id_from_query_identity(
    identity: &ForgeQueryEntityIdentity,
) -> Result<EntityId, ForgeQueryWorkspaceError> {
    let Some(parts) = identity.relational_record_parts() else {
        return Err(ForgeQueryWorkspaceError::new(
            "topology production runtime requires a typed relational entity identity",
        ));
    };
    if parts.kind() != RelationalBridgeRecordIdentityKind::Entity {
        return Err(ForgeQueryWorkspaceError::new(
            "topology production runtime expected entity identity, got relation identity",
        ));
    }
    Ok(EntityId::new(
        PartitionId(parts.partition_id()),
        parts.local_slot(),
        parts.generation(),
    ))
}

pub(super) fn relation_id_from_query_identity(
    identity: &ForgeQueryEntityIdentity,
) -> Result<RelationId, ForgeQueryWorkspaceError> {
    let Some(parts) = identity.relational_record_parts() else {
        return Err(ForgeQueryWorkspaceError::new(
            "topology production runtime requires a typed relational relation identity",
        ));
    };
    if parts.kind() != RelationalBridgeRecordIdentityKind::Relation {
        return Err(ForgeQueryWorkspaceError::new(
            "topology production runtime expected relation identity, got entity identity",
        ));
    }
    Ok(RelationId::new(
        PartitionId(parts.partition_id()),
        parts.local_slot(),
        parts.generation(),
    ))
}

pub(super) fn parse_entity_identity(identity: &str) -> Result<EntityId, ForgeQueryWorkspaceError> {
    let mut parts = identity.split(':');
    let kind = parts.next().unwrap_or_default();
    if kind != "entity" {
        return Err(ForgeQueryWorkspaceError::new(format!(
            "topology production runtime expected entity identity, got `{identity}`"
        )));
    }
    let partition_id = parts
        .next()
        .ok_or_else(|| ForgeQueryWorkspaceError::new("missing entity partition id"))?
        .parse::<u32>()
        .map_err(|_| ForgeQueryWorkspaceError::new("invalid entity partition id"))?;
    let local_slot = parts
        .next()
        .ok_or_else(|| ForgeQueryWorkspaceError::new("missing entity local slot"))?
        .parse::<u64>()
        .map_err(|_| ForgeQueryWorkspaceError::new("invalid entity local slot"))?;
    let generation = parts
        .next()
        .ok_or_else(|| ForgeQueryWorkspaceError::new("missing entity generation"))?
        .parse::<u32>()
        .map_err(|_| ForgeQueryWorkspaceError::new("invalid entity generation"))?;
    if parts.next().is_some() {
        return Err(ForgeQueryWorkspaceError::new(format!(
            "entity identity `{identity}` had too many fields"
        )));
    }
    Ok(EntityId::new(
        PartitionId(partition_id),
        local_slot,
        generation,
    ))
}

pub(super) fn ensure_live_entity_exists(
    runtime: &Arc<RwLock<RelationalRuntime>>,
    entity_id: EntityId,
    label: &str,
) -> Result<(), ForgeQueryWorkspaceError> {
    let runtime = runtime
        .read()
        .expect("topology runtime write authority lock poisoned");
    let version_id = runtime
        .publication()
        .latest_bundle()
        .map(|bundle| bundle.commit.version_id)
        .ok_or_else(|| {
            ForgeQueryWorkspaceError::new(format!(
                "topology production runtime requires current-head truth before `{label}` relation endpoint resolution"
            ))
        })?;
    if runtime
        .read_truth()
        .read_version(version_id)
        .get_entity(entity_id)
        .is_none()
    {
        return Err(ForgeQueryWorkspaceError::new(format!(
            "topology production runtime relation `{label}` endpoint `{entity_id:?}` does not exist in current-head truth"
        )));
    }
    Ok(())
}

pub(super) fn live_entity_label_exists(
    runtime: &Arc<RwLock<RelationalRuntime>>,
    label: &str,
) -> bool {
    let runtime = runtime
        .read()
        .expect("topology runtime write authority lock poisoned");
    let Some(version_id) = runtime
        .publication()
        .latest_bundle()
        .map(|bundle| bundle.commit.version_id)
    else {
        return false;
    };
    let read_view = runtime.read_truth().read_version(version_id);
    EntityKind::ALL.into_iter().any(|kind| {
        read_view
            .entities()
            .iter()
            .filter(|record| record.kind.kind_id == kind.kind_id())
            .any(|record| {
                entity_record_domain_label(&record).is_some_and(|existing| existing == label)
            })
    })
}

pub(super) fn target_collection_for_patch(
    runtime: &RelationalRuntime,
    version_id: forge_relational::facade::identity::VersionId,
    target: &RecordRef,
) -> Option<String> {
    let read_view = runtime.read_truth().read_version(version_id);
    match target {
        RecordRef::Entity(entity_id) => {
            let record = read_view.get_entity(*entity_id)?;
            let kind = EntityKind::from_kind_id(record.kind.kind_id)?;
            if kind.is_topological() {
                Some("TopologyEntity".to_string())
            } else if kind == EntityKind::Naming(NamingEntityKind::PersistentName) {
                Some("PersistentName".to_string())
            } else {
                None
            }
        }
        RecordRef::Relation(relation_id) => {
            let record = read_view.get_relation(*relation_id)?;
            match RelationKind::from_kind_id(record.kind.kind_id)? {
                RelationKind::Topology(_) => Some("TopologyRelation".to_string()),
                _ => None,
            }
        }
    }
}

fn mutation_kind(change: RecordStructuralChange) -> ForgeQueryMutationKind {
    match change {
        RecordStructuralChange::Created => ForgeQueryMutationKind::Created,
        RecordStructuralChange::Updated => ForgeQueryMutationKind::Updated,
        RecordStructuralChange::Deleted | RecordStructuralChange::RetainedForAudit => {
            ForgeQueryMutationKind::Deleted
        }
        _ => ForgeQueryMutationKind::Updated,
    }
}

pub(super) fn record_identity(target: &RecordRef) -> ForgeQueryEntityIdentity {
    match target {
        RecordRef::Entity(entity) => entity_identity(*entity),
        RecordRef::Relation(relation) => relation_identity(*relation),
    }
}

pub(super) fn entity_identity(entity: EntityId) -> ForgeQueryEntityIdentity {
    ForgeQueryEntityIdentity::from_relational_record(RelationalBridgeRecordIdentityParts::entity(
        entity.partition_id.0,
        entity.local_slot.0,
        entity.generation.0,
    ))
}

pub(super) fn relation_identity(relation: RelationId) -> ForgeQueryEntityIdentity {
    ForgeQueryEntityIdentity::from_relational_record(RelationalBridgeRecordIdentityParts::relation(
        relation.partition_id.0,
        relation.local_slot.0,
        relation.generation.0,
    ))
}

pub(super) fn entity_identity_label(entity: EntityId) -> String {
    format!(
        "entity:{}:{}:{}",
        entity.partition_id.0, entity.local_slot.0, entity.generation.0
    )
}

pub(super) fn relation_identity_label(relation: RelationId) -> String {
    format!(
        "relation:{}:{}:{}",
        relation.partition_id.0, relation.local_slot.0, relation.generation.0
    )
}
