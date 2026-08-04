use std::{fs, num::NonZeroU32, path::Path};

use sha2::{Digest, Sha256};
use worth_proof::{NonEmpty, TransitionOutcome};
use worth_signal::facade::TemporalDuration;
use worth_store::physical_runtime::certification::MediaFaultDirective;
use worth_store::physical_runtime::{
    FilesystemMediaAdmission, PhysicalMutationDeadline, PhysicalMutationIdempotencyMaterial,
    PhysicalMutationPreparationSuccess, PhysicalMutationRequest, PhysicalRecordInitialization,
    PhysicalRecordOpen, PhysicalRuntimeAdmission, PhysicalStore,
    PhysicalWalGroupAppendFailureCause, PhysicalWalGroupAppendOutcome,
    PhysicalWalReservationDenial, RecordAppendBatch,
};
use worth_store_physical_backend::{
    ArtifactAppendRange, CertificationMediaFaultActivation, FilesystemAccessPosture,
    MediaOperationRole,
};
use worth_store_wal::artifact_store::{
    verify_bounded_wal_segment, BoundedWalSegmentVerificationRequest,
};

use super::super::{configuration, durability_with_group_limit, media};

#[test]
fn ordinary_prepared_mutation_appends_one_independently_verifiable_wal_frame() {
    let parent = tempfile::tempdir().unwrap();
    let store_root = parent.path().join("store");
    let media = media(&store_root);
    let policy = durability_with_group_limit(&media, NonZeroU32::new(32).unwrap());
    let (format, placement, access) = configuration();
    let serving = super::super::success(media.initialize_record_store(
        PhysicalRecordInitialization::new(format, placement, access, policy),
    ));
    let submission = serving.certification_record_submission();
    let key = submission
        .issue_idempotency_key(PhysicalMutationIdempotencyMaterial::new([61; 32]))
        .unwrap();
    let request_fingerprint;
    let prepared = match submission
        .prepare_durable_append(
            RecordAppendBatch::try_from_iter([b"first-redo".as_slice(), b"second-redo".as_slice()])
                .unwrap(),
            placement,
            PhysicalMutationRequest::platform_durable(
                key.clone(),
                PhysicalMutationDeadline::at(TemporalDuration::temporal_duration(1_000).unwrap()),
            ),
        )
        .into_raw()
    {
        TransitionOutcome::Success(PhysicalMutationPreparationSuccess::Prepared(prepared)) => {
            request_fingerprint = prepared.request_fingerprint();
            prepared
        }
        _ => panic!("fresh physical mutation preparation must succeed"),
    };

    let appended = match submission.append_prepared_wal_group(NonEmpty::new(prepared, Vec::new())) {
        PhysicalWalGroupAppendOutcome::Appended(appended) => appended,
        _ => panic!("the canonical WAL route must append the prepared mutation"),
    };
    let appended = appended.members()[0].mutation();
    let declaration = appended.reserved().declaration();
    assert_eq!(
        appended.reserved().request_fingerprint(),
        request_fingerprint
    );
    assert_eq!(
        appended.reserved().bound_redo_digest(),
        appended.reserved().redo().digest()
    );
    assert_eq!(declaration.segment().get(), 1);
    assert_eq!(declaration.generation().get(), 1);
    assert_eq!(declaration.lsn_range().start().get(), 1);
    assert_eq!(declaration.lsn_range().end_exclusive().get(), 3);
    assert_eq!(appended.settlement().range(), declaration.artifact_range());
    let observed = submission.wal_observation().unwrap();
    assert_eq!(observed.appended_frames(), 1);
    assert_eq!(
        observed.appended_bytes(),
        declaration.artifact_range().byte_count()
    );
    assert_eq!(observed.last_lsn_end(), Some(3));
    assert!(!observed.sealed_for_inspection());
    serving.close();

    let artifact = store_root
        .join("families")
        .join("wal")
        .join("segment-1-generation-1.wal");
    let bytes = fs::read(&artifact).unwrap();
    assert_eq!(
        bytes.len() as u64,
        declaration.artifact_range().byte_count()
    );
    assert!(contains(&bytes, &key.identity().bytes()));
    assert!(contains(&bytes, &request_fingerprint.bytes()));
    assert!(contains(
        &bytes,
        &key.lease().issuance_generation().get().to_le_bytes()
    ));
    assert!(contains(
        &bytes,
        &key.lease().expiry_generation().get().to_le_bytes()
    ));
    assert!(contains(&bytes, b"first-redo"));
    assert!(contains(&bytes, b"second-redo"));

    let artifact_digest = Sha256::digest(&bytes).into();
    let request = BoundedWalSegmentVerificationRequest::new(
        declaration.segment().get(),
        declaration.generation().get(),
        declaration.lsn_range().start().get(),
        declaration.lsn_range().end_exclusive().get(),
        bytes.len() as u64,
        artifact_digest,
        4 * 1024,
    )
    .unwrap();
    let independently_observed = verify_bounded_wal_segment(&artifact, request).unwrap();
    assert_eq!(independently_observed.frame_count(), 1);
    assert_eq!(independently_observed.lsn_interval(), (1, 3));
    assert_eq!(independently_observed.bytes_read(), bytes.len() as u64);
}

#[test]
fn torn_wal_frame_is_indeterminate_and_seals_later_range_allocation() {
    let parent = tempfile::tempdir().unwrap();
    let store_root = parent.path().join("store");
    super::super::serving_from_initialization(&store_root).close();
    let (media, fault_activation) = media_with_write_prefix_fault(&store_root, 3);
    let policy = durability_with_group_limit(&media, NonZeroU32::new(32).unwrap());
    let (format, placement, access) = configuration();
    let serving = super::super::success(
        media.open_record_store(PhysicalRecordOpen::new(format, access, policy)),
    );
    let submission = serving.certification_record_submission();
    let first = prepared(
        &submission,
        placement,
        PhysicalMutationIdempotencyMaterial::new([71; 32]),
        b"torn-redo",
    );
    let second = prepared(
        &submission,
        placement,
        PhysicalMutationIdempotencyMaterial::new([72; 32]),
        b"must-not-allocate",
    );
    let second_identity = second.mutation_identity();
    fault_activation.arm().unwrap();
    let uncertain = match submission.append_prepared_wal_group(NonEmpty::new(first, Vec::new())) {
        PhysicalWalGroupAppendOutcome::Indeterminate(uncertain) => uncertain,
        _ => panic!("a strict nonzero prefix must be classified indeterminate"),
    };
    assert!(submission
        .wal_observation()
        .unwrap()
        .sealed_for_inspection());

    let denied = submission.append_prepared_wal_group(NonEmpty::new(second, Vec::new()));
    let continuation = match denied {
        PhysicalWalGroupAppendOutcome::NotStarted(continuation)
            if matches!(
                continuation.cause(),
                PhysicalWalGroupAppendFailureCause::Reservation(
                    PhysicalWalReservationDenial::InspectionRequired
                )
            ) =>
        {
            continuation
        }
        _ => panic!("inspection sealing must preserve the denied prepared mutation"),
    };
    assert_eq!(continuation.remaining_member_count(), 1);
    assert_eq!(
        continuation.basis().member_count().get(),
        1,
        "the exact singleton authority remains in the continuation"
    );
    let declaration = uncertain
        .uncertain_member()
        .expect("the append was indeterminate at its exact member")
        .mutation()
        .declaration();
    assert_ne!(
        continuation.basis().identity().bytes(),
        [0; 32],
        "the preserved continuation retains a real group identity"
    );
    assert_ne!(second_identity.operation_identity().get(), 0);
    serving.close();

    let artifact = wal_artifact(&store_root);
    let bytes = fs::read(&artifact).unwrap();
    assert_eq!(bytes.len(), 3);
    let request = BoundedWalSegmentVerificationRequest::new(
        declaration.segment().get(),
        declaration.generation().get(),
        declaration.lsn_range().start().get(),
        declaration.lsn_range().end_exclusive().get(),
        declaration.artifact_range().byte_count(),
        [0; 32],
        4 * 1024,
    )
    .unwrap();
    assert!(matches!(
        verify_bounded_wal_segment(&artifact, request),
        Err(
            worth_store_wal::artifact_store::BoundedWalSegmentDenial::LengthMismatch {
                expected: _,
                actual: 3
            }
        )
    ));
}

#[test]
fn consecutive_mutations_receive_distinct_contiguous_wal_bindings() {
    let parent = tempfile::tempdir().unwrap();
    let store_root = parent.path().join("store");
    let media = media(&store_root);
    let policy = durability_with_group_limit(&media, NonZeroU32::new(32).unwrap());
    let (format, placement, access) = configuration();
    let serving = super::super::success(media.initialize_record_store(
        PhysicalRecordInitialization::new(format, placement, access, policy),
    ));
    let submission = serving.certification_record_submission();

    let first = prepared(
        &submission,
        placement,
        PhysicalMutationIdempotencyMaterial::new([73; 32]),
        b"first",
    );
    let second = prepared(
        &submission,
        placement,
        PhysicalMutationIdempotencyMaterial::new([74; 32]),
        b"second",
    );
    let first = match submission.append_prepared_wal_group(NonEmpty::new(first, Vec::new())) {
        PhysicalWalGroupAppendOutcome::Appended(appended) => appended,
        _ => panic!("first mutation must append"),
    };
    let second = match submission.append_prepared_wal_group(NonEmpty::new(second, Vec::new())) {
        PhysicalWalGroupAppendOutcome::Appended(appended) => appended,
        _ => panic!("second mutation must append"),
    };
    let first = first.members()[0].mutation();
    let second = second.members()[0].mutation();

    let first_declaration = first.reserved().declaration();
    let second_declaration = second.reserved().declaration();
    assert_ne!(first.mutation_identity(), second.mutation_identity());
    assert_ne!(
        first.reserved().member_basis().member_identity(),
        second.reserved().member_basis().member_identity()
    );
    assert_eq!(
        first_declaration.lsn_range().end_exclusive(),
        second_declaration.lsn_range().start()
    );
    assert_eq!(
        first_declaration.artifact_range().end_exclusive(),
        second_declaration.artifact_range().offset()
    );
    assert_ne!(
        first.settlement().work_identity(),
        second.settlement().work_identity()
    );
    assert_ne!(
        first.settlement().backend_operation(),
        second.settlement().backend_operation()
    );
    assert_eq!(
        first.settlement().range(),
        first_declaration.artifact_range()
    );
    assert_eq!(
        second.settlement().range(),
        second_declaration.artifact_range()
    );
    let exact_work = first.settlement().work_identity();
    let exact_range = first_declaration.artifact_range();
    let exact_digest = first_declaration.payload_digest();
    let exact_artifact = first.reserved().artifact();
    let exact_disposition = first_declaration.disposition();
    assert!(first.settlement().matches_completion_binding(
        exact_work,
        exact_artifact,
        exact_range,
        exact_digest,
        exact_disposition,
    ));
    assert!(!first.settlement().matches_completion_binding(
        second.settlement().work_identity(),
        exact_artifact,
        exact_range,
        exact_digest,
        exact_disposition,
    ));
    let wrong_range =
        ArtifactAppendRange::new(exact_range.offset() + 1, exact_range.byte_count()).unwrap();
    assert!(!first.settlement().matches_completion_binding(
        exact_work,
        exact_artifact,
        wrong_range,
        exact_digest,
        exact_disposition,
    ));
    let mut wrong_digest = exact_digest;
    wrong_digest[0] ^= 0xff;
    assert!(!first.settlement().matches_completion_binding(
        exact_work,
        exact_artifact,
        exact_range,
        wrong_digest,
        exact_disposition,
    ));
    serving.close();
}

pub(super) fn prepared(
    submission: &worth_store::physical_runtime::PhysicalRecordSubmission,
    placement: worth_store::physical_runtime::AdmittedRecordPlacementPolicy,
    material: PhysicalMutationIdempotencyMaterial,
    bytes: &[u8],
) -> worth_store::physical_runtime::PreparedPhysicalMutation {
    let key = submission.issue_idempotency_key(material).unwrap();
    match submission
        .prepare_durable_append(
            RecordAppendBatch::try_from_iter([bytes]).unwrap(),
            placement,
            PhysicalMutationRequest::platform_durable(
                key,
                PhysicalMutationDeadline::at(TemporalDuration::temporal_duration(1_000).unwrap()),
            ),
        )
        .into_raw()
    {
        TransitionOutcome::Success(PhysicalMutationPreparationSuccess::Prepared(prepared)) => {
            prepared
        }
        _ => panic!("fresh physical mutation preparation must succeed"),
    }
}

fn media_with_write_prefix_fault(
    root: &Path,
    bytes: u64,
) -> (
    worth_store::physical_runtime::MediaOwnedPhysicalRuntime,
    CertificationMediaFaultActivation,
) {
    let admission =
        FilesystemMediaAdmission::certification(FilesystemAccessPosture::CoordinatedServiceAccount);
    let authority = admission.fault_schedule_authority();
    let activation = authority.one_shot_activation();
    let schedule = authority
        .schedule(vec![authority
            .rule(
                MediaOperationRole::PositionedWrite,
                1,
                MediaFaultDirective::AllowPrefix { bytes },
            )
            .for_next_identified_operation_after_activation(
                activation.clone(),
            )])
        .unwrap();
    let runtime = PhysicalStore::admit(PhysicalRuntimeAdmission::new(root).unwrap()).unwrap();
    match runtime
        .try_admit_filesystem_media(admission.with_fault_schedule(schedule))
        .into_raw()
    {
        TransitionOutcome::Success(media) => (media, activation),
        _ => panic!("fault-scheduled media admission must succeed"),
    }
}

fn wal_artifact(store_root: &Path) -> std::path::PathBuf {
    store_root.join("families/wal/segment-1-generation-1.wal")
}

fn contains(haystack: &[u8], needle: &[u8]) -> bool {
    haystack
        .windows(needle.len())
        .any(|window| window == needle)
}
