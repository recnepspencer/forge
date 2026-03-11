use serde_json::json;

use crate::authority::mutation::record_changes::delete_entity_with_cascade;
use crate::authority::mutation::stale_targets::ensure_entity_target_is_current;
use crate::authority::mutation::{MutationEffect, MutationWorkspace};
use crate::diagnostics::data::{DiagnosticCode, RelationalDiagnosticsEntry};
use crate::transactions::data::{CommitConflict, TransactionIntent};

pub(super) fn apply(
    intent: &TransactionIntent,
    workspace: &mut MutationWorkspace<'_>,
) -> Result<MutationEffect, CommitConflict> {
    let TransactionIntent::DeleteEntity { entity_id } = intent else {
        unreachable!("delete_entity handler only accepts DeleteEntity");
    };
    let (draft, _symbols, config, schema, version_id) = workspace.as_parts_mut();
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
    effect.record_diagnostic(RelationalDiagnosticsEntry {
        code: DiagnosticCode::EntityDeleted,
        message: "entity deleted".to_string(),
        fields: json!({
            "partition_id": entity_id.partition_id.0,
            "entity_slot": entity_id.local_slot.0,
        }),
    });
    Ok(effect)
}
