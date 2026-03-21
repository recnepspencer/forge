use super::*;

pub(crate) fn patch_detail_contains(
    record: &crate::facade::publication::PatchRecord,
    needle: &str,
) -> bool {
    match &record.detail {
        PatchDetail::StructuredJson(value) => value.to_string().contains(needle),
        PatchDetail::Payload(payload) => payload
            .as_json()
            .map(|value| value.to_string().contains(needle))
            .unwrap_or(false),
        PatchDetail::DenseBitset(_) => false,
    }
}

pub(crate) fn assert_patch_omits_detail(result: &CommitResult, needle: &str) {
    assert!(result
        .patch()
        .iter()
        .all(|record| !patch_detail_contains(record, needle)));
}

pub(crate) fn assert_subscriber_stream_omits_detail(
    runtime: &RelationalRuntime,
    checkpoint: crate::publication::cdc::data::SubscriberCheckpoint,
    needle: &str,
) {
    let subscriber = runtime
        .publication_access()
        .read_subscriber_stream(SubscriberResumeRequest::resume_after(checkpoint, 8))
        .unwrap();
    assert!(subscriber
        .patches
        .iter()
        .flat_map(|patch| patch.records.iter())
        .all(|record| !patch_detail_contains(record, needle)));
}
