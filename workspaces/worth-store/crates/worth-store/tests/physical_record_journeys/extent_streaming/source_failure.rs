use worth_store::physical_runtime::{
    PhysicalRecordInitialization, PhysicalRecordOpen, RecordAppendBatch, RecordAppendDenial,
    RecordAppendError, RecordServingTerminalPosture, RecordStreamFailureKind,
    UnpublishedRecordEffectFate,
};

use super::super::{
    media, scenario_configuration::dense_configuration, stream_fixture::PatternSource, success,
};

#[test]
fn source_length_drift_reports_exact_completed_range_without_publishing() {
    for (source, expected_kind, completed, expected_effect_fate) in [
        (
            PatternSource::truncated(20_000, 12_345),
            RecordStreamFailureKind::SourceEndedEarly,
            12_345,
            UnpublishedRecordEffectFate::DeniedBeforeEffect,
        ),
        (
            PatternSource::overlong(20_000),
            RecordStreamFailureKind::SourceExceededDeclaredLength,
            20_000,
            UnpublishedRecordEffectFate::EffectPossible,
        ),
    ] {
        let parent = tempfile::tempdir().unwrap();
        let root = parent.path().join("store");
        let (format, placement, access) = dense_configuration(4);
        let serving = success(
            media(&root).initialize_record_store(PhysicalRecordInitialization::new(
                format, placement, access,
            )),
        );
        let catalog_before =
            std::fs::read(root.join("families/records/bootstrap.catalog")).unwrap();
        let error = serving
            .record_submission()
            .append_batch(
                RecordAppendBatch::builder()
                    .push_source(source)
                    .build()
                    .unwrap(),
                placement,
            )
            .unwrap_err();
        let RecordAppendError::Unpublished(failure) = error else {
            panic!("known-unpublished stream failure expected")
        };
        let worth_store::physical_runtime::UnpublishedRecordBatchCause::Stream(stream_failure) =
            failure.cause()
        else {
            panic!("stream cause expected")
        };
        assert_eq!(stream_failure.kind(), expected_kind);
        assert_eq!(stream_failure.completed_range(), 0..completed);
        assert_eq!(failure.effect_fate(), expected_effect_fate);
        assert_eq!(
            failure.residue().is_empty(),
            expected_effect_fate == UnpublishedRecordEffectFate::DeniedBeforeEffect
        );
        assert_eq!(
            std::fs::read(root.join("families/records/bootstrap.catalog")).unwrap(),
            catalog_before
        );
        verify_retry_posture(
            serving,
            &root,
            format,
            placement,
            access,
            expected_effect_fate,
        );
    }
}

fn verify_retry_posture(
    serving: worth_store::physical_runtime::ServingPhysicalRuntime,
    root: &std::path::Path,
    format: worth_store::physical_runtime::AdmittedPhysicalRecordFormat,
    placement: worth_store::physical_runtime::AdmittedRecordPlacementPolicy,
    access: worth_store::physical_runtime::AdmittedRecordAccessPolicy,
    effect_fate: UnpublishedRecordEffectFate,
) {
    if effect_fate == UnpublishedRecordEffectFate::EffectPossible {
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
        assert_eq!(
            serving.abort().records().posture(),
            RecordServingTerminalPosture::InspectionRequired
        );
        let reopened =
            success(media(root).open_record_store(PhysicalRecordOpen::new(format, access)));
        assert!(reopened.observed_non_authoritative_residue());
        assert_eq!(
            reopened
                .record_submission()
                .append_batch(
                    RecordAppendBatch::try_from_iter([b"retry".as_slice()]).unwrap(),
                    placement,
                )
                .unwrap_err(),
            RecordAppendError::Denied(RecordAppendDenial::ServingRequiresInspection)
        );
        reopened.abort();
    } else {
        assert!(!serving.observed_non_authoritative_residue());
        serving
            .record_submission()
            .append_batch(
                RecordAppendBatch::try_from_iter([b"retry".as_slice()]).unwrap(),
                placement,
            )
            .unwrap();
        assert_eq!(
            serving.abort().records().posture(),
            RecordServingTerminalPosture::NoInspectionRequired
        );
        let reopened =
            success(media(root).open_record_store(PhysicalRecordOpen::new(format, access)));
        assert!(!reopened.observed_non_authoritative_residue());
        reopened.abort();
    }
}
