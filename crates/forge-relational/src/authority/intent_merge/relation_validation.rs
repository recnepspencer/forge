use std::collections::{BTreeMap, BTreeSet};

use crate::capabilities::{SchemaSource, StorageRead};
use crate::identity::data::VersionId;
use crate::logic::runtime::RelationalRuntime;
use crate::logic::runtime::RuntimeInstrumentation;
use crate::transactions::data::{
    CommitConflict, ConflictClass, CreateIntent, CreatedEntityRef, EntityReference,
    ExistingRecordTarget, MutationIntent, RelationIdentity, RelationSpec,
};

use super::record_lookup::{
    entity_exists_in_state, relation_exists_in_state, relation_exists_in_version_basis,
};
use super::schema_conflicts::schema_error_to_commit_conflict;

pub(super) fn validate_relation_intent(
    runtime: &RelationalRuntime,
    state: &impl StorageRead,
    schema_source: &impl SchemaSource,
    default_cross_context_policy: crate::config::data::CrossContextPolicy,
    instrumentation: &RuntimeInstrumentation,
    branch_basis_version_id: Option<VersionId>,
    created_entities: &BTreeSet<CreatedEntityRef>,
    intent: &MutationIntent,
) -> Result<(), CommitConflict> {
    match intent {
        MutationIntent::Create(CreateIntent::Relation(spec)) => validate_relation_creation(
            state,
            schema_source,
            default_cross_context_policy,
            instrumentation,
            created_entities,
            spec,
        ),
        MutationIntent::Create(CreateIntent::BulkRelations(spec)) => {
            validate_bulk_relation_creation(
                state,
                schema_source,
                default_cross_context_policy,
                instrumentation,
                created_entities,
                spec.partition_id,
                spec.kind_id,
                &spec.endpoints,
            )
        }
        MutationIntent::Relation(crate::transactions::data::RelationMutationIntent::Delete(
            spec,
        )) => {
            if relation_exists_in_state(state, spec.relation_id)
                || branch_basis_version_id.is_some_and(|version_id| {
                    relation_exists_in_version_basis(runtime, version_id, spec.relation_id)
                })
            {
                Ok(())
            } else {
                Err(CommitConflict::new(ConflictClass::StaleTarget {
                    target: ExistingRecordTarget::Relation(spec.relation_id),
                    context: "relation validation".to_string(),
                }))
            }
        }
        MutationIntent::Create(CreateIntent::Entity(_))
        | MutationIntent::Create(CreateIntent::BulkEntities(_))
        | MutationIntent::Entity(_) => Ok(()),
    }
}

fn validate_bulk_relation_creation(
    state: &impl StorageRead,
    schema_source: &impl SchemaSource,
    default_cross_context_policy: crate::config::data::CrossContextPolicy,
    instrumentation: &RuntimeInstrumentation,
    created_entities: &BTreeSet<CreatedEntityRef>,
    partition_id: crate::identity::data::PartitionId,
    kind_id: crate::identity::data::KindId,
    endpoints: &[(EntityReference, EntityReference)],
) -> Result<(), CommitConflict> {
    let schema_registry = schema_source.schema_registry();
    schema_registry
        .resolve_relation(kind_id)
        .map_err(schema_error_to_commit_conflict)?;
    let relation_registration = schema_registry
        .relation_registration(kind_id)
        .map_err(schema_error_to_commit_conflict)?;
    let mut seen_batch_keys = BTreeSet::new();
    let mut targets_by_source = BTreeMap::new();
    for (source, target) in endpoints {
        let identity = RelationIdentity {
            partition_id,
            kind_id,
            source: source.clone(),
            target: target.clone(),
        };
        if !seen_batch_keys.insert(identity) {
            return Err(CommitConflict::new(
                ConflictClass::DuplicateRelationIdentity {
                    detail: "duplicate relation identity within bulk batch".to_string(),
                },
            ));
        }
        validate_relation_creation_primitives(
            state,
            relation_registration.cross_context_policy,
            default_cross_context_policy,
            &source,
            target,
            created_entities,
        )?;
        targets_by_source
            .entry(source.clone())
            .or_insert_with(BTreeSet::new)
            .insert(target.clone());
    }

    for (source, targets) in targets_by_source {
        if existing_relation_targets_for_source(
            state,
            instrumentation,
            partition_id,
            kind_id,
            &source,
            &targets,
        ) {
            return Err(CommitConflict::new(
                ConflictClass::DuplicateRelationIdentity {
                    detail: "duplicate relation identity".to_string(),
                },
            ));
        }
    }
    Ok(())
}

fn validate_relation_creation(
    state: &impl StorageRead,
    schema_source: &impl SchemaSource,
    default_cross_context_policy: crate::config::data::CrossContextPolicy,
    instrumentation: &RuntimeInstrumentation,
    created_entities: &BTreeSet<CreatedEntityRef>,
    spec: &RelationSpec,
) -> Result<(), CommitConflict> {
    let schema_registry = schema_source.schema_registry();
    schema_registry
        .resolve_relation(spec.kind_id)
        .map_err(schema_error_to_commit_conflict)?;
    let relation_registration = schema_registry
        .relation_registration(spec.kind_id)
        .map_err(schema_error_to_commit_conflict)?;
    validate_relation_creation_primitives(
        state,
        relation_registration.cross_context_policy,
        default_cross_context_policy,
        &spec.source,
        &spec.target,
        created_entities,
    )?;
    if existing_relation_targets_for_source(
        state,
        instrumentation,
        spec.partition_id,
        spec.kind_id,
        &spec.source,
        &BTreeSet::from([spec.target.clone()]),
    ) {
        return Err(CommitConflict::new(
            ConflictClass::DuplicateRelationIdentity {
                detail: "duplicate relation identity".to_string(),
            },
        ));
    }
    Ok(())
}

fn validate_relation_creation_primitives(
    state: &impl StorageRead,
    relation_cross_context_policy: crate::config::data::CrossContextPolicy,
    default_cross_context_policy: crate::config::data::CrossContextPolicy,
    source: &EntityReference,
    target: &EntityReference,
    created_entities: &BTreeSet<CreatedEntityRef>,
) -> Result<(), CommitConflict> {
    if source.partition_id() != target.partition_id() {
        let effective_cross_context_policy = match relation_cross_context_policy {
            crate::config::data::CrossContextPolicy::SchemaControlled => {
                default_cross_context_policy
            }
            explicit_policy => explicit_policy,
        };
        if effective_cross_context_policy != crate::config::data::CrossContextPolicy::AllowExplicit
        {
            return Err(CommitConflict::new(
                ConflictClass::InvalidRelationEndpoint {
                    detail:
                        "cross-context relation endpoints are not allowed for this relation kind"
                            .to_string(),
                },
            ));
        }
    }
    if !entity_reference_exists_in_commit_scope(state, source, created_entities)
        || !entity_reference_exists_in_commit_scope(state, target, created_entities)
    {
        return Err(CommitConflict::new(
            ConflictClass::InvalidRelationEndpoint {
                detail: "relation endpoints must be live entities".to_string(),
            },
        ));
    }
    Ok(())
}

fn existing_relation_targets_for_source(
    state: &impl StorageRead,
    instrumentation: &RuntimeInstrumentation,
    partition_id: crate::identity::data::PartitionId,
    kind_id: crate::identity::data::KindId,
    source: &EntityReference,
    targets: &BTreeSet<EntityReference>,
) -> bool {
    let EntityReference::Existing(source_entity) = source else {
        return false;
    };
    let Some(source_partition) = state.get_partition(source_entity.partition_id) else {
        return false;
    };
    let Some(outgoing_relations) = source_partition.adjacency.get(source_entity.local_slot.0 as usize)
    else {
        return false;
    };
    for relation_id in outgoing_relations.as_slice().iter().copied() {
        instrumentation.count(|counters| counters.relation_identity_candidates_scanned += 1);
        if relation_id.partition_id != partition_id {
            continue;
        }
        let Some(relation_partition) = state.get_partition(relation_id.partition_id) else {
            continue;
        };
        let Some(relation_slot) = relation_partition.relation_arena.get(&relation_id) else {
            continue;
        };
        if relation_slot.kind_id() != Some(kind_id) {
            continue;
        }
        let Some(endpoints) = relation_slot.extra().as_ref() else {
            continue;
        };
        if endpoints.source == *source_entity
            && targets.contains(&EntityReference::Existing(endpoints.target))
        {
            return true;
        }
    }
    false
}

fn entity_reference_exists_in_commit_scope(
    state: &impl StorageRead,
    entity_reference: &EntityReference,
    created_entities: &BTreeSet<CreatedEntityRef>,
) -> bool {
    match entity_reference {
        EntityReference::Existing(entity_id) => entity_exists_in_state(state, *entity_id),
        EntityReference::Created(created) => created_entities.contains(created),
    }
}

#[cfg(test)]
mod tests {
    use std::collections::{BTreeMap, BTreeSet};

    use crate::config::data::{AdjacencyBackend, AdjacencyPolicy};
    use crate::identity::data::{EntityId, KindId, PartitionId, RelationId, VersionId};
    use crate::logic::runtime::RuntimeInstrumentation;
    use crate::storage::logic::state::{
        AdjacencySet, EntityArena, PartitionState, RelationArena, RelationEndpoints, WorkingState,
    };
    use crate::storage::substrate::{EntityRecordKind, RecordKind, SlotInit};
    use crate::transactions::facade::EntityReference;

    use super::existing_relation_targets_for_source;

    #[test]
    fn existing_relation_targets_scans_shared_source_once_for_batched_targets() {
        let adjacency_policy = AdjacencyPolicy {
            backend: AdjacencyBackend::InlineSmallDegreeAdjacency,
            small_degree_inline_capacity: 4,
        };
        let partition_id = PartitionId(1);
        let mut entity_arena = EntityArena::with_capacity(3);
        let (source_slot, source_generation, _) = entity_arena.push_slot(SlotInit {
            partition_id,
            kind_id: KindId(1),
            payload: None,
            version_id: VersionId(1),
            extra: EntityRecordKind::empty_extra(),
        });
        let (left_slot, left_generation, _) = entity_arena.push_slot(SlotInit {
            partition_id,
            kind_id: KindId(1),
            payload: None,
            version_id: VersionId(1),
            extra: EntityRecordKind::empty_extra(),
        });
        let (right_slot, right_generation, _) = entity_arena.push_slot(SlotInit {
            partition_id,
            kind_id: KindId(1),
            payload: None,
            version_id: VersionId(1),
            extra: EntityRecordKind::empty_extra(),
        });
        let source = EntityId::new(partition_id, source_slot as u64, source_generation);
        let left = EntityId::new(partition_id, left_slot as u64, left_generation);
        let right = EntityId::new(partition_id, right_slot as u64, right_generation);

        let mut relation_arena = RelationArena::with_capacity(1);
        let (relation_slot, relation_generation, _) = relation_arena.push_slot(SlotInit {
            partition_id,
            kind_id: KindId(9),
            payload: None,
            version_id: VersionId(1),
            extra: Some(RelationEndpoints {
                source,
                target: left,
            }),
        });
        let relation_id = RelationId::new(partition_id, relation_slot as u64, relation_generation);
        let mut adjacency = vec![AdjacencySet::new(&adjacency_policy); 3];
        adjacency[source_slot].insert(relation_id);

        let mut partitions = BTreeMap::new();
        partitions.insert(
            partition_id,
            PartitionState {
                partition_id,
                adjacency_policy,
                relation_overlay_is_sparse: false,
                entity_arena,
                relation_arena,
                adjacency,
                reverse_adjacency: vec![
                    AdjacencySet::new(&AdjacencyPolicy {
                        backend: AdjacencyBackend::InlineSmallDegreeAdjacency,
                        small_degree_inline_capacity: 4,
                    });
                    3
                ],
            },
        );
        let state = WorkingState::new(
            partitions,
            AdjacencyPolicy {
                backend: AdjacencyBackend::InlineSmallDegreeAdjacency,
                small_degree_inline_capacity: 4,
            },
        );
        let instrumentation = RuntimeInstrumentation::new();
        let found = existing_relation_targets_for_source(
            &state,
            &instrumentation,
            partition_id,
            KindId(9),
            &EntityReference::Existing(source),
            &BTreeSet::from([
                EntityReference::Existing(left),
                EntityReference::Existing(right),
            ]),
        );
        let counters = instrumentation
            .complexity_counters
            .lock()
            .expect("complexity counter lock poisoned")
            .clone();

        assert!(found);
        assert_eq!(counters.relation_identity_candidates_scanned, 1);
    }
}
