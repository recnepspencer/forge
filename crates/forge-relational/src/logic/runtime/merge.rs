use std::collections::BTreeSet;
use std::cell::RefCell;

use crate::data::diagnostics::DiagnosticCode;
use crate::data::identity::{EntityId, RelationId};
use crate::data::schema::RelationalSchemaRegistry;
use crate::data::transaction::{CommitConflict, TransactionIntent};
use crate::logic::runtime::RecordLifecycleState;

use super::complexity::RuntimeComplexityCounters;
use super::invariants::schema_error_to_commit_conflict;
use super::state::PartitionAccess;

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
        TransactionIntent::ReplaceEntity { entity_id, replacement } => (
            3,
            format!(
                "{}->{:08}:{:010}:{:?}",
                entity_key(*entity_id),
                replacement.partition_id.0,
                replacement.kind_id.0,
                replacement.client_key
            ),
        ),
        TransactionIntent::DeleteEntity { entity_id } => (4, entity_key(*entity_id)),
        TransactionIntent::CreateRelation(spec) => (5, relation_create_key(spec)),
        TransactionIntent::BulkCreateRelations {
            partition_id,
            kind_id,
            endpoints,
            ..
        } => (
            6,
            format!("{:08}:{:010}:{:?}", partition_id.0, kind_id.0, endpoints),
        ),
        TransactionIntent::DeleteRelation { relation_id } => (7, relation_key(*relation_id)),
    }
}

pub(super) fn validate_intent(
    state: &impl PartitionAccess,
    schema_registry: &RelationalSchemaRegistry,
    complexity_counters: &RefCell<RuntimeComplexityCounters>,
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
        | TransactionIntent::DeleteEntity { entity_id }
        | TransactionIntent::ReplaceEntity { entity_id, .. } => {
            if entity_exists_in_state(state, *entity_id) {
                match intent {
                    TransactionIntent::ReplaceEntity { replacement, .. } => schema_registry
                        .resolve_entity(replacement.kind_id)
                        .map(|_| ())
                        .map_err(schema_error_to_commit_conflict),
                    _ => Ok(()),
                }
            } else {
                Err(CommitConflict {
                    code: DiagnosticCode::StaleHandle,
                    detail: format!("entity {:?} is stale or absent", entity_id),
                })
            }
        }
        TransactionIntent::CreateRelation(spec) => {
            validate_relation_creation(state, schema_registry, complexity_counters, spec)
        }
        TransactionIntent::BulkCreateRelations {
            partition_id,
            kind_id,
            endpoints,
            payloads: _,
            client_keys: _,
            ..
        } => {
            schema_registry
                .resolve_relation(*kind_id)
                .map_err(schema_error_to_commit_conflict)?;
            let mut seen_batch_keys = BTreeSet::new();
            for (source, target) in endpoints {
                let spec = crate::data::transaction::RelationSpec {
                    partition_id: *partition_id,
                    kind_id: *kind_id,
                    client_key: crate::data::symbols::InternedString::from("bulk"),
                    source: *source,
                    target: *target,
                    payload: None,
                };
                let batch_key = relation_identity_key(&spec);
                if !seen_batch_keys.insert(batch_key) {
                    return Err(CommitConflict {
                        code: DiagnosticCode::DuplicateRelationIdentity,
                        detail: "duplicate relation identity within bulk batch".to_string(),
                    });
                }
                validate_relation_creation(state, schema_registry, complexity_counters, &spec)?;
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
    state: &impl PartitionAccess,
    schema_registry: &RelationalSchemaRegistry,
    complexity_counters: &RefCell<RuntimeComplexityCounters>,
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
    let Some(source_partition) = state.get_partition(spec.source.partition_id) else {
        return Ok(());
    };
    let Some(outgoing_relations) = source_partition
        .adjacency
        .get(spec.source.local_slot.0 as usize)
    else {
        return Ok(());
    };
    for relation_id in outgoing_relations.ids() {
        complexity_counters
            .borrow_mut()
            .relation_identity_candidates_scanned += 1;
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
        let Some(endpoints) = relation_partition.relation_arena.endpoints[slot].as_ref() else {
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

pub(super) fn detect_conflicting_updates(
    intents: &[TransactionIntent],
) -> Result<(), CommitConflict> {
    let mut seen_updates = BTreeSet::new();
    for intent in intents {
        match intent {
            TransactionIntent::UpdateEntity { entity_id, .. }
            | TransactionIntent::DeleteEntity { entity_id }
            | TransactionIntent::ReplaceEntity { entity_id, .. } => {
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

    let mut seen_relation_creates = BTreeSet::new();
    for intent in intents {
        match intent {
            TransactionIntent::CreateRelation(spec) => {
                if !seen_relation_creates.insert(relation_identity_key(spec)) {
                    return Err(CommitConflict {
                        code: DiagnosticCode::DuplicateRelationIdentity,
                        detail: "duplicate relation identity in merged plan".to_string(),
                    });
                }
            }
            TransactionIntent::BulkCreateRelations {
                partition_id,
                kind_id,
                endpoints,
                ..
            } => {
                for (source, target) in endpoints {
                    let spec = crate::data::transaction::RelationSpec {
                        partition_id: *partition_id,
                        kind_id: *kind_id,
                        client_key: crate::data::symbols::InternedString::from("bulk"),
                        source: *source,
                        target: *target,
                        payload: None,
                    };
                    if !seen_relation_creates.insert(relation_identity_key(&spec)) {
                        return Err(CommitConflict {
                            code: DiagnosticCode::DuplicateRelationIdentity,
                            detail: "duplicate relation identity in merged plan".to_string(),
                        });
                    }
                }
            }
            TransactionIntent::CreateEntity(_)
            | TransactionIntent::BulkCreateEntities { .. }
            | TransactionIntent::UpdateEntity { .. }
            | TransactionIntent::ReplaceEntity { .. }
            | TransactionIntent::DeleteEntity { .. }
            | TransactionIntent::DeleteRelation { .. } => {}
        }
    }
    Ok(())
}

pub(super) fn entity_exists_in_state(state: &impl PartitionAccess, entity_id: EntityId) -> bool {
    let slot = entity_id.local_slot.0 as usize;
    state.get_partition(entity_id.partition_id).is_some_and(|partition| {
            partition.entity_arena.generations.get(slot) == Some(&entity_id.generation.0)
                && partition.entity_arena.lifecycle.get(slot) == Some(&RecordLifecycleState::Live)
        })
}

pub(super) fn relation_exists_in_state(state: &impl PartitionAccess, relation_id: RelationId) -> bool {
    let slot = relation_id.local_slot.0 as usize;
    state.get_partition(relation_id.partition_id).is_some_and(|partition| {
            partition.relation_arena.generations.get(slot) == Some(&relation_id.generation.0)
                && partition.relation_arena.lifecycle.get(slot)
                    == Some(&RecordLifecycleState::Live)
        })
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

fn relation_identity_key(spec: &crate::data::transaction::RelationSpec) -> String {
    format!(
        "{:08}:{:010}:{:08}:{:020}:{:010}:{:08}:{:020}:{:010}",
        spec.partition_id.0,
        spec.kind_id.0,
        spec.source.partition_id.0,
        spec.source.local_slot.0,
        spec.source.generation.0,
        spec.target.partition_id.0,
        spec.target.local_slot.0,
        spec.target.generation.0,
    )
}
