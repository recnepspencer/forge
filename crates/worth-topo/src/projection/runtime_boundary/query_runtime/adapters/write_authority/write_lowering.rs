use std::collections::BTreeMap;
use std::sync::{Arc, RwLock};

use forge_foundational::facade::AspectValue;
use forge_query::facade::{
    ForgeQueryEntityIdentity, ForgeQuerySymbolicAspectReference, ForgeQueryWorkspaceError,
};
use forge_relational::facade::identity::PartitionId;
use forge_relational::facade::runtime::RelationalRuntime;
use forge_relational::facade::symbols::ClientKey;
use forge_relational::facade::transactions::{
    CreateIntent, EntityReference, EntitySpec, MutationIntent, RelationSpec,
    UpdateRelationEndpointsIntent,
};
use schema::facade::platform::entities::{EntityKind, NamingEntityKind};
use schema::facade::platform::relations::{NamingRelationKind, RelationKind};

use super::super::write_support::{
    aspect_touch_key, ensure_live_entity_exists, live_entity_label_exists, optional_text,
    parse_entity_identity, relation_id_from_query_identity, required_text,
};
use crate::relational_aspect_boundary::{
    persistent_name_create_fields, topology_entity_create_fields,
};

pub(super) fn lower_topology_entity_insert(
    runtime: &Arc<RwLock<RelationalRuntime>>,
    aspects: &BTreeMap<String, AspectValue>,
) -> Result<(Vec<MutationIntent>, EntityReference), ForgeQueryWorkspaceError> {
    let kind_name = required_text(aspects, "topology.kind")?;
    let kind = EntityKind::ALL
        .into_iter()
        .find(|kind| kind.is_topological() && kind.kind_name() == kind_name)
        .ok_or_else(|| {
            ForgeQueryWorkspaceError::new(format!(
                "topology production runtime does not admit non-topology entity kind `{kind_name}`"
            ))
        })?;
    let structure = required_text(aspects, "topology.structure")?;
    if live_entity_label_exists(runtime, &structure) {
        return Err(ForgeQueryWorkspaceError::new(format!(
            "topology production runtime rejected duplicate live entity label `{structure}`"
        )));
    }
    let persistent_name =
        optional_text(aspects, "naming.persistent_name").unwrap_or(structure.clone());
    let topology_ref = forge_relational::facade::transactions::CreatedEntityRef {
        partition_id: PartitionId::main(),
        kind_id: kind.kind_id(),
        client_key: ClientKey::raw(structure.clone()),
    };
    let naming_key = format!("{persistent_name}.persistent_name");
    let naming_ref = forge_relational::facade::transactions::CreatedEntityRef {
        partition_id: PartitionId::main(),
        kind_id: EntityKind::Naming(NamingEntityKind::PersistentName).kind_id(),
        client_key: ClientKey::raw(naming_key.clone()),
    };
    Ok((
        vec![
            MutationIntent::Create(CreateIntent::Entity(EntitySpec {
                partition_id: PartitionId::main(),
                kind_id: kind.kind_id(),
                client_key: ClientKey::raw(structure.clone()),
                fields: topology_entity_create_fields(kind, &structure),
            })),
            MutationIntent::Create(CreateIntent::Entity(EntitySpec {
                partition_id: PartitionId::main(),
                kind_id: EntityKind::Naming(NamingEntityKind::PersistentName).kind_id(),
                client_key: ClientKey::raw(naming_key),
                fields: persistent_name_create_fields(&persistent_name),
            })),
            MutationIntent::Create(CreateIntent::Relation(RelationSpec {
                partition_id: PartitionId::main(),
                kind_id: RelationKind::Naming(NamingRelationKind::PersistentNameTargetsEntity)
                    .kind_id(),
                client_key: ClientKey::raw(format!("{persistent_name}.targets")),
                source: EntityReference::Created(naming_ref),
                target: EntityReference::Created(topology_ref.clone()),
                fields: Default::default(),
            })),
        ],
        EntityReference::Created(topology_ref),
    ))
}

pub(super) fn lower_topology_relation_insert(
    runtime: &Arc<RwLock<RelationalRuntime>>,
    aspects: &BTreeMap<String, AspectValue>,
    symbolic_aspect_references: &[ForgeQuerySymbolicAspectReference],
    created_entities: &BTreeMap<String, EntityReference>,
) -> Result<RelationSpec, ForgeQueryWorkspaceError> {
    let kind_name = required_text(aspects, "topology.kind")?;
    let kind = RelationKind::ALL
        .into_iter()
        .find(|kind| matches!(kind, RelationKind::Topology(_)) && kind.kind_name() == kind_name)
        .ok_or_else(|| {
            ForgeQueryWorkspaceError::new(format!(
                "topology production runtime does not admit non-topology relation kind `{kind_name}`"
            ))
        })?;
    let source = lower_relation_endpoint(
        runtime,
        aspects,
        symbolic_aspect_references,
        created_entities,
        "topology.source_identity",
        "source",
    )?;
    let target = lower_relation_endpoint(
        runtime,
        aspects,
        symbolic_aspect_references,
        created_entities,
        "topology.target_identity",
        "target",
    )?;
    let source_identity = relation_endpoint_identity(&source)?;
    let target_identity = relation_endpoint_identity(&target)?;
    let client_key = format!(
        "{}:{}:{}",
        kind.kind_name(),
        source_identity,
        target_identity
    );
    Ok(RelationSpec {
        partition_id: PartitionId::main(),
        kind_id: kind.kind_id(),
        client_key: ClientKey::raw(client_key),
        source,
        target,
        fields: Default::default(),
    })
}

pub(super) fn lower_topology_relation_update(
    runtime: &Arc<RwLock<RelationalRuntime>>,
    aspects: &BTreeMap<String, AspectValue>,
    symbolic_aspect_references: &[ForgeQuerySymbolicAspectReference],
    created_entities: &BTreeMap<String, EntityReference>,
    resolved_target_identity: &ForgeQueryEntityIdentity,
) -> Result<UpdateRelationEndpointsIntent, ForgeQueryWorkspaceError> {
    let kind_name = required_text(aspects, "topology.kind")?;
    let kind = RelationKind::ALL
        .into_iter()
        .find(|kind| matches!(kind, RelationKind::Topology(_)) && kind.kind_name() == kind_name)
        .ok_or_else(|| {
            ForgeQueryWorkspaceError::new(format!(
                "topology production runtime does not admit non-topology relation kind `{kind_name}`"
            ))
        })?;
    let source = lower_relation_endpoint(
        runtime,
        aspects,
        symbolic_aspect_references,
        created_entities,
        "topology.source_identity",
        "source",
    )?;
    let target = lower_relation_endpoint(
        runtime,
        aspects,
        symbolic_aspect_references,
        created_entities,
        "topology.target_identity",
        "target",
    )?;
    Ok(UpdateRelationEndpointsIntent {
        relation_id: relation_id_from_query_identity(resolved_target_identity)?,
        kind_id: kind.kind_id(),
        source,
        target,
    })
}

fn lower_relation_endpoint(
    runtime: &Arc<RwLock<RelationalRuntime>>,
    aspects: &BTreeMap<String, AspectValue>,
    symbolic_aspect_references: &[ForgeQuerySymbolicAspectReference],
    created_entities: &BTreeMap<String, EntityReference>,
    aspect_path: &str,
    label: &str,
) -> Result<EntityReference, ForgeQueryWorkspaceError> {
    if let Some(reference) = symbolic_aspect_references
        .iter()
        .find(|reference| aspect_touch_key(reference.aspect_touch()) == aspect_path)
    {
        return created_entities
            .get(reference.reference().symbol())
            .cloned()
            .ok_or_else(|| {
                ForgeQueryWorkspaceError::new(format!(
                    "topology production runtime could not resolve same-mutation-set created entity `{}` for relation `{label}` endpoint",
                    reference.reference().symbol()
                ))
            });
    }

    let identity = required_text(aspects, aspect_path)?;
    let entity_id = parse_entity_identity(&identity)?;
    ensure_live_entity_exists(runtime, entity_id, label)?;
    Ok(EntityReference::Existing(entity_id))
}

pub(super) fn relation_endpoint_identity(
    endpoint: &EntityReference,
) -> Result<String, ForgeQueryWorkspaceError> {
    match endpoint {
        EntityReference::Existing(entity_id) => Ok(format!(
            "entity:{}:{}:{}",
            entity_id.partition_id.0, entity_id.local_slot.0, entity_id.generation.0
        )),
        EntityReference::Created(created) => {
            Ok(format!("created:{}", created.client_key.canonical_text()))
        }
    }
}

pub(super) fn kind_name_for_relation_kind_id(
    kind_id: forge_relational::facade::identity::KindId,
) -> &'static str {
    RelationKind::from_kind_id(kind_id)
        .expect("topology runtime only lowers admitted relation kinds")
        .kind_name()
}
