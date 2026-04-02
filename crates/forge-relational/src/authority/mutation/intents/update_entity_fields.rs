use crate::authority::mutation::outcomes::MutationOutcome;
use crate::authority::mutation::stale_targets::ensure_entity_target_is_current;
use crate::authority::mutation::MutationWorkspace;
use crate::payloads::data::{canonicalize_json, RecordPayload};
use crate::storage::logic::state::PartitionAccess;
use crate::transactions::data::{CommitConflict, ConflictClass, UpdateEntityFieldsIntent};
use serde_json::Value;

pub(super) fn apply(
    intent: &UpdateEntityFieldsIntent,
    workspace: &mut MutationWorkspace<'_>,
) -> Result<MutationOutcome, CommitConflict> {
    let version_id = workspace.version_id();
    let slot = intent.entity_id.local_slot.0 as usize;
    let (kind_id, old_payload, new_payload) = workspace.with_context(|context| {
        ensure_entity_target_is_current(context.state, intent.entity_id)?;
        let partition = context
            .state
            .get_partition(intent.entity_id.partition_id)
            .ok_or_else(|| {
                CommitConflict::new(ConflictClass::MutationStateInconsistency {
                    detail:
                        "entity field update requires an existing partition after stale-target validation"
                            .to_string(),
                    fields: serde_json::json!({
                        "record_class": "entity",
                        "entity_id": intent.entity_id,
                        "phase": "update_entity_fields",
                        "missing": "partition",
                    }),
                })
            })?;
        let slot_view = partition.entity_arena.get_slot(slot).ok_or_else(|| {
            CommitConflict::new(ConflictClass::MutationStateInconsistency {
                detail: "entity field update requires an existing slot after stale-target validation"
                    .to_string(),
                fields: serde_json::json!({
                    "record_class": "entity",
                    "entity_id": intent.entity_id,
                    "phase": "update_entity_fields",
                    "missing": "slot",
                }),
            })
        })?;
        let kind_id = slot_view.kind_id().ok_or_else(|| {
            CommitConflict::new(ConflictClass::MutationStateInconsistency {
                detail: "entity field update requires a retained kind id after stale-target validation"
                    .to_string(),
                fields: serde_json::json!({
                    "record_class": "entity",
                    "entity_id": intent.entity_id,
                    "phase": "update_entity_fields",
                    "missing": "kind_id",
                }),
            })
        })?;
        let old_payload = slot_view.payload().cloned().ok_or_else(|| {
            CommitConflict::new(ConflictClass::MutationStateInconsistency {
                detail: "entity field update requires a retained payload after stale-target validation"
                    .to_string(),
                fields: serde_json::json!({
                    "record_class": "entity",
                    "entity_id": intent.entity_id,
                    "phase": "update_entity_fields",
                    "missing": "payload",
                }),
            })
        })?;
        let existing_json = old_payload.as_json().ok_or_else(|| {
            CommitConflict::new(ConflictClass::MutationStateInconsistency {
                detail: "entity field update requires a structured-json payload".to_string(),
                fields: serde_json::json!({
                    "record_class": "entity",
                    "entity_id": intent.entity_id,
                    "phase": "update_entity_fields",
                    "payload_class": "non_structured_json",
                }),
            })
        })?;
        let mut object = match existing_json {
            Value::Object(map) => map.clone(),
            _ => {
                return Err(CommitConflict::new(ConflictClass::MutationStateInconsistency {
                    detail: "entity field update requires the existing payload to be a JSON object"
                        .to_string(),
                    fields: serde_json::json!({
                        "record_class": "entity",
                        "entity_id": intent.entity_id,
                        "phase": "update_entity_fields",
                        "payload_shape": "non_object",
                    }),
                }))
            }
        };
        for (key, value) in &intent.fields {
            object.insert(key.clone(), canonicalize_json(value));
        }
        let new_payload = RecordPayload::StructuredJson(canonicalize_json(&Value::Object(object)));
        context
            .state
            .mark_entity_slot_touched(intent.entity_id.partition_id, slot);
        let partition = context.state.get_partition_mut(intent.entity_id.partition_id);
        partition
            .entity_arena
            .apply_payload_update(slot, new_payload.clone(), version_id);
        Ok::<_, CommitConflict>((kind_id, old_payload, new_payload))
    })?;
    Ok(MutationOutcome::entity_updated(
        intent.entity_id,
        kind_id,
        old_payload,
        new_payload,
    ))
}
