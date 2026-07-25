use worth_proof::TransitionOutcome;
use worth_store::physical_runtime::{
    AdmittedPhysicalRecordFormat, AdmittedRecordAccessPolicy, ExternalPhysicalRecordLocator,
    PhysicalRecordAccessPolicy, PhysicalRecordOpen, RecordAppendBatch, RecordBootstrapDenial,
    RecordByteLimit, RecordReadDenial, RecordReadLimits,
};
use worth_store_physical_backend::MediaOperationRole;

use super::{configuration, media, serving_from_initialization, success};

fn permissive_access() -> (AdmittedPhysicalRecordFormat, AdmittedRecordAccessPolicy) {
    let (format, _, _) = configuration();
    let access = PhysicalRecordAccessPolicy::builder()
        .transfer_limit(RecordByteLimit::new(u32::MAX).unwrap())
        .scratch_limit(RecordByteLimit::new(u32::MAX).unwrap())
        .admit(format)
        .unwrap();
    (format, access)
}

#[test]
fn permissive_access_policy_cannot_expand_fixed_page_reads() {
    let parent = tempfile::tempdir().unwrap();
    let root = parent.path().join("store");
    let (_, placement, _) = configuration();
    let serving = serving_from_initialization(&root);
    let record_id = serving
        .record_submission()
        .append_batch(
            RecordAppendBatch::try_from_iter([b"bounded".as_slice()]).unwrap(),
            placement,
        )
        .unwrap()
        .record_id(0)
        .unwrap();
    let locator = ExternalPhysicalRecordLocator::new(serving.store_identity(), record_id);
    serving.close();

    let page =
        root.join("families/records/segments/segment-0000000000000001-0000000000000001.pages");
    std::fs::OpenOptions::new()
        .write(true)
        .open(page)
        .unwrap()
        .set_len(16_385)
        .unwrap();
    let (format, access) = permissive_access();
    let reopened = success(media(&root).open_record_store(PhysicalRecordOpen::new(format, access)));
    reopened.drain_clean_residency();
    let before = reopened.media_counters();
    assert!(matches!(
        reopened.records().open_external(
            locator,
            RecordReadLimits::new(RecordByteLimit::new(1024).unwrap()),
        ),
        Err(error) if error.denial() == RecordReadDenial::ArtifactDamaged
    ));
    let after = reopened.media_counters();
    assert_eq!(
        after.completed_bytes_for(MediaOperationRole::PositionedRead)
            - before.completed_bytes_for(MediaOperationRole::PositionedRead),
        288,
        "the oversized data artifact is rejected by length before a frame allocation or page read",
    );
    reopened.close();
}

#[test]
fn permissive_access_policy_cannot_expand_current_root_bootstrap() {
    let parent = tempfile::tempdir().unwrap();
    let root = parent.path().join("store");
    serving_from_initialization(&root).close();
    std::fs::OpenOptions::new()
        .write(true)
        .open(root.join("families/records/roots/root-0000000000000001.manifest"))
        .unwrap()
        .set_len(16_385)
        .unwrap();

    let (format, access) = permissive_access();
    let open_media = media(&root);
    let before = open_media.media_counters();
    let denied = open_media
        .open_record_store(PhysicalRecordOpen::new(format, access))
        .into_raw();
    let TransitionOutcome::Denied(denial) = denied else {
        panic!("oversized current root must be denied");
    };
    assert_eq!(denial.reason(), RecordBootstrapDenial::CurrentRootDamaged);
    let returned = denial.into_runtime();
    let after = returned.media_counters();
    assert_eq!(
        after.completed_bytes_for(MediaOperationRole::PositionedRead)
            - before.completed_bytes_for(MediaOperationRole::PositionedRead),
        74
    );
    returned.close();
}
