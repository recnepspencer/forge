use crate::authority::mutation::outcomes::MutationOutcome;
use crate::authority::mutation::record_changes::delete_entity_with_cascade;
use crate::authority::mutation::stale_targets::ensure_entity_target_is_current;
use crate::authority::mutation::MutationWorkspace;
use crate::transactions::data::{CommitConflict, DeleteEntityIntent};

pub(super) fn apply(
    intent: &DeleteEntityIntent,
    workspace: &mut MutationWorkspace<'_>,
) -> Result<MutationOutcome, CommitConflict> {
    let version_id = workspace.version_id();
    let cascade_delete_policy = workspace.cascade_delete_policy();
    let mut outcome = MutationOutcome::entity_deleted(intent.entity_id);
    workspace.with_context(|context| {
        ensure_entity_target_is_current(context.state, intent.entity_id)?;
        delete_entity_with_cascade(
            context.state,
            version_id,
            intent.entity_id,
            context.schema,
            cascade_delete_policy,
            &mut outcome,
        )?;
        Ok::<(), CommitConflict>(())
    })?;
    Ok(outcome)
}
