use worth_proof::TransitionOutcome;
use worth_store::physical_runtime::{
    FilesystemMediaAdmission, PhysicalRecordInitialization, PhysicalRecordOpen,
    PhysicalRuntimeAdmission, PhysicalStore, RecordAppendBatch, RecordAppendDenial,
    RecordAppendError,
};
use worth_store_physical_backend::{
    FilesystemAccessPosture, MediaFaultDirective, MediaOperationRole,
};

use super::{configuration, media, serving_from_open, success};

#[test]
fn fresh_open_detects_failed_reusable_segment_cow_and_blocks_writes() {
    let parent = tempfile::tempdir().unwrap();
    let root = parent.path().join("store");
    let (format, placement, access) = configuration();
    let seeded = success(initialize_record_store!(media(&root), |durability| {
        PhysicalRecordInitialization::new(format, placement, access, durability)
    }));
    seeded
        .record_submission()
        .append_batch(
            RecordAppendBatch::try_from_iter([b"published".as_slice()]).unwrap(),
            placement,
        )
        .unwrap();
    seeded.close();

    let baseline = serving_from_open(&root);
    let prior_writes = baseline
        .media_counters()
        .attempts_for(MediaOperationRole::PositionedWrite);
    baseline.close();
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
    let faulted_media = match runtime
        .try_admit_filesystem_media(admission.with_fault_schedule(schedule))
        .into_raw()
    {
        TransitionOutcome::Success(media) => media,
        _ => panic!("fault schedule must admit"),
    };
    let faulted = success(open_record_store!(faulted_media, |durability| {
        PhysicalRecordOpen::new(format, access, durability)
    }));
    let failure = match faulted.record_submission().append_batch(
        RecordAppendBatch::try_from_iter([b"candidate".as_slice()]).unwrap(),
        placement,
    ) {
        Err(RecordAppendError::Unpublished(failure)) => failure,
        _ => panic!("failed reusable-segment COW must remain unpublished"),
    };
    assert!(failure.residue().reusable_segment_artifacts());
    assert!(!failure.residue().next_segment_artifacts());
    faulted.abort();
    assert!(root
        .join("families/records/segments/segment-0000000000000001-0000000000000002.pages")
        .is_file());

    let reopened = serving_from_open(&root);
    assert!(reopened.observed_non_authoritative_residue());
    assert!(reopened.publication_residue().reusable_segment_artifacts());
    assert!(matches!(
        reopened.record_submission().append_batch(
            RecordAppendBatch::try_from_iter([b"blocked".as_slice()]).unwrap(),
            placement,
        ),
        Err(RecordAppendError::Denied(
            RecordAppendDenial::ServingRequiresInspection
        ))
    ));
    reopened.close();
}
