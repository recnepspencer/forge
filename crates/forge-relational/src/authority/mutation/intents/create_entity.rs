use crate::authority::mutation::aspect_versions::{
    write_entity_aspect_versions,
};
use crate::authority::mutation::outcomes::{MutationEvent, MutationOutcome, RecordMutation};
use crate::authority::mutation::record_changes::allocate_entity;
use crate::authority::mutation::MutationWorkspace;
use crate::transactions::data::{CommitConflict, EntitySpec};

pub(super) fn apply(
    spec: &EntitySpec,
    workspace: &mut MutationWorkspace<'_>,
) -> Result<MutationOutcome, CommitConflict> {
    let version_id = workspace.version_id();
    let entity_id = workspace.with_context(|context| {
        let entity_id = allocate_entity(
            context.state,
            version_id,
            spec.partition_id,
            spec.kind_id,
            spec.payload.clone(),
        );
        context
            .state
            .mark_entity_slot_touched(entity_id.partition_id, entity_id.local_slot.0 as usize);
        write_entity_aspect_versions(
            context.state,
            entity_id,
            version_id,
            &spec.payload,
            context.symbols,
        );
        entity_id
    });
    let mut outcome = MutationOutcome::default();
    outcome.record_change(RecordMutation::EntityCreated {
        entity_id,
        payload: spec.payload.clone(),
    });
    outcome.record_event(MutationEvent::EntityCreated {
        entity_id,
        kind_id: spec.kind_id,
    });
    Ok(outcome)
}
