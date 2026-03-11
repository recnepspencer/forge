use serde_json::json;

use crate::authority::mutation::aspect_versions::{
    aspect_keys_for_payload, write_relation_aspect_versions,
};
use crate::authority::mutation::patch_details::patch_detail_for_relation;
use crate::authority::mutation::record_changes::allocate_relation;
use crate::authority::mutation::{
    AdjacencyDelta, AdjacencyDeltaKind, MutationEffect, MutationWorkspace,
};
use crate::diagnostics::data::{DiagnosticCode, RelationalDiagnosticsEntry};
use crate::publication::data::diff::{PatchRecord, PatchRecordKind};
use crate::transactions::data::{CommitConflict, RecordRef, RelationSpec};

pub(super) fn apply(
    spec: &RelationSpec,
    workspace: &mut MutationWorkspace<'_>,
) -> Result<MutationEffect, CommitConflict> {
    let version_id = workspace.version_id();
    let patch_surface_policy = workspace.patch_surface_policy();
    let relation_id = workspace.with_draft_and_symbols(|draft, symbols| {
        let relation_id = allocate_relation(draft, version_id, spec);
        draft.mark_relation_slot_touched(relation_id.partition_id, relation_id.local_slot.0 as usize);
        write_relation_aspect_versions(draft, relation_id, version_id, spec.payload.as_ref(), symbols);
        relation_id
    });
    let aspects = workspace.with_draft_and_symbols(|_, symbols| {
        aspect_keys_for_payload(spec.payload.as_ref(), symbols)
    });

    let mut effect = MutationEffect::default();
    effect.record_change(RecordRef::Relation(relation_id));
    effect.record_adjacency_delta(AdjacencyDelta {
        relation_id,
        kind: AdjacencyDeltaKind::Created {
            source: spec.source,
            target: spec.target,
        },
    });
    effect.record_diagnostic(RelationalDiagnosticsEntry {
        code: DiagnosticCode::RelationCreated,
        message: "relation created".to_string(),
        fields: json!({
            "partition_id": relation_id.partition_id.0,
            "relation_slot": relation_id.local_slot.0,
            "source_partition": spec.source.partition_id.0,
            "source_slot": spec.source.local_slot.0,
            "target_partition": spec.target.partition_id.0,
            "target_slot": spec.target.local_slot.0,
            "kind_id": spec.kind_id.0,
        }),
    });
    effect.record_patch(PatchRecord {
        kind: PatchRecordKind::Created,
        target: RecordRef::Relation(relation_id),
        aspects,
        detail: patch_detail_for_relation(
            patch_surface_policy,
            PatchRecordKind::Created,
            relation_id,
            spec.source,
            spec.target,
            spec.payload.as_ref(),
        ),
    });
    Ok(effect)
}
