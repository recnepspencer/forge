use worth_proof::TransitionOutcome;
use worth_store::physical_runtime::{
    ExternalPhysicalRecordLocator, FilesystemMediaAdmission, PhysicalRecordOpen,
    PhysicalRuntimeAdmission, PhysicalStore, RecordAppendBatch, RecordAppendError, RecordByteLimit,
    RecordReadDenial, RecordReadLimits, UnpublishedRecordBatchCause,
};
use worth_store_physical_backend::{
    FilesystemAccessPosture, MediaFaultDirective, MediaOperationRole,
};
use worth_store_physical_format::inspect_inline_page_records;

use super::{configuration, media, serving_from_initialization, success};

#[test]
fn premature_identity_subset_and_success_mutants_fail_causally() {
    let baseline_parent = tempfile::tempdir().unwrap();
    let baseline_root = baseline_parent.path().join("baseline");
    serving_from_initialization(&baseline_root).close();
    let baseline = media(&baseline_root);
    let prior_syncs = baseline
        .media_counters()
        .attempts_for(MediaOperationRole::SynchronizeFileState);
    baseline.close();

    let parent = tempfile::tempdir().unwrap();
    let root = parent.path().join("store");
    serving_from_initialization(&root).close();
    let admission =
        FilesystemMediaAdmission::production(FilesystemAccessPosture::CoordinatedServiceAccount);
    let authority = admission.fault_schedule_authority();
    let schedule = authority
        .schedule(vec![authority
            .rule(
                MediaOperationRole::SynchronizeFileState,
                prior_syncs + 3,
                MediaFaultDirective::FailBarrier {
                    kind: std::io::ErrorKind::Other,
                    raw_os_error: None,
                },
            )
            .for_identified_operation()])
        .unwrap();
    let runtime = PhysicalStore::admit(PhysicalRuntimeAdmission::new(&root).unwrap()).unwrap();
    let admitted = runtime
        .try_admit_filesystem_media(admission.with_fault_schedule(schedule))
        .into_raw();
    let TransitionOutcome::Success(media) = admitted else {
        panic!("the controlled mutation must target publication")
    };
    let (format, placement, access) = configuration();
    let serving = success(media.open_record_store(PhysicalRecordOpen::new(format, access)));
    let store = serving.store_identity();
    let outcome = serving.record_submission().append_batch(
        RecordAppendBatch::try_from_iter([
            b"batch-a".as_slice(),
            b"batch-b".as_slice(),
            b"batch-c".as_slice(),
        ])
        .unwrap(),
        placement,
    );
    let RecordAppendError::Unpublished(failure) = outcome.as_ref().unwrap_err() else {
        panic!(
            "a failed required barrier must never return Published: outcome={outcome:?}, counters={:?}",
            serving.media_counters()
        )
    };
    assert_eq!(
        failure.attempted_records(),
        3,
        "C5_PREDICATE:batch-atomicity"
    );
    let UnpublishedRecordBatchCause::PhysicalWork {
        stage,
        failure: physical,
    } = failure.cause()
    else {
        panic!("barrier fault bypassed canonical physical work: {failure:?}");
    };
    assert_eq!(
        *stage,
        worth_store::physical_runtime::RecordPublicationStage::DataSynchronization
    );
    assert!(physical.identity().is_some());
    assert!(matches!(
        physical.cause(),
        worth_store::physical_runtime::PhysicalRecordMutationFailureCause::Backend(_)
    ));
    assert_eq!(
        physical.effect_fate(),
        worth_store::physical_runtime::PhysicalWorkEffectFate::ProvenNoEffect
    );

    let candidate = std::fs::read_dir(root.join("families/records/segments"))
        .unwrap()
        .map(|entry| entry.unwrap().path())
        .find(|path| {
            path.extension()
                .is_some_and(|extension| extension == "pages")
        })
        .expect("candidate data must make premature identities observable");
    let descriptors =
        inspect_inline_page_records(format.declaration(), &std::fs::read(candidate).unwrap())
            .unwrap();
    assert_eq!(descriptors.len(), 3);
    serving.abort();

    let reopened = super::serving_from_open(&root);
    assert!(matches!(
        reopened.records().scan(
            worth_store::physical_runtime::RecordScanRequest::from_start()
                .with_batch_limit(worth_store::physical_runtime::RecordCountLimit::new(3).unwrap())
        ),
        Err(error)
            if error.denial()
                == worth_store::physical_runtime::RecordScanDenial::ServingRequiresInspection
    ));
    for descriptor in descriptors {
        let mut encoded = [0_u8; 40];
        encoded[..16].copy_from_slice(&store.bytes());
        encoded[16..32].copy_from_slice(&descriptor.record().allocation_epoch());
        encoded[32..].copy_from_slice(&descriptor.record().ordinal().to_le_bytes());
        let locator = ExternalPhysicalRecordLocator::decode(encoded).unwrap();
        let error = match reopened.records().open_external(
            locator,
            RecordReadLimits::new(RecordByteLimit::new(64).unwrap()),
        ) {
            Err(error) => error,
            Ok(_) => panic!("a candidate identity must not open before root publication"),
        };
        assert_eq!(error.denial(), RecordReadDenial::ServingRequiresInspection);
    }
    let offline = worth_store_offline_verifier::walk_current_durable_record_manifest(
        &root,
        format.declaration(),
    )
    .unwrap();
    assert!(offline.placements().is_empty());
    reopened.close();
}
