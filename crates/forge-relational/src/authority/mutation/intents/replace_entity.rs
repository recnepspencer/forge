use serde_json::json;

use crate::authority::mutation::aspect_versions::{
    aspect_keys_for_payload, write_entity_aspect_versions,
};
use crate::authority::mutation::patch_details::patch_detail_for_entity;
use crate::authority::mutation::record_changes::{allocate_entity, delete_entity_with_cascade};
use crate::authority::mutation::stale_targets::ensure_entity_target_is_current;
use crate::authority::mutation::{MutationEffect, MutationWorkspace};
use crate::diagnostics::data::{DiagnosticCode, RelationalDiagnosticsEntry};
use crate::publication::data::diff::{PatchRecord, PatchRecordKind};
use crate::transactions::data::{CommitConflict, RecordRef, ReplaceEntityIntent};

pub(super) fn apply(
    intent: &ReplaceEntityIntent,
    workspace: &mut MutationWorkspace<'_>,
) -> Result<MutationEffect, CommitConflict> {
    let version_id = workspace.version_id();
    let patch_surface_policy = workspace.patch_surface_policy();
    let cascade_delete_policy = workspace.cascade_delete_policy();
    let mut effect = MutationEffect::default();
    let replacement_id = workspace.with_draft_symbols_and_schema(|draft, symbols, schema| {
        ensure_entity_target_is_current(draft, intent.entity_id)?;
        delete_entity_with_cascade(
            draft,
            version_id,
            intent.entity_id,
            patch_surface_policy,
            schema,
            cascade_delete_policy,
            &mut effect,
        );
        let replacement_id = allocate_entity(
            draft,
            version_id,
            intent.replacement.partition_id,
            intent.replacement.kind_id,
            intent.replacement.payload.clone(),
        );
        draft.mark_entity_slot_touched(
            replacement_id.partition_id,
            replacement_id.local_slot.0 as usize,
        );
        write_entity_aspect_versions(
            draft,
            replacement_id,
            version_id,
            &intent.replacement.payload,
            symbols,
        );
        Ok::<_, CommitConflict>(replacement_id)
    })?;
    let aspects = workspace.with_draft_and_symbols(|_, symbols| {
        aspect_keys_for_payload(Some(&intent.replacement.payload), symbols)
    });
    effect.record_change(RecordRef::Entity(replacement_id));
    effect.record_diagnostic(RelationalDiagnosticsEntry {
        code: DiagnosticCode::EntityUpdated,
        message: "entity replaced".to_string(),
        fields: json!({
            "replaced_partition_id": intent.entity_id.partition_id.0,
            "replaced_entity_slot": intent.entity_id.local_slot.0,
            "replacement_partition_id": replacement_id.partition_id.0,
            "replacement_entity_slot": replacement_id.local_slot.0,
            "kind_id": intent.replacement.kind_id.0,
        }),
    });
    effect.record_patch(PatchRecord {
        kind: PatchRecordKind::Created,
        target: RecordRef::Entity(replacement_id),
        aspects,
        detail: patch_detail_for_entity(
            patch_surface_policy,
            PatchRecordKind::Created,
            replacement_id,
            Some(&intent.replacement.payload),
        ),
    });
    Ok(effect)
}
