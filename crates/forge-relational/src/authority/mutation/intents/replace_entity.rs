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
use crate::transactions::data::{CommitConflict, RecordRef, TransactionIntent};

pub(super) fn apply(
    intent: &TransactionIntent,
    workspace: &mut MutationWorkspace<'_>,
) -> Result<MutationEffect, CommitConflict> {
    let TransactionIntent::ReplaceEntity {
        entity_id,
        replacement,
    } = intent
    else {
        unreachable!("replace_entity handler only accepts ReplaceEntity");
    };
    let (draft, symbols, config, schema, version_id) = workspace.as_parts_mut();
    ensure_entity_target_is_current(draft, *entity_id)?;
    let mut effect = MutationEffect::default();
    delete_entity_with_cascade(
        draft,
        version_id,
        *entity_id,
        config.patch_surface_policy,
        schema,
        config.cascade_delete_policy,
        &mut effect,
    );
    let replacement_id = allocate_entity(
        draft,
        version_id,
        replacement.partition_id,
        replacement.kind_id,
        replacement.payload.clone(),
    );
    draft.mark_entity_slot_touched(replacement_id.partition_id, replacement_id.local_slot.0 as usize);
    write_entity_aspect_versions(
        draft,
        replacement_id,
        version_id,
        &replacement.payload,
        symbols,
    );
    effect.record_change(RecordRef::Entity(replacement_id));
    effect.record_diagnostic(RelationalDiagnosticsEntry {
        code: DiagnosticCode::EntityUpdated,
        message: "entity replaced".to_string(),
        fields: json!({
            "replaced_partition_id": entity_id.partition_id.0,
            "replaced_entity_slot": entity_id.local_slot.0,
            "replacement_partition_id": replacement_id.partition_id.0,
            "replacement_entity_slot": replacement_id.local_slot.0,
            "kind_id": replacement.kind_id.0,
        }),
    });
    effect.record_patch(PatchRecord {
        kind: PatchRecordKind::Created,
        target: RecordRef::Entity(replacement_id),
        aspects: aspect_keys_for_payload(Some(&replacement.payload), symbols),
        detail: patch_detail_for_entity(
            config.patch_surface_policy,
            PatchRecordKind::Created,
            replacement_id,
            Some(&replacement.payload),
        ),
    });
    Ok(effect)
}
