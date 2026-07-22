use std::path::Path;

#[path = "physical_record_journeys/allocation_probe.rs"]
mod allocation_probe;

#[path = "physical_record_journeys/batch_admission.rs"]
mod batch_admission;
#[path = "physical_record_journeys/bootstrap_faults.rs"]
mod bootstrap_faults;
#[path = "physical_record_journeys/c6_preparation.rs"]
mod c6_preparation;
#[path = "physical_record_journeys/child_process.rs"]
mod child_process;
#[path = "physical_record_journeys/configuration_boundaries.rs"]
mod configuration_boundaries;
#[path = "physical_record_journeys/courtroom_child.rs"]
mod courtroom_child;
#[path = "c5/courtroom_evidence_support.rs"]
mod courtroom_evidence_support;
#[path = "c5/courtroom_oracle.rs"]
mod courtroom_oracle;
#[path = "c5/extent_child.rs"]
mod extent_child;
#[path = "physical_record_journeys/extent_streaming.rs"]
mod extent_streaming;
#[path = "physical_record_journeys/format_readmission.rs"]
mod format_readmission;
#[path = "physical_record_journeys/foundational_evidence.rs"]
mod foundational_evidence;
#[path = "physical_record_journeys/generation_policy_truth.rs"]
mod generation_policy_truth;
#[path = "physical_record_journeys/locator_free_space.rs"]
mod locator_free_space;
#[path = "physical_record_journeys/manifest_fixture.rs"]
mod manifest_fixture;
#[path = "physical_record_journeys/manifest_scale.rs"]
mod manifest_scale;
#[path = "c5/observer.rs"]
mod observer;
#[path = "physical_record_journeys/page_packing_oracle.rs"]
mod page_packing_oracle;
#[path = "c5/courtrooms.rs"]
mod production_courtrooms;
#[path = "physical_record_journeys/publication_failure_topology.rs"]
mod publication_failure_topology;
#[path = "physical_record_journeys/publication_faults.rs"]
mod publication_faults;
#[path = "physical_record_journeys/publication_mutants.rs"]
mod publication_mutants;
#[path = "physical_record_journeys/read_boundaries.rs"]
mod read_boundaries;
#[path = "physical_record_journeys/residue_safety.rs"]
mod residue_safety;
#[path = "physical_record_journeys/reusable_segment_residue.rs"]
mod reusable_segment_residue;
#[path = "c5/scale_invalid_worlds.rs"]
mod scale_invalid_worlds;
#[path = "c5/scale_policy_evolution.rs"]
mod scale_policy_evolution;
#[path = "c5/scale.rs"]
mod scale_support;
#[path = "physical_record_journeys/scan_journeys.rs"]
mod scan_journeys;
#[path = "physical_record_journeys/scenario_artifact_evidence.rs"]
mod scenario_artifact_evidence;
#[path = "physical_record_journeys/scenario_configuration.rs"]
mod scenario_configuration;
#[path = "physical_record_journeys/scenario_evidence.rs"]
mod scenario_evidence;
#[path = "physical_record_journeys/scenario_process_evidence.rs"]
mod scenario_process_evidence;
#[path = "physical_record_journeys/segment_journeys.rs"]
mod segment_journeys;
#[path = "physical_record_journeys/segment_truth.rs"]
mod segment_truth;
#[path = "physical_record_journeys/serving_lifecycle.rs"]
mod serving_lifecycle;
#[path = "physical_record_journeys/stream_fixture.rs"]
mod stream_fixture;
#[path = "physical_record_journeys/writeback_courtroom.rs"]
mod writeback_courtroom;

use child_process::{decode_locator, run_child};

use worth_proof::TransitionOutcome;
use worth_store::physical_runtime::{
    AdmittedPhysicalRecordFormat, AdmittedRecordAccessPolicy, AdmittedRecordPlacementPolicy,
    FilesystemMediaAdmission, ManifestEntryCapacity, MediaOwnedPhysicalRuntime,
    PhysicalPageSizeClass, PhysicalRecordAccessPolicy, PhysicalRecordFormatDeclaration,
    PhysicalRecordInitialization, PhysicalRecordOpen, PhysicalRecordPlacementPolicy,
    PhysicalRuntimeAdmission, PhysicalStore, RecordAppendBatch, RecordBootstrapDenial,
    RecordByteLimit, RecordReadObservation, RecordReadSession, RecordServingAdmissionOutcome,
    RecordStoreInitializationOutcome, RecordStoreOpenOutcome, ServingPhysicalRuntime,
};
use worth_store_physical_backend::FilesystemAccessPosture;
use worth_store_physical_backend::MediaOperationRole;

fn configuration() -> (
    AdmittedPhysicalRecordFormat,
    AdmittedRecordPlacementPolicy,
    AdmittedRecordAccessPolicy,
) {
    let format = AdmittedPhysicalRecordFormat::admit(
        PhysicalRecordFormatDeclaration::builder().admit().unwrap(),
    );
    let placement = PhysicalRecordPlacementPolicy::builder()
        .manifest_capacity(ManifestEntryCapacity::new(64).unwrap())
        .admit(format)
        .unwrap();
    let access = PhysicalRecordAccessPolicy::builder().admit(format).unwrap();
    (format, placement, access)
}

fn media(root: &Path) -> MediaOwnedPhysicalRuntime {
    let runtime = PhysicalStore::admit(PhysicalRuntimeAdmission::new(root).unwrap()).unwrap();
    match runtime
        .try_admit_filesystem_media(FilesystemMediaAdmission::production(
            FilesystemAccessPosture::CoordinatedServiceAccount,
        ))
        .into_raw()
    {
        TransitionOutcome::Success(media) => media,
        _ => panic!("real filesystem media admission must succeed"),
    }
}

fn serving_from_initialization(root: &Path) -> ServingPhysicalRuntime {
    let (format, placement, access) = configuration();
    success(
        media(root)
            .initialize_record_store(PhysicalRecordInitialization::new(format, placement, access)),
    )
}

fn serving_from_open(root: &Path) -> ServingPhysicalRuntime {
    let (format, _, access) = configuration();
    success(media(root).open_record_store(PhysicalRecordOpen::new(format, access)))
}

fn success<Denial>(outcome: RecordServingAdmissionOutcome<Denial>) -> ServingPhysicalRuntime {
    match outcome.into_raw() {
        TransitionOutcome::Success(serving) => serving,
        _ => panic!("record-serving progression must succeed"),
    }
}

fn read_record(
    mut session: RecordReadSession<'_>,
    expected_bytes: usize,
) -> (Vec<u8>, RecordReadObservation) {
    let mut bytes = vec![0_u8; expected_bytes];
    let mut completed = 0;
    while completed < bytes.len() {
        let count = session.read_next(&mut bytes[completed..]).unwrap();
        assert!(count > 0, "the record ended before its declared length");
        completed += count;
    }
    assert_eq!(session.read_next(&mut [0_u8; 1]).unwrap(), 0);
    (bytes, session.observation())
}

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
    let serving = success(
        absent
            .initialize_record_store(PhysicalRecordInitialization::new(format, placement, access)),
    );
    serving.close();

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
    let mut reopened =
        success(media(&root).open_record_store(PhysicalRecordOpen::new(format, wider_access)));
    let changed_placement = PhysicalRecordPlacementPolicy::builder()
        .manifest_capacity(ManifestEntryCapacity::new(128).unwrap())
        .admit(format)
        .unwrap();
    reopened
        .records_mut()
        .append_batch_reconstructing_manifest_capacity(
            RecordAppendBatch::try_from_iter([b"policy drift".as_slice()]).unwrap(),
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

#[test]
fn admission_denials_have_no_effect_and_successors_receive_fresh_identity() {
    let parent = tempfile::tempdir().unwrap();
    let root = parent.path().join("store");
    let (_, placement, _) = configuration();
    let mut serving = serving_from_initialization(&root);
    let first = serving
        .records_mut()
        .append_batch(
            RecordAppendBatch::try_from_iter([b"first".as_slice()]).unwrap(),
            placement,
        )
        .unwrap()
        .record_id(0)
        .unwrap();
    let excessive_batch =
        RecordAppendBatch::try_from_iter((0..65).map(|_| b"x".as_slice())).unwrap();
    let fanout_crossing = serving
        .records_mut()
        .append_batch(excessive_batch, placement)
        .unwrap();
    assert_eq!(fanout_crossing.observation().records(), 65);
    assert!(fanout_crossing.observation().manifest_blocks_read() > 0);
    let second = serving
        .records_mut()
        .append_batch(
            RecordAppendBatch::try_from_iter([b"second".as_slice()]).unwrap(),
            placement,
        )
        .unwrap()
        .record_id(0)
        .unwrap();
    assert_ne!(second.allocation_epoch(), first.allocation_epoch());
    assert_eq!(second.ordinal(), 1);
    serving.close();
}

#[test]
fn one_inline_record_survives_writer_loss() {
    let parent = tempfile::tempdir().unwrap();
    let root = parent.path().join("store");
    let writer = run_child("writer", &root, None);
    let first = writer
        .lines()
        .find_map(|line| line.strip_prefix("C5_LOCATOR "))
        .expect("writer must report a locator");
    let second = writer
        .lines()
        .find_map(|line| line.strip_prefix("C5_LOCATOR_2 "))
        .expect("writer must report a successor locator");
    assert_ne!(first, second);
    let first_bytes = decode_locator(first).encode();
    let second_bytes = decode_locator(second).encode();
    assert_ne!(&first_bytes[16..32], &second_bytes[16..32]);
    assert_eq!(
        u64::from_le_bytes(first_bytes[32..40].try_into().unwrap()),
        1
    );
    assert_eq!(
        u64::from_le_bytes(second_bytes[32..40].try_into().unwrap()),
        1
    );
    let locators = format!("{first},{second}");
    let reader = run_child("reader", &root, Some(&locators));
    assert!(reader.lines().any(|line| line == "C5_PAYLOAD 616c706861"));
    assert!(reader.lines().any(|line| line == "C5_PAYLOAD_2 62657461"));
}
