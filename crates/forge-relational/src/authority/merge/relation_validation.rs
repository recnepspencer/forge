use std::collections::BTreeSet;

use crate::capabilities::{SchemaSource, StorageRead};
use crate::logic::runtime::RuntimeInstrumentation;
use crate::transactions::data::{
    CommitConflict, ConflictClass, ExistingRecordTarget, RelationIdentity, RelationSpec,
    CreateIntent, MutationIntent,
};
use crate::validation::logic::schema_error_to_commit_conflict;

use super::record_lookup::{entity_exists_in_state, relation_exists_in_state};

pub(super) fn validate_relation_intent(
    state: &impl StorageRead,
    schema_source: &impl SchemaSource,
    default_cross_context_policy: crate::config::data::CrossContextPolicy,
    instrumentation: &RuntimeInstrumentation,
    intent: &MutationIntent,
) -> Result<(), CommitConflict> {
    match intent {
        MutationIntent::Create(CreateIntent::Relation(spec)) => validate_relation_creation(
            state,
            schema_source,
            default_cross_context_policy,
            instrumentation,
            spec,
        ),
        MutationIntent::Create(CreateIntent::BulkRelations(spec)) => validate_bulk_relation_creation(
            state,
            schema_source,
            default_cross_context_policy,
            instrumentation,
            spec.partition_id,
            spec.kind_id,
            &spec.endpoints,
        ),
        MutationIntent::Relation(crate::transactions::data::RelationMutationIntent::Delete(spec)) => {
            if relation_exists_in_state(state, spec.relation_id) {
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
    partition_id: crate::identity::data::PartitionId,
    kind_id: crate::identity::data::KindId,
    endpoints: &[(crate::identity::data::EntityId, crate::identity::data::EntityId)],
) -> Result<(), CommitConflict> {
    let schema_registry = schema_source.schema_registry();
    schema_registry
        .resolve_relation(kind_id)
        .map_err(schema_error_to_commit_conflict)?;
    let mut seen_batch_keys = BTreeSet::new();
    for (source, target) in endpoints {
        let identity = RelationIdentity {
            partition_id,
            kind_id,
            source: *source,
            target: *target,
        };
        if !seen_batch_keys.insert(identity) {
            return Err(CommitConflict::new(ConflictClass::DuplicateRelationIdentity {
                    detail: "duplicate relation identity within bulk batch".to_string(),
                }));
        }
        let spec = RelationSpec {
            partition_id,
            kind_id,
            client_key: crate::symbols::data::InternedString::from("bulk"),
            source: *source,
            target: *target,
            payload: None,
        };
        validate_relation_creation(
            state,
            schema_source,
            default_cross_context_policy,
            instrumentation,
            &spec,
        )?;
    }
    Ok(())
}

fn validate_relation_creation(
    state: &impl StorageRead,
    schema_source: &impl SchemaSource,
    default_cross_context_policy: crate::config::data::CrossContextPolicy,
    instrumentation: &RuntimeInstrumentation,
    spec: &RelationSpec,
) -> Result<(), CommitConflict> {
    let schema_registry = schema_source.schema_registry();
    schema_registry
        .resolve_relation(spec.kind_id)
        .map_err(schema_error_to_commit_conflict)?;
    let relation_registration = schema_registry
        .relation_registration(spec.kind_id)
        .map_err(schema_error_to_commit_conflict)?;
    if spec.source.partition_id != spec.target.partition_id {
        let effective_cross_context_policy = match relation_registration.cross_context_policy {
            crate::config::data::CrossContextPolicy::SchemaControlled => {
                default_cross_context_policy
            }
            explicit_policy => explicit_policy,
        };
        if effective_cross_context_policy != crate::config::data::CrossContextPolicy::AllowExplicit
        {
            return Err(CommitConflict::new(ConflictClass::InvalidRelationEndpoint {
                    detail:
                        "cross-context relation endpoints are not allowed for this relation kind"
                            .to_string(),
                }));
        }
    }
    if !entity_exists_in_state(state, spec.source) || !entity_exists_in_state(state, spec.target) {
        return Err(CommitConflict::new(ConflictClass::InvalidRelationEndpoint {
                detail: "relation endpoints must be live entities".to_string(),
            }));
    }
    let Some(source_partition) = state.get_partition(spec.source.partition_id) else {
        return Ok(());
    };
    let Some(outgoing_relations) = source_partition
        .adjacency
        .get(spec.source.local_slot.0 as usize)
    else {
        return Ok(());
    };
    for relation_id in outgoing_relations.as_slice().iter().copied() {
        instrumentation.count(|counters| counters.relation_identity_candidates_scanned += 1);
        if relation_id.partition_id != spec.partition_id {
            continue;
        }
        let Some(relation_partition) = state.get_partition(relation_id.partition_id) else {
            continue;
        };
        let Some(relation_slot) = relation_partition.relation_arena.get(&relation_id) else {
            continue;
        };
        let Some(endpoints) = relation_slot.extra().as_ref() else {
            continue;
        };
        let same_kind = relation_slot.kind_id() == Some(spec.kind_id);
        let same_endpoints = endpoints.source == spec.source && endpoints.target == spec.target;
        if same_kind && same_endpoints {
            return Err(CommitConflict::new(ConflictClass::DuplicateRelationIdentity {
                    detail: "duplicate relation identity".to_string(),
                }));
        }
    }
    Ok(())
}
