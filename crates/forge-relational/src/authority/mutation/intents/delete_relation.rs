use serde_json::json;

use crate::authority::mutation::record_changes::delete_relation;
use crate::authority::mutation::stale_targets::ensure_relation_target_is_current;
use crate::authority::mutation::{MutationEffect, MutationWorkspace};
use crate::diagnostics::data::{DiagnosticCode, RelationalDiagnosticsEntry};
use crate::transactions::data::{CommitConflict, TransactionIntent};

pub(super) fn apply(
    intent: &TransactionIntent,
    workspace: &mut MutationWorkspace<'_>,
) -> Result<MutationEffect, CommitConflict> {
    let TransactionIntent::DeleteRelation { relation_id } = intent else {
        unreachable!("delete_relation handler only accepts DeleteRelation");
    };
    let (draft, _symbols, config, _schema, version_id) = workspace.as_parts_mut();
    ensure_relation_target_is_current(draft, *relation_id)?;
    let mut effect = MutationEffect::default();
    delete_relation(
        draft,
        version_id,
        *relation_id,
        config.patch_surface_policy,
        &mut effect,
    );
    effect.record_diagnostic(RelationalDiagnosticsEntry {
        code: DiagnosticCode::RelationDeleted,
        message: "relation deleted".to_string(),
        fields: json!({
            "partition_id": relation_id.partition_id.0,
            "relation_slot": relation_id.local_slot.0,
        }),
    });
    Ok(effect)
}
