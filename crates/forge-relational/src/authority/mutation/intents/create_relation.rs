use crate::authority::mutation::outcomes::MutationOutcome;
use crate::authority::mutation::record_changes::allocate_relation;
use crate::authority::mutation::MutationWorkspace;
use crate::transactions::data::{CommitConflict, ConflictClass, EntityReference, RelationSpec};

pub(super) fn apply(
    spec: &RelationSpec,
    workspace: &mut MutationWorkspace<'_>,
) -> Result<MutationOutcome, CommitConflict> {
    let version_id = workspace.version_id();
    let source = resolve_entity_reference(workspace, &spec.source)?;
    let target = resolve_entity_reference(workspace, &spec.target)?;
    let relation_id = workspace.with_context(|context| {
        let relation_id = allocate_relation(
            context.state,
            version_id,
            spec.partition_id,
            spec.kind_id,
            source,
            target,
            spec.payload.clone(),
        );
        context.state.mark_relation_slot_touched(
            relation_id.partition_id,
            relation_id.local_slot.0 as usize,
        );
        relation_id
    });
    Ok(MutationOutcome::relation_created(
        relation_id,
        source,
        target,
        spec.kind_id,
        spec.payload.clone(),
    ))
}

fn resolve_entity_reference(
    workspace: &MutationWorkspace<'_>,
    entity_reference: &EntityReference,
) -> Result<crate::identity::data::EntityId, CommitConflict> {
    workspace
        .resolve_entity_reference(entity_reference)
        .ok_or_else(|| CommitConflict::new(ConflictClass::InvalidRelationEndpoint {
            detail: "relation endpoints must resolve within the same authoritative commit scope"
                .to_string(),
        }))
}
