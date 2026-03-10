use std::collections::BTreeSet;

use crate::data::diagnostics::DiagnosticCode;
use crate::data::identity::{EntityId, RelationId};
use crate::data::schema::RelationalSchemaRegistry;
use crate::data::transaction::{CommitConflict, TransactionIntent};
use crate::logic::runtime::RecordLifecycleState;

use super::invariants::schema_error_to_commit_conflict;
use super::state::WorkingState;

pub(super) fn canonical_intent_key(intent: &TransactionIntent) -> (u8, String) {
    match intent {
        TransactionIntent::CreateEntity(spec) => (
            0,
            format!(
                "{:08}:{:010}:{:?}",
                spec.partition_id.0, spec.kind_id.0, spec.client_key
            ),
        ),
        TransactionIntent::BulkCreateEntities {
            partition_id,
            kind_id,
            client_keys,
            ..
        } => (
            1,
            format!("{:08}:{:010}:{:?}", partition_id.0, kind_id.0, client_keys),
        ),
        TransactionIntent::UpdateEntity { entity_id, .. } => (2, entity_key(*entity_id)),
        TransactionIntent::DeleteEntity { entity_id } => (3, entity_key(*entity_id)),
        TransactionIntent::CreateRelation(spec) => (4, relation_create_key(spec)),
        TransactionIntent::BulkCreateRelations {
            partition_id,
            kind_id,
            endpoints,
            ..
        } => (
            5,
            format!("{:08}:{:010}:{:?}", partition_id.0, kind_id.0, endpoints),
        ),
        TransactionIntent::DeleteRelation { relation_id } => (6, relation_key(*relation_id)),
    }
}

pub(super) fn validate_intent(
    state: &WorkingState,
    schema_registry: &RelationalSchemaRegistry,
    intent: &TransactionIntent,
) -> Result<(), CommitConflict> {
    match intent {
        TransactionIntent::CreateEntity(spec) => schema_registry
            .resolve_entity(spec.kind_id)
            .map(|_| ())
            .map_err(schema_error_to_commit_conflict),
        TransactionIntent::BulkCreateEntities { kind_id, .. } => schema_registry
            .resolve_entity(*kind_id)
            .map(|_| ())
            .map_err(schema_error_to_commit_conflict),
        TransactionIntent::UpdateEntity { entity_id, .. }
        | TransactionIntent::DeleteEntity { entity_id } => {
            if entity_exists_in_state(state, *entity_id) {
                Ok(())
            } else {
                Err(CommitConflict {
                    code: DiagnosticCode::StaleHandle,
                    detail: format!("entity {:?} is stale or absent", entity_id),
                })
            }
        }
        TransactionIntent::CreateRelation(spec) => validate_relation_creation(state, schema_registry, spec),
        TransactionIntent::BulkCreateRelations {
            partition_id,
            kind_id,
            endpoints,
            ..
        } => {
            schema_registry
                .resolve_relation(*kind_id)
                .map_err(schema_error_to_commit_conflict)?;
            for (source, target) in endpoints {
                let spec = crate::data::transaction::RelationSpec {
                    partition_id: *partition_id,
                    kind_id: *kind_id,
                    client_key: crate::data::symbols::InternedString::from("bulk"),
                    source: *source,
                    target: *target,
                    payload: None,
                };
                validate_relation_creation(state, schema_registry, &spec)?;
            }
            Ok(())
        }
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
    }
}

fn validate_relation_creation(
    state: &WorkingState,
    schema_registry: &RelationalSchemaRegistry,
    spec: &crate::data::transaction::RelationSpec,
) -> Result<(), CommitConflict> {
    schema_registry
        .resolve_relation(spec.kind_id)
        .map_err(schema_error_to_commit_conflict)?;
    if !entity_exists_in_state(state, spec.source) || !entity_exists_in_state(state, spec.target) {
        return Err(CommitConflict {
            code: DiagnosticCode::InvalidRelationEndpoint,
            detail: "relation endpoints must be live entities".to_string(),
        });
    }
    for slot in 0..state.relation_arena.generations.len() {
        if state.relation_arena.lifecycle[slot] != RecordLifecycleState::Live {
            continue;
        }
        let Some(endpoints) = state.relation_arena.endpoints[slot].as_ref() else {
            continue;
        };
        let same_kind = state.relation_arena.kind_ids[slot] == Some(spec.kind_id);
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

pub(super) fn detect_conflicting_updates(
    intents: &[TransactionIntent],
) -> Result<(), CommitConflict> {
    let mut seen_updates = BTreeSet::new();
    for intent in intents {
        match intent {
            TransactionIntent::UpdateEntity { entity_id, .. }
            | TransactionIntent::DeleteEntity { entity_id } => {
                if !seen_updates.insert(format!("entity:{}", entity_key(*entity_id))) {
                    return Err(CommitConflict {
                        code: DiagnosticCode::ConflictingIntent,
                        detail: format!(
                            "conflicting entity intent for slot {}",
                            entity_id.local_slot.0
                        ),
                    });
                }
            }
            TransactionIntent::DeleteRelation { relation_id } => {
                if !seen_updates.insert(format!("relation:{}", relation_key(*relation_id))) {
                    return Err(CommitConflict {
                        code: DiagnosticCode::ConflictingIntent,
                        detail: format!(
                            "conflicting relation intent for slot {}",
                            relation_id.local_slot.0
                        ),
                    });
                }
            }
            TransactionIntent::CreateEntity(_)
            | TransactionIntent::BulkCreateEntities { .. }
            | TransactionIntent::CreateRelation(_)
            | TransactionIntent::BulkCreateRelations { .. } => {}
        }
    }
    Ok(())
}

pub(super) fn entity_exists_in_state(state: &WorkingState, entity_id: EntityId) -> bool {
    let slot = entity_id.local_slot.0 as usize;
    state.entity_arena.partition_ids.get(slot) == Some(&entity_id.partition_id)
        && state.entity_arena.generations.get(slot) == Some(&entity_id.generation.0)
        && state.entity_arena.lifecycle.get(slot) == Some(&RecordLifecycleState::Live)
}

pub(super) fn relation_exists_in_state(state: &WorkingState, relation_id: RelationId) -> bool {
    let slot = relation_id.local_slot.0 as usize;
    state.relation_arena.partition_ids.get(slot) == Some(&relation_id.partition_id)
        && state.relation_arena.generations.get(slot) == Some(&relation_id.generation.0)
        && state.relation_arena.lifecycle.get(slot) == Some(&RecordLifecycleState::Live)
}

fn entity_key(entity_id: EntityId) -> String {
    format!(
        "{:08}:{:020}:{:010}",
        entity_id.partition_id.0, entity_id.local_slot.0, entity_id.generation.0
    )
}

fn relation_key(relation_id: RelationId) -> String {
    format!(
        "{:08}:{:020}:{:010}",
        relation_id.partition_id.0, relation_id.local_slot.0, relation_id.generation.0
    )
}

fn relation_create_key(spec: &crate::data::transaction::RelationSpec) -> String {
    format!(
        "{:08}:{:010}:{:08}:{:020}:{:08}:{:020}:{:?}",
        spec.partition_id.0,
        spec.kind_id.0,
        spec.source.partition_id.0,
        spec.source.local_slot.0,
        spec.target.partition_id.0,
        spec.target.local_slot.0,
        spec.client_key
    )
}
