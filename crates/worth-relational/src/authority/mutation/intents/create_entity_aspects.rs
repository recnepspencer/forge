use worth_foundational::facade::PortablePatchReadmissionPurpose;

use crate::authority::mutation::outcomes::MutationOutcome;
use crate::authority::mutation::record_changes::allocate_entity_with_extra;
use crate::authority::mutation::MutationWorkspace;
use crate::storage::substrate::EntityExtra;
use crate::transactions::data::{
    CommitConflict, CreatedEntityRef, EntityAspectCreateIntent, RecordAspectPatchTarget,
};

use super::record_aspect_patch;

pub(super) fn apply(
    intent: &EntityAspectCreateIntent,
    workspace: &mut MutationWorkspace<'_>,
) -> Result<MutationOutcome, CommitConflict> {
    let target = RecordAspectPatchTarget::EntityCreation {
        kind_id: intent.kind_id,
    };
    let patch = record_aspect_patch::readmit(
        intent.aspect_patch.clone(),
        PortablePatchReadmissionPurpose::RecordCreation,
        workspace.entity_aspect_plan(intent.kind_id),
        target,
    )?;
    let authoritative_aspect_state = record_aspect_patch::apply(None, &patch, target)?;
    let version_id = workspace.version_id();
    let entity_id = workspace.with_context(|context| {
        let entity_id = allocate_entity_with_extra(
            context.state,
            version_id,
            intent.partition_id,
            intent.kind_id,
            EntityExtra {
                authoritative_aspect_state,
                ..EntityExtra::default()
            },
        );
        context
            .state
            .mark_entity_slot_touched(entity_id.partition_id, entity_id.slot_index());
        entity_id
    });
    workspace.register_created_entity(
        CreatedEntityRef {
            partition_id: intent.partition_id,
            kind_id: intent.kind_id,
            client_key: intent.client_key.clone(),
        },
        entity_id,
    );
    Ok(MutationOutcome::entity_created(
        entity_id,
        intent.kind_id,
        record_aspect_patch::published_patch(patch),
    ))
}
