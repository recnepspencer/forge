use std::{fs, num::NonZeroU32, path::Path};

use worth_proof::TransitionOutcome;
use worth_signal::facade::TemporalDuration;
use worth_store::physical_runtime::certification::CertificationPhysicalExecutionCheckpoint;
use worth_store::physical_runtime::{
    AdmittedRecordPlacementPolicy, PhysicalMutationDeadline, PhysicalMutationIdempotencyMaterial,
    PhysicalMutationRequest, PhysicalRecordInitialization, PhysicalRecordOpen,
    PhysicalRecordSubmission, PhysicalWalAppendOutcome, PhysicalWalReservationDenial,
    PreparedPhysicalMutation, RecordAppendBatch,
};

use super::super::{configuration, durability_with_group_limit, media, success};

#[test]
fn concurrent_append_denial_preserves_the_second_preparation_for_retry() {
    let parent = tempfile::tempdir().unwrap();
    let store_root = parent.path().join("store");
    let media = media(&store_root);
    let policy = durability_with_group_limit(&media, NonZeroU32::new(32).unwrap());
    let (format, placement, access) = configuration();
    let serving = success(
        media.initialize_record_store(PhysicalRecordInitialization::new(
            format, placement, access, policy,
        )),
    );
    let submission = serving.record_submission();
    let first = prepare(
        &submission,
        placement,
        PhysicalMutationIdempotencyMaterial::new([84; 32]),
    );
    let second = prepare(
        &submission,
        placement,
        PhysicalMutationIdempotencyMaterial::new([85; 32]),
    );
    let second_identity = second.mutation_identity();
    let gate = serving.certification_pause_physical_execution_at(
        CertificationPhysicalExecutionCheckpoint::BeforeBackendDispatch,
    );
    let first_submission = submission.clone();
    let first_append = std::thread::spawn(move || first_submission.append_prepared_wal(first));
    assert!(gate.await_arrival());

    let preserved = match submission.append_prepared_wal(second) {
        PhysicalWalAppendOutcome::ReservationDenied {
            prepared,
            cause: PhysicalWalReservationDenial::AppendInFlight,
        } => prepared,
        _ => panic!("the concurrent append must be denied with its preparation intact"),
    };
    assert_eq!(preserved.mutation_identity(), second_identity);
    gate.release();
    assert!(matches!(
        first_append.join().unwrap(),
        PhysicalWalAppendOutcome::Appended(_)
    ));
    assert!(matches!(
        submission.append_prepared_wal(preserved),
        PhysicalWalAppendOutcome::Appended(_)
    ));
    assert_eq!(submission.wal_observation().unwrap().appended_frames(), 2);
    serving.close();
}

#[test]
fn foreign_store_preparation_is_denied_without_effect_and_remains_appendable_by_its_owner() {
    let parent = tempfile::tempdir().unwrap();
    let first_root = parent.path().join("first");
    let second_root = parent.path().join("second");
    let first_media = media(&first_root);
    let second_media = media(&second_root);
    let first_policy = durability_with_group_limit(&first_media, NonZeroU32::new(32).unwrap());
    let second_policy = durability_with_group_limit(&second_media, NonZeroU32::new(32).unwrap());
    let (format, placement, access) = configuration();
    let first = success(
        first_media.initialize_record_store(PhysicalRecordInitialization::new(
            format,
            placement,
            access,
            first_policy,
        )),
    );
    let (format, _, access) = configuration();
    let second = success(
        second_media.initialize_record_store(PhysicalRecordInitialization::new(
            format,
            placement,
            access,
            second_policy,
        )),
    );
    let first_submission = first.record_submission();
    let second_submission = second.record_submission();
    let prepared = prepare(
        &first_submission,
        placement,
        PhysicalMutationIdempotencyMaterial::new([81; 32]),
    );
    let identity = prepared.mutation_identity();
    let fingerprint = prepared.request_fingerprint();

    let preserved = match second_submission.append_prepared_wal(prepared) {
        PhysicalWalAppendOutcome::ReservationDenied {
            prepared,
            cause: PhysicalWalReservationDenial::ForeignStore,
        } => prepared,
        _ => panic!("a foreign Store preparation must be denied before WAL allocation"),
    };
    assert_eq!(preserved.mutation_identity(), identity);
    assert_eq!(preserved.request_fingerprint(), fingerprint);
    assert_untouched_wal(&second_root, &second_submission);

    let first_appended = match first_submission.append_prepared_wal(preserved) {
        PhysicalWalAppendOutcome::Appended(appended) => appended,
        _ => panic!("the rightful Store must append its preserved preparation"),
    };
    let second_prepared = prepare(
        &second_submission,
        placement,
        PhysicalMutationIdempotencyMaterial::new([81; 32]),
    );
    assert_eq!(
        first_appended.mutation_identity().operation_identity(),
        second_prepared.mutation_identity().operation_identity(),
        "both independent runtimes begin at the same local mutation ordinal"
    );
    let second_appended = match second_submission.append_prepared_wal(second_prepared) {
        PhysicalWalAppendOutcome::Appended(appended) => appended,
        _ => panic!("the second Store must append its own preparation"),
    };
    assert_ne!(
        first_appended.reserved().member_basis().member_identity(),
        second_appended.reserved().member_basis().member_identity(),
        "equal local ordinals under different Store/runtime identities cannot collide"
    );
    first.close();
    second.close();
}

#[test]
fn stale_runtime_preparation_is_denied_without_effect_and_preserves_exact_identity() {
    let parent = tempfile::tempdir().unwrap();
    let store_root = parent.path().join("store");
    let initial_media = media(&store_root);
    let initial_policy = durability_with_group_limit(&initial_media, NonZeroU32::new(32).unwrap());
    let (format, placement, access) = configuration();
    let initial = success(initial_media.initialize_record_store(
        PhysicalRecordInitialization::new(format, placement, access, initial_policy),
    ));
    let prepared = prepare(
        &initial.record_submission(),
        placement,
        PhysicalMutationIdempotencyMaterial::new([82; 32]),
    );
    let identity = prepared.mutation_identity();
    let fingerprint = prepared.request_fingerprint();
    initial.close();

    let reopened_media = media(&store_root);
    let reopened_policy =
        durability_with_group_limit(&reopened_media, NonZeroU32::new(32).unwrap());
    let (format, _, access) = configuration();
    let reopened = success(reopened_media.open_record_store(PhysicalRecordOpen::new(
        format,
        access,
        reopened_policy,
    )));
    assert_eq!(identity.store_identity(), reopened.store_identity());
    assert_ne!(identity.runtime_identity(), reopened.runtime_identity());
    let submission = reopened.record_submission();

    let preserved = match submission.append_prepared_wal(prepared) {
        PhysicalWalAppendOutcome::ReservationDenied {
            prepared,
            cause: PhysicalWalReservationDenial::StaleRuntime,
        } => prepared,
        _ => panic!("a stale runtime preparation must be denied before WAL allocation"),
    };
    assert_eq!(preserved.mutation_identity(), identity);
    assert_eq!(preserved.request_fingerprint(), fingerprint);
    assert_untouched_wal(&store_root, &submission);
    reopened.close();
}

#[test]
fn released_submission_reports_lifecycle_loss_without_fabricating_wal_inspection() {
    let parent = tempfile::tempdir().unwrap();
    let store_root = parent.path().join("store");
    let media = media(&store_root);
    let policy = durability_with_group_limit(&media, NonZeroU32::new(32).unwrap());
    let (format, placement, access) = configuration();
    let serving = success(
        media.initialize_record_store(PhysicalRecordInitialization::new(
            format, placement, access, policy,
        )),
    );
    let submission = serving.record_submission();
    let prepared = prepare(
        &submission,
        placement,
        PhysicalMutationIdempotencyMaterial::new([86; 32]),
    );
    let identity = prepared.mutation_identity();
    let fingerprint = prepared.request_fingerprint();
    serving.close();

    let preserved = match submission.append_prepared_wal(prepared) {
        PhysicalWalAppendOutcome::ReservationDenied {
            prepared,
            cause: PhysicalWalReservationDenial::PublicationAuthorityReleased,
        } => prepared,
        _ => panic!("released publication authority must not claim WAL inspection is required"),
    };
    assert_eq!(preserved.mutation_identity(), identity);
    assert_eq!(preserved.request_fingerprint(), fingerprint);
    assert!(submission.wal_observation().is_none());
}

fn prepare(
    submission: &PhysicalRecordSubmission,
    placement: AdmittedRecordPlacementPolicy,
    material: PhysicalMutationIdempotencyMaterial,
) -> PreparedPhysicalMutation {
    let key = submission.issue_idempotency_key(material).unwrap();
    match submission
        .prepare_durable_append(
            RecordAppendBatch::try_from_iter([b"authority-bound-redo"]).unwrap(),
            placement,
            PhysicalMutationRequest::platform_durable(
                key,
                PhysicalMutationDeadline::at(TemporalDuration::temporal_duration(1_000).unwrap()),
            ),
        )
        .into_raw()
    {
        TransitionOutcome::Success(prepared) => prepared,
        _ => panic!("authority fixture preparation must succeed"),
    }
}

fn assert_untouched_wal(root: &Path, submission: &PhysicalRecordSubmission) {
    let observation = submission.wal_observation().unwrap();
    assert_eq!(observation.appended_frames(), 0);
    assert_eq!(observation.appended_bytes(), 0);
    assert_eq!(observation.valid_prefix_bytes(), 0);
    assert_eq!(observation.last_lsn_end(), None);
    assert!(!observation.sealed_for_inspection());
    assert_eq!(
        fs::metadata(
            root.join("families")
                .join("wal")
                .join("segment-1-generation-1.wal")
        )
        .unwrap()
        .len(),
        0
    );
}
