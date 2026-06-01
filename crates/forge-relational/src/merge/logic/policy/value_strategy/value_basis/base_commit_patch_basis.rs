use crate::history::data::CommitId;
use crate::transactions::data::RecordRef;
use forge_foundational::facade::{AspectKey, AspectValue};

pub(super) fn scalar_from_base_commit_patch(
    runtime: &crate::logic::runtime::RelationalRuntime,
    base_commit_id: CommitId,
    candidate_targets: &[RecordRef],
    aspect_key: &AspectKey,
) -> Option<AspectValue> {
    let history = runtime.history();
    let envelope = history.commit_envelope(base_commit_id)?;
    candidate_targets.iter().find_map(|target| {
        envelope
            .committed_record_changes_for_target(target)
            .find_map(|change| {
                change
                    .record
                    .authoritative_patch
                    .scalar_set_for(aspect_key)
                    .cloned()
            })
    })
}
