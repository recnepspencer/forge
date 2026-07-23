use worth_proof::TransitionOutcome;
use worth_store::physical_runtime::{
    ExternalPhysicalRecordLocator, FilesystemMediaAdmission, ManifestEntryCapacity,
    PageFillPercent, PhysicalRecordInitialization, PhysicalRecordPlacementPolicy,
    PhysicalRuntimeAdmission, PhysicalStore, RecordAppendBatch, RecordByteLimit, RecordReadLimits,
    SegmentPageCount,
};
use worth_store_physical_backend::{
    FilesystemAccessPosture, MediaFaultDirective, MediaOperationRole,
};

use super::{configuration, media, read_record, serving_from_open, success};

#[test]
fn possible_catalog_cutover_is_typed_indeterminate_and_close_adds_no_publication_effect() {
    let control_parent = tempfile::tempdir().unwrap();
    let control_root = control_parent.path().join("control");
    let (format, placement, access) = configuration();
    let mut control = success(
        media(&control_root)
            .initialize_record_store(PhysicalRecordInitialization::new(format, placement, access)),
    );
    control
        .records_mut()
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
        let schedule = authority
            .schedule(vec![authority.rule(
                role,
                control_after.attempts_for(role),
                MediaFaultDirective::IndeterminateAfterEffect,
            )])
            .unwrap();
        let runtime = PhysicalStore::admit(PhysicalRuntimeAdmission::new(&root).unwrap()).unwrap();
        let media = match runtime
            .try_admit_filesystem_media(admission.with_fault_schedule(schedule))
            .into_raw()
        {
            TransitionOutcome::Success(media) => media,
            _ => panic!("fault must target append publication"),
        };
        let mut serving = success(
            media.initialize_record_store(PhysicalRecordInitialization::new(
                format, placement, access,
            )),
        );
        let error = serving
            .records_mut()
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
            2
        );
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
        let reopened = serving_from_open(&root);
        let found = super::scan_journeys::collect_scan(&reopened, 1, 32);
        assert_eq!(found.len(), 1);
        assert_eq!(found[0].1, b"published");
        reopened.close();
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
    let mut serving = success(media.open_record_store(
        worth_store::physical_runtime::PhysicalRecordOpen::new(format, access),
    ));
    let store_identity = serving.store_identity();
    let error = serving
        .records_mut()
        .append_batch(
            RecordAppendBatch::try_from_iter([b"residue".as_slice()]).unwrap(),
            placement,
        )
        .unwrap_err();
    assert!(matches!(
        error,
        worth_store::physical_runtime::RecordAppendError::Unpublished(ref failure)
            if matches!(failure.cause(),
                worth_store::physical_runtime::UnpublishedRecordBatchCause::Backend {
                    stage: worth_store::physical_runtime::RecordPublicationStage::CandidateDataWrite,
                    ..
                })
    ));
    assert!(matches!(
        serving.records_mut().append_batch(
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
fn publication_barrier_omission_is_observable() {
    let baseline_parent = tempfile::tempdir().unwrap();
    let baseline_root = baseline_parent.path().join("baseline");
    super::serving_from_initialization(&baseline_root).close();
    let baseline = serving_from_open(&baseline_root);
    let prior_file_sync_attempts = baseline
        .media_counters()
        .attempts_for(MediaOperationRole::SynchronizeFileState);
    baseline.close();

    let parent = tempfile::tempdir().unwrap();
    let root = parent.path().join("store");
    super::serving_from_initialization(&root).close();
    let admission =
        FilesystemMediaAdmission::production(FilesystemAccessPosture::CoordinatedServiceAccount);
    let authority = admission.fault_schedule_authority();
    let schedule = authority
        .schedule(vec![authority.rule(
            MediaOperationRole::SynchronizeFileState,
            prior_file_sync_attempts + 1,
            MediaFaultDirective::FailBarrier {
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
        _ => panic!("fault must target record publication, not media admission"),
    };
    let (format, placement, access) = configuration();
    let mut serving = success(media.open_record_store(
        worth_store::physical_runtime::PhysicalRecordOpen::new(format, access),
    ));
    let store_identity = serving.records().store_identity();
    let before = serving.media_counters();
    let error = serving
        .records_mut()
        .append_batch(
            RecordAppendBatch::try_from_iter([b"unpublished".as_slice()]).unwrap(),
            placement,
        )
        .unwrap_err();
    assert!(
        matches!(
            error,
            worth_store::physical_runtime::RecordAppendError::Unpublished(ref failure)
                if matches!(failure.cause(),
                    worth_store::physical_runtime::UnpublishedRecordBatchCause::Backend {
                        stage: worth_store::physical_runtime::RecordPublicationStage::DataSynchronization,
                        ..
                    })
        ),
        "C5_PREDICATE:publication-durability"
    );
    let after = serving.media_counters();
    assert_eq!(after.replacements(), before.replacements());
    assert_eq!(after.fault_matches(), before.fault_matches() + 1);
    let candidate_page = std::fs::read(
        root.join("families/records/segments/segment-0000000000000001-0000000000000001.pages"),
    )
    .unwrap();
    let mut unpublished_locator = [0_u8; 40];
    unpublished_locator[..16].copy_from_slice(&store_identity.bytes());
    unpublished_locator[16..40].copy_from_slice(&candidate_page[48..72]);
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
    assert!(matches!(
        reopened.records().open_external(
            ExternalPhysicalRecordLocator::decode(unpublished_locator).unwrap(),
            RecordReadLimits::new(RecordByteLimit::new(1024).unwrap()),
        ),
        Err(error)
            if error.denial()
                == worth_store::physical_runtime::RecordReadDenial::RecordNotFound
    ));
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
    let mut serving = success(
        media(&root)
            .initialize_record_store(PhysicalRecordInitialization::new(format, placement, access)),
    );
    let prior = serving
        .records_mut()
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
    let mut serving = success(media.open_record_store(
        worth_store::physical_runtime::PhysicalRecordOpen::new(format, access),
    ));
    assert!(matches!(
        serving.records_mut().append_batch(
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
    let session = reopened
        .records()
        .open(
            prior.record_id(0).unwrap(),
            RecordReadLimits::new(RecordByteLimit::new(8_000).unwrap()),
        )
        .unwrap();
    assert_eq!(read_record(session, 8_000).0, vec![0x11; 8_000]);
    reopened.close();
}
