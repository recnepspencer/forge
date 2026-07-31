use std::num::NonZeroU32;

use worth_proof::TransitionOutcome;
use worth_signal::facade::TemporalDuration;
use worth_store::physical_runtime::{
    PhysicalEffectObligation, PhysicalMutationDeadline, PhysicalMutationIdempotencyMaterial,
    PhysicalMutationRequest, PhysicalRecordInitialization, PhysicalWalAppendFailureCause,
    PhysicalWalAppendOutcome, PhysicalWorkCapacity, PhysicalWorkCapacityDimension,
    PhysicalWorkReadiness, RecordAppendBatch,
};

use super::super::{
    configuration, durability_with_group_limit, media, physical_work::work_fixture, success,
};

#[test]
fn bounded_work_deferral_returns_exact_cause_and_preparation_for_retry() {
    let parent = tempfile::tempdir().unwrap();
    let store_root = parent.path().join("store");
    let media = media(&store_root);
    let policy = durability_with_group_limit(&media, NonZeroU32::new(32).unwrap());
    let (profile, read_request, _) = work_fixture();
    let profile = profile.with_capacity(
        PhysicalWorkCapacity::new(1, 256, 256, 1024 * 1024, 1024 * 1024)
            .unwrap()
            .with_terminal_evidence_capacity(4)
            .unwrap(),
    );
    let (format, placement, access) = configuration();
    let serving = success(
        media.initialize_record_store(
            PhysicalRecordInitialization::new(format, placement, access, policy)
                .with_physical_work_profile(profile),
        ),
    );
    let occupied = match serving
        .physical_read_submission()
        .submit(read_request)
        .into_raw()
    {
        TransitionOutcome::Success(receipt) => receipt,
        other => panic!("the one command slot must be occupied: {other:?}"),
    };
    let submission = serving.record_submission();
    let key = submission
        .issue_idempotency_key(PhysicalMutationIdempotencyMaterial::new([87; 32]))
        .unwrap();
    let prepared = match submission
        .prepare_durable_append(
            RecordAppendBatch::try_from_iter([b"capacity-retry-redo"]).unwrap(),
            placement,
            PhysicalMutationRequest::platform_durable(
                key,
                PhysicalMutationDeadline::at(TemporalDuration::temporal_duration(1_000).unwrap()),
            ),
        )
        .into_raw()
    {
        TransitionOutcome::Success(prepared) => prepared,
        _ => panic!("durable preparation must precede work admission"),
    };
    let identity = prepared.mutation_identity();
    let fingerprint = prepared.request_fingerprint();

    let preserved = match submission.append_prepared_wal(prepared) {
        PhysicalWalAppendOutcome::ProvenNoEffect {
            prepared,
            cause: PhysicalWalAppendFailureCause::SubmissionDeferred(deferred),
        } => {
            assert_eq!(
                deferred.dimension(),
                PhysicalWorkCapacityDimension::Commands
            );
            assert_eq!(deferred.capacity(), 1);
            prepared
        }
        _ => panic!("bounded work pressure must retain its exact deferred cause"),
    };
    assert_eq!(preserved.mutation_identity(), identity);
    assert_eq!(preserved.request_fingerprint(), fingerprint);
    assert_eq!(submission.wal_observation().unwrap().appended_frames(), 0);

    let admitted = serving.admit_physical_work(occupied).unwrap();
    let ready = match serving.request_physical_work(admitted).unwrap() {
        PhysicalWorkReadiness::Ready(ready) => ready,
        PhysicalWorkReadiness::Blocked(_) => panic!("the fixture dependency must be ready"),
    };
    let cancellation = serving
        .cancel_physical_work(ready.consumer_handle())
        .unwrap();
    assert_eq!(
        cancellation.obligation(),
        PhysicalEffectObligation::NotDispatched
    );
    drop(ready);

    assert!(matches!(
        submission.append_prepared_wal(preserved),
        PhysicalWalAppendOutcome::Appended(_)
    ));
    serving.close();
}
