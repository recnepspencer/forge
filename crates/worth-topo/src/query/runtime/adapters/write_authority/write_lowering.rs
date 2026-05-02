use std::collections::BTreeMap;
use std::sync::{Arc, RwLock};

use forge_query::facade::{
    ForgeQuerySymbolicAspectReference, ForgeQueryWorkspaceError, ForgeQueryWriteCommand,
};
use forge_relational::facade::identity::PartitionId;
use forge_relational::facade::payloads::RecordPayload;
use forge_relational::facade::runtime::RelationalRuntime;
use forge_relational::facade::symbols::InternedString;
use forge_relational::facade::transactions::{
    CreateIntent, DeleteEntityIntent, DeleteRelationIntent, EntityMutationIntent, EntityReference,
    EntitySpec, MutationIntent, RelationMutationIntent, RelationSpec,
    UpdateRelationEndpointsIntent,
};
use serde_json::{json, Value};
use worth_schema::facade::{
    WorthEntityKind, WorthNamingEntityKind, WorthNamingRelationKind, WorthRelationKind,
};

use super::super::write_support::{
    aspect_map, ensure_live_entity_exists, live_entity_label_exists, optional_text,
    parse_entity_identity, parse_relation_identity, required_text, write_command_label,
};
use super::patch_matching::LoweredPatchMatch;

pub(super) struct LoweredWriteCommand {
    pub(super) batch_label: String,
    pub(super) intents: Vec<MutationIntent>,
    pub(super) declared_aspect_paths: Vec<String>,
    pub(super) expected_observable_patch_count: usize,
    pub(super) patch_match: LoweredPatchMatch,
    pub(super) fallback_collection: Option<String>,
}

pub(super) fn lower_write_command(
    runtime: &Arc<RwLock<RelationalRuntime>>,
    created_entities: &mut BTreeMap<String, EntityReference>,
    command: ForgeQueryWriteCommand,
) -> Result<LoweredWriteCommand, ForgeQueryWorkspaceError> {
    match command {
        ForgeQueryWriteCommand::InsertAspects {
            collection,
            aspects,
            symbolic_aspect_references,
            symbolic_target_reference,
            ..
        } => {
            let aspect_map = aspect_map(&aspects);
            let declared_aspect_paths = aspects
                .iter()
                .map(|aspect| aspect.aspect_path().to_string())
                .chain(
                    symbolic_aspect_references
                        .iter()
                        .map(|aspect| aspect.aspect_path().to_string()),
                )
                .collect::<Vec<_>>();
            match collection.as_str() {
                "WorthTopologyEntity" => {
                    let (intents, created_entity_ref) =
                        lower_topology_entity_insert(runtime, &aspect_map)?;
                    if let Some(reference) = symbolic_target_reference {
                        created_entities.insert(reference.symbol().to_string(), created_entity_ref);
                    }
                    Ok(LoweredWriteCommand {
                        batch_label: "worth-query-runtime-batch-insert-entity".to_string(),
                        intents,
                        declared_aspect_paths,
                        expected_observable_patch_count: 2,
                        patch_match: LoweredPatchMatch::TopologyEntityInsert {
                            structure_label: required_text(&aspect_map, "topology.structure")?,
                            persistent_name: optional_text(&aspect_map, "naming.persistent_name")
                                .unwrap_or_else(|| {
                                    required_text(&aspect_map, "topology.structure")
                                        .expect("topology.structure should exist for insert")
                                }),
                        },
                        fallback_collection: None,
                    })
                }
                "WorthTopologyRelation" => {
                    let relation = lower_topology_relation_insert(
                        runtime,
                        &aspect_map,
                        &symbolic_aspect_references,
                        created_entities,
                    )?;
                    Ok(LoweredWriteCommand {
                        batch_label: "worth-query-runtime-batch-insert-relation".to_string(),
                        intents: vec![MutationIntent::Create(CreateIntent::Relation(
                            relation.clone(),
                        ))],
                        declared_aspect_paths,
                        expected_observable_patch_count: 1,
                        patch_match: LoweredPatchMatch::TopologyRelationInsert {
                            kind_name: kind_name_for_relation_kind_id(relation.kind_id)
                                .to_string(),
                            source_identity: relation_endpoint_identity(&relation.source)?,
                            target_identity: relation_endpoint_identity(&relation.target)?,
                        },
                        fallback_collection: None,
                    })
                }
                other => Err(ForgeQueryWorkspaceError::new(format!(
                    "worth topology production runtime does not admit insert collection `{other}`"
                ))),
            }
        }
        ForgeQueryWriteCommand::DeleteExistingAspects {
            binding,
            touched_aspect_paths,
            ..
        } => {
            let collection = binding
                .target_collection()
                .ok_or_else(|| {
                    ForgeQueryWorkspaceError::new(
                        "worth topology production runtime delete requires a declared target collection",
                    )
                })?
                .to_string();
            let intent = match collection.as_str() {
                "WorthTopologyEntity" => MutationIntent::Entity(EntityMutationIntent::Delete(
                    DeleteEntityIntent {
                        entity_id: parse_entity_identity(binding.resolved_target_identity())?,
                    },
                )),
                "WorthTopologyRelation" => MutationIntent::Relation(RelationMutationIntent::Delete(
                    DeleteRelationIntent {
                        relation_id: parse_relation_identity(binding.resolved_target_identity())?,
                    },
                )),
                other => {
                    return Err(ForgeQueryWorkspaceError::new(format!(
                        "worth topology production runtime does not admit delete collection `{other}`"
                    )))
                }
            };
            Ok(LoweredWriteCommand {
                batch_label: "worth-query-runtime-batch-delete".to_string(),
                intents: vec![intent],
                declared_aspect_paths: touched_aspect_paths,
                expected_observable_patch_count: 1,
                patch_match: LoweredPatchMatch::ExistingTargetIdentity {
                    resolved_target_identity: binding.resolved_target_identity().to_string(),
                },
                fallback_collection: Some(collection),
            })
        }
        ForgeQueryWriteCommand::UpdateExistingAspects {
            binding,
            aspects,
            ..
        } => {
            let collection = binding
                .target_collection()
                .ok_or_else(|| {
                    ForgeQueryWorkspaceError::new(
                        "worth topology production runtime update requires a declared target collection",
                    )
                })?
                .to_string();
            let aspect_map = aspect_map(&aspects);
            let declared_aspect_paths = aspects
                .iter()
                .map(|aspect| aspect.aspect_path().to_string())
                .collect::<Vec<_>>();
            let intent = match collection.as_str() {
                "WorthTopologyRelation" => MutationIntent::Relation(
                    RelationMutationIntent::UpdateEndpoints(lower_topology_relation_update(
                        runtime,
                        &aspect_map,
                        &[],
                        created_entities,
                        binding.resolved_target_identity(),
                    )?),
                ),
                other => {
                    return Err(ForgeQueryWorkspaceError::new(format!(
                        "worth topology production runtime does not admit update collection `{other}`"
                    )))
                }
            };
            Ok(LoweredWriteCommand {
                batch_label: "worth-query-runtime-batch-update".to_string(),
                intents: vec![intent],
                declared_aspect_paths,
                expected_observable_patch_count: 1,
                patch_match: LoweredPatchMatch::ExistingTargetIdentity {
                    resolved_target_identity: binding.resolved_target_identity().to_string(),
                },
                fallback_collection: Some(collection),
            })
        }
        ForgeQueryWriteCommand::VerifyThenUpdateExistingAspects {
            binding,
            aspects,
            symbolic_aspect_references,
            ..
        } => {
            let collection = binding
                .target_collection()
                .ok_or_else(|| {
                    ForgeQueryWorkspaceError::new(
                        "worth topology production runtime update requires a declared target collection",
                    )
                })?
                .to_string();
            let aspect_map = aspect_map(&aspects);
            let declared_aspect_paths = aspects
                .iter()
                .map(|aspect| aspect.aspect_path().to_string())
                .chain(
                    symbolic_aspect_references
                        .iter()
                        .map(|reference| reference.aspect_path().to_string()),
                )
                .collect::<Vec<_>>();
            let intent = match collection.as_str() {
                "WorthTopologyRelation" => MutationIntent::Relation(
                    RelationMutationIntent::UpdateEndpoints(lower_topology_relation_update(
                        runtime,
                        &aspect_map,
                        &symbolic_aspect_references,
                        created_entities,
                        binding.resolved_target_identity(),
                    )?),
                ),
                other => {
                    return Err(ForgeQueryWorkspaceError::new(format!(
                        "worth topology production runtime does not admit update collection `{other}`"
                    )))
                }
            };
            Ok(LoweredWriteCommand {
                batch_label: "worth-query-runtime-batch-update".to_string(),
                intents: vec![intent],
                declared_aspect_paths,
                expected_observable_patch_count: 1,
                patch_match: LoweredPatchMatch::ExistingTargetIdentity {
                    resolved_target_identity: binding.resolved_target_identity().to_string(),
                },
                fallback_collection: Some(collection),
            })
        }
        ForgeQueryWriteCommand::VerifyThenDeleteExistingAspects {
            binding,
            touched_aspect_paths,
            ..
        } => {
            let collection = binding
                .target_collection()
                .ok_or_else(|| {
                    ForgeQueryWorkspaceError::new(
                        "worth topology production runtime delete requires a declared target collection",
                    )
                })?
                .to_string();
            let intent = match collection.as_str() {
                "WorthTopologyEntity" => MutationIntent::Entity(EntityMutationIntent::Delete(
                    DeleteEntityIntent {
                        entity_id: parse_entity_identity(binding.resolved_target_identity())?,
                    },
                )),
                "WorthTopologyRelation" => MutationIntent::Relation(RelationMutationIntent::Delete(
                    DeleteRelationIntent {
                        relation_id: parse_relation_identity(binding.resolved_target_identity())?,
                    },
                )),
                other => {
                    return Err(ForgeQueryWorkspaceError::new(format!(
                        "worth topology production runtime does not admit delete collection `{other}`"
                    )))
                }
            };
            Ok(LoweredWriteCommand {
                batch_label: "worth-query-runtime-batch-delete".to_string(),
                intents: vec![intent],
                declared_aspect_paths: touched_aspect_paths,
                expected_observable_patch_count: 1,
                patch_match: LoweredPatchMatch::ExistingTargetIdentity {
                    resolved_target_identity: binding.resolved_target_identity().to_string(),
                },
                fallback_collection: Some(collection),
            })
        }
        other => Err(ForgeQueryWorkspaceError::new(format!(
            "worth topology production runtime current-head slice does not admit `{}` write command yet",
            write_command_label(&other)
        ))),
    }
}

pub(super) fn lower_topology_entity_insert(
    runtime: &Arc<RwLock<RelationalRuntime>>,
    aspects: &BTreeMap<String, Value>,
) -> Result<(Vec<MutationIntent>, EntityReference), ForgeQueryWorkspaceError> {
    let kind_name = required_text(aspects, "topology.kind")?;
    let kind = WorthEntityKind::ALL
        .into_iter()
        .find(|kind| kind.is_topological() && kind.kind_name() == kind_name)
        .ok_or_else(|| {
            ForgeQueryWorkspaceError::new(format!(
                "worth topology production runtime does not admit non-topology entity kind `{kind_name}`"
            ))
        })?;
    let structure = required_text(aspects, "topology.structure")?;
    if live_entity_label_exists(runtime, &structure) {
        return Err(ForgeQueryWorkspaceError::new(format!(
            "worth topology production runtime rejected duplicate live entity label `{structure}`"
        )));
    }
    let persistent_name =
        optional_text(aspects, "naming.persistent_name").unwrap_or(structure.clone());
    let topology_ref = forge_relational::facade::transactions::CreatedEntityRef {
        partition_id: PartitionId::main(),
        kind_id: kind.kind_id(),
        client_key: InternedString::Raw(structure.clone()),
    };
    let naming_key = format!("{persistent_name}.persistent_name");
    let naming_ref = forge_relational::facade::transactions::CreatedEntityRef {
        partition_id: PartitionId::main(),
        kind_id: WorthEntityKind::Naming(WorthNamingEntityKind::PersistentName).kind_id(),
        client_key: InternedString::Raw(naming_key.clone()),
    };
    Ok((
        vec![
            MutationIntent::Create(CreateIntent::Entity(EntitySpec {
                partition_id: PartitionId::main(),
                kind_id: kind.kind_id(),
                client_key: InternedString::Raw(structure.clone()),
                payload: RecordPayload::StructuredJson(json!({
                    "label": structure,
                    "structure": persistent_name,
                    "topology": { "structure": persistent_name }
                })),
            })),
            MutationIntent::Create(CreateIntent::Entity(EntitySpec {
                partition_id: PartitionId::main(),
                kind_id: WorthEntityKind::Naming(WorthNamingEntityKind::PersistentName).kind_id(),
                client_key: InternedString::Raw(naming_key),
                payload: RecordPayload::StructuredJson(json!({
                    "label": persistent_name,
                    "persistent_name": persistent_name,
                    "naming": { "persistent_name": persistent_name }
                })),
            })),
            MutationIntent::Create(CreateIntent::Relation(RelationSpec {
                partition_id: PartitionId::main(),
                kind_id: WorthRelationKind::Naming(
                    WorthNamingRelationKind::PersistentNameTargetsEntity,
                )
                .kind_id(),
                client_key: InternedString::Raw(format!("{persistent_name}.targets")),
                source: EntityReference::Created(naming_ref),
                target: EntityReference::Created(topology_ref.clone()),
                payload: None,
            })),
        ],
        EntityReference::Created(topology_ref),
    ))
}

pub(super) fn lower_topology_relation_insert(
    runtime: &Arc<RwLock<RelationalRuntime>>,
    aspects: &BTreeMap<String, Value>,
    symbolic_aspect_references: &[ForgeQuerySymbolicAspectReference],
    created_entities: &BTreeMap<String, EntityReference>,
) -> Result<RelationSpec, ForgeQueryWorkspaceError> {
    let kind_name = required_text(aspects, "topology.kind")?;
    let kind = WorthRelationKind::ALL
        .into_iter()
        .find(|kind| matches!(kind, WorthRelationKind::Topology(_)) && kind.kind_name() == kind_name)
        .ok_or_else(|| {
            ForgeQueryWorkspaceError::new(format!(
                "worth topology production runtime does not admit non-topology relation kind `{kind_name}`"
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
        client_key: InternedString::Raw(client_key),
        source,
        target,
        payload: None,
    })
}

fn lower_topology_relation_update(
    runtime: &Arc<RwLock<RelationalRuntime>>,
    aspects: &BTreeMap<String, Value>,
    symbolic_aspect_references: &[ForgeQuerySymbolicAspectReference],
    created_entities: &BTreeMap<String, EntityReference>,
    resolved_target_identity: &str,
) -> Result<UpdateRelationEndpointsIntent, ForgeQueryWorkspaceError> {
    let kind_name = required_text(aspects, "topology.kind")?;
    let kind = WorthRelationKind::ALL
        .into_iter()
        .find(|kind| matches!(kind, WorthRelationKind::Topology(_)) && kind.kind_name() == kind_name)
        .ok_or_else(|| {
            ForgeQueryWorkspaceError::new(format!(
                "worth topology production runtime does not admit non-topology relation kind `{kind_name}`"
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
        relation_id: parse_relation_identity(resolved_target_identity)?,
        kind_id: kind.kind_id(),
        source,
        target,
    })
}

fn lower_relation_endpoint(
    runtime: &Arc<RwLock<RelationalRuntime>>,
    aspects: &BTreeMap<String, Value>,
    symbolic_aspect_references: &[ForgeQuerySymbolicAspectReference],
    created_entities: &BTreeMap<String, EntityReference>,
    aspect_path: &str,
    label: &str,
) -> Result<EntityReference, ForgeQueryWorkspaceError> {
    if let Some(reference) = symbolic_aspect_references
        .iter()
        .find(|reference| reference.aspect_path() == aspect_path)
    {
        return created_entities
            .get(reference.reference().symbol())
            .cloned()
            .ok_or_else(|| {
                ForgeQueryWorkspaceError::new(format!(
                    "worth topology production runtime could not resolve same-batch created entity `{}` for relation `{label}` endpoint",
                    reference.reference().symbol()
                ))
            });
    }

    let identity = required_text(aspects, aspect_path)?;
    let entity_id = parse_entity_identity(&identity)?;
    ensure_live_entity_exists(runtime, entity_id, label)?;
    Ok(EntityReference::Existing(entity_id))
}

fn relation_endpoint_identity(
    endpoint: &EntityReference,
) -> Result<String, ForgeQueryWorkspaceError> {
    match endpoint {
        EntityReference::Existing(entity_id) => Ok(format!(
            "entity:{}:{}:{}",
            entity_id.partition_id.0, entity_id.local_slot.0, entity_id.generation.0
        )),
        EntityReference::Created(created) => match &created.client_key {
            InternedString::Raw(label) => Ok(format!("created:{label}")),
            other => Err(ForgeQueryWorkspaceError::new(format!(
                "worth topology production runtime could not derive stable created endpoint identity from `{other:?}`"
            ))),
        },
    }
}

fn kind_name_for_relation_kind_id(
    kind_id: forge_relational::facade::identity::KindId,
) -> &'static str {
    WorthRelationKind::from_kind_id(kind_id)
        .expect("worth topology runtime only lowers admitted relation kinds")
        .kind_name()
}
