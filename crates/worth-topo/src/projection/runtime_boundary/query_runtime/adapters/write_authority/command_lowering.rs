use std::collections::BTreeMap;
use std::sync::{Arc, RwLock};

use forge_query::facade::{
    ForgeQueryAdmittedAspectValue, ForgeQueryAspectTouch, ForgeQueryBackendAdmissibleMutation,
    ForgeQueryEntityIdentity, ForgeQueryExistingTruthTargetBinding, ForgeQueryMutationFamily,
    ForgeQuerySymbolicAspectReference, ForgeQuerySymbolicTargetReference, ForgeQueryWorkspaceError,
};
use forge_relational::facade::runtime::RelationalRuntime;
use forge_relational::facade::transactions::{
    CreateIntent, DeleteEntityIntent, DeleteRelationIntent, EntityMutationIntent, EntityReference,
    MutationIntent, RelationMutationIntent,
};

use super::super::write_support::{
    aspect_map, entity_id_from_query_identity, optional_text, relation_id_from_query_identity,
    required_text,
};
use super::patch_matching::LoweredPatchMatch;
use super::write_lowering::{
    kind_name_for_relation_kind_id, lower_topology_entity_insert, lower_topology_relation_insert,
    lower_topology_relation_update, relation_endpoint_identity,
};

pub(super) struct LoweredWriteCommand {
    pub(super) mutation_label: &'static str,
    pub(super) intents: Vec<MutationIntent>,
    pub(super) declared_aspect_touches: Vec<ForgeQueryAspectTouch>,
    pub(super) expected_observable_patch_count: usize,
    pub(super) patch_match: LoweredPatchMatch,
    pub(super) declared_target_collection: Option<String>,
}

pub(super) fn lower_write_command(
    runtime: &Arc<RwLock<RelationalRuntime>>,
    created_entities: &mut BTreeMap<String, EntityReference>,
    mutation: &ForgeQueryBackendAdmissibleMutation,
) -> Result<LoweredWriteCommand, ForgeQueryWorkspaceError> {
    match mutation.mutation_family() {
        ForgeQueryMutationFamily::Insert => lower_insert_command(
            runtime,
            created_entities,
            mutation
                .declared_collection_identity()
                .ok_or_else(|| {
                    ForgeQueryWorkspaceError::new(
                        "topology production runtime insert requires declared target collection",
                    )
                })?
                .as_str(),
            mutation.admitted_aspect_values(),
            mutation.symbolic_aspect_references(),
            mutation.symbolic_target_reference(),
        ),
        ForgeQueryMutationFamily::Update => lower_existing_relation_update_command(
            runtime,
            created_entities,
            required_existing_truth_binding(mutation, "update")?.clone(),
            mutation.admitted_aspect_values(),
            mutation.symbolic_aspect_references(),
        ),
        ForgeQueryMutationFamily::Delete => lower_existing_delete_command(
            required_existing_truth_binding(mutation, "delete")?.clone(),
            mutation.admitted_touched_aspects(),
        ),
        ForgeQueryMutationFamily::Assertion => Err(ForgeQueryWorkspaceError::new(
            "topology production runtime write authority does not execute assertion-only mutations",
        )),
    }
}

fn lower_insert_command(
    runtime: &Arc<RwLock<RelationalRuntime>>,
    created_entities: &mut BTreeMap<String, EntityReference>,
    collection: &str,
    aspects: &[ForgeQueryAdmittedAspectValue],
    symbolic_aspect_references: &[ForgeQuerySymbolicAspectReference],
    symbolic_target_reference: Option<&ForgeQuerySymbolicTargetReference>,
) -> Result<LoweredWriteCommand, ForgeQueryWorkspaceError> {
    let aspect_map = aspect_map(aspects)?;
    let declared_aspect_touches =
        declared_insert_aspect_touches(aspects, symbolic_aspect_references);
    match collection {
        "TopologyEntity"
        | crate::construction::TOPOLOGY_PRIMITIVE_CONSTRUCTION_BIRTH_COMPOSE_COLLECTION => {
            lower_entity_insert_command(
                runtime,
                created_entities,
                &aspect_map,
                declared_aspect_touches,
                symbolic_target_reference.cloned(),
            )
        }
        "TopologyRelation" => lower_relation_insert_command(
            runtime,
            created_entities,
            &aspect_map,
            symbolic_aspect_references,
            declared_aspect_touches,
        ),
        other => Err(ForgeQueryWorkspaceError::new(format!(
            "topology production runtime does not admit insert collection `{other}`"
        ))),
    }
}

fn lower_entity_insert_command(
    runtime: &Arc<RwLock<RelationalRuntime>>,
    created_entities: &mut BTreeMap<String, EntityReference>,
    aspect_map: &BTreeMap<String, forge_foundational::facade::AspectValue>,
    declared_aspect_touches: Vec<ForgeQueryAspectTouch>,
    symbolic_target_reference: Option<ForgeQuerySymbolicTargetReference>,
) -> Result<LoweredWriteCommand, ForgeQueryWorkspaceError> {
    let (intents, created_entity_ref) = lower_topology_entity_insert(runtime, aspect_map)?;
    if let Some(reference) = symbolic_target_reference {
        created_entities.insert(reference.symbol().to_string(), created_entity_ref);
    }
    Ok(LoweredWriteCommand {
        mutation_label: "query-runtime-mutation-insert-entity",
        intents,
        declared_aspect_touches,
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
        declared_target_collection: None,
    })
}

fn lower_relation_insert_command(
    runtime: &Arc<RwLock<RelationalRuntime>>,
    created_entities: &BTreeMap<String, EntityReference>,
    aspect_map: &BTreeMap<String, forge_foundational::facade::AspectValue>,
    symbolic_aspect_references: &[ForgeQuerySymbolicAspectReference],
    declared_aspect_touches: Vec<ForgeQueryAspectTouch>,
) -> Result<LoweredWriteCommand, ForgeQueryWorkspaceError> {
    let relation = lower_topology_relation_insert(
        runtime,
        aspect_map,
        symbolic_aspect_references,
        created_entities,
    )?;
    Ok(LoweredWriteCommand {
        mutation_label: "query-runtime-mutation-insert-relation",
        intents: vec![MutationIntent::Create(CreateIntent::Relation(
            relation.clone(),
        ))],
        declared_aspect_touches,
        expected_observable_patch_count: 1,
        patch_match: LoweredPatchMatch::TopologyRelationInsert {
            kind_name: kind_name_for_relation_kind_id(relation.kind_id).to_string(),
            source_identity: relation_endpoint_identity(&relation.source)?,
            target_identity: relation_endpoint_identity(&relation.target)?,
        },
        declared_target_collection: None,
    })
}

fn lower_existing_delete_command(
    binding: ForgeQueryExistingTruthTargetBinding,
    touched_aspects: &[ForgeQueryAspectTouch],
) -> Result<LoweredWriteCommand, ForgeQueryWorkspaceError> {
    let collection = required_target_collection(&binding, "delete")?;
    let intent = match collection.as_str() {
        "TopologyEntity" => {
            MutationIntent::Entity(EntityMutationIntent::Delete(DeleteEntityIntent {
                entity_id: entity_id_from_query_identity(binding.resolved_target_identity())?,
            }))
        }
        "TopologyRelation" => {
            MutationIntent::Relation(RelationMutationIntent::Delete(DeleteRelationIntent {
                relation_id: relation_id_from_query_identity(binding.resolved_target_identity())?,
            }))
        }
        other => {
            return Err(ForgeQueryWorkspaceError::new(format!(
                "topology production runtime does not admit delete collection `{other}`"
            )))
        }
    };
    Ok(existing_target_command(
        "query-runtime-mutation-delete",
        binding.resolved_target_identity(),
        vec![intent],
        touched_aspects.to_vec(),
        collection,
    ))
}

fn lower_existing_relation_update_command(
    runtime: &Arc<RwLock<RelationalRuntime>>,
    created_entities: &mut BTreeMap<String, EntityReference>,
    binding: ForgeQueryExistingTruthTargetBinding,
    aspects: &[ForgeQueryAdmittedAspectValue],
    symbolic_aspect_references: &[ForgeQuerySymbolicAspectReference],
) -> Result<LoweredWriteCommand, ForgeQueryWorkspaceError> {
    let collection = required_target_collection(&binding, "update")?;
    let aspect_map = aspect_map(aspects)?;
    let declared_aspect_touches =
        declared_update_aspect_touches(aspects, symbolic_aspect_references);
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
        "query-runtime-mutation-update",
        binding.resolved_target_identity(),
        vec![intent],
        declared_aspect_touches,
        collection,
    ))
}

fn existing_target_command(
    mutation_label: &'static str,
    resolved_target_identity: &ForgeQueryEntityIdentity,
    intents: Vec<MutationIntent>,
    declared_aspect_touches: Vec<ForgeQueryAspectTouch>,
    collection: String,
) -> LoweredWriteCommand {
    LoweredWriteCommand {
        mutation_label,
        intents,
        declared_aspect_touches,
        expected_observable_patch_count: 1,
        patch_match: LoweredPatchMatch::ExistingTargetIdentity {
            resolved_target_identity: resolved_target_identity.clone(),
        },
        declared_target_collection: Some(collection),
    }
}

fn required_target_collection(
    binding: &ForgeQueryExistingTruthTargetBinding,
    mutation_family: &str,
) -> Result<String, ForgeQueryWorkspaceError> {
    binding
        .target_collection_identity()
        .map(|collection| collection.as_str().to_string())
        .ok_or_else(|| {
            ForgeQueryWorkspaceError::new(format!(
                "topology production runtime {mutation_family} requires a declared target collection"
            ))
        })
}

fn required_existing_truth_binding<'a>(
    mutation: &'a ForgeQueryBackendAdmissibleMutation,
    family: &str,
) -> Result<&'a ForgeQueryExistingTruthTargetBinding, ForgeQueryWorkspaceError> {
    mutation.existing_truth_binding().ok_or_else(|| {
        ForgeQueryWorkspaceError::new(format!(
            "topology production runtime {family} requires backend-admitted existing truth binding"
        ))
    })
}

fn declared_insert_aspect_touches(
    aspects: &[ForgeQueryAdmittedAspectValue],
    symbolic_aspect_references: &[ForgeQuerySymbolicAspectReference],
) -> Vec<ForgeQueryAspectTouch> {
    declared_aspect_touches(aspects, symbolic_aspect_references)
}

fn declared_update_aspect_touches(
    aspects: &[ForgeQueryAdmittedAspectValue],
    symbolic_aspect_references: &[ForgeQuerySymbolicAspectReference],
) -> Vec<ForgeQueryAspectTouch> {
    declared_aspect_touches(aspects, symbolic_aspect_references)
}

fn declared_aspect_touches(
    aspects: &[ForgeQueryAdmittedAspectValue],
    symbolic_aspect_references: &[ForgeQuerySymbolicAspectReference],
) -> Vec<ForgeQueryAspectTouch> {
    aspects
        .iter()
        .map(ForgeQueryAdmittedAspectValue::aspect_touch)
        .chain(
            symbolic_aspect_references
                .iter()
                .map(|reference| reference.aspect_touch().clone()),
        )
        .collect()
}
