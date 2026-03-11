use serde_json::json;

use crate::authority::mutation::aspect_versions::{
    aspect_keys_for_payload, write_entity_aspect_versions,
};
use crate::authority::mutation::patch_details::patch_detail_for_entity;
use crate::authority::mutation::stale_targets::ensure_entity_target_is_current;
use crate::authority::mutation::{MutationEffect, MutationWorkspace};
use crate::diagnostics::data::{DiagnosticCode, RelationalDiagnosticsEntry};
use crate::publication::data::diff::{PatchRecord, PatchRecordKind};
use crate::transactions::data::{CommitConflict, RecordRef, TransactionIntent};

pub(super) fn apply(
    intent: &TransactionIntent,
    workspace: &mut MutationWorkspace<'_>,
) -> Result<MutationEffect, CommitConflict> {
    let TransactionIntent::UpdateEntity { entity_id, payload } = intent else {
        unreachable!("update_entity handler only accepts UpdateEntity");
    };
    let (draft, symbols, config, _schema, version_id) = workspace.as_parts_mut();
    ensure_entity_target_is_current(draft, *entity_id)?;
    let payload = payload.canonicalized();
    let slot = entity_id.local_slot.0 as usize;
    draft.mark_entity_slot_touched(entity_id.partition_id, slot);
    let partition = draft.get_partition_mut(entity_id.partition_id);
    partition
        .entity_arena
        .apply_payload_update(slot, payload.clone(), version_id);
    write_entity_aspect_versions(draft, *entity_id, version_id, &payload, symbols);

    let mut effect = MutationEffect::default();
    effect.record_change(RecordRef::Entity(*entity_id));
    effect.record_diagnostic(RelationalDiagnosticsEntry {
        code: DiagnosticCode::EntityUpdated,
        message: "entity updated".to_string(),
        fields: json!({
            "partition_id": entity_id.partition_id.0,
            "entity_slot": entity_id.local_slot.0,
        }),
    });
    effect.record_patch(PatchRecord {
        kind: PatchRecordKind::Updated,
        target: RecordRef::Entity(*entity_id),
        aspects: aspect_keys_for_payload(Some(&payload), symbols),
        detail: patch_detail_for_entity(
            config.patch_surface_policy,
            PatchRecordKind::Updated,
            *entity_id,
            Some(&payload),
        ),
    });
    Ok(effect)
}
