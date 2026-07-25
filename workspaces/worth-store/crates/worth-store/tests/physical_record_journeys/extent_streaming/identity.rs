use worth_store::physical_runtime::{
    PhysicalRecordInitialization, PhysicalRecordOpen, RecordAppendBatch,
};

use super::super::{
    media, scenario_configuration::dense_configuration, stream_fixture::PatternSource, success,
};

pub(super) fn prove_abandoned_candidate_non_reuse() {
    let parent = tempfile::tempdir().unwrap();
    let root = parent.path().join("store");
    let (format, placement, access) = dense_configuration(4);
    let serving = success(
        media(&root)
            .initialize_record_store(PhysicalRecordInitialization::new(format, placement, access)),
    );
    let error = serving
        .record_submission()
        .append_batch(
            RecordAppendBatch::builder()
                .push_source(PatternSource::overlong(20_000))
                .build()
                .unwrap(),
            placement,
        )
        .unwrap_err();
    assert!(
        matches!(
            error,
            worth_store::physical_runtime::RecordAppendError::Unpublished(ref failure)
                if matches!(failure.cause(), worth_store::physical_runtime::UnpublishedRecordBatchCause::Stream(_))
        ),
        "unexpected abandoned-candidate failure: {error:?}"
    );
    serving.abort();

    let orphan =
        root.join("families/records/extents/extent-0000000000000001-0000000000000001.data");
    let orphan_bytes = std::fs::read(&orphan).unwrap();
    let abandoned_epoch: [u8; 16] = orphan_bytes[40..56].try_into().unwrap();
    std::fs::remove_file(orphan).unwrap();

    let reopened = success(media(&root).open_record_store(PhysicalRecordOpen::new(format, access)));
    let published = reopened
        .record_submission()
        .append_batch(
            RecordAppendBatch::builder()
                .push_source(PatternSource::exact(20_000))
                .build()
                .unwrap(),
            placement,
        )
        .unwrap();
    assert_ne!(
        published.record_id(0).unwrap().allocation_epoch(),
        abandoned_epoch,
        "C5_PREDICATE:identity-authority"
    );
    reopened.close();
}
