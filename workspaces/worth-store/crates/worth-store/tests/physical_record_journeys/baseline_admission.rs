use worth_proof::TransitionOutcome;
use worth_store::physical_runtime::{
    AdmittedPhysicalRecordFormat, ManifestEntryCapacity, PhysicalPageSizeClass,
    PhysicalRecordAccessPolicy, PhysicalRecordFormatDeclaration, PhysicalRecordInitialization,
    PhysicalRecordOpen, PhysicalRecordPlacementPolicy, RecordBootstrapDenial, RecordByteLimit,
    RecordStoreInitializationOutcome, RecordStoreOpenOutcome,
};
use worth_store_physical_backend::MediaOperationRole;

use super::{configuration, media, serving_from_initialization, serving_from_open, success};

#[test]
fn empty_bootstrap_create_and_reopen_converge() {
    let parent = tempfile::tempdir().unwrap();
    let root = parent.path().join("store");
    let serving = serving_from_initialization(&root);
    let store = serving.store_identity();
    assert!(root.join("families/records/bootstrap.catalog").is_file());
    assert!(root
        .join("families/records/roots/root-0000000000000001.manifest")
        .is_file());
    serving.close();

    let (format, _, access) = configuration();
    let open_media = media(&root);
    let before = open_media.media_counters();
    let reopened = success(open_media.open_record_store(PhysicalRecordOpen::new(format, access)));
    let after = reopened.media_counters();
    assert_eq!(reopened.store_identity(), store);
    assert!(!reopened.observed_staging_residue());
    assert_eq!(
        after.attempts_for(MediaOperationRole::PositionedRead)
            - before.attempts_for(MediaOperationRole::PositionedRead),
        3
    );
    assert_eq!(
        after.completed_bytes_for(MediaOperationRole::PositionedRead)
            - before.completed_bytes_for(MediaOperationRole::PositionedRead),
        602
    );
    assert_eq!(after.replacements(), before.replacements());
    reopened.close();
}

#[test]
fn initialize_and_open_never_substitute_for_each_other() {
    let parent = tempfile::tempdir().unwrap();
    let root = parent.path().join("store");
    let (format, placement, access) = configuration();
    let absent = media(&root);
    let open_outcome: RecordStoreOpenOutcome =
        absent.open_record_store(PhysicalRecordOpen::new(format, access));
    let absent = match open_outcome.into_raw() {
        TransitionOutcome::Denied(denial) => denial.into_runtime(),
        _ => panic!("C5_PREDICATE:lifecycle open must not initialize an absent record family"),
    };
    success(
        absent
            .initialize_record_store(PhysicalRecordInitialization::new(format, placement, access)),
    )
    .close();

    let existing = media(&root);
    let existing: RecordStoreInitializationOutcome = existing
        .initialize_record_store(PhysicalRecordInitialization::new(format, placement, access));
    assert!(matches!(existing.into_raw(), TransitionOutcome::Denied(_)));
}

#[test]
fn operational_policy_drift_reopens_but_format_drift_does_not() {
    let parent = tempfile::tempdir().unwrap();
    let root = parent.path().join("store");
    serving_from_initialization(&root).close();

    let (format, _, _) = configuration();
    let wider_access = PhysicalRecordAccessPolicy::builder()
        .transfer_limit(RecordByteLimit::new(32_768).unwrap())
        .scratch_limit(RecordByteLimit::new(32_768).unwrap())
        .admit(format)
        .unwrap();
    let reopened =
        success(media(&root).open_record_store(PhysicalRecordOpen::new(format, wider_access)));
    let changed_placement = PhysicalRecordPlacementPolicy::builder()
        .manifest_capacity(ManifestEntryCapacity::new(128).unwrap())
        .admit(format)
        .unwrap();
    reopened
        .record_submission()
        .append_batch_reconstructing_manifest_capacity(
            worth_store::physical_runtime::RecordAppendBatch::try_from_iter([
                b"policy drift".as_slice()
            ])
            .unwrap(),
            changed_placement,
        )
        .unwrap();
    reopened.close();

    let changed_format = AdmittedPhysicalRecordFormat::admit(
        PhysicalRecordFormatDeclaration::builder()
            .page_size(PhysicalPageSizeClass::KiB32)
            .admit()
            .unwrap(),
    );
    let changed_access = PhysicalRecordAccessPolicy::builder()
        .admit(changed_format)
        .unwrap();
    let wrong_media = media(&root);
    let replacements = wrong_media.media_counters().replacements();
    let rejected = wrong_media
        .open_record_store(PhysicalRecordOpen::new(changed_format, changed_access))
        .into_raw();
    let TransitionOutcome::Denied(denial) = rejected else {
        panic!("persisted format drift must be denied");
    };
    let RecordBootstrapDenial::PhysicalRecordFormatMismatch(mismatch) = denial.reason() else {
        panic!("format mismatch must retain both exact declarations")
    };
    assert_eq!(mismatch.expected(), changed_format.declaration());
    assert_eq!(mismatch.persisted(), format.declaration());
    let returned = denial.into_runtime();
    assert_eq!(returned.media_counters().replacements(), replacements);
    returned.close();
}

#[test]
fn namespace_residue_cannot_elect_current_truth() {
    let parent = tempfile::tempdir().unwrap();
    let root = parent.path().join("store");
    serving_from_initialization(&root).close();
    let catalog = root.join("families/records/bootstrap.catalog");
    std::fs::copy(
        &catalog,
        root.join("staging/records/bootstrap-0000000000000002.candidate"),
    )
    .unwrap();
    std::fs::write(
        root.join("staging/records/duplicate.candidate"),
        b"duplicate",
    )
    .unwrap();
    let selected = root.join("families/records/roots/root-0000000000000001.manifest");
    std::fs::copy(
        &selected,
        root.join("families/records/roots/root-0000000000000000.manifest"),
    )
    .unwrap();
    std::fs::copy(
        &selected,
        root.join("families/records/roots/root-0000000000000002.manifest"),
    )
    .unwrap();
    let foreign_root = parent.path().join("foreign-store");
    serving_from_initialization(&foreign_root).close();
    std::fs::copy(
        foreign_root.join("families/records/roots/root-0000000000000001.manifest"),
        root.join("families/records/roots/root-ffffffffffffffff.manifest"),
    )
    .unwrap();
    let reopened = serving_from_open(&root);
    assert!(reopened.observed_non_authoritative_residue());
    reopened.close();
}
