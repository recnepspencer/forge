use crate::history::data::CommitId;
use crate::publication::patch::data::{
    PublishedAuthoritativePatch, PublishedAuthoritativePatchOperation,
    PublishedAuthoritativePatchValue,
};
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
                scalar_from_published_patch(&change.record.authoritative_patch, aspect_key)
            })
    })
}

fn scalar_from_published_patch(
    patch: &PublishedAuthoritativePatch,
    aspect_key: &AspectKey,
) -> Option<AspectValue> {
    patch
        .operations
        .iter()
        .find_map(|operation| match operation {
            PublishedAuthoritativePatchOperation::WholeAspectSet {
                aspect_key: operation_aspect_key,
                value: PublishedAuthoritativePatchValue::Scalar(value),
            } if operation_aspect_key == aspect_key => Some(value.clone()),
            PublishedAuthoritativePatchOperation::FieldLevelPatch {
                aspect_key: operation_aspect_key,
                field_sets,
                ..
            } if operation_aspect_key == aspect_key => {
                field_sets.first().map(|field_set| field_set.value.clone())
            }
            _ => None,
        })
}
