use worth_store::physical_runtime::{RecordAppendBatch, RecordAppendDenial, RecordAppendError};
use worth_store_physical_format::{RecordArtifactFile, RecordFrameCoordinate};

use super::{configuration, serving_from_initialization};

#[test]
fn c6_semantic_rejection_consumes_the_admitted_projection_failure_capability() {
    let parent = tempfile::tempdir().unwrap();
    let serving = serving_from_initialization(&parent.path().join("store"));
    let (_, placement, _) = configuration();
    let invalidations_before = serving
        .physical_signal_observation()
        .unwrap()
        .aspect_invalidation_count();
    let coordinate =
        RecordFrameCoordinate::new(RecordArtifactFile::BootstrapCatalog, 0, 8).unwrap();
    let frame = serving
        .c6_physical_work_handoff()
        .residency_work()
        .pin_exact(coordinate)
        .unwrap();
    assert_eq!(frame.physical_work_count(), 1);

    frame.reject_projection_failure();

    assert_eq!(
        serving
            .physical_signal_observation()
            .unwrap()
            .aspect_invalidation_count(),
        invalidations_before + 1
    );
    assert_eq!(
        serving
            .record_submission()
            .append_batch(
                RecordAppendBatch::try_from_iter([b"fenced".as_slice()]).unwrap(),
                placement,
            )
            .unwrap_err(),
        RecordAppendError::Denied(RecordAppendDenial::ServingRequiresInspection)
    );
    assert!(serving.close_plan().execute().requires_inspection());
}
