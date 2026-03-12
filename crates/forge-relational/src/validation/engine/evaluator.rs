use std::collections::{HashMap, HashSet};

use crate::diagnostics::data::DiagnosticCode;
use crate::storage::data::RecordLifecycleState;
use crate::storage::logic::state::{
    EntityRecordKind, RecordKind, RelationRecordKind, SlotView,
};
use crate::transactions::data::MergedCommitPlan;
use crate::validation::data::{InvariantClass, InvariantRule, InvariantViolation, RecordKindTag};

use super::context::InvariantExecutionContext;
use super::state_view::InvariantStateView;

pub(crate) fn evaluate_rule(
    context: &InvariantExecutionContext<'_>,
    class: InvariantClass,
    rule: &InvariantRule,
)-> Option<InvariantViolation> {
    match rule {
        InvariantRule::LiveRecordRequiresSidecar(kind) => {
            evaluate_live_record_sidecar_rule(context, class, kind)
        }
        InvariantRule::MaxMergedIntents(limit) => {
            let merged_len = context
                .merged_plan
                .map(|plan| plan.merged_intents.len())
                .unwrap_or(0);
            if merged_len > *limit {
                Some(InvariantViolation {
                    class,
                    code: DiagnosticCode::InvariantViolation,
                    detail: format!(
                        "merged commit plan has {} intents, limit is {}",
                        merged_len, limit
                    ),
                })
            } else {
                None
            }
        }
        InvariantRule::MaxSnapshotEntities(limit) => {
            evaluate_max_snapshot_entities(context, class, *limit)
        }
        InvariantRule::UniqueEntityPayloadField(field) => {
            evaluate_unique_entity_payload_field(context, class, field)
        }
    }
}

fn evaluate_live_record_sidecar_rule(
    context: &InvariantExecutionContext<'_>,
    class: InvariantClass,
    kind: &RecordKindTag,
)-> Option<InvariantViolation> {
    match kind {
        RecordKindTag::Entity => evaluate_live_record_sidecar::<EntityRecordKind>(
            context,
            class,
            |state, partition_id| state.touched_entity_slots(partition_id),
            |slot_view| slot_view.kind_id().is_some(),
            "kind id",
            |context, slots| {
                context.metrics().count_entity_slot_scans(slots);
            },
        ),
        RecordKindTag::Relation => evaluate_live_record_sidecar::<RelationRecordKind>(
            context,
            class,
            |state, partition_id| state.touched_relation_slots(partition_id),
            |slot_view| slot_view.extra().is_some(),
            "endpoints",
            |context, slots| {
                context.metrics().count_relation_slot_scans(slots);
            },
        ),
    }
}

fn evaluate_live_record_sidecar<K: RecordKind>(
    context: &InvariantExecutionContext<'_>,
    class: InvariantClass,
    touched_slots: impl Fn(
        &dyn crate::logic::runtime::PartitionAccess,
        crate::identity::data::PartitionId,
    ) -> Option<Vec<usize>>,
    has_required_sidecar: impl Fn(&SlotView<'_, K>) -> bool,
    missing_label: &str,
    count_scans: impl Fn(&InvariantExecutionContext<'_>, usize),
)-> Option<InvariantViolation> {
    for partition_id in context.state.partition_ids() {
        let partition = context
            .state
            .get_partition(partition_id)
            .expect("partition for invariant scan");
        if let Some(slots) = touched_slots(context.state, partition_id) {
            count_scans(context, slots.len());
            for slot in slots {
                if let Some(violation) = sidecar_violation_for_slot(
                    class,
                    partition,
                    slot,
                    &has_required_sidecar,
                    missing_label,
                ) {
                    return Some(violation);
                }
            }
        } else {
            let arena = K::arena(partition);
            count_scans(context, arena.slot_count());
            for slot in 0..arena.slot_count() {
                if let Some(violation) = sidecar_violation_for_slot(
                    class,
                    partition,
                    slot,
                    &has_required_sidecar,
                    missing_label,
                ) {
                    return Some(violation);
                }
            }
        }
    }
    None
}

fn sidecar_violation_for_slot<K: RecordKind>(
    class: InvariantClass,
    partition: &crate::storage::logic::state::PartitionState,
    slot: usize,
    has_required_sidecar: &impl Fn(&SlotView<'_, K>) -> bool,
    missing_label: &str,
) -> Option<InvariantViolation> {
    let slot_view = K::arena(partition).get_slot(slot)?;
    if slot_view.lifecycle() == RecordLifecycleState::Live && !has_required_sidecar(&slot_view) {
        return Some(InvariantViolation {
            class,
            code: DiagnosticCode::SidecarConsistencyFailure,
            detail: format!(
                "live slot {} in partition {} missing {}",
                slot, partition.partition_id.0, missing_label
            ),
        });
    }
    None
}

fn evaluate_max_snapshot_entities(
    context: &InvariantExecutionContext<'_>,
    class: InvariantClass,
    limit: usize,
)-> Option<InvariantViolation> {
    let state_view = context.state_view();
    let mut visible_entities = 0;
    if state_view.version_id() == context.current_version_id {
        for partition_id in state_view.state().partition_ids() {
            let partition = state_view
                .state()
                .get_partition(partition_id)
                .expect("partition for invariant scan");
            visible_entities += partition.entity_arena.live_bitset.count_ones();
        }
    } else {
        for partition_id in state_view.state().partition_ids() {
            let partition = state_view
                .state()
                .get_partition(partition_id)
                .expect("partition for invariant scan");
            context
                .metrics()
                .count_entity_slot_scans(partition.entity_arena.slot_count());
            visible_entities += (0..partition.entity_arena.slot_count())
                .filter(|slot| state_view.entity_visible_at_version(&partition.entity_arena, *slot))
                .count();
        }
    }
    if visible_entities > limit {
        return Some(InvariantViolation {
            class,
            code: DiagnosticCode::InvariantViolation,
            detail: format!(
                "snapshot at version {} has {} entities, limit is {}",
                state_view.version_id().0,
                visible_entities,
                limit
            ),
        });
    }
    None
}

fn evaluate_unique_entity_payload_field(
    context: &InvariantExecutionContext<'_>,
    class: InvariantClass,
    field: &str,
)-> Option<InvariantViolation> {
    let state_view = context.state_view();
    if let Some(violation) = planned_unique_entity_payload_violation(context, class, field) {
        return Some(violation);
    } else if let Some(touched_entity_ids) = state_view.touched_visible_entity_ids() {
        let mut touched_value_to_entity = HashMap::with_capacity(touched_entity_ids.len());
        let touched_set = InvariantStateView::touched_entity_set(&touched_entity_ids);
        for entity_id in touched_entity_ids {
            context.metrics().count_entity_slot_scans(1);
            let Some(payload) = state_view.entity_payload(entity_id) else {
                continue;
            };
            let Some(value) = payload
                .as_json()
                .and_then(|value| value.get(field))
                .and_then(|value| value.as_str())
            else {
                continue;
            };
            if touched_value_to_entity.insert(value.to_owned(), entity_id).is_some() {
                return Some(duplicate_field_violation(class, field, value));
            }
            if context
                .indexes()
                .conflicts_with_entity_value_outside(field, value, &touched_set)
            {
                return Some(duplicate_field_violation(class, field, value));
            }
        }
    } else if state_view.version_id() == context.current_version_id {
        if let Some(value) = context.indexes().first_duplicate_entity_value(field) {
            return Some(duplicate_field_violation(class, field, value));
        }
    } else {
        let mut seen = HashSet::new();
        for partition_id in state_view.state().partition_ids() {
            let partition = state_view
                .state()
                .get_partition(partition_id)
                .expect("partition for invariant scan");
            for slot in 0..partition.entity_arena.slot_count() {
                context.metrics().count_entity_slot_scans(1);
                let Some(payload) = partition
                    .entity_arena
                    .payload_history_at(slot)
                    .and_then(|history| state_view.visible_payload(history))
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
                if !seen.insert(value.to_string()) {
                    return Some(duplicate_field_violation(class, field, value));
                }
            }
        }
    }
    None
}

fn planned_entity_field_values(
    merged_plan: Option<&MergedCommitPlan>,
    field: &str,
) -> Option<Vec<(Option<crate::identity::data::EntityId>, String)>> {
    let plan = merged_plan?;
    let mut values = Vec::new();
    for intent in &plan.merged_intents {
        match intent {
            crate::transactions::data::MutationIntent::Create(
                crate::transactions::data::CreateIntent::Entity(create),
            ) => {
                let Some(value) = payload_field_value(&create.payload, field) else {
                    continue;
                };
                values.push((None, value));
            }
            crate::transactions::data::MutationIntent::Create(
                crate::transactions::data::CreateIntent::BulkEntities(bulk),
            ) => {
                for payload in &bulk.payloads {
                    let Some(value) = payload_field_value(payload, field) else {
                        continue;
                    };
                    values.push((None, value));
                }
            }
            crate::transactions::data::MutationIntent::Entity(
                crate::transactions::data::EntityMutationIntent::Update(update),
            ) => {
                let Some(value) = payload_field_value(&update.payload, field) else {
                    continue;
                };
                values.push((Some(update.entity_id), value));
            }
            crate::transactions::data::MutationIntent::Entity(
                crate::transactions::data::EntityMutationIntent::Replace(replace),
            ) => {
                let Some(value) = payload_field_value(&replace.replacement.payload, field) else {
                    continue;
                };
                values.push((Some(replace.entity_id), value));
            }
            _ => {}
        }
    }
    Some(values)
}

fn planned_unique_entity_payload_violation(
    context: &InvariantExecutionContext<'_>,
    class: InvariantClass,
    field: &str,
) -> Option<InvariantViolation> {
    let planned_values = planned_entity_field_values(context.merged_plan, field)?;
    let mut planned_value_to_entity = HashMap::with_capacity(planned_values.len());
    for (entity_id, value) in planned_values {
        context.metrics().count_entity_slot_scans(1);
        if let Some(existing_entity_id) = planned_value_to_entity.insert(value.clone(), entity_id) {
            if existing_entity_id != entity_id || entity_id.is_none() {
                return Some(duplicate_field_violation(class, field, &value));
            }
        }
        if context
            .indexes()
            .conflicts_with_entity_value(field, &value, entity_id)
        {
            return Some(duplicate_field_violation(class, field, &value));
        }
    }
    None
}

fn payload_field_value(
    payload: &crate::payloads::data::RecordPayload,
    field: &str,
) -> Option<String> {
    payload
        .as_json()?
        .get(field)
        .and_then(|value| value.as_str())
        .map(ToOwned::to_owned)
}

fn duplicate_field_violation(
    class: InvariantClass,
    field: &str,
    value: &str,
) -> InvariantViolation {
    InvariantViolation {
        class,
        code: DiagnosticCode::InvariantViolation,
        detail: format!(
            "entity payload field '{}' must be unique, duplicate value '{}'",
            field, value
        ),
    }
}
