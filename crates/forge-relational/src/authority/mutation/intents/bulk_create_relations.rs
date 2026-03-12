use crate::authority::mutation::aspect_versions::{
    write_relation_aspect_versions,
};
use crate::authority::mutation::outcomes::{MutationEvent, MutationOutcome, RecordMutation};
use crate::authority::mutation::record_changes::{
    allocate_relation, reserve_bulk_relation_capacity,
};
use crate::authority::mutation::MutationWorkspace;
use crate::symbols::data::InternedString;
use crate::transactions::data::{
    BulkRelationCreateIntent, CommitConflict, RelationSpec,
};

pub(super) fn apply(
    intent: &BulkRelationCreateIntent,
    workspace: &mut MutationWorkspace<'_>,
) -> Result<MutationOutcome, CommitConflict> {
    let version_id = workspace.version_id();
    let mut outcome = MutationOutcome::default();
    workspace.with_context(|context| {
        reserve_bulk_relation_capacity(
            context.state,
            intent.partition_id,
            intent.endpoints.len(),
        );
    });
    for (index, (source, target)) in intent.endpoints.iter().enumerate() {
        let spec = RelationSpec {
            partition_id: intent.partition_id,
            kind_id: intent.kind_id,
            client_key: InternedString::from("bulk"),
            source: *source,
            target: *target,
            payload: intent.payloads.get(index).cloned().unwrap_or(None),
        };
        let relation_id = workspace.with_context(|context| {
            let relation_id = allocate_relation(context.state, version_id, &spec);
            context.state.mark_relation_slot_touched(
                relation_id.partition_id,
                relation_id.local_slot.0 as usize,
            );
            write_relation_aspect_versions(
                context.state,
                relation_id,
                version_id,
                spec.payload.as_ref(),
                context.symbols,
            );
            relation_id
        });
        outcome.record_change(RecordMutation::RelationCreated {
            relation_id,
            source: spec.source,
            target: spec.target,
            payload: spec.payload.clone(),
        });
    }
    outcome.record_event(MutationEvent::BulkRelationsCreated {
        partition_id: intent.partition_id,
        kind_id: intent.kind_id,
        count: intent.endpoints.len(),
    });
    Ok(outcome)
}
