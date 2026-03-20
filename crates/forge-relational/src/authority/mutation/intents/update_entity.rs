use crate::authority::mutation::outcomes::MutationOutcome;
use crate::authority::mutation::stale_targets::ensure_entity_target_is_current;
use crate::authority::mutation::MutationWorkspace;
use crate::storage::logic::state::PartitionAccess;
use crate::transactions::data::{CommitConflict, UpdateEntityIntent};

pub(super) fn apply(
    intent: &UpdateEntityIntent,
    workspace: &mut MutationWorkspace<'_>,
) -> Result<MutationOutcome, CommitConflict> {
    let version_id = workspace.version_id();
    let new_payload = intent.payload.canonicalized();
    let slot = intent.entity_id.local_slot.0 as usize;
    let (kind_id, old_payload) = workspace.with_context(|context| {
        ensure_entity_target_is_current(context.state, intent.entity_id)?;
        let partition = context
            .state
            .get_partition(intent.entity_id.partition_id)
            .expect("current entity partition must exist after stale target validation");
        let slot_view = partition
            .entity_arena
            .get_slot(slot)
            .expect("current entity slot must exist after stale target validation");
        let kind_id = slot_view
            .kind_id()
            .expect("current entity kind must exist after stale target validation");
        let old_payload = slot_view
            .payload()
            .cloned()
            .expect("current entity payload must exist after stale target validation");
        context
            .state
            .mark_entity_slot_touched(intent.entity_id.partition_id, slot);
        let partition = context
            .state
            .get_partition_mut(intent.entity_id.partition_id);
        partition
            .entity_arena
            .apply_payload_update(slot, new_payload.clone(), version_id);
        Ok::<_, CommitConflict>((kind_id, old_payload))
    })?;
    Ok(MutationOutcome::entity_updated(
        intent.entity_id,
        kind_id,
        old_payload,
        new_payload,
    ))
}
