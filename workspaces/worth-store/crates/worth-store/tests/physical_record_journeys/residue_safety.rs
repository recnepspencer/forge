use worth_store::physical_runtime::{
    RecordAppendBatch, RecordAppendDenial, RecordAppendError, RecordServingTerminalPosture,
    UnpublishedRecordEffectFate, UnpublishedRecordWorldFate,
};

use super::{configuration, serving_from_initialization};

#[test]
fn deterministic_page_residue_requires_inspection_instead_of_retry() {
    let parent = tempfile::tempdir().unwrap();
    let root = parent.path().join("store");
    let serving = serving_from_initialization(&root);
    let orphan =
        root.join("families/records/segments/segment-0000000000000001-0000000000000001.pages");
    std::fs::write(&orphan, b"existing residue").unwrap();
    let (_, placement, _) = configuration();
    let error = serving
        .record_submission()
        .append_batch(
            RecordAppendBatch::try_from_iter([b"must not overwrite".as_slice()]).unwrap(),
            placement,
        )
        .unwrap_err();
    let RecordAppendError::Unpublished(failure) = error else {
        panic!("pre-existing candidate residue must be an unpublished failure: {error:?}");
    };
    assert_eq!(
        failure.effect_fate(),
        UnpublishedRecordEffectFate::DeniedBeforeEffect
    );
    assert_eq!(
        failure.world_fate(),
        UnpublishedRecordWorldFate::InspectionRequired
    );
    assert!(!failure.residue().is_empty());
    assert_eq!(std::fs::read(orphan).unwrap(), b"existing residue");
    assert_eq!(
        serving
            .record_submission()
            .append_batch(
                RecordAppendBatch::try_from_iter([b"retry".as_slice()]).unwrap(),
                placement,
            )
            .unwrap_err(),
        RecordAppendError::Denied(RecordAppendDenial::ServingRequiresInspection)
    );
    let shutdown = serving.abort();
    assert_eq!(
        shutdown.records().posture(),
        RecordServingTerminalPosture::InspectionRequired
    );
}
