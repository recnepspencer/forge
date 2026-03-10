use std::collections::{BTreeMap, BTreeSet};

use crate::diagnostics::data::DiagnosticCode;
use crate::logic::runtime::RelationalRuntime;
use crate::storage::data::RecordLifecycleState;
use crate::storage::logic::state::PartitionAccess;
use crate::transactions::data::{MergedCommitPlan, TransactionIntent};
use crate::validation::data::{InvariantClass, InvariantRule, InvariantViolation};

use super::helpers::{
    entity_payload_for_state, entity_visible_at_version, touched_entity_set,
    touched_visible_entity_ids, visible_payload,
};

pub(crate) fn evaluate_rule(
    runtime: &RelationalRuntime,
    state: &impl PartitionAccess,
    version_id: crate::identity::data::VersionId,
    class: InvariantClass,
    rule: &InvariantRule,
    merged_plan: Option<&MergedCommitPlan>,
    violations: &mut Vec<InvariantViolation>,
) {
    match rule {
        InvariantRule::LiveEntityRequiresKind => {
            for partition_id in state.partition_ids() {
                let partition = state
                    .get_partition(partition_id)
                    .expect("partition for invariant scan");
                let slots = state
                    .touched_entity_slots(partition_id)
                    .unwrap_or_else(|| (0..partition.entity_arena.generations.len()).collect());
                runtime
                    .instrumentation
                    .complexity_counters
                    .borrow_mut()
                    .invariant_entity_slot_scans += slots.len();
                for slot in slots {
                    if partition.entity_arena.lifecycle[slot] == RecordLifecycleState::Live
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
                let slots = state
                    .touched_relation_slots(partition_id)
                    .unwrap_or_else(|| (0..partition.relation_arena.generations.len()).collect());
                runtime
                    .instrumentation
                    .complexity_counters
                    .borrow_mut()
                    .invariant_relation_slot_scans += slots.len();
                for slot in slots {
                    if partition.relation_arena.lifecycle[slot] == RecordLifecycleState::Live
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
            if version_id == runtime.current_version_id() {
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
                    runtime
                        .instrumentation
                        .complexity_counters
                        .borrow_mut()
                        .invariant_entity_slot_scans += partition.entity_arena.generations.len();
                    visible_entities += (0..partition.entity_arena.generations.len())
                        .filter(|slot| {
                            entity_visible_at_version(&partition.entity_arena, *slot, version_id)
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
            evaluate_unique_entity_payload_field(
                runtime,
                state,
                version_id,
                class,
                field,
                merged_plan,
                violations,
            );
        }
    }
}

fn evaluate_unique_entity_payload_field(
    runtime: &RelationalRuntime,
    state: &impl PartitionAccess,
    version_id: crate::identity::data::VersionId,
    class: InvariantClass,
    field: &str,
    merged_plan: Option<&MergedCommitPlan>,
    violations: &mut Vec<InvariantViolation>,
) {
    if let Some(planned_values) = planned_entity_field_values(merged_plan, field) {
        let mut planned_value_to_entity = BTreeMap::new();
        for (entity_id, value) in planned_values {
            runtime
                .instrumentation
                .complexity_counters
                .borrow_mut()
                .invariant_entity_slot_scans += 1;
            if let Some(existing_entity_id) =
                planned_value_to_entity.insert(value.clone(), entity_id)
            {
                if existing_entity_id != entity_id || entity_id.is_none() {
                    violations.push(duplicate_field_violation(class, field, &value));
                    continue;
                }
            }
            if runtime
                .indexes
                .entity_unique_field_index
                .get(field)
                .and_then(|values| values.get(&value))
                .is_some_and(|existing| {
                    existing
                        .iter()
                        .any(|existing_id| entity_id != Some(*existing_id))
                })
            {
                violations.push(duplicate_field_violation(class, field, &value));
            }
        }
    } else if let Some(touched_entity_ids) = touched_visible_entity_ids(state, version_id) {
        let mut touched_value_to_entity = BTreeMap::new();
        let touched_set = touched_entity_set(&touched_entity_ids);
        for entity_id in touched_entity_ids {
            runtime
                .instrumentation
                .complexity_counters
                .borrow_mut()
                .invariant_entity_slot_scans += 1;
            let Some(payload) = entity_payload_for_state(state, entity_id, version_id) else {
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
                violations.push(duplicate_field_violation(class, field, value));
                continue;
            }
            if runtime
                .indexes
                .entity_unique_field_index
                .get(field)
                .and_then(|values| values.get(value))
                .is_some_and(|existing| {
                    existing
                        .iter()
                        .any(|existing_id| !touched_set.contains(existing_id))
                })
            {
                violations.push(duplicate_field_violation(class, field, value));
            }
        }
    } else {
        let mut seen = BTreeSet::new();
        for partition_id in state.partition_ids() {
            let partition = state
                .get_partition(partition_id)
                .expect("partition for invariant scan");
            runtime
                .instrumentation
                .complexity_counters
                .borrow_mut()
                .invariant_entity_slot_scans += partition.entity_arena.generations.len();
            for slot in 0..partition.entity_arena.generations.len() {
                if !entity_visible_at_version(&partition.entity_arena, slot, version_id) {
                    continue;
                }
                let Some(payload) =
                    visible_payload(&partition.entity_arena.payload_history[slot], version_id)
                else {
                    continue;
                };
                if let Some(value) = payload
                    .as_json()
                    .and_then(|value| value.get(field))
                    .and_then(|value| value.as_str())
                {
                    if !seen.insert(value.to_string()) {
                        violations.push(duplicate_field_violation(class, field, value));
                    }
                }
            }
        }
    }
}

fn duplicate_field_violation(
    class: InvariantClass,
    field: &str,
    value: &str,
) -> InvariantViolation {
    InvariantViolation {
        class,
        code: DiagnosticCode::InvariantViolation,
        detail: format!("duplicate entity payload field {}={}", field, value),
    }
}

fn planned_entity_field_values(
    merged_plan: Option<&MergedCommitPlan>,
    field: &str,
) -> Option<Vec<(Option<crate::identity::data::EntityId>, String)>> {
    let merged_plan = merged_plan?;
    let mut values = Vec::new();
    let mut saw_entity_change = false;
    for intent in &merged_plan.merged_intents {
        match intent {
            TransactionIntent::CreateEntity(spec) => {
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
            TransactionIntent::BulkCreateEntities { payloads, .. } => {
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
            TransactionIntent::UpdateEntity { entity_id, payload } => {
                saw_entity_change = true;
                if let Some(value) = payload
                    .as_json()
                    .and_then(|value| value.get(field))
                    .and_then(|value| value.as_str())
                {
                    values.push((Some(*entity_id), value.to_string()));
                }
            }
            TransactionIntent::ReplaceEntity {
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
            TransactionIntent::DeleteEntity { .. }
            | TransactionIntent::CreateRelation(_)
            | TransactionIntent::BulkCreateRelations { .. }
            | TransactionIntent::DeleteRelation { .. } => {}
        }
    }
    saw_entity_change.then_some(values)
}
