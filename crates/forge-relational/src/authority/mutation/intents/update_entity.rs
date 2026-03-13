use crate::authority::mutation::aspect_versions::write_entity_aspect_versions;
use crate::authority::mutation::outcomes::MutationOutcome;
use crate::authority::mutation::stale_targets::ensure_entity_target_is_current;
use crate::authority::mutation::MutationWorkspace;
use crate::transactions::data::{CommitConflict, UpdateEntityIntent};

pub(super) fn apply(
    intent: &UpdateEntityIntent,
    workspace: &mut MutationWorkspace<'_>,
) -> Result<MutationOutcome, CommitConflict> {
    let version_id = workspace.version_id();
    let payload = intent.payload.canonicalized();
    let slot = intent.entity_id.local_slot.0 as usize;
    workspace.with_context(|context| {
        ensure_entity_target_is_current(context.state, intent.entity_id)?;
        context
            .state
            .mark_entity_slot_touched(intent.entity_id.partition_id, slot);
        let partition = context
            .state
            .get_partition_mut(intent.entity_id.partition_id);
        partition
            .entity_arena
            .apply_payload_update(slot, payload.clone(), version_id);
        write_entity_aspect_versions(
            context.state,
            intent.entity_id,
            version_id,
            &payload,
            context.symbols,
        );
        Ok::<(), CommitConflict>(())
    })?;
    Ok(MutationOutcome::entity_updated(intent.entity_id, payload))
}
