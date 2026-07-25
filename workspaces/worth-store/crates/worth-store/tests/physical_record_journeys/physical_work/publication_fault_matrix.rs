use worth_store::physical_runtime::{
    FilesystemMediaAdmission, IndeterminateRecordPublicationCause,
    PhysicalRecordMutationFailureEvidence, PhysicalWorkEffectFate, PhysicalWorkRecoveryTarget,
    RecordAppendBatch, RecordAppendError, RecordPublicationStage, RecordServingTerminalPosture,
    UnpublishedRecordBatchCause, UnpublishedRecordEffectFate, UnpublishedRecordWorldFate,
};
use worth_store_physical_backend::{
    FilesystemAccessPosture, MediaCounterSnapshot, MediaFaultDirective, MediaOperationRole,
};
use worth_store_physical_format::RecordArtifactFile;

use super::{configuration, serving_from_initialization};

#[test]
fn every_publication_stage_reports_typed_failure_from_real_effects() {
    let calibration = calibrate();
    for case in cases(
        calibration.file_sync_count,
        calibration.directory_sync_count,
    ) {
        exercise(case, calibration.before);
    }
}

#[derive(Clone, Copy)]
struct Calibration {
    before: MediaCounterSnapshot,
    file_sync_count: u64,
    directory_sync_count: u64,
}

#[derive(Clone, Copy)]
struct FaultCase {
    name: &'static str,
    stage: RecordPublicationStage,
    role: MediaOperationRole,
    offset: u64,
    outcome: ExpectedOutcome,
    target: ExpectedTarget,
}

#[derive(Clone, Copy)]
enum ExpectedOutcome {
    Unpublished(PhysicalWorkEffectFate),
    Indeterminate,
}

#[derive(Clone, Copy)]
enum ExpectedTarget {
    DataRange,
    DataSync,
    PayloadManifestSync,
    ManifestSync,
    CatalogCandidateSync,
    CatalogReplacement,
    NamespaceSync,
}

fn calibrate() -> Calibration {
    let parent = tempfile::tempdir().unwrap();
    let root = parent.path().join("control");
    serving_from_initialization(&root).close();
    let serving = super::super::serving_from_open(&root);
    let before = serving.media_counters();
    let (_, placement, _) = configuration();
    serving
        .record_submission()
        .append_batch(
            RecordAppendBatch::try_from_iter([vec![0xA5; 20_000]]).unwrap(),
            placement,
        )
        .unwrap();
    let after = serving.media_counters();
    let file_sync_count = after
        .identified_operation_attempts_for(MediaOperationRole::SynchronizeFileState)
        - before.identified_operation_attempts_for(MediaOperationRole::SynchronizeFileState);
    let directory_sync_count = after
        .identified_operation_attempts_for(MediaOperationRole::SynchronizeDirectoryPublication)
        - before
            .identified_operation_attempts_for(MediaOperationRole::SynchronizeDirectoryPublication);
    assert!(
        file_sync_count >= 4,
        "extent publication must expose data, payload-manifest, manifest, and catalog syncs"
    );
    assert!(
        directory_sync_count >= 2,
        "publication must expose recovery-journal and record-namespace directory syncs"
    );
    assert!(!serving.close_plan().execute().requires_inspection());
    Calibration {
        before,
        file_sync_count,
        directory_sync_count,
    }
}

fn cases(last_file_sync: u64, last_directory_sync: u64) -> [FaultCase; 7] {
    [
        fault(
            "candidate-data",
            RecordPublicationStage::CandidateDataWrite,
            MediaOperationRole::PositionedWrite,
            1,
            ExpectedOutcome::Unpublished(PhysicalWorkEffectFate::Indeterminate),
            ExpectedTarget::DataRange,
        ),
        fault(
            "data-sync",
            RecordPublicationStage::DataSynchronization,
            MediaOperationRole::SynchronizeFileState,
            1,
            ExpectedOutcome::Unpublished(PhysicalWorkEffectFate::ProvenNoEffect),
            ExpectedTarget::DataSync,
        ),
        fault(
            "payload-manifest-sync",
            RecordPublicationStage::PayloadManifestSynchronization,
            MediaOperationRole::SynchronizeFileState,
            2,
            ExpectedOutcome::Unpublished(PhysicalWorkEffectFate::ProvenNoEffect),
            ExpectedTarget::PayloadManifestSync,
        ),
        fault(
            "manifest-sync",
            RecordPublicationStage::ManifestSynchronization,
            MediaOperationRole::SynchronizeFileState,
            3,
            ExpectedOutcome::Unpublished(PhysicalWorkEffectFate::ProvenNoEffect),
            ExpectedTarget::ManifestSync,
        ),
        fault(
            "catalog-candidate-sync",
            RecordPublicationStage::CatalogCandidateSynchronization,
            MediaOperationRole::SynchronizeFileState,
            last_file_sync,
            ExpectedOutcome::Unpublished(PhysicalWorkEffectFate::ProvenNoEffect),
            ExpectedTarget::CatalogCandidateSync,
        ),
        fault(
            "catalog-replacement",
            RecordPublicationStage::CatalogReplacement,
            MediaOperationRole::AtomicReplace,
            1,
            ExpectedOutcome::Indeterminate,
            ExpectedTarget::CatalogReplacement,
        ),
        fault(
            "namespace-sync",
            RecordPublicationStage::NamespaceSynchronization,
            MediaOperationRole::SynchronizeDirectoryPublication,
            last_directory_sync,
            ExpectedOutcome::Indeterminate,
            ExpectedTarget::NamespaceSync,
        ),
    ]
}

const fn fault(
    name: &'static str,
    stage: RecordPublicationStage,
    role: MediaOperationRole,
    offset: u64,
    outcome: ExpectedOutcome,
    target: ExpectedTarget,
) -> FaultCase {
    FaultCase {
        name,
        stage,
        role,
        offset,
        outcome,
        target,
    }
}

fn exercise(case: FaultCase, calibrated_before: MediaCounterSnapshot) {
    let parent = tempfile::tempdir().unwrap();
    let root = parent.path().join(case.name);
    serving_from_initialization(&root).close();
    let admission =
        FilesystemMediaAdmission::production(FilesystemAccessPosture::CoordinatedServiceAccount);
    let authority = admission.fault_schedule_authority();
    let baseline = calibrated_before.identified_operation_attempts_for(case.role);
    let directive = match case.outcome {
        ExpectedOutcome::Unpublished(PhysicalWorkEffectFate::ProvenNoEffect) => {
            MediaFaultDirective::FailBarrier {
                kind: std::io::ErrorKind::Other,
                raw_os_error: None,
            }
        }
        ExpectedOutcome::Unpublished(_) => MediaFaultDirective::FailBefore {
            kind: std::io::ErrorKind::Other,
            raw_os_error: None,
        },
        ExpectedOutcome::Indeterminate => MediaFaultDirective::IndeterminateAfterEffect,
    };
    let schedule = authority
        .schedule(vec![authority
            .rule(case.role, baseline + case.offset, directive)
            .for_identified_operation_ordinal()])
        .unwrap();
    let serving = super::fault_fixture::serving_from_open_with_schedule(&root, schedule);
    assert_eq!(
        serving
            .media_counters()
            .identified_operation_attempts_for(case.role),
        baseline,
        "{} calibration drifted before append",
        case.name
    );
    let (_, placement, _) = configuration();
    let before = serving.media_counters();
    let invalidations_before = serving
        .physical_signal_observation()
        .unwrap()
        .aspect_invalidation_count();
    let error = serving
        .record_submission()
        .append_batch(
            RecordAppendBatch::try_from_iter([vec![0x5A; 20_000]]).unwrap(),
            placement,
        )
        .unwrap_err();
    let evidence = assert_outcome(case, &error);
    assert_target(case.target, evidence.recovery_target(), case.name);
    assert!(error_trace(&error)
        .iter()
        .any(|effect| effect.stage() == case.stage));
    assert_eq!(
        serving.media_counters().fault_matches(),
        before.fault_matches() + 1,
        "{} did not hit exactly one scheduled backend fault",
        case.name
    );
    assert_eq!(
        serving
            .physical_signal_observation()
            .unwrap()
            .aspect_invalidation_count(),
        invalidations_before,
        "{} failed mutation must not emit a read-dependency delta",
        case.name
    );
    assert_eq!(
        serving.abort().records().posture(),
        RecordServingTerminalPosture::InspectionRequired,
        "{} failure did not retain record inspection posture",
        case.name,
    );
}

fn assert_outcome(
    case: FaultCase,
    error: &RecordAppendError,
) -> PhysicalRecordMutationFailureEvidence {
    match (case.outcome, error) {
        (ExpectedOutcome::Unpublished(inner_fate), RecordAppendError::Unpublished(failure)) => {
            assert_eq!(
                failure.effect_fate(),
                UnpublishedRecordEffectFate::EffectPossible
            );
            assert_eq!(
                failure.world_fate(),
                UnpublishedRecordWorldFate::InspectionRequired
            );
            let UnpublishedRecordBatchCause::PhysicalWork { stage, failure } = failure.cause()
            else {
                panic!("{} lost its physical-work cause: {error:?}", case.name);
            };
            assert_eq!(*stage, case.stage, "{}", case.name);
            assert_eq!(failure.effect_fate(), inner_fate, "{}", case.name);
            **failure
        }
        (ExpectedOutcome::Indeterminate, RecordAppendError::Indeterminate(failure)) => {
            assert_eq!(failure.stage(), case.stage, "{}", case.name);
            let IndeterminateRecordPublicationCause::PhysicalWork(evidence) = failure.cause()
            else {
                panic!("{} lost its physical-work cause: {error:?}", case.name);
            };
            assert_ne!(
                evidence.effect_fate(),
                PhysicalWorkEffectFate::ProvenNoEffect,
                "{} indeterminate stage claimed no effect",
                case.name
            );
            **evidence
        }
        _ => panic!("{} produced the wrong outcome: {error:?}", case.name),
    }
}

fn error_trace(
    error: &RecordAppendError,
) -> &[worth_store::physical_runtime::RecordPublicationWorkEffect] {
    match error {
        RecordAppendError::Unpublished(failure) => failure.physical_work().effects(),
        RecordAppendError::Indeterminate(failure) => failure.physical_work().effects(),
        unexpected => panic!("fault matrix produced non-publication error: {unexpected:?}"),
    }
}

fn assert_target(
    expected: ExpectedTarget,
    observed: Option<PhysicalWorkRecoveryTarget>,
    name: &str,
) {
    let matches = match (expected, observed) {
        (ExpectedTarget::DataRange, Some(PhysicalWorkRecoveryTarget::Range(_))) => true,
        (
            ExpectedTarget::DataSync,
            Some(PhysicalWorkRecoveryTarget::ArtifactFileSynchronization(
                RecordArtifactFile::Extent { .. },
            )),
        ) => true,
        (
            ExpectedTarget::PayloadManifestSync,
            Some(PhysicalWorkRecoveryTarget::ArtifactFileSynchronization(
                RecordArtifactFile::ExtentManifest { .. },
            )),
        ) => true,
        (
            ExpectedTarget::ManifestSync,
            Some(PhysicalWorkRecoveryTarget::ArtifactFileSynchronization(artifact)),
        ) => !matches!(
            artifact,
            RecordArtifactFile::Extent { .. }
                | RecordArtifactFile::ExtentManifest { .. }
                | RecordArtifactFile::CatalogCandidate { .. }
        ),
        (
            ExpectedTarget::CatalogCandidateSync,
            Some(PhysicalWorkRecoveryTarget::ArtifactFileSynchronization(
                RecordArtifactFile::CatalogCandidate { .. },
            )),
        ) => true,
        (
            ExpectedTarget::CatalogReplacement,
            Some(PhysicalWorkRecoveryTarget::CatalogReplacement(
                RecordArtifactFile::CatalogCandidate { .. },
            )),
        ) => true,
        (
            ExpectedTarget::NamespaceSync,
            Some(PhysicalWorkRecoveryTarget::RecordNamespaceSynchronization),
        ) => true,
        _ => false,
    };
    assert!(
        matches,
        "{name} exposed wrong recovery target: {observed:?}"
    );
}
