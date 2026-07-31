use std::{num::NonZeroU32, path::Path};

use worth_proof::TransitionOutcome;
use worth_signal::facade::TemporalDuration;
use worth_store::physical_runtime::certification::MediaFaultDirective;
use worth_store::physical_runtime::{
    FilesystemMediaAdmission, PhysicalMutationDeadline, PhysicalMutationIdempotencyMaterial,
    PhysicalMutationRequest, PhysicalRecordOpen, PhysicalRuntimeAdmission, PhysicalStore,
    PhysicalWalAppendFailureCause, PhysicalWalAppendOutcome, RecordAppendBatch,
};
use worth_store_physical_backend::{
    ArtifactTreeFailureKind, CertificationMediaFaultActivation, FilesystemAccessPosture,
    MediaOperationRole,
};

use super::super::{configuration, durability_with_group_limit};

#[test]
fn denied_before_effect_returns_the_exact_preparation_and_retries_through_the_canonical_route() {
    let parent = tempfile::tempdir().unwrap();
    let store_root = parent.path().join("store");
    super::super::serving_from_initialization(&store_root).close();
    let (media, activation) = media_with_write_denial(&store_root);
    let policy = durability_with_group_limit(&media, NonZeroU32::new(32).unwrap());
    let (format, placement, access) = configuration();
    let serving = super::super::success(
        media.open_record_store(PhysicalRecordOpen::new(format, access, policy)),
    );
    let submission = serving.record_submission();
    let key = submission
        .issue_idempotency_key(PhysicalMutationIdempotencyMaterial::new([83; 32]))
        .unwrap();
    let prepared = match submission
        .prepare_durable_append(
            RecordAppendBatch::try_from_iter([b"retryable-wal-redo"]).unwrap(),
            placement,
            PhysicalMutationRequest::platform_durable(
                key,
                PhysicalMutationDeadline::at(TemporalDuration::temporal_duration(1_000).unwrap()),
            ),
        )
        .into_raw()
    {
        TransitionOutcome::Success(prepared) => prepared,
        _ => panic!("retry fixture preparation must succeed"),
    };
    let identity = prepared.mutation_identity();
    let fingerprint = prepared.request_fingerprint();

    activation.arm().unwrap();
    let preserved = match submission.append_prepared_wal(prepared) {
        PhysicalWalAppendOutcome::ProvenNoEffect {
            prepared,
            cause: PhysicalWalAppendFailureCause::MediaDeniedBeforeEffect(failure),
        } => {
            assert_eq!(failure.kind(), ArtifactTreeFailureKind::DeniedBeforeEffect);
            assert_eq!(failure.io_kind(), Some(std::io::ErrorKind::Other));
            prepared
        }
        _ => panic!("the denied WAL write must return one retryable preparation"),
    };
    assert_eq!(preserved.mutation_identity(), identity);
    assert_eq!(preserved.request_fingerprint(), fingerprint);
    let before_retry = submission.wal_observation().unwrap();
    assert_eq!(before_retry.appended_frames(), 0);
    assert_eq!(before_retry.valid_prefix_bytes(), 0);
    assert!(!before_retry.sealed_for_inspection());

    assert!(matches!(
        submission.append_prepared_wal(preserved),
        PhysicalWalAppendOutcome::Appended(_)
    ));
    let after_retry = submission.wal_observation().unwrap();
    assert_eq!(after_retry.appended_frames(), 1);
    assert!(after_retry.valid_prefix_bytes() > 0);
    serving.close();
}

fn media_with_write_denial(
    root: &Path,
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
                MediaFaultDirective::FailBefore {
                    kind: std::io::ErrorKind::Other,
                    raw_os_error: None,
                },
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
