use std::collections::BTreeSet;

use crate::logic::runtime::PartitionAccess;
use crate::logic::runtime::RelationalRuntime;
use crate::payloads::data::RecordPayload;

pub(crate) fn refresh_unique_field_index_for_records(
    runtime: &mut RelationalRuntime,
    changed_records: &[crate::transactions::data::RecordRef],
    version_id: crate::identity::data::VersionId,
) {
    let tracked_fields = tracked_unique_entity_fields(runtime);
    if tracked_fields.is_empty() {
        return;
    }
    let state = runtime.storage_access().current_state();
    let mut refreshed_values = Vec::new();
    for record in changed_records {
        let crate::transactions::data::RecordRef::Entity(entity_id) = record else {
            continue;
        };
        for field in &tracked_fields {
            if let Some(payload) = entity_payload_for_state(&state, *entity_id, version_id) {
                if let Some(value) = payload
                    .as_json()
                    .and_then(|value| value.get(field))
                    .and_then(|value| value.as_str())
                {
                    refreshed_values.push((field.clone(), value.to_string(), *entity_id));
                }
            }
        }
    }
    for record in changed_records {
        let crate::transactions::data::RecordRef::Entity(entity_id) = record else {
            continue;
        };
        for field in &tracked_fields {
            if let Some(values) = runtime.indexes.entity_unique_field_index.get_mut(field) {
                values.retain(|_, entity_ids| {
                    entity_ids.remove(entity_id);
                    !entity_ids.is_empty()
                });
            }
        }
    }
    for (field, value, entity_id) in refreshed_values {
        runtime
            .indexes
            .entity_unique_field_index
            .entry(field)
            .or_default()
            .entry(value)
            .or_default()
            .insert(entity_id);
    }
}

pub(crate) fn rebuild_unique_field_indexes(runtime: &mut RelationalRuntime) {
    runtime.indexes.entity_unique_field_index.clear();
    let tracked_fields = tracked_unique_entity_fields(runtime);
    if tracked_fields.is_empty() {
        return;
    }
    let state = runtime.storage_access().current_state();
    let version_id = runtime.current_version_id();
    let mut rebuilt_values = Vec::new();
    for partition_id in state.partition_ids() {
        let partition = state
            .get_partition(partition_id)
            .expect("partition for unique field rebuild");
        for slot in 0..partition.entity_arena.slot_count() {
            let Some(slot_view) = partition.entity_arena.get_slot(slot) else {
                continue;
            };
            if slot_view.lifecycle() == crate::storage::data::RecordLifecycleState::Reusable {
                continue;
            }
            let entity_id = crate::identity::data::EntityId::new(
                partition_id,
                slot as u64,
                slot_view.generation(),
            );
            if let Some(payload) = entity_payload_for_state(&state, entity_id, version_id) {
                for field in &tracked_fields {
                    if let Some(value) = payload
                        .as_json()
                        .and_then(|value| value.get(field))
                        .and_then(|value| value.as_str())
                    {
                        rebuilt_values.push((field.clone(), value.to_string(), entity_id));
                    }
                }
            }
        }
    }
    for (field, value, entity_id) in rebuilt_values {
        runtime
            .indexes
            .entity_unique_field_index
            .entry(field)
            .or_default()
            .entry(value)
            .or_default()
            .insert(entity_id);
    }
}

fn entity_payload_for_state(
    state: &dyn PartitionAccess,
    entity_id: crate::identity::data::EntityId,
    version_id: crate::identity::data::VersionId,
) -> Option<&RecordPayload> {
    let partition = state.get_partition(entity_id.partition_id)?;
    let slot = entity_id.local_slot.0 as usize;
    if partition
        .entity_arena
        .get(&entity_id)
        .map(|slot_view| slot_view.generation())
        != Some(entity_id.generation.0)
    {
        return None;
    }
    let history = partition.entity_arena.payload_history_at(slot)?;
    let end = history.partition_point(|entry| entry.effective_at <= version_id);
    history[..end]
        .iter()
        .rev()
        .find(|entry| {
            entry.effective_at <= version_id
                && entry.retired_at.is_none_or(|retired| version_id < retired)
        })
        .map(|entry| &entry.value)
}

fn tracked_unique_entity_fields(runtime: &RelationalRuntime) -> BTreeSet<String> {
    let mut fields = BTreeSet::new();
    for registration in &runtime.config.schema.invariant_catalog.registrations {
        if let crate::validation::data::InvariantRule::UniqueEntityPayloadField(field) =
            &registration.rule
        {
            fields.insert(field.clone());
        }
    }
    fields
}

pub(super) fn payload_field_key(payload: &RecordPayload, field: &str) -> Option<String> {
    payload.as_json()?.get(field).map(|value| match value {
        serde_json::Value::String(text) => text.clone(),
        other => other.to_string(),
    })
}

pub(super) fn payload_field_key_optional(
    payload: &Option<RecordPayload>,
    field: &str,
) -> Option<String> {
    payload
        .as_ref()
        .and_then(|payload| payload_field_key(payload, field))
}
