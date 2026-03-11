use serde_json::json;

use crate::authority::mutation::record_changes::delete_relation;
use crate::authority::mutation::stale_targets::ensure_relation_target_is_current;
use crate::authority::mutation::{MutationEffect, MutationWorkspace};
use crate::diagnostics::data::{DiagnosticCode, RelationalDiagnosticsEntry};
use crate::transactions::data::{CommitConflict, DeleteRelationIntent};

pub(super) fn apply(
    intent: &DeleteRelationIntent,
    workspace: &mut MutationWorkspace<'_>,
) -> Result<MutationEffect, CommitConflict> {
    let version_id = workspace.version_id();
    let patch_surface_policy = workspace.patch_surface_policy();
    ensure_relation_target_is_current(workspace.draft_mut(), intent.relation_id)?;
    let mut effect = MutationEffect::default();
    delete_relation(
        workspace.draft_mut(),
        version_id,
        intent.relation_id,
        patch_surface_policy,
        &mut effect,
    );
    effect.record_diagnostic(RelationalDiagnosticsEntry {
        code: DiagnosticCode::RelationDeleted,
        message: "relation deleted".to_string(),
        fields: json!({
            "partition_id": intent.relation_id.partition_id.0,
            "relation_slot": intent.relation_id.local_slot.0,
        }),
    });
    Ok(effect)
}
