use serde_json::json;

use crate::authority::mutation::aspect_versions::{
    aspect_keys_for_payload, write_relation_aspect_versions,
};
use crate::authority::mutation::patch_details::patch_detail_for_relation;
use crate::authority::mutation::record_changes::{
    allocate_relation, reserve_bulk_relation_capacity,
};
use crate::authority::mutation::{
    AdjacencyDelta, AdjacencyDeltaKind, MutationEffect, MutationWorkspace,
};
use crate::diagnostics::data::{DiagnosticCode, RelationalDiagnosticsEntry};
use crate::publication::data::diff::{PatchRecord, PatchRecordKind};
use crate::symbols::data::InternedString;
use crate::transactions::data::{
    BulkRelationCreateIntent, CommitConflict, RecordRef, RelationSpec,
};

pub(super) fn apply(
    intent: &BulkRelationCreateIntent,
    workspace: &mut MutationWorkspace<'_>,
) -> Result<MutationEffect, CommitConflict> {
    let version_id = workspace.version_id();
    let patch_surface_policy = workspace.patch_surface_policy();
    let mut effect = MutationEffect::default();
    workspace.with_draft_and_symbols(|draft, _| {
        reserve_bulk_relation_capacity(draft, intent.partition_id, intent.endpoints.len());
    });
    for (index, (source, target)) in intent.endpoints.iter().enumerate() {
        let spec = RelationSpec {
            partition_id: intent.partition_id,
            kind_id: intent.kind_id,
            client_key: InternedString::from("bulk"),
            source: *source,
            target: *target,
            payload: intent.payloads.get(index).cloned().unwrap_or(None),
        };
        let relation_id = workspace.with_draft_and_symbols(|draft, symbols| {
            let relation_id = allocate_relation(draft, version_id, &spec);
            draft.mark_relation_slot_touched(relation_id.partition_id, relation_id.local_slot.0 as usize);
            write_relation_aspect_versions(draft, relation_id, version_id, spec.payload.as_ref(), symbols);
            relation_id
        });
        effect.record_change(RecordRef::Relation(relation_id));
        effect.record_adjacency_delta(AdjacencyDelta {
            relation_id,
            kind: AdjacencyDeltaKind::Created {
                source: spec.source,
                target: spec.target,
            },
        });
        effect.record_patch(PatchRecord {
            kind: PatchRecordKind::Created,
            target: RecordRef::Relation(relation_id),
            aspects: workspace.with_draft_and_symbols(|_, symbols| {
                aspect_keys_for_payload(spec.payload.as_ref(), symbols)
            }),
            detail: patch_detail_for_relation(
                patch_surface_policy,
                PatchRecordKind::Created,
                relation_id,
                spec.source,
                spec.target,
                spec.payload.as_ref(),
            ),
        });
    }
    effect.record_diagnostic(RelationalDiagnosticsEntry {
        code: DiagnosticCode::RelationCreated,
        message: "bulk relations created".to_string(),
        fields: json!({
            "partition_id": intent.partition_id.0,
            "kind_id": intent.kind_id.0
        }),
    });
    Ok(effect)
}
