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
    CommitConflict, RecordRef, RelationSpec, TransactionIntent,
};

pub(super) fn apply(
    intent: &TransactionIntent,
    workspace: &mut MutationWorkspace<'_>,
) -> Result<MutationEffect, CommitConflict> {
    let TransactionIntent::BulkCreateRelations {
        partition_id,
        kind_id,
        client_keys: _,
        endpoints,
        payloads,
    } = intent
    else {
        unreachable!("bulk_create_relations handler only accepts BulkCreateRelations");
    };
    let (draft, symbols, config, _schema, version_id) = workspace.as_parts_mut();
    let mut effect = MutationEffect::default();
    reserve_bulk_relation_capacity(draft, *partition_id, endpoints.len());
    for (index, (source, target)) in endpoints.iter().enumerate() {
        let spec = RelationSpec {
            partition_id: *partition_id,
            kind_id: *kind_id,
            client_key: InternedString::from("bulk"),
            source: *source,
            target: *target,
            payload: payloads.get(index).cloned().unwrap_or(None),
        };
        let relation_id = allocate_relation(draft, version_id, &spec);
        draft.mark_relation_slot_touched(relation_id.partition_id, relation_id.local_slot.0 as usize);
        write_relation_aspect_versions(draft, relation_id, version_id, spec.payload.as_ref(), symbols);
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
            aspects: aspect_keys_for_payload(spec.payload.as_ref(), symbols),
            detail: patch_detail_for_relation(
                config.patch_surface_policy,
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
        fields: json!({"partition_id": partition_id.0, "kind_id": kind_id.0}),
    });
    Ok(effect)
}
