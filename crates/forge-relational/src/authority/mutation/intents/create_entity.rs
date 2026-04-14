use crate::authority::mutation::outcomes::MutationOutcome;
use crate::authority::mutation::record_changes::allocate_entity;
use crate::authority::mutation::MutationWorkspace;
use crate::transactions::data::{CommitConflict, CreatedEntityRef, EntitySpec};

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
        entity_id
    });
    workspace.register_created_entity(
        CreatedEntityRef {
            partition_id: spec.partition_id,
            kind_id: spec.kind_id,
            client_key: spec.client_key.clone(),
        },
        entity_id,
    );
    Ok(MutationOutcome::entity_created(
        entity_id,
        spec.kind_id,
        spec.payload.clone(),
    ))
}
