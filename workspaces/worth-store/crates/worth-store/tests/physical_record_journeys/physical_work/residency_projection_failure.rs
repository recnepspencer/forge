use worth_proof::TransitionOutcome;
use worth_store::physical_runtime::{
    PhysicalManifestCapacityTransition, PhysicalMutationIdempotencyMaterial,
    PhysicalMutationPreparationDenial, RecordAppendBatch, RecordAppendDenial,
};
use worth_store_physical_format::{RecordArtifactFile, RecordFrameCoordinate};

use super::{configuration, serving_from_initialization};

#[test]
fn semantic_rejection_consumes_the_admitted_residency_projection_failure() {
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
        .certification_physical_residency()
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
    assert!(matches!(
        super::super::durable_publication::prepare_single(
            &serving.record_submission(),
            placement,
            PhysicalManifestCapacityTransition::PreserveCurrent,
            PhysicalMutationIdempotencyMaterial::new([212; 32]),
            RecordAppendBatch::try_from_iter([b"fenced".as_slice()]).unwrap(),
        )
        .into_raw(),
        TransitionOutcome::Denied(PhysicalMutationPreparationDenial::RecordAppend(
            RecordAppendDenial::ServingRequiresInspection
        ))
    ));
    assert!(serving.close_plan().execute().requires_inspection());
}
