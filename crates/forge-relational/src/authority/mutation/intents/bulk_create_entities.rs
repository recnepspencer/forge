use crate::authority::mutation::aspect_versions::{
    write_entity_aspect_versions,
};
use crate::authority::mutation::outcomes::{MutationEvent, MutationOutcome, RecordMutation};
use crate::authority::mutation::record_changes::{
    allocate_entity, reserve_bulk_entity_capacity,
};
use crate::authority::mutation::MutationWorkspace;
use crate::transactions::data::{BulkEntityCreateIntent, CommitConflict};

pub(super) fn apply(
    intent: &BulkEntityCreateIntent,
    workspace: &mut MutationWorkspace<'_>,
) -> Result<MutationOutcome, CommitConflict> {
    let version_id = workspace.version_id();
    let mut outcome = MutationOutcome::default();
    workspace.with_context(|context| {
        reserve_bulk_entity_capacity(
            context.state,
            intent.partition_id,
            intent.payloads.len(),
        );
    });
    for payload in &intent.payloads {
        let entity_id = workspace.with_context(|context| {
            let entity_id = allocate_entity(
                context.state,
                version_id,
                intent.partition_id,
                intent.kind_id,
                payload.clone(),
            );
            context
                .state
                .mark_entity_slot_touched(entity_id.partition_id, entity_id.local_slot.0 as usize);
            write_entity_aspect_versions(
                context.state,
                entity_id,
                version_id,
                payload,
                context.symbols,
            );
            entity_id
        });
        outcome.record_change(RecordMutation::EntityCreated {
            entity_id,
            payload: payload.clone(),
        });
    }
    outcome.record_event(MutationEvent::BulkEntitiesCreated {
        partition_id: intent.partition_id,
        kind_id: intent.kind_id,
        count: intent.payloads.len(),
    });
    Ok(outcome)
}
