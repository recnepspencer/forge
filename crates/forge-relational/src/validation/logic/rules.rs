use std::collections::{BTreeMap, BTreeSet};

use crate::diagnostics::data::DiagnosticCode;
use crate::logic::runtime::RelationalRuntime;
use crate::storage::data::RecordLifecycleState;
use crate::storage::logic::state::{
    EntityRecordKind, PartitionAccess, RecordKind, RelationRecordKind, SlotView,
};
use crate::transactions::data::MergedCommitPlan;
use crate::validation::data::{InvariantClass, InvariantRule, InvariantViolation, RecordKindTag};

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
        InvariantRule::LiveRecordRequiresSidecar(kind) => {
            evaluate_live_record_sidecar_rule(runtime, state, class, kind, violations);
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
                    runtime.services.instrumentation.count(|counters| {
                        counters.invariant_entity_slot_scans += partition.entity_arena.slot_count();
                    });
                    visible_entities += (0..partition.entity_arena.slot_count())
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

fn evaluate_live_record_sidecar_rule(
    runtime: &RelationalRuntime,
    state: &impl PartitionAccess,
    class: InvariantClass,
    kind: &RecordKindTag,
    violations: &mut Vec<InvariantViolation>,
) {
    match kind {
        RecordKindTag::Entity => evaluate_live_record_sidecar::<EntityRecordKind>(
            runtime,
            state,
            class,
            violations,
            |state, partition_id| state.touched_entity_slots(partition_id),
            |slot_view| slot_view.kind_id().is_some(),
            "kind id",
            |runtime, slots| {
                runtime
                    .services
                    .instrumentation
                    .count(|counters| counters.invariant_entity_slot_scans += slots);
            },
        ),
        RecordKindTag::Relation => evaluate_live_record_sidecar::<RelationRecordKind>(
            runtime,
            state,
            class,
            violations,
            |state, partition_id| state.touched_relation_slots(partition_id),
            |slot_view| slot_view.extra().is_some(),
            "endpoints",
            |runtime, slots| {
                runtime
                    .services
                    .instrumentation
                    .count(|counters| counters.invariant_relation_slot_scans += slots);
            },
        ),
    }
}

fn evaluate_live_record_sidecar<K: RecordKind>(
    runtime: &RelationalRuntime,
    state: &impl PartitionAccess,
    class: InvariantClass,
    violations: &mut Vec<InvariantViolation>,
    touched_slots: impl Fn(&dyn PartitionAccess, crate::identity::data::PartitionId) -> Option<Vec<usize>>,
    has_required_sidecar: impl Fn(&SlotView<'_, K>) -> bool,
    missing_label: &str,
    count_scans: impl Fn(&RelationalRuntime, usize),
) {
    for partition_id in state.partition_ids() {
        let partition = state
            .get_partition(partition_id)
            .expect("partition for invariant scan");
        let slots = touched_slots(state, partition_id)
            .unwrap_or_else(|| (0..K::arena(partition).slot_count()).collect());
        count_scans(runtime, slots.len());
        for slot in slots {
            let Some(slot_view) = K::arena(partition).get_slot(slot) else {
                continue;
            };
            if slot_view.lifecycle() == RecordLifecycleState::Live
                && !has_required_sidecar(&slot_view)
            {
                violations.push(InvariantViolation {
                    class,
                    code: DiagnosticCode::SidecarConsistencyFailure,
                    detail: format!(
                        "live slot {} in partition {} missing {}",
                        slot, partition.partition_id.0, missing_label
                    ),
                });
            }
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
                .services
                .instrumentation
                .complexity_counters
                .lock()
                .expect("complexity counter lock poisoned")
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
                .services
                .instrumentation
                .complexity_counters
                .lock()
                .expect("complexity counter lock poisoned")
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
                .services
                .instrumentation
                .complexity_counters
                .lock()
                .expect("complexity counter lock poisoned")
                .invariant_entity_slot_scans += partition.entity_arena.slot_count();
            for slot in 0..partition.entity_arena.slot_count() {
                if !entity_visible_at_version(&partition.entity_arena, slot, version_id) {
                    continue;
                }
                let Some(payload) = partition
                    .entity_arena
                    .payload_history_at(slot)
                    .and_then(|history| visible_payload(history, version_id))
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
        if intent.collect_planned_entity_field_values(field, &mut values) {
            saw_entity_change = true;
        }
    }
    saw_entity_change.then_some(values)
}
