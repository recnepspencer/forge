use crate::authority::mutation::record_changes::delete_relation;
use crate::authority::mutation::stale_targets::ensure_relation_target_is_current;
use crate::authority::mutation::outcomes::{MutationEvent, MutationOutcome};
use crate::authority::mutation::MutationWorkspace;
use crate::transactions::data::{CommitConflict, DeleteRelationIntent};

pub(super) fn apply(
    intent: &DeleteRelationIntent,
    workspace: &mut MutationWorkspace<'_>,
) -> Result<MutationOutcome, CommitConflict> {
    let version_id = workspace.version_id();
    let mut outcome = MutationOutcome::default();
    workspace.with_context(|context| {
        ensure_relation_target_is_current(context.state, intent.relation_id)?;
        delete_relation(context.state, version_id, intent.relation_id, &mut outcome);
        Ok::<(), CommitConflict>(())
    })?;
    outcome.record_event(MutationEvent::RelationDeleted {
        relation_id: intent.relation_id,
    });
    Ok(outcome)
}
