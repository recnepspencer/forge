use worth_store::physical_runtime::{
    PhysicalRecordInitialization, PhysicalRecordOpen, RecordAppendBatch,
};

use super::super::{
    media, scenario_configuration::dense_configuration, stream_fixture::PatternSource, success,
};

#[test]
fn fresh_open_reports_known_unpublished_extent_residue_and_blocks_collision() {
    let parent = tempfile::tempdir().unwrap();
    let root = parent.path().join("store");
    let (format, placement, access) = dense_configuration(4);
    let serving = success(initialize_record_store!(media(&root), |durability| {
        PhysicalRecordInitialization::new(format, placement, access, durability)
    }));
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
    assert!(matches!(
        error,
        worth_store::physical_runtime::RecordAppendError::Unpublished(_)
    ));
    serving.abort();

    let reopened = success(open_record_store!(media(&root), |durability| {
        PhysicalRecordOpen::new(format, access, durability)
    }));
    let residue = reopened.publication_residue();
    assert!(residue.next_extent_artifacts());
    assert!(!residue.successor_root());
    assert!(matches!(
        reopened.record_submission().append_batch(
            RecordAppendBatch::try_from_iter([b"must not collide".as_slice()]).unwrap(),
            placement,
        ),
        Err(worth_store::physical_runtime::RecordAppendError::Denied(
            worth_store::physical_runtime::RecordAppendDenial::ServingRequiresInspection
        ))
    ));
    reopened.close();
}
