use crate::authority::mutation::outcomes::MutationOutcome;
use crate::authority::mutation::record_changes::{
    allocate_entity_with_extra, delete_entity_with_cascade,
};
use crate::authority::mutation::stale_targets::ensure_entity_target_is_current;
use crate::authority::mutation::MutationWorkspace;
use crate::transactions::data::{CommitConflict, ConflictClass, ReplaceEntityIntent};

use super::entity_field_creation_aspects::plan_entity_field_creation_aspects;

pub(super) fn apply(
    intent: &ReplaceEntityIntent,
    workspace: &mut MutationWorkspace<'_>,
) -> Result<MutationOutcome, CommitConflict> {
    let version_id = workspace.version_id();
    let cascade_delete_policy = workspace.cascade_delete_policy();
    let replacement_aspect_plan = plan_entity_field_creation_aspects(
        intent.replacement.kind_id,
        workspace.entity_aspect_plan(intent.replacement.kind_id),
        &intent.replacement.fields,
    )
    .map_err(|denial| {
        CommitConflict::new(ConflictClass::EntityAuthoritativeAspectStateDenied {
            kind_id: intent.replacement.kind_id,
            denial,
        })
    })?;
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
        )?;
        let replacement_id = allocate_entity_with_extra(
            context.state,
            version_id,
            intent.replacement.partition_id,
            intent.replacement.kind_id,
            replacement_aspect_plan.extra,
        );
        context
            .state
            .mark_entity_slot_touched(replacement_id.partition_id, replacement_id.slot_index());
        Ok::<_, CommitConflict>(replacement_id)
    })?;
    outcome.extend(MutationOutcome::entity_replaced(
        intent.entity_id,
        replacement_id,
        intent.replacement.kind_id,
        replacement_aspect_plan.authoritative_patch,
    ));
    Ok(outcome)
}
