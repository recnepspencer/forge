use tempfile::tempdir;
use worth_proof::TransitionOutcome;
use worth_store::physical_runtime::{
    PhysicalEffectObligation, PhysicalExecutorCommand, PhysicalStoreCloseOutcome,
    PhysicalWorkCapacity, PhysicalWorkCapacityDimension, PhysicalWorkReadiness,
    PhysicalWorkSubmissionOutcome,
};
use worth_store_physical_backend::MediaOperationRole;

use super::fixture::{
    family_locality_fixture, serving_from_initialization_with_work_profile, work_fixture,
};
use super::scheduler::{policy_receipt, ready_work, write_demand};

#[test]
fn bounded_command_arena_defers_without_retaining_an_unadmitted_identity() {
    let root = tempdir().unwrap();
    let (profile, request, _) = work_fixture();
    let capacity = PhysicalWorkCapacity::new(4, 1, 1_024, 1024 * 1024, 4 * 1024 * 1024).unwrap();
    let serving =
        serving_from_initialization_with_work_profile(root.path(), profile.with_capacity(capacity));
    let submission = serving.physical_read_submission();
    for _ in 0..4 {
        success(submission.submit(request.clone()));
    }
    assert!(matches!(
        submission.submit(request).into_raw(),
        TransitionOutcome::Deferred(deferred)
            if deferred.capacity() == 4
                && deferred.dimension() == PhysicalWorkCapacityDimension::Commands
    ));
    let closed = serving.close();
    assert_eq!(closed.work().declared(), 4);
    assert_eq!(closed.work().terminal().len(), 4);
    assert_eq!(closed.work().residual(), 0);
}

#[test]
fn completed_pre_effect_work_releases_capacity_for_sustained_churn() {
    let root = tempdir().unwrap();
    let (profile, request, _) = work_fixture();
    let capacity = PhysicalWorkCapacity::new(1, 1, 1, 1024 * 1024, 1024 * 1024)
        .unwrap()
        .with_terminal_evidence_capacity(4)
        .unwrap();
    let serving =
        serving_from_initialization_with_work_profile(root.path(), profile.with_capacity(capacity));
    let submission = serving.physical_read_submission();

    for _ in 0..32 {
        let receipt = success(submission.submit(request.clone()));
        let admitted = serving.admit_physical_work(receipt).unwrap();
        match serving.request_physical_work(admitted).unwrap() {
            PhysicalWorkReadiness::Ready(ready) => {
                let cancellation = serving
                    .cancel_physical_work(ready.consumer_handle())
                    .unwrap();
                assert_eq!(
                    cancellation.obligation(),
                    PhysicalEffectObligation::NotDispatched
                );
                drop(ready);
            }
            PhysicalWorkReadiness::Blocked(blocked) => {
                panic!(
                    "clean dependency unexpectedly blocked: {:?}",
                    blocked.condition()
                )
            }
        }
    }

    let closed = serving.close_plan().execute();
    assert!(matches!(closed, PhysicalStoreCloseOutcome::Closed { .. }));
    assert_eq!(closed.shutdown().work().declared(), 32);
    assert_eq!(
        closed
            .shutdown()
            .work()
            .drain()
            .cancelled_before_dispatch()
            .len(),
        4
    );
    assert_eq!(closed.shutdown().work().drain().evidence_capacity(), 4);
    assert_eq!(closed.shutdown().work().drain().evidence_overflow(), 0);
    assert_eq!(closed.shutdown().work().drain().safe_evidence_elided(), 28);
    assert_eq!(closed.shutdown().work().drain().exact_identity_count(), 32);
}

#[test]
fn successful_media_settlement_releases_capacity_for_sustained_churn() {
    let root = tempdir().unwrap();
    let (profile, _, mutation) = work_fixture();
    let capacity = PhysicalWorkCapacity::new(1, 1, 1, 1024 * 1024, 1024 * 1024)
        .unwrap()
        .with_terminal_evidence_capacity(4)
        .unwrap();
    let serving =
        serving_from_initialization_with_work_profile(root.path(), profile.with_capacity(capacity));
    let before = serving.media_counters();

    for _ in 0..16 {
        let demand = write_demand(&serving, ready_work(&serving, mutation.clone()));
        let queue = demand.queue_work();
        let backend = serving
            .admit_physical_scheduler_capability(queue.backend_requirement())
            .unwrap();
        let admitted = serving
            .admit_physical_scheduler_demand(
                demand,
                &backend,
                policy_receipt(queue.requested_budget()),
            )
            .unwrap();
        let command =
            PhysicalExecutorCommand::exact_write(admitted, b"settled!".as_slice()).unwrap();
        serving.execute_physical_work(command).unwrap();
    }

    let after = serving.media_counters();
    assert_eq!(
        after.attempts_for(MediaOperationRole::PositionedWrite)
            - before.attempts_for(MediaOperationRole::PositionedWrite),
        16
    );
    let closed = serving.close_plan().execute();
    assert!(matches!(closed, PhysicalStoreCloseOutcome::Closed { .. }));
    assert_eq!(closed.shutdown().work().declared(), 16);
    assert_eq!(closed.shutdown().work().residual(), 0);
}

#[test]
fn close_reconciles_a_dropped_pre_dispatch_signal_consumer() {
    let root = tempdir().unwrap();
    let (profile, request, _) = work_fixture();
    let serving = serving_from_initialization_with_work_profile(root.path(), profile);
    let receipt = success(serving.physical_read_submission().submit(request));
    let admitted = serving.admit_physical_work(receipt).unwrap();
    let ready = match serving.request_physical_work(admitted).unwrap() {
        PhysicalWorkReadiness::Ready(ready) => ready,
        PhysicalWorkReadiness::Blocked(_) => panic!("dependency should be ready"),
    };

    drop(ready);

    let closed = serving.close_plan().execute();
    assert!(matches!(closed, PhysicalStoreCloseOutcome::Closed { .. }));
    assert_eq!(
        closed
            .shutdown()
            .signal_summary()
            .unwrap()
            .active_in_flight_node_count(),
        0
    );
}

#[test]
fn dropped_ready_work_releases_signal_and_command_capacity_one_before_close() {
    let root = tempdir().unwrap();
    let (profile, request, _) = work_fixture();
    let capacity = PhysicalWorkCapacity::new(1, 1, 1, 1024 * 1024, 1024 * 1024)
        .unwrap()
        .with_terminal_evidence_capacity(16)
        .unwrap();
    let serving =
        serving_from_initialization_with_work_profile(root.path(), profile.with_capacity(capacity));
    let before = serving.media_counters();

    for _ in 0..8 {
        let receipt = success(serving.physical_read_submission().submit(request.clone()));
        let admitted = serving.admit_physical_work(receipt).unwrap();
        let ready = match serving.request_physical_work(admitted).unwrap() {
            PhysicalWorkReadiness::Ready(ready) => ready,
            PhysicalWorkReadiness::Blocked(_) => panic!("clean dependency should be ready"),
        };
        drop(ready);
        await_signal_cleanup(&serving);
    }

    assert_eq!(serving.media_counters(), before);
    let closed = serving.close_plan().execute();
    assert!(matches!(closed, PhysicalStoreCloseOutcome::Closed { .. }));
    assert_eq!(closed.shutdown().work().declared(), 8);
    assert_eq!(closed.shutdown().work().residual(), 0);
}

#[test]
fn dropped_blocked_revalidation_releases_its_minimum_two_lineage_capacity() {
    let root = tempdir().unwrap();
    let (profile, request, _, delta) = family_locality_fixture();
    let capacity = PhysicalWorkCapacity::new(2, 1, 2, 1024 * 1024, 2 * 1024 * 1024)
        .unwrap()
        .with_terminal_evidence_capacity(16)
        .unwrap();
    let serving =
        serving_from_initialization_with_work_profile(root.path(), profile.with_capacity(capacity));
    let before = serving.media_counters();

    for _ in 0..4 {
        let older = success(serving.physical_read_submission().submit(request.clone()));
        let newer = success(serving.physical_read_submission().submit(request.clone()));
        let older = serving.admit_physical_work(older).unwrap();
        let newer = serving.admit_physical_work(newer).unwrap();
        let older = match serving.request_physical_work(older).unwrap() {
            PhysicalWorkReadiness::Ready(ready) => ready,
            PhysicalWorkReadiness::Blocked(_) => panic!("older request should initially be ready"),
        };
        let newer = match serving.request_physical_work(newer).unwrap() {
            PhysicalWorkReadiness::Ready(ready) => ready,
            PhysicalWorkReadiness::Blocked(_) => panic!("newer request should be ready"),
        };
        serving
            .certification_apply_physical_aspect_delta(delta.clone())
            .unwrap();
        let blocked = match serving.revalidate_physical_work(older).unwrap() {
            PhysicalWorkReadiness::Blocked(blocked) => blocked,
            PhysicalWorkReadiness::Ready(_) => {
                panic!("superseded active lineage should remain blocked")
            }
        };
        drop(newer);
        drop(blocked);
        await_signal_cleanup(&serving);
    }

    assert_eq!(serving.media_counters(), before);
    let closed = serving.close_plan().execute();
    assert!(matches!(closed, PhysicalStoreCloseOutcome::Closed { .. }));
    assert_eq!(closed.shutdown().work().declared(), 8);
    assert_eq!(closed.shutdown().work().residual(), 0);
}

#[test]
fn aggregate_scope_and_per_work_semantic_budgets_defer_before_retention() {
    let root = tempdir().unwrap();
    let (profile, request, _) = work_fixture();
    let scope_limited = PhysicalWorkCapacity::new(4, 1, 1, 1024 * 1024, 4 * 1024 * 1024).unwrap();
    let serving = serving_from_initialization_with_work_profile(
        root.path(),
        profile.clone().with_capacity(scope_limited),
    );
    let submission = serving.physical_read_submission();
    success(submission.submit(request.clone()));
    assert!(matches!(
        submission.submit(request.clone()).into_raw(),
        TransitionOutcome::Deferred(deferred)
            if deferred.dimension() == PhysicalWorkCapacityDimension::TotalScopeMembers
    ));
    serving.close();

    let semantic_limited = PhysicalWorkCapacity::new(4, 1, 1_024, 1, 4).unwrap();
    let reopened = super::fixture::serving_from_open_with_work_profile(
        root.path(),
        profile.with_capacity(semantic_limited),
    );
    assert!(matches!(
        reopened.physical_read_submission().submit(request).into_raw(),
        TransitionOutcome::Deferred(deferred)
            if deferred.dimension() == PhysicalWorkCapacityDimension::SemanticBytesPerWork
    ));
    assert_eq!(reopened.close().work().declared(), 0);
}

fn success(
    outcome: PhysicalWorkSubmissionOutcome,
) -> worth_store::physical_runtime::PhysicalWorkSubmissionReceipt {
    match outcome.into_raw() {
        TransitionOutcome::Success(receipt) => receipt,
        outcome => panic!("physical work should submit: {outcome:?}"),
    }
}

fn await_signal_cleanup(serving: &worth_store::physical_runtime::ServingPhysicalRuntime) {
    let deadline = std::time::Instant::now() + std::time::Duration::from_secs(5);
    loop {
        let observation = serving.physical_signal_observation().unwrap();
        if observation.active_locality_count() == 0 && observation.active_in_flight_count() == 0 {
            return;
        }
        assert!(
            std::time::Instant::now() < deadline,
            "dropped physical work retained Signal or locality capacity: locality={}, in_flight={}",
            observation.active_locality_count(),
            observation.active_in_flight_count(),
        );
        std::thread::yield_now();
    }
}
