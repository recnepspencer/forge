use serde_json::json;

use crate::authority::mutation::record_changes::delete_entity_with_cascade;
use crate::authority::mutation::stale_targets::ensure_entity_target_is_current;
use crate::authority::mutation::{MutationEffect, MutationWorkspace};
use crate::diagnostics::data::{DiagnosticCode, RelationalDiagnosticsEntry};
use crate::transactions::data::{CommitConflict, DeleteEntityIntent};

pub(super) fn apply(
    intent: &DeleteEntityIntent,
    workspace: &mut MutationWorkspace<'_>,
) -> Result<MutationEffect, CommitConflict> {
    let version_id = workspace.version_id();
    let patch_surface_policy = workspace.patch_surface_policy();
    let cascade_delete_policy = workspace.cascade_delete_policy();
    let mut effect = MutationEffect::default();
    workspace.with_draft_and_schema(|draft, schema| {
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
        Ok::<(), CommitConflict>(())
    })?;
    effect.record_diagnostic(RelationalDiagnosticsEntry {
        code: DiagnosticCode::EntityDeleted,
        message: "entity deleted".to_string(),
        fields: json!({
            "partition_id": intent.entity_id.partition_id.0,
            "entity_slot": intent.entity_id.local_slot.0,
        }),
    });
    Ok(effect)
}
