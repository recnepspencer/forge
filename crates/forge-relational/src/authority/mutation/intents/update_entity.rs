use serde_json::json;

use crate::authority::mutation::aspect_versions::{
    aspect_keys_for_payload, write_entity_aspect_versions,
};
use crate::authority::mutation::patch_details::patch_detail_for_entity;
use crate::authority::mutation::stale_targets::ensure_entity_target_is_current;
use crate::authority::mutation::{MutationEffect, MutationWorkspace};
use crate::diagnostics::data::{DiagnosticCode, RelationalDiagnosticsEntry};
use crate::publication::data::diff::{PatchRecord, PatchRecordKind};
use crate::transactions::data::{CommitConflict, RecordRef, UpdateEntityIntent};

pub(super) fn apply(
    intent: &UpdateEntityIntent,
    workspace: &mut MutationWorkspace<'_>,
) -> Result<MutationEffect, CommitConflict> {
    let version_id = workspace.version_id();
    let patch_surface_policy = workspace.patch_surface_policy();
    let payload = intent.payload.canonicalized();
    let slot = intent.entity_id.local_slot.0 as usize;
    workspace.with_draft_and_symbols(|draft, symbols| {
        ensure_entity_target_is_current(draft, intent.entity_id)?;
        draft.mark_entity_slot_touched(intent.entity_id.partition_id, slot);
        let partition = draft.get_partition_mut(intent.entity_id.partition_id);
        partition
            .entity_arena
            .apply_payload_update(slot, payload.clone(), version_id);
        write_entity_aspect_versions(draft, intent.entity_id, version_id, &payload, symbols);
        Ok::<(), CommitConflict>(())
    })?;

    let mut effect = MutationEffect::default();
    let aspects =
        workspace.with_draft_and_symbols(|_, symbols| aspect_keys_for_payload(Some(&payload), symbols));
    effect.record_change(RecordRef::Entity(intent.entity_id));
    effect.record_diagnostic(RelationalDiagnosticsEntry {
        code: DiagnosticCode::EntityUpdated,
        message: "entity updated".to_string(),
        fields: json!({
            "partition_id": intent.entity_id.partition_id.0,
            "entity_slot": intent.entity_id.local_slot.0,
        }),
    });
    effect.record_patch(PatchRecord {
        kind: PatchRecordKind::Updated,
        target: RecordRef::Entity(intent.entity_id),
        aspects,
        detail: patch_detail_for_entity(
            patch_surface_policy,
            PatchRecordKind::Updated,
            intent.entity_id,
            Some(&payload),
        ),
    });
    Ok(effect)
}
