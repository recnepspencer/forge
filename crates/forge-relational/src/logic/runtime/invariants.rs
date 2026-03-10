use std::collections::{BTreeMap, BTreeSet};

use crate::data::diagnostics::DiagnosticCode;
use crate::data::payload::RecordPayload;
use crate::data::schema::SchemaRegistryError;
use crate::data::transaction::{CommitConflict, MergedCommitPlan};
use crate::logic::runtime::{
    InvariantCheckResult, InvariantClass, InvariantExecutionPoint, InvariantFailureEffect,
    InvariantRule, InvariantViolation, RecordLifecycleState, RelationalRuntime,
};

use super::state::PartitionAccess;

impl RelationalRuntime {
    pub(super) fn run_invariants_for_state(
        &self,
        state: &impl PartitionAccess,
        version_id: crate::data::identity::VersionId,
        execution_point: InvariantExecutionPoint,
        include_harness_heavy: bool,
        merged_plan: Option<&MergedCommitPlan>,
    ) -> Vec<InvariantCheckResult> {
        let mut results = Vec::new();
        let groups = match execution_point {
            InvariantExecutionPoint::MutationSensitive => vec![(
                InvariantClass::AlwaysOnStructural,
                InvariantFailureEffect::BlockCommit,
                &self.config.invariant_catalog.always_on_structural,
            )],
            InvariantExecutionPoint::CommitBoundary => vec![(
                InvariantClass::CommitBoundary,
                InvariantFailureEffect::BlockCommit,
                &self.config.invariant_catalog.commit_boundary,
            )],
            InvariantExecutionPoint::SnapshotPublication => vec![(
                InvariantClass::SnapshotAudit,
                InvariantFailureEffect::BlockPublication,
                &self.config.invariant_catalog.snapshot_audit,
            )],
            InvariantExecutionPoint::HarnessAudit => {
                if include_harness_heavy {
                    vec![(
                        InvariantClass::HarnessHeavy,
                        InvariantFailureEffect::AuditOnly,
                        &self.config.invariant_catalog.harness_heavy,
                    )]
                } else {
                    Vec::new()
                }
            }
        };

        for (class, failure_effect, rules) in groups {
            let mut violations = Vec::new();
            for rule in rules {
                match rule {
                    InvariantRule::LiveEntityRequiresKind => {
                        for partition_id in state.partition_ids() {
                            let partition = state
                                .get_partition(partition_id)
                                .expect("partition for invariant scan");
                            let slots = state.touched_entity_slots(partition_id).unwrap_or_else(|| {
                                (0..partition.entity_arena.generations.len()).collect()
                            });
                            self.complexity_counters
                                .borrow_mut()
                                .invariant_entity_slot_scans += slots.len();
                            for slot in slots {
                                if partition.entity_arena.lifecycle[slot]
                                    == RecordLifecycleState::Live
                                    && partition.entity_arena.kind_ids[slot].is_none()
                                {
                                    violations.push(InvariantViolation {
                                        class,
                                        code: DiagnosticCode::SidecarConsistencyFailure,
                                        detail: format!(
                                            "live entity slot {} in partition {} missing kind id",
                                            slot, partition.partition_id.0
                                        ),
                                    });
                                }
                            }
                        }
                    }
                    InvariantRule::LiveRelationRequiresEndpoints => {
                        for partition_id in state.partition_ids() {
                            let partition = state
                                .get_partition(partition_id)
                                .expect("partition for invariant scan");
                            let slots = state.touched_relation_slots(partition_id).unwrap_or_else(|| {
                                (0..partition.relation_arena.generations.len()).collect()
                            });
                            self.complexity_counters
                                .borrow_mut()
                                .invariant_relation_slot_scans += slots.len();
                            for slot in slots {
                                if partition.relation_arena.lifecycle[slot]
                                    == RecordLifecycleState::Live
                                    && partition.relation_arena.endpoints[slot].is_none()
                                {
                                    violations.push(InvariantViolation {
                                        class,
                                        code: DiagnosticCode::SidecarConsistencyFailure,
                                        detail: format!(
                                            "live relation slot {} in partition {} missing endpoints",
                                            slot, partition.partition_id.0
                                        ),
                                    });
                                }
                            }
                        }
                    }
                    InvariantRule::MaxMergedIntents(limit) => {
                        let merged_len = merged_plan
                            .map(|plan| plan.merged_intents.len())
                            .unwrap_or(0);
                        if merged_len > *limit {
                            violations.push(InvariantViolation {
                                class,
                                code: DiagnosticCode::InvariantViolation,
                                detail: format!(
                                    "merged commit plan has {} intents, limit is {}",
                                    merged_len, limit
                                ),
                            });
                        }
                    }
                    InvariantRule::MaxSnapshotEntities(limit) => {
                        let mut visible_entities = 0;
                        if version_id == self.current_version_id() {
                            for partition_id in state.partition_ids() {
                                let partition = state
                                    .get_partition(partition_id)
                                    .expect("partition for invariant scan");
                                visible_entities += partition.entity_arena.live_bitset.count_ones();
                            }
                        } else {
                            for partition_id in state.partition_ids() {
                                let partition = state
                                    .get_partition(partition_id)
                                    .expect("partition for invariant scan");
                                self.complexity_counters
                                    .borrow_mut()
                                    .invariant_entity_slot_scans +=
                                    partition.entity_arena.generations.len();
                                visible_entities += (0..partition.entity_arena.generations.len())
                                    .filter(|slot| {
                                        entity_visible_at_version(
                                            &partition.entity_arena,
                                            *slot,
                                            version_id,
                                        )
                                    })
                                    .count();
                            }
                        }
                        if visible_entities > *limit {
                            violations.push(InvariantViolation {
                                class,
                                code: DiagnosticCode::InvariantViolation,
                                detail: format!(
                                    "snapshot at version {} has {} entities, limit is {}",
                                    version_id.0, visible_entities, limit
                                ),
                            });
                        }
                    }
                    InvariantRule::UniqueEntityPayloadField(field) => {
                        if let Some(planned_values) =
                            planned_entity_field_values(merged_plan, field)
                        {
                            let mut planned_value_to_entity = BTreeMap::new();
                            for (entity_id, value) in planned_values {
                                self.complexity_counters.borrow_mut().invariant_entity_slot_scans += 1;
                                if let Some(existing_entity_id) = planned_value_to_entity
                                    .insert(value.clone(), entity_id)
                                {
                                    if existing_entity_id != entity_id || entity_id.is_none() {
                                        violations.push(InvariantViolation {
                                            class,
                                            code: DiagnosticCode::InvariantViolation,
                                            detail: format!(
                                                "duplicate entity payload field {}={}",
                                                field, value
                                            ),
                                        });
                                        continue;
                                    }
                                }
                                if self
                                    .entity_unique_field_index
                                    .get(field)
                                    .and_then(|values| values.get(&value))
                                    .is_some_and(|existing| {
                                        existing.iter().any(|existing_id| {
                                            entity_id != Some(*existing_id)
                                        })
                                    })
                                {
                                    violations.push(InvariantViolation {
                                        class,
                                        code: DiagnosticCode::InvariantViolation,
                                        detail: format!(
                                            "duplicate entity payload field {}={}",
                                            field, value
                                        ),
                                    });
                                }
                            }
                        } else if let Some(touched_entity_ids) =
                            touched_visible_entity_ids(state, version_id)
                        {
                            let mut touched_value_to_entity = BTreeMap::new();
                            let touched_set = touched_entity_ids.iter().copied().collect::<BTreeSet<_>>();
                            for entity_id in touched_entity_ids {
                                self.complexity_counters.borrow_mut().invariant_entity_slot_scans += 1;
                                let Some(payload) =
                                    entity_payload_for_state(state, entity_id, version_id)
                                else {
                                    continue;
                                };
                                let Some(value) = payload
                                    .as_json()
                                    .and_then(|value| value.get(field))
                                    .and_then(|value| value.as_str())
                                else {
                                    continue;
                                };
                                if touched_value_to_entity
                                    .insert(value.to_string(), entity_id)
                                    .is_some()
                                {
                                    violations.push(InvariantViolation {
                                        class,
                                        code: DiagnosticCode::InvariantViolation,
                                        detail: format!(
                                            "duplicate entity payload field {}={}",
                                            field, value
                                        ),
                                    });
                                    continue;
                                }
                                if self
                                    .entity_unique_field_index
                                    .get(field)
                                    .and_then(|values| values.get(value))
                                    .is_some_and(|existing| {
                                        existing
                                            .iter()
                                            .any(|existing_id| !touched_set.contains(existing_id))
                                    })
                                {
                                    violations.push(InvariantViolation {
                                        class,
                                        code: DiagnosticCode::InvariantViolation,
                                        detail: format!(
                                            "duplicate entity payload field {}={}",
                                            field, value
                                        ),
                                    });
                                }
                            }
                        } else {
                            let mut seen = BTreeSet::new();
                            for partition_id in state.partition_ids() {
                                let partition = state
                                    .get_partition(partition_id)
                                    .expect("partition for invariant scan");
                                self.complexity_counters
                                    .borrow_mut()
                                    .invariant_entity_slot_scans +=
                                    partition.entity_arena.generations.len();
                                for slot in 0..partition.entity_arena.generations.len() {
                                    if !entity_visible_at_version(
                                        &partition.entity_arena,
                                        slot,
                                        version_id,
                                    ) {
                                        continue;
                                    }
                                    let Some(payload) = visible_payload(
                                        &partition.entity_arena.payload_history[slot],
                                        version_id,
                                    ) else {
                                        continue;
                                    };
                                    if let Some(value) = payload
                                        .as_json()
                                        .and_then(|value| value.get(field))
                                        .and_then(|value| value.as_str())
                                    {
                                        if !seen.insert(value.to_string()) {
                                            violations.push(InvariantViolation {
                                                class,
                                                code: DiagnosticCode::InvariantViolation,
                                                detail: format!(
                                                    "duplicate entity payload field {}={}",
                                                    field, value
                                                ),
                                            });
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            }
            if !violations.is_empty() || !rules.is_empty() {
                results.push(InvariantCheckResult {
                    class,
                    execution_point,
                    failure_effect,
                    violations,
                });
            }
        }

        results
    }
}

pub(super) fn touched_visible_entity_ids(
    state: &impl PartitionAccess,
    version_id: crate::data::identity::VersionId,
) -> Option<Vec<crate::data::identity::EntityId>> {
    let mut ids = Vec::new();
    let mut saw_any = false;
    for partition_id in state.partition_ids() {
        let partition = state.get_partition(partition_id)?;
        let Some(slots) = state.touched_entity_slots(partition_id) else {
            continue;
        };
        saw_any = true;
        for slot in slots {
            if slot >= partition.entity_arena.generations.len()
                || !entity_visible_at_version(&partition.entity_arena, slot, version_id)
            {
                continue;
            }
            ids.push(crate::data::identity::EntityId::new(
                partition_id,
                slot as u64,
                partition.entity_arena.generations[slot],
            ));
        }
    }
    if saw_any {
        Some(ids)
    } else {
        None
    }
}

fn planned_entity_field_values(
    merged_plan: Option<&MergedCommitPlan>,
    field: &str,
) -> Option<Vec<(Option<crate::data::identity::EntityId>, String)>> {
    let merged_plan = merged_plan?;
    let mut values = Vec::new();
    let mut saw_entity_change = false;
    for intent in &merged_plan.merged_intents {
        match intent {
            crate::data::transaction::TransactionIntent::CreateEntity(spec) => {
                saw_entity_change = true;
                if let Some(value) = spec
                    .payload
                    .as_json()
                    .and_then(|value| value.get(field))
                    .and_then(|value| value.as_str())
                {
                    values.push((None, value.to_string()));
                }
            }
            crate::data::transaction::TransactionIntent::BulkCreateEntities { payloads, .. } => {
                saw_entity_change = true;
                for payload in payloads {
                    if let Some(value) = payload
                        .as_json()
                        .and_then(|value| value.get(field))
                        .and_then(|value| value.as_str())
                    {
                        values.push((None, value.to_string()));
                    }
                }
            }
            crate::data::transaction::TransactionIntent::UpdateEntity { entity_id, payload } => {
                saw_entity_change = true;
                if let Some(value) = payload
                    .as_json()
                    .and_then(|value| value.get(field))
                    .and_then(|value| value.as_str())
                {
                    values.push((Some(*entity_id), value.to_string()));
                }
            }
            crate::data::transaction::TransactionIntent::ReplaceEntity {
                entity_id,
                replacement,
            } => {
                saw_entity_change = true;
                if let Some(value) = replacement
                    .payload
                    .as_json()
                    .and_then(|value| value.get(field))
                    .and_then(|value| value.as_str())
                {
                    values.push((Some(*entity_id), value.to_string()));
                }
            }
            crate::data::transaction::TransactionIntent::DeleteEntity { .. }
            | crate::data::transaction::TransactionIntent::CreateRelation(_)
            | crate::data::transaction::TransactionIntent::BulkCreateRelations { .. }
            | crate::data::transaction::TransactionIntent::DeleteRelation { .. } => {}
        }
    }
    saw_entity_change.then_some(values)
}

pub(super) fn entity_payload_for_state(
    state: &impl PartitionAccess,
    entity_id: crate::data::identity::EntityId,
    version_id: crate::data::identity::VersionId,
) -> Option<&RecordPayload> {
    let partition = state.get_partition(entity_id.partition_id)?;
    let slot = entity_id.local_slot.0 as usize;
    if slot >= partition.entity_arena.payload_history.len() {
        return None;
    }
    visible_payload(&partition.entity_arena.payload_history[slot], version_id)
}

fn visible_payload(
    history: &[super::state::VersionedValue],
    version_id: crate::data::identity::VersionId,
) -> Option<&RecordPayload> {
    history
        .iter()
        .find(|entry| {
            entry.effective_at <= version_id
                && entry.retired_at.is_none_or(|retired| version_id < retired)
        })
        .map(|entry| &entry.value)
}

fn entity_visible_at_version(
    arena: &super::state::EntityArena,
    slot: usize,
    version_id: crate::data::identity::VersionId,
) -> bool {
    arena.lifecycle[slot] != RecordLifecycleState::Reusable
        && arena.created_at[slot] <= version_id
        && arena.retired_at[slot].is_none_or(|retired| version_id < retired)
}

pub(super) fn first_blocking_invariant_error(
    results: &[InvariantCheckResult],
) -> Option<CommitConflict> {
    results
        .iter()
        .find(|result| {
            result.failure_effect == InvariantFailureEffect::BlockCommit
                && !result.violations.is_empty()
        })
        .and_then(|result| result.violations.first())
        .map(|violation| CommitConflict {
            code: violation.code,
            detail: violation.detail.clone(),
        })
}

pub(super) fn first_publication_invariant_error(
    results: &[InvariantCheckResult],
) -> Option<CommitConflict> {
    results
        .iter()
        .find(|result| {
            result.failure_effect == InvariantFailureEffect::BlockPublication
                && !result.violations.is_empty()
        })
        .and_then(|result| result.violations.first())
        .map(|violation| CommitConflict {
            code: violation.code,
            detail: violation.detail.clone(),
        })
}

pub(super) fn schema_error_to_commit_conflict(error: SchemaRegistryError) -> CommitConflict {
    CommitConflict {
        code: DiagnosticCode::InvariantViolation,
        detail: format!("{error:?}"),
    }
}
