use crate::authority::mutation::outcomes::MutationOutcome;
use crate::authority::mutation::record_changes::allocate_entity_with_extra;
use crate::authority::mutation::MutationWorkspace;
use crate::transactions::data::{CommitConflict, ConflictClass, CreatedEntityRef, EntitySpec};

use super::entity_field_creation_aspects::plan_entity_field_creation_aspects;

pub(super) fn apply(
    spec: &EntitySpec,
    workspace: &mut MutationWorkspace<'_>,
) -> Result<MutationOutcome, CommitConflict> {
    let version_id = workspace.version_id();
    let aspect_plan = plan_entity_field_creation_aspects(
        spec.kind_id,
        workspace.entity_aspect_plan(spec.kind_id),
        &spec.fields,
    )
    .map_err(|denial| {
        CommitConflict::new(ConflictClass::EntityAuthoritativeAspectStateDenied {
            kind_id: spec.kind_id,
            denial,
        })
    })?;
    let entity_id = workspace.with_context(|context| {
        let entity_id = allocate_entity_with_extra(
            context.state,
            version_id,
            spec.partition_id,
            spec.kind_id,
            aspect_plan.extra,
        );
        context
            .state
            .mark_entity_slot_touched(entity_id.partition_id, entity_id.slot_index());
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
        aspect_plan.authoritative_patch,
    ))
}
