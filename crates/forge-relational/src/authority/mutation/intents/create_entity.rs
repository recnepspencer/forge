use serde_json::json;

use crate::authority::mutation::aspect_versions::{
    aspect_keys_for_payload, write_entity_aspect_versions,
};
use crate::authority::mutation::patch_details::patch_detail_for_entity;
use crate::authority::mutation::record_changes::allocate_entity;
use crate::authority::mutation::{MutationEffect, MutationWorkspace};
use crate::diagnostics::data::{DiagnosticCode, RelationalDiagnosticsEntry};
use crate::publication::data::diff::{PatchRecord, PatchRecordKind};
use crate::transactions::data::{CommitConflict, EntitySpec, RecordRef};

pub(super) fn apply(
    spec: &EntitySpec,
    workspace: &mut MutationWorkspace<'_>,
) -> Result<MutationEffect, CommitConflict> {
    let version_id = workspace.version_id();
    let patch_surface_policy = workspace.patch_surface_policy();
    let mut effect = MutationEffect::default();
    let entity_id = workspace.with_draft_and_symbols(|draft, symbols| {
        let entity_id = allocate_entity(
            draft,
            version_id,
            spec.partition_id,
            spec.kind_id,
            spec.payload.clone(),
        );
        draft.mark_entity_slot_touched(entity_id.partition_id, entity_id.local_slot.0 as usize);
        write_entity_aspect_versions(draft, entity_id, version_id, &spec.payload, symbols);
        entity_id
    });
    let aspects =
        workspace.with_draft_and_symbols(|_, symbols| aspect_keys_for_payload(Some(&spec.payload), symbols));
    effect.record_change(RecordRef::Entity(entity_id));
    effect.record_diagnostic(RelationalDiagnosticsEntry {
        code: DiagnosticCode::EntityCreated,
        message: "entity created".to_string(),
        fields: json!({
            "partition_id": entity_id.partition_id.0,
            "entity_slot": entity_id.local_slot.0,
            "kind_id": spec.kind_id.0,
        }),
    });
    effect.record_patch(PatchRecord {
        kind: PatchRecordKind::Created,
        target: RecordRef::Entity(entity_id),
        aspects,
        detail: patch_detail_for_entity(
            patch_surface_policy,
            PatchRecordKind::Created,
            entity_id,
            Some(&spec.payload),
        ),
    });
    Ok(effect)
}
