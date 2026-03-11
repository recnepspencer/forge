use std::collections::BTreeSet;

use crate::diagnostics::data::DiagnosticCode;
use crate::logic::runtime::{RecordLifecycleState, RuntimeInstrumentation};
use crate::schema::data::RelationalSchemaRegistry;
use crate::storage::logic::state::PartitionAccess;
use crate::transactions::data::{CommitConflict, RelationIdentity, RelationSpec, TransactionIntent};
use crate::validation::logic::schema_error_to_commit_conflict;

use super::record_lookup::{entity_exists_in_state, relation_exists_in_state};

pub(super) fn validate_relation_intent(
    state: &impl PartitionAccess,
    schema_registry: &RelationalSchemaRegistry,
    default_cross_context_policy: crate::config::data::CrossContextPolicy,
    instrumentation: &RuntimeInstrumentation,
    intent: &TransactionIntent,
) -> Result<(), CommitConflict> {
    match intent {
        TransactionIntent::CreateRelation(spec) => validate_relation_creation(
            state,
            schema_registry,
            default_cross_context_policy,
            instrumentation,
            spec,
        ),
        TransactionIntent::BulkCreateRelations {
            partition_id,
            kind_id,
            endpoints,
            ..
        } => validate_bulk_relation_creation(
            state,
            schema_registry,
            default_cross_context_policy,
            instrumentation,
            *partition_id,
            *kind_id,
            endpoints,
        ),
        TransactionIntent::DeleteRelation { relation_id } => {
            if relation_exists_in_state(state, *relation_id) {
                Ok(())
            } else {
                Err(CommitConflict {
                    code: DiagnosticCode::StaleHandle,
                    detail: format!("relation {:?} is stale or absent", relation_id),
                })
            }
        }
        TransactionIntent::CreateEntity(_)
        | TransactionIntent::BulkCreateEntities { .. }
        | TransactionIntent::UpdateEntity { .. }
        | TransactionIntent::ReplaceEntity { .. }
        | TransactionIntent::DeleteEntity { .. } => Ok(()),
    }
}

fn validate_bulk_relation_creation(
    state: &impl PartitionAccess,
    schema_registry: &RelationalSchemaRegistry,
    default_cross_context_policy: crate::config::data::CrossContextPolicy,
    instrumentation: &RuntimeInstrumentation,
    partition_id: crate::identity::data::PartitionId,
    kind_id: crate::identity::data::KindId,
    endpoints: &[(crate::identity::data::EntityId, crate::identity::data::EntityId)],
) -> Result<(), CommitConflict> {
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
            return Err(CommitConflict {
                code: DiagnosticCode::DuplicateRelationIdentity,
                detail: "duplicate relation identity within bulk batch".to_string(),
            });
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
            schema_registry,
            default_cross_context_policy,
            instrumentation,
            &spec,
        )?;
    }
    Ok(())
}

fn validate_relation_creation(
    state: &impl PartitionAccess,
    schema_registry: &RelationalSchemaRegistry,
    default_cross_context_policy: crate::config::data::CrossContextPolicy,
    instrumentation: &RuntimeInstrumentation,
    spec: &RelationSpec,
) -> Result<(), CommitConflict> {
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
            return Err(CommitConflict {
                code: DiagnosticCode::InvalidRelationEndpoint,
                detail: "cross-context relation endpoints are not allowed for this relation kind"
                    .to_string(),
            });
        }
    }
    if !entity_exists_in_state(state, spec.source) || !entity_exists_in_state(state, spec.target) {
        return Err(CommitConflict {
            code: DiagnosticCode::InvalidRelationEndpoint,
            detail: "relation endpoints must be live entities".to_string(),
        });
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
        let slot = relation_id.local_slot.0 as usize;
        if relation_partition.relation_arena.lifecycle.get(slot)
            != Some(&RecordLifecycleState::Live)
        {
            continue;
        }
        let Some(endpoints) = relation_partition.relation_arena.extra[slot].as_ref() else {
            continue;
        };
        let same_kind = relation_partition.relation_arena.kind_ids[slot] == Some(spec.kind_id);
        let same_endpoints = endpoints.source == spec.source && endpoints.target == spec.target;
        if same_kind && same_endpoints {
            return Err(CommitConflict {
                code: DiagnosticCode::DuplicateRelationIdentity,
                detail: "duplicate relation identity".to_string(),
            });
        }
    }
    Ok(())
}
