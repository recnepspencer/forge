use worth_proof::TransitionOutcome;
use worth_store::physical_runtime::{
    FilesystemMediaAdmission, ManifestEntryCapacity, PageFillPercent, PhysicalRecordInitialization,
    PhysicalRecordPlacementPolicy, PhysicalRuntimeAdmission, PhysicalStore,
    PhysicalWorkOperationFamily, PhysicalWorkRecoveryDisposition, PhysicalWorkRecoveryTarget,
    RecordAppendBatch, RecordByteLimit, RecordCountLimit, RecordReadLimits, RecordScanDenial,
    RecordScanRequest, SegmentPageCount,
};
use worth_store_physical_backend::{
    FilesystemAccessPosture, MediaFaultDirective, MediaOperationRole,
};
use worth_store_physical_format::RecordArtifactFile;

use super::{configuration, media, serving_from_open, success};

#[test]
fn possible_catalog_cutover_is_typed_indeterminate_and_close_adds_no_publication_effect() {
    let control_parent = tempfile::tempdir().unwrap();
    let control_root = control_parent.path().join("control");
    let (format, placement, access) = configuration();
    let control = success(
        media(&control_root)
            .initialize_record_store(PhysicalRecordInitialization::new(format, placement, access)),
    );
    control
        .record_submission()
        .append_batch(
            RecordAppendBatch::try_from_iter([b"published".as_slice()]).unwrap(),
            placement,
        )
        .unwrap();
    let control_after = control.media_counters();
    control.close();

    for (index, (role, expected_stage)) in [
        (
            MediaOperationRole::AtomicReplace,
            worth_store::physical_runtime::RecordPublicationStage::CatalogReplacement,
        ),
        (
            MediaOperationRole::SynchronizeDirectoryPublication,
            worth_store::physical_runtime::RecordPublicationStage::NamespaceSynchronization,
        ),
    ]
    .into_iter()
    .enumerate()
    {
        let parent = tempfile::tempdir().unwrap();
        let root = parent.path().join(format!("store-{index}"));
        let admission = FilesystemMediaAdmission::production(
            FilesystemAccessPosture::CoordinatedServiceAccount,
        );
        let authority = admission.fault_schedule_authority();
        let target_ordinal = control_after.identified_operation_attempts_for(role);
        assert_ne!(target_ordinal, 0);
        let schedule = authority
            .schedule(vec![authority
                .rule(
                    role,
                    target_ordinal,
                    MediaFaultDirective::IndeterminateAfterEffect,
                )
                .for_identified_operation_ordinal()])
            .unwrap();
        let runtime = PhysicalStore::admit(PhysicalRuntimeAdmission::new(&root).unwrap()).unwrap();
        let media = match runtime
            .try_admit_filesystem_media(admission.with_fault_schedule(schedule))
            .into_raw()
        {
            TransitionOutcome::Success(media) => media,
            _ => panic!("fault must target append publication"),
        };
        let serving = success(
            media.initialize_record_store(PhysicalRecordInitialization::new(
                format, placement, access,
            )),
        );
        let error = serving
            .record_submission()
            .append_batch(
                RecordAppendBatch::try_from_iter([b"published".as_slice()]).unwrap(),
                placement,
            )
            .expect_err("C5_PREDICATE:outcome-order");
        let worth_store::physical_runtime::RecordAppendError::Indeterminate(indeterminate) = error
        else {
            panic!("possible catalog cutover must be indeterminate")
        };
        assert_eq!(indeterminate.stage(), expected_stage);
        assert_eq!(
            indeterminate.recovery_locator().candidate_root_generation(),
            Some(2)
        );
        let expected_publication = indeterminate.recovery_locator().publication();
        let catalog_before_close =
            std::fs::read(root.join("families/records/bootstrap.catalog")).unwrap();
        let counters_before_close = serving.media_counters();
        let observer = serving.observer();
        let closed = serving.close();
        assert_eq!(
            closed.records().posture(),
            worth_store::physical_runtime::RecordServingTerminalPosture::InspectionRequired
        );
        assert_eq!(closed.records().residue(), indeterminate.residue());
        assert_eq!(
            std::fs::read(root.join("families/records/bootstrap.catalog")).unwrap(),
            catalog_before_close,
        );
        let counters_after_close = observer.media_counters();
        for observed_role in [
            MediaOperationRole::PositionedWrite,
            MediaOperationRole::SynchronizeFileState,
            MediaOperationRole::AtomicReplace,
            MediaOperationRole::SynchronizeDirectoryPublication,
        ] {
            assert_eq!(
                counters_after_close.attempts_for(observed_role),
                counters_before_close.attempts_for(observed_role),
                "{role:?} close attempted {observed_role:?}",
            );
        }
        let offline = worth_store_offline_verifier::walk_current_durable_record_manifest(
            &root,
            format.declaration(),
        )
        .unwrap();
        assert_eq!(offline.root_generation(), 2);
        assert_eq!(offline.placements().len(), 1);
        assert_eq!(offline.payload_bytes(), b"published".len() as u64);
        let reopened = serving_from_open(&root);
        assert!(!reopened.observed_non_authoritative_residue());
        assert!(!reopened.physical_recovery_evidence_damaged());
        let obligations = reopened.physical_recovery_obligations();
        assert_eq!(obligations.len(), 1);
        let recovery = obligations[0];
        assert_eq!(recovery.store(), reopened.store_identity());
        assert_ne!(recovery.runtime(), 0);
        assert_ne!(recovery.generation(), 0);
        assert_ne!(recovery.operation(), 0);
        assert_eq!(
            recovery.family(),
            PhysicalWorkOperationFamily::ArtifactPublication
        );
        assert_eq!(
            recovery.recovery_disposition(),
            PhysicalWorkRecoveryDisposition::InspectionRequired
        );
        match (role, recovery.target()) {
            (
                MediaOperationRole::AtomicReplace,
                PhysicalWorkRecoveryTarget::CatalogReplacement(
                    RecordArtifactFile::CatalogCandidate { publication },
                ),
            ) if publication == expected_publication => {}
            (
                MediaOperationRole::SynchronizeDirectoryPublication,
                PhysicalWorkRecoveryTarget::RecordNamespaceSynchronization,
            ) => {}
            unexpected => panic!("unexpected catalog recovery target: {unexpected:?}"),
        }
        let denied_scan = reopened.records().scan(
            RecordScanRequest::from_start().with_batch_limit(RecordCountLimit::new(1).unwrap()),
        );
        assert!(matches!(
            denied_scan,
            Err(error) if error.denial() == RecordScanDenial::ServingRequiresInspection
        ));
        let closed = reopened.close();
        assert_eq!(
            closed.records().posture(),
            worth_store::physical_runtime::RecordServingTerminalPosture::InspectionRequired
        );
    }
}

#[test]
fn created_artifact_with_denied_write_seals_serving_authority() {
    let baseline_parent = tempfile::tempdir().unwrap();
    let baseline_root = baseline_parent.path().join("baseline");
    super::serving_from_initialization(&baseline_root).close();
    let baseline = serving_from_open(&baseline_root);
    let prior_writes = baseline
        .media_counters()
        .attempts_for(MediaOperationRole::PositionedWrite);
    baseline.close();

    let parent = tempfile::tempdir().unwrap();
    let root = parent.path().join("store");
    super::serving_from_initialization(&root).close();
    let admission =
        FilesystemMediaAdmission::production(FilesystemAccessPosture::CoordinatedServiceAccount);
    let authority = admission.fault_schedule_authority();
    let schedule = authority
        .schedule(vec![authority.rule(
            MediaOperationRole::PositionedWrite,
            prior_writes + 1,
            MediaFaultDirective::FailBefore {
                kind: std::io::ErrorKind::Other,
                raw_os_error: None,
            },
        )])
        .unwrap();
    let runtime = PhysicalStore::admit(PhysicalRuntimeAdmission::new(&root).unwrap()).unwrap();
    let media = match runtime
        .try_admit_filesystem_media(admission.with_fault_schedule(schedule))
        .into_raw()
    {
        TransitionOutcome::Success(media) => media,
        _ => panic!("fault must target record publication"),
    };
    let (format, placement, access) = configuration();
    let serving = success(media.open_record_store(
        worth_store::physical_runtime::PhysicalRecordOpen::new(format, access),
    ));
    let store_identity = serving.store_identity();
    let error = serving
        .record_submission()
        .append_batch(
            RecordAppendBatch::try_from_iter([b"residue".as_slice()]).unwrap(),
            placement,
        )
        .unwrap_err();
    let worth_store::physical_runtime::RecordAppendError::Unpublished(failure) = &error else {
        panic!("unexpected created-artifact failure: {error:?}");
    };
    let worth_store::physical_runtime::UnpublishedRecordBatchCause::PhysicalWork { stage, failure } =
        failure.cause()
    else {
        panic!("created-artifact failure bypassed physical work: {error:?}");
    };
    assert_eq!(
        *stage,
        worth_store::physical_runtime::RecordPublicationStage::CandidateDataWrite
    );
    assert!(failure.identity().is_some());
    assert!(matches!(
        failure.cause(),
        worth_store::physical_runtime::PhysicalRecordMutationFailureCause::Terminal(
            worth_store::physical_runtime::PhysicalWorkTerminalCause::Backend(_)
        )
    ));
    assert_eq!(
        failure.effect_fate(),
        worth_store::physical_runtime::PhysicalWorkEffectFate::Indeterminate
    );
    assert!(matches!(
        failure.recovery_target(),
        Some(worth_store::physical_runtime::PhysicalWorkRecoveryTarget::Range(_))
    ));
    assert_eq!(
        failure.recovery(),
        Some(worth_store::physical_runtime::PhysicalWorkRecoveryDisposition::InspectionRequired)
    );
    assert!(failure.backend_operation().is_some());
    assert!(matches!(
        serving.record_submission().append_batch(
            RecordAppendBatch::try_from_iter([b"retry".as_slice()]).unwrap(),
            placement,
        ),
        Err(worth_store::physical_runtime::RecordAppendError::Denied(
            worth_store::physical_runtime::RecordAppendDenial::ServingRequiresInspection
        ))
    ));
    serving.abort();

    let offline = worth_store_offline_verifier::walk_current_durable_record_manifest(
        &root,
        format.declaration(),
    )
    .unwrap();
    assert_eq!(offline.root_generation(), 1);
    assert!(offline.placements().is_empty());
    let reopened = serving_from_open(&root);
    assert_eq!(reopened.store_identity(), store_identity);
    reopened.close();
}

#[test]
fn incomplete_rollover_segment_never_replaces_prior_truth() {
    let parent = tempfile::tempdir().unwrap();
    let root = parent.path().join("store");
    let (format, _, access) = configuration();
    let placement = PhysicalRecordPlacementPolicy::builder()
        .segment_pages(SegmentPageCount::new(1).unwrap())
        .page_fill(PageFillPercent::new(50).unwrap())
        .extent_threshold(RecordByteLimit::new(8_100).unwrap())
        .manifest_capacity(ManifestEntryCapacity::new(16).unwrap())
        .admit(format)
        .unwrap();
    let serving = success(
        media(&root)
            .initialize_record_store(PhysicalRecordInitialization::new(format, placement, access)),
    );
    let prior = serving
        .record_submission()
        .append_batch(
            RecordAppendBatch::try_from_iter([vec![0x11; 8_000]]).unwrap(),
            placement,
        )
        .unwrap();
    let catalog_before = std::fs::read(root.join("families/records/bootstrap.catalog")).unwrap();
    serving.close();

    let baseline = media(&root);
    let admission_writes = baseline
        .media_counters()
        .attempts_for(MediaOperationRole::PositionedWrite);
    baseline.close();
    let admission =
        FilesystemMediaAdmission::production(FilesystemAccessPosture::CoordinatedServiceAccount);
    let authority = admission.fault_schedule_authority();
    let schedule = authority
        .schedule(vec![authority.rule(
            MediaOperationRole::PositionedWrite,
            admission_writes + 1,
            MediaFaultDirective::FailBefore {
                kind: std::io::ErrorKind::Other,
                raw_os_error: None,
            },
        )])
        .unwrap();
    let runtime = PhysicalStore::admit(PhysicalRuntimeAdmission::new(&root).unwrap()).unwrap();
    let media = match runtime
        .try_admit_filesystem_media(admission.with_fault_schedule(schedule))
        .into_raw()
    {
        TransitionOutcome::Success(media) => media,
        _ => panic!("fault must target successor-segment data"),
    };
    let serving = success(media.open_record_store(
        worth_store::physical_runtime::PhysicalRecordOpen::new(format, access),
    ));
    assert!(matches!(
        serving.record_submission().append_batch(
            RecordAppendBatch::try_from_iter([vec![0x22; 8_000]]).unwrap(),
            placement,
        ),
        Err(worth_store::physical_runtime::RecordAppendError::Unpublished(_))
    ));
    serving.abort();
    assert_eq!(
        std::fs::read(root.join("families/records/bootstrap.catalog")).unwrap(),
        catalog_before
    );
    let offline = worth_store_offline_verifier::walk_current_durable_record_manifest(
        &root,
        format.declaration(),
    )
    .unwrap();
    assert_eq!(offline.root_generation(), 2);
    assert_eq!(offline.placements().len(), 1);

    let reopened = serving_from_open(&root);
    let error = match reopened.records().open(
        prior.record_id(0).unwrap(),
        RecordReadLimits::new(RecordByteLimit::new(8_000).unwrap()),
    ) {
        Err(error) => error,
        Ok(_) => panic!("inspection-fenced serving must not open a prior record"),
    };
    assert_eq!(
        error.denial(),
        worth_store::physical_runtime::RecordReadDenial::ServingRequiresInspection
    );
    reopened.close();
}
