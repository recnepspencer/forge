use std::collections::BTreeMap;
use std::sync::{Arc, RwLock};

use forge_query::facade::{
    ForgeQueryAspectValue, ForgeQueryExistingTruthTargetBinding, ForgeQuerySymbolicAspectReference,
    ForgeQuerySymbolicTargetReference, ForgeQueryWorkspaceError, ForgeQueryWriteCommand,
};
use forge_relational::facade::runtime::RelationalRuntime;
use forge_relational::facade::transactions::{
    CreateIntent, DeleteEntityIntent, DeleteRelationIntent, EntityMutationIntent, EntityReference,
    MutationIntent, RelationMutationIntent,
};

use super::super::write_support::{
    aspect_map, optional_text, parse_entity_identity, parse_relation_identity, required_text,
    write_command_label,
};
use super::patch_matching::LoweredPatchMatch;
use super::write_lowering::{
    kind_name_for_relation_kind_id, lower_topology_entity_insert, lower_topology_relation_insert,
    lower_topology_relation_update, relation_endpoint_identity,
};

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
        } => lower_insert_command(
            runtime,
            created_entities,
            collection,
            aspects,
            symbolic_aspect_references,
            symbolic_target_reference,
        ),
        ForgeQueryWriteCommand::DeleteExistingAspects {
            binding,
            touched_aspect_paths,
            ..
        } => lower_existing_delete_command(binding, touched_aspect_paths),
        ForgeQueryWriteCommand::UpdateExistingAspects {
            binding, aspects, ..
        } => {
            lower_existing_relation_update_command(runtime, created_entities, binding, aspects, &[])
        }
        ForgeQueryWriteCommand::VerifyThenUpdateExistingAspects {
            binding,
            aspects,
            symbolic_aspect_references,
            ..
        } => lower_existing_relation_update_command(
            runtime,
            created_entities,
            binding,
            aspects,
            &symbolic_aspect_references,
        ),
        ForgeQueryWriteCommand::VerifyThenDeleteExistingAspects {
            binding,
            touched_aspect_paths,
            ..
        } => lower_existing_delete_command(binding, touched_aspect_paths),
        other => Err(ForgeQueryWorkspaceError::new(format!(
            "topology production runtime current-head slice does not admit `{}` write command yet",
            write_command_label(&other)
        ))),
    }
}

fn lower_insert_command(
    runtime: &Arc<RwLock<RelationalRuntime>>,
    created_entities: &mut BTreeMap<String, EntityReference>,
    collection: String,
    aspects: Vec<ForgeQueryAspectValue>,
    symbolic_aspect_references: Vec<ForgeQuerySymbolicAspectReference>,
    symbolic_target_reference: Option<ForgeQuerySymbolicTargetReference>,
) -> Result<LoweredWriteCommand, ForgeQueryWorkspaceError> {
    let aspect_map = aspect_map(&aspects);
    let declared_aspect_paths = declared_insert_aspect_paths(&aspects, &symbolic_aspect_references);
    match collection.as_str() {
        "TopologyEntity" => lower_entity_insert_command(
            runtime,
            created_entities,
            &aspect_map,
            declared_aspect_paths,
            symbolic_target_reference,
        ),
        "TopologyRelation" => lower_relation_insert_command(
            runtime,
            created_entities,
            &aspect_map,
            &symbolic_aspect_references,
            declared_aspect_paths,
        ),
        other => Err(ForgeQueryWorkspaceError::new(format!(
            "topology production runtime does not admit insert collection `{other}`"
        ))),
    }
}

fn lower_entity_insert_command(
    runtime: &Arc<RwLock<RelationalRuntime>>,
    created_entities: &mut BTreeMap<String, EntityReference>,
    aspect_map: &BTreeMap<String, serde_json::Value>,
    declared_aspect_paths: Vec<String>,
    symbolic_target_reference: Option<ForgeQuerySymbolicTargetReference>,
) -> Result<LoweredWriteCommand, ForgeQueryWorkspaceError> {
    let (intents, created_entity_ref) = lower_topology_entity_insert(runtime, aspect_map)?;
    if let Some(reference) = symbolic_target_reference {
        created_entities.insert(reference.symbol().to_string(), created_entity_ref);
    }
    Ok(LoweredWriteCommand {
        batch_label: "query-runtime-batch-insert-entity".to_string(),
        intents,
        declared_aspect_paths,
        expected_observable_patch_count: 2,
        patch_match: LoweredPatchMatch::TopologyEntityInsert {
            structure_label: required_text(aspect_map, "topology.structure")?,
            persistent_name: optional_text(aspect_map, "naming.persistent_name").unwrap_or_else(
                || {
                    required_text(aspect_map, "topology.structure")
                        .expect("topology.structure should exist for insert")
                },
            ),
        },
        fallback_collection: None,
    })
}

fn lower_relation_insert_command(
    runtime: &Arc<RwLock<RelationalRuntime>>,
    created_entities: &BTreeMap<String, EntityReference>,
    aspect_map: &BTreeMap<String, serde_json::Value>,
    symbolic_aspect_references: &[ForgeQuerySymbolicAspectReference],
    declared_aspect_paths: Vec<String>,
) -> Result<LoweredWriteCommand, ForgeQueryWorkspaceError> {
    let relation = lower_topology_relation_insert(
        runtime,
        aspect_map,
        symbolic_aspect_references,
        created_entities,
    )?;
    Ok(LoweredWriteCommand {
        batch_label: "query-runtime-batch-insert-relation".to_string(),
        intents: vec![MutationIntent::Create(CreateIntent::Relation(
            relation.clone(),
        ))],
        declared_aspect_paths,
        expected_observable_patch_count: 1,
        patch_match: LoweredPatchMatch::TopologyRelationInsert {
            kind_name: kind_name_for_relation_kind_id(relation.kind_id).to_string(),
            source_identity: relation_endpoint_identity(&relation.source)?,
            target_identity: relation_endpoint_identity(&relation.target)?,
        },
        fallback_collection: None,
    })
}

fn lower_existing_delete_command(
    binding: ForgeQueryExistingTruthTargetBinding,
    touched_aspect_paths: Vec<String>,
) -> Result<LoweredWriteCommand, ForgeQueryWorkspaceError> {
    let collection = required_target_collection(&binding, "delete")?;
    let intent = match collection.as_str() {
        "TopologyEntity" => {
            MutationIntent::Entity(EntityMutationIntent::Delete(DeleteEntityIntent {
                entity_id: parse_entity_identity(binding.resolved_target_identity())?,
            }))
        }
        "TopologyRelation" => {
            MutationIntent::Relation(RelationMutationIntent::Delete(DeleteRelationIntent {
                relation_id: parse_relation_identity(binding.resolved_target_identity())?,
            }))
        }
        other => {
            return Err(ForgeQueryWorkspaceError::new(format!(
                "topology production runtime does not admit delete collection `{other}`"
            )))
        }
    };
    Ok(existing_target_command(
        "query-runtime-batch-delete",
        binding.resolved_target_identity(),
        vec![intent],
        touched_aspect_paths,
        collection,
    ))
}

fn lower_existing_relation_update_command(
    runtime: &Arc<RwLock<RelationalRuntime>>,
    created_entities: &mut BTreeMap<String, EntityReference>,
    binding: ForgeQueryExistingTruthTargetBinding,
    aspects: Vec<ForgeQueryAspectValue>,
    symbolic_aspect_references: &[ForgeQuerySymbolicAspectReference],
) -> Result<LoweredWriteCommand, ForgeQueryWorkspaceError> {
    let collection = required_target_collection(&binding, "update")?;
    let aspect_map = aspect_map(&aspects);
    let declared_aspect_paths = declared_update_aspect_paths(&aspects, symbolic_aspect_references);
    let intent = match collection.as_str() {
        "TopologyRelation" => MutationIntent::Relation(RelationMutationIntent::UpdateEndpoints(
            lower_topology_relation_update(
                runtime,
                &aspect_map,
                symbolic_aspect_references,
                created_entities,
                binding.resolved_target_identity(),
            )?,
        )),
        other => {
            return Err(ForgeQueryWorkspaceError::new(format!(
                "topology production runtime does not admit update collection `{other}`"
            )))
        }
    };
    Ok(existing_target_command(
        "query-runtime-batch-update",
        binding.resolved_target_identity(),
        vec![intent],
        declared_aspect_paths,
        collection,
    ))
}

fn existing_target_command(
    batch_label: &str,
    resolved_target_identity: &str,
    intents: Vec<MutationIntent>,
    declared_aspect_paths: Vec<String>,
    collection: String,
) -> LoweredWriteCommand {
    LoweredWriteCommand {
        batch_label: batch_label.to_string(),
        intents,
        declared_aspect_paths,
        expected_observable_patch_count: 1,
        patch_match: LoweredPatchMatch::ExistingTargetIdentity {
            resolved_target_identity: resolved_target_identity.to_string(),
        },
        fallback_collection: Some(collection),
    }
}

fn required_target_collection(
    binding: &ForgeQueryExistingTruthTargetBinding,
    mutation_family: &str,
) -> Result<String, ForgeQueryWorkspaceError> {
    binding
        .target_collection()
        .map(str::to_string)
        .ok_or_else(|| {
            ForgeQueryWorkspaceError::new(format!(
                "topology production runtime {mutation_family} requires a declared target collection"
            ))
        })
}

fn declared_insert_aspect_paths(
    aspects: &[ForgeQueryAspectValue],
    symbolic_aspect_references: &[ForgeQuerySymbolicAspectReference],
) -> Vec<String> {
    declared_aspect_paths(aspects, symbolic_aspect_references)
}

fn declared_update_aspect_paths(
    aspects: &[ForgeQueryAspectValue],
    symbolic_aspect_references: &[ForgeQuerySymbolicAspectReference],
) -> Vec<String> {
    declared_aspect_paths(aspects, symbolic_aspect_references)
}

fn declared_aspect_paths(
    aspects: &[ForgeQueryAspectValue],
    symbolic_aspect_references: &[ForgeQuerySymbolicAspectReference],
) -> Vec<String> {
    aspects
        .iter()
        .map(|aspect| aspect.aspect_path().to_string())
        .chain(
            symbolic_aspect_references
                .iter()
                .map(|reference| reference.aspect_path().to_string()),
        )
        .collect()
}
