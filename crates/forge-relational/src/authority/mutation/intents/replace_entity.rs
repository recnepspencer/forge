use crate::authority::mutation::outcomes::MutationOutcome;
use crate::authority::mutation::record_changes::{allocate_entity, delete_entity_with_cascade};
use crate::authority::mutation::stale_targets::ensure_entity_target_is_current;
use crate::authority::mutation::MutationWorkspace;
use crate::transactions::data::{CommitConflict, ReplaceEntityIntent};

pub(super) fn apply(
    intent: &ReplaceEntityIntent,
    workspace: &mut MutationWorkspace<'_>,
) -> Result<MutationOutcome, CommitConflict> {
    let version_id = workspace.version_id();
    let cascade_delete_policy = workspace.cascade_delete_policy();
    let mut outcome = MutationOutcome::entity_deleted(intent.entity_id);
    let replacement_id = workspace.with_context(|context| {
        ensure_entity_target_is_current(context.state, intent.entity_id)?;
        delete_entity_with_cascade(
            context.state,
            version_id,
            intent.entity_id,
            context.schema,
            cascade_delete_policy,
            &mut outcome,
        );
        let replacement_id = allocate_entity(
            context.state,
            version_id,
            intent.replacement.partition_id,
            intent.replacement.kind_id,
            intent.replacement.payload.clone(),
        );
        context.state.mark_entity_slot_touched(
            replacement_id.partition_id,
            replacement_id.local_slot.0 as usize,
        );
        Ok::<_, CommitConflict>(replacement_id)
    })?;
    let replacement = MutationOutcome::entity_replaced(
        intent.entity_id,
        replacement_id,
        intent.replacement.kind_id,
        intent.replacement.payload.clone(),
    );
    outcome.extend(replacement);
    Ok(outcome)
}
