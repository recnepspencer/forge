use serde_json::json;

use crate::authority::mutation::aspect_versions::{
    aspect_keys_for_payload, write_entity_aspect_versions,
};
use crate::authority::mutation::patch_details::patch_detail_for_entity;
use crate::authority::mutation::record_changes::{
    allocate_entity, reserve_bulk_entity_capacity,
};
use crate::authority::mutation::{MutationEffect, MutationWorkspace};
use crate::diagnostics::data::{DiagnosticCode, RelationalDiagnosticsEntry};
use crate::publication::data::diff::{PatchRecord, PatchRecordKind};
use crate::transactions::data::{CommitConflict, RecordRef, TransactionIntent};

pub(super) fn apply(
    intent: &TransactionIntent,
    workspace: &mut MutationWorkspace<'_>,
) -> Result<MutationEffect, CommitConflict> {
    let TransactionIntent::BulkCreateEntities {
        partition_id,
        kind_id,
        client_keys: _,
        payloads,
    } = intent
    else {
        unreachable!("bulk_create_entities handler only accepts BulkCreateEntities");
    };
    let (draft, symbols, config, _schema, version_id) = workspace.as_parts_mut();
    let mut effect = MutationEffect::default();
    reserve_bulk_entity_capacity(draft, *partition_id, payloads.len());
    for payload in payloads {
        let entity_id = allocate_entity(draft, version_id, *partition_id, *kind_id, payload.clone());
        draft.mark_entity_slot_touched(entity_id.partition_id, entity_id.local_slot.0 as usize);
        write_entity_aspect_versions(draft, entity_id, version_id, payload, symbols);
        effect.record_change(RecordRef::Entity(entity_id));
        effect.record_patch(PatchRecord {
            kind: PatchRecordKind::Created,
            target: RecordRef::Entity(entity_id),
            aspects: aspect_keys_for_payload(Some(payload), symbols),
            detail: patch_detail_for_entity(
                config.patch_surface_policy,
                PatchRecordKind::Created,
                entity_id,
                Some(payload),
            ),
        });
    }
    effect.record_diagnostic(RelationalDiagnosticsEntry {
        code: DiagnosticCode::EntityCreated,
        message: "bulk entities created".to_string(),
        fields: json!({
            "partition_id": partition_id.0,
            "kind_id": kind_id.0,
        }),
    });
    Ok(effect)
}
