use worth_proof::TransitionOutcome;
use worth_store::physical_runtime::{
    ExternalPhysicalRecordLocator, FilesystemMediaAdmission, PhysicalRecordOpen,
    PhysicalRuntimeAdmission, PhysicalStore, RecordAppendBatch, RecordAppendDenial,
    RecordAppendError, RecordByteLimit, RecordReadLimits, RecordServingTerminalPosture,
};
use worth_store_physical_backend::{
    FilesystemAccessPosture, MediaFaultDirective, MediaOperationRole,
};

use super::{configuration, serving_from_open, success};

fn synchronization_fault_schedule(
    admission: &FilesystemMediaAdmission,
    ordinal: u64,
    identified: bool,
) -> worth_store_physical_backend::MediaFaultSchedule {
    let authority = admission.fault_schedule_authority();
    let rule = authority.rule(
        MediaOperationRole::SynchronizeFileState,
        ordinal,
        MediaFaultDirective::FailBarrier {
            kind: std::io::ErrorKind::Other,
            raw_os_error: None,
        },
    );
    let rule = if identified {
        rule.for_identified_operation()
    } else {
        rule
    };
    authority.schedule(vec![rule]).unwrap()
}

fn identified_synchronization_fault_schedule(
    admission: &FilesystemMediaAdmission,
    ordinal: u64,
) -> worth_store_physical_backend::MediaFaultSchedule {
    let authority = admission.fault_schedule_authority();
    authority
        .schedule(vec![authority
            .rule(
                MediaOperationRole::SynchronizeFileState,
                ordinal,
                MediaFaultDirective::FailBarrier {
                    kind: std::io::ErrorKind::Other,
                    raw_os_error: None,
                },
            )
            .for_identified_operation_ordinal()])
        .unwrap()
}

fn serving_with_schedule(
    root: &std::path::Path,
    schedule: worth_store_physical_backend::MediaFaultSchedule,
) -> worth_store::physical_runtime::ServingPhysicalRuntime {
    let admission =
        FilesystemMediaAdmission::production(FilesystemAccessPosture::CoordinatedServiceAccount);
    let runtime = PhysicalStore::admit(PhysicalRuntimeAdmission::new(root).unwrap()).unwrap();
    let media = match runtime
        .try_admit_filesystem_media(admission.with_fault_schedule(schedule))
        .into_raw()
    {
        TransitionOutcome::Success(media) => media,
        _ => panic!("fault must target serving work"),
    };
    let (format, _, access) = configuration();
    success(media.open_record_store(PhysicalRecordOpen::new(format, access)))
}

#[test]
fn record_barrier_fault_is_not_laundered_as_recovery_journal_failure() {
    let baseline_parent = tempfile::tempdir().unwrap();
    let baseline_root = baseline_parent.path().join("baseline");
    super::serving_from_initialization(&baseline_root).close();
    let baseline = serving_from_open(&baseline_root);
    let prior_syncs = baseline
        .media_counters()
        .attempts_for(MediaOperationRole::SynchronizeFileState);
    baseline.close();

    let parent = tempfile::tempdir().unwrap();
    let root = parent.path().join("store");
    super::serving_from_initialization(&root).close();
    let admission =
        FilesystemMediaAdmission::production(FilesystemAccessPosture::CoordinatedServiceAccount);
    let schedule = synchronization_fault_schedule(&admission, prior_syncs + 3, true);
    let serving = serving_with_schedule(&root, schedule);
    let (format, placement, _) = configuration();
    let store_identity = serving.records().store_identity();
    let before = serving.media_counters();
    let error = serving
        .record_submission()
        .append_batch(
            RecordAppendBatch::try_from_iter([b"unpublished".as_slice()]).unwrap(),
            placement,
        )
        .unwrap_err();
    let RecordAppendError::Unpublished(failure) = &error else {
        panic!("C5_PREDICATE:publication-durability {error:?}");
    };
    assert_eq!(
        failure.effect_fate(),
        worth_store::physical_runtime::UnpublishedRecordEffectFate::EffectPossible,
        "the completed candidate write must contribute to aggregate publication fate"
    );
    let worth_store::physical_runtime::UnpublishedRecordBatchCause::PhysicalWork { stage, failure } =
        failure.cause()
    else {
        panic!("C5_PREDICATE:publication-durability {error:?}");
    };
    assert_eq!(
        *stage,
        worth_store::physical_runtime::RecordPublicationStage::DataSynchronization
    );
    assert!(failure.identity().is_some());
    assert!(matches!(
        failure.cause(),
        worth_store::physical_runtime::PhysicalRecordMutationFailureCause::Backend(_)
    ));
    assert_eq!(
        failure.effect_fate(),
        worth_store::physical_runtime::PhysicalWorkEffectFate::ProvenNoEffect
    );
    assert_eq!(
        failure.recovery_target(),
        Some(
            worth_store::physical_runtime::PhysicalWorkRecoveryTarget::ArtifactFileSynchronization(
                worth_store_physical_format::RecordArtifactFile::Segment {
                    segment: 1,
                    generation: 1,
                },
            ),
        ),
    );
    assert!(failure.recovery().is_none());
    assert!(failure.backend_operation().is_none());
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
                == worth_store::physical_runtime::RecordReadDenial::ServingRequiresInspection
    ));
    reopened.close();
}

#[test]
fn manifest_sync_failure_cannot_erase_the_accepted_candidate_write() {
    let baseline_parent = tempfile::tempdir().unwrap();
    let baseline_root = baseline_parent.path().join("baseline");
    super::serving_from_initialization(&baseline_root).close();
    let baseline = serving_from_open(&baseline_root);
    let prior_identified_syncs = baseline
        .media_counters()
        .identified_operation_attempts_for(MediaOperationRole::SynchronizeFileState);
    baseline.close();

    let parent = tempfile::tempdir().unwrap();
    let root = parent.path().join("store");
    super::serving_from_initialization(&root).close();
    let admission =
        FilesystemMediaAdmission::production(FilesystemAccessPosture::CoordinatedServiceAccount);
    let schedule =
        identified_synchronization_fault_schedule(&admission, prior_identified_syncs + 2);
    let serving = serving_with_schedule(&root, schedule);
    let (_, placement, _) = configuration();
    let residency_before = serving.residency_counters();

    let outcome = serving.record_submission().append_batch(
        RecordAppendBatch::try_from_iter([b"manifest-sync-fault".as_slice()]).unwrap(),
        placement,
    );
    let error = match outcome {
        Err(error) => error,
        Ok(_) => panic!(
            "C5_PREDICATE:publication-durability manifest publication omitted its file-state barrier"
        ),
    };
    let RecordAppendError::Unpublished(failure) = &error else {
        panic!("manifest synchronization must remain unpublished: {error:?}");
    };
    assert_eq!(
        failure.effect_fate(),
        worth_store::physical_runtime::UnpublishedRecordEffectFate::EffectPossible,
        "two completed writes precede the no-effect synchronization failure"
    );
    assert_eq!(
        failure.world_fate(),
        worth_store::physical_runtime::UnpublishedRecordWorldFate::InspectionRequired
    );
    let worth_store::physical_runtime::UnpublishedRecordBatchCause::PhysicalWork { stage, failure } =
        failure.cause()
    else {
        panic!("manifest synchronization bypassed canonical physical work: {error:?}");
    };
    assert_eq!(
        *stage,
        worth_store::physical_runtime::RecordPublicationStage::ManifestSynchronization,
        "C5_PREDICATE:publication-durability manifest publication skipped its file-state barrier"
    );
    assert_eq!(
        failure.effect_fate(),
        worth_store::physical_runtime::PhysicalWorkEffectFate::ProvenNoEffect
    );

    let residency_after = serving.residency_counters();
    assert_eq!(
        residency_after.candidate_publications(),
        residency_before.candidate_publications() + 2,
        "the data and manifest writes must both be accepted before the later sync failure"
    );
    assert_eq!(
        residency_after.administrative_drains(),
        residency_before.administrative_drains(),
        "a no-effect synchronization failure must not be reinterpreted as a no-effect frame write"
    );
    assert_eq!(residency_after.dirty_frames(), 0);
    assert_eq!(residency_after.candidate_frames(), 0);
    assert_eq!(residency_after.pinned_frames(), 0);
    assert_eq!(residency_after.pin_leases(), 0);
    assert!(!serving.abort().residency().requires_inspection());
}

#[test]
fn recovery_journal_barrier_failure_fences_without_record_effect() {
    let parent = tempfile::tempdir().unwrap();
    let root = parent.path().join("store");
    super::serving_from_initialization(&root).close();
    let admission =
        FilesystemMediaAdmission::production(FilesystemAccessPosture::CoordinatedServiceAccount);
    let schedule = synchronization_fault_schedule(&admission, 1, false);
    let serving = serving_with_schedule(&root, schedule);
    let (_, placement, _) = configuration();
    let before = serving.media_counters();
    let error = serving
        .record_submission()
        .append_batch(
            RecordAppendBatch::try_from_iter([b"journal-fault".as_slice()]).unwrap(),
            placement,
        )
        .unwrap_err();
    let RecordAppendError::Denied(RecordAppendDenial::PhysicalWorkUnavailable(failure)) = error
    else {
        panic!("journal preparation must deny before record effect: {error:?}");
    };
    assert!(failure.identity().is_some());
    assert_eq!(
        failure.cause(),
        worth_store::physical_runtime::PhysicalRecordMutationFailureCause::PreEffect(
            worth_store::physical_runtime::PhysicalWorkPreEffectDenial::RecoveryJournalUnavailable
        )
    );
    assert_eq!(
        failure.effect_fate(),
        worth_store::physical_runtime::PhysicalWorkEffectFate::ProvenNoEffect
    );
    let after = serving.media_counters();
    assert_eq!(after.replacements(), before.replacements());
    assert_eq!(
        after.attempts_for(MediaOperationRole::PositionedWrite),
        before.attempts_for(MediaOperationRole::PositionedWrite)
    );
    assert!(matches!(
        serving.record_submission().append_batch(
            RecordAppendBatch::try_from_iter([b"retry".as_slice()]).unwrap(),
            placement,
        ),
        Err(RecordAppendError::Denied(
            RecordAppendDenial::ServingRequiresInspection
        ))
    ));
    let shutdown = serving.abort();
    assert_eq!(
        shutdown.records().posture(),
        RecordServingTerminalPosture::InspectionRequired
    );
    let reopened = serving_from_open(&root);
    assert_eq!(reopened.physical_recovery_obligations().len(), 1);
    assert!(matches!(
        reopened.record_submission().append_batch(
            RecordAppendBatch::try_from_iter([b"fresh-retry".as_slice()]).unwrap(),
            placement,
        ),
        Err(RecordAppendError::Denied(
            RecordAppendDenial::ServingRequiresInspection
        ))
    ));
    reopened.abort();
}
