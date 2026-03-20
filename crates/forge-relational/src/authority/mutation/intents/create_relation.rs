use crate::authority::mutation::outcomes::MutationOutcome;
use crate::authority::mutation::record_changes::allocate_relation;
use crate::authority::mutation::MutationWorkspace;
use crate::transactions::data::{CommitConflict, RelationSpec};

pub(super) fn apply(
    spec: &RelationSpec,
    workspace: &mut MutationWorkspace<'_>,
) -> Result<MutationOutcome, CommitConflict> {
    let version_id = workspace.version_id();
    let relation_id = workspace.with_context(|context| {
        let relation_id = allocate_relation(context.state, version_id, spec);
        context.state.mark_relation_slot_touched(
            relation_id.partition_id,
            relation_id.local_slot.0 as usize,
        );
        relation_id
    });
    Ok(MutationOutcome::relation_created(
        relation_id,
        spec.source,
        spec.target,
        spec.kind_id,
        spec.payload.clone(),
    ))
}
