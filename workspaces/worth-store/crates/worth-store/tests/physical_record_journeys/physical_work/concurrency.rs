use tempfile::tempdir;
use worth_proof::TransitionOutcome;
use worth_store::physical_runtime::{
    PhysicalEffectObligation, PhysicalExecutorCommand, PhysicalStoreCloseOutcome,
    PhysicalStoreClosePhase, PhysicalWorkEffectFate, PhysicalWorkExecutionOutcome,
    PhysicalWorkPreEffectDenial,
};
use worth_store_physical_backend::MediaOperationRole;
use worth_store_physical_backend::{FilesystemAccessPosture, MediaFaultDirective};

use super::{
    executor::admitted_write,
    fixture::{
        disjoint_artifact_mutation_fixture, serving_from_initialization_with_work_profile,
        work_fixture,
    },
};

#[test]
fn independent_mutation_capabilities_execute_without_a_global_runtime_borrow() {
    let root = tempdir().unwrap();
    let (profile, first_request, second_request) = disjoint_artifact_mutation_fixture();
    serving_from_initialization_with_work_profile(root.path(), profile.clone()).close();
    let admission = worth_store::physical_runtime::FilesystemMediaAdmission::production(
        FilesystemAccessPosture::CoordinatedServiceAccount,
    );
    let authority = admission.fault_schedule_authority();
    let first_gate = authority.pause_gate();
    let second_gate = authority.pause_gate();
    let schedule = authority
        .schedule(vec![
            authority.rule(
                MediaOperationRole::PositionedWrite,
                1,
                MediaFaultDirective::PauseBefore(first_gate.clone()),
            ),
            authority.rule(
                MediaOperationRole::PositionedWrite,
                2,
                MediaFaultDirective::PauseBefore(second_gate.clone()),
            ),
        ])
        .unwrap();
    let runtime = worth_store::physical_runtime::PhysicalStore::admit(
        worth_store::physical_runtime::PhysicalRuntimeAdmission::new(root.path()).unwrap(),
    )
    .unwrap();
    let media = match runtime
        .try_admit_filesystem_media(admission.with_fault_schedule(schedule))
        .into_raw()
    {
        TransitionOutcome::Success(media) => media,
        _ => panic!("faulted media should admit"),
    };
    let (format, _, access) = super::super::configuration();
    let serving = super::super::success(
        media.open_record_store(
            worth_store::physical_runtime::PhysicalRecordOpen::new(format, access)
                .with_physical_work_profile(profile),
        ),
    );
    let before = serving.media_counters();
    let first = admitted_write(&serving, first_request);
    let second = admitted_write(&serving, second_request);
    assert_eq!(
        first
            .concurrency_scope()
            .relation(&second.concurrency_scope()),
        worth_store::physical_runtime::PhysicalWorkConcurrencyRelation::DisjointArtifacts,
    );
    let first = PhysicalExecutorCommand::exact_write(first, b"thread01".as_slice()).unwrap();
    let second = PhysicalExecutorCommand::exact_write(second, b"thread02".as_slice()).unwrap();
    let (first, second, overlapped) = std::thread::scope(|scope| {
        let first = scope.spawn(|| serving.execute_physical_work(first));
        let second = scope.spawn(|| serving.execute_physical_work(second));
        let deadline = std::time::Instant::now() + std::time::Duration::from_secs(5);
        while (first_gate.reached_context().is_none() || second_gate.reached_context().is_none())
            && std::time::Instant::now() < deadline
        {
            std::thread::yield_now();
        }
        let overlapped =
            first_gate.reached_context().is_some() && second_gate.reached_context().is_some();
        let first_context = first_gate.reached_context();
        let second_context = second_gate.reached_context();
        first_gate.release();
        second_gate.release();
        if let (Some(first_context), Some(second_context)) = (first_context, second_context) {
            assert_ne!(first_context.role_ordinal(), second_context.role_ordinal());
            assert_eq!(first_context.requested_bytes(), 8);
            assert_eq!(second_context.requested_bytes(), 8);
        }
        (
            first.join().unwrap().unwrap(),
            second.join().unwrap().unwrap(),
            overlapped,
        )
    });

    assert_completed_distinct(&first, &second);
    let first_effect = first.settled().effect_identity().unwrap();
    let second_effect = second.settled().effect_identity().unwrap();
    assert_eq!(first_effect.work(), first.settled().intent().identity());
    assert_eq!(second_effect.work(), second.settled().intent().identity());
    assert_ne!(
        first_effect.backend_operation(),
        second_effect.backend_operation()
    );
    assert!(
        overlapped,
        "both disjoint target effects must reach the backend while the other is paused"
    );
    assert_eq!(
        serving
            .media_counters()
            .attempts_for(MediaOperationRole::PositionedWrite)
            - before.attempts_for(MediaOperationRole::PositionedWrite),
        2
    );
    assert!(matches!(
        serving.close_plan().execute(),
        PhysicalStoreCloseOutcome::Closed { .. }
    ));
}

#[test]
fn close_waits_for_a_dispatched_execution_capability_before_disposal() {
    let root = tempdir().unwrap();
    let (profile, _, request) = work_fixture();
    serving_from_initialization_with_work_profile(root.path(), profile.clone()).close();
    let admission = worth_store::physical_runtime::FilesystemMediaAdmission::production(
        FilesystemAccessPosture::CoordinatedServiceAccount,
    );
    let authority = admission.fault_schedule_authority();
    let gate = authority.pause_gate();
    let schedule = authority
        .schedule(vec![authority.rule(
            MediaOperationRole::PositionedWrite,
            1,
            MediaFaultDirective::PauseBefore(gate.clone()),
        )])
        .unwrap();
    let runtime = worth_store::physical_runtime::PhysicalStore::admit(
        worth_store::physical_runtime::PhysicalRuntimeAdmission::new(root.path()).unwrap(),
    )
    .unwrap();
    let media = match runtime
        .try_admit_filesystem_media(admission.with_fault_schedule(schedule))
        .into_raw()
    {
        TransitionOutcome::Success(media) => media,
        _ => panic!("faulted media should admit"),
    };
    let (format, _, access) = super::super::configuration();
    let serving = super::super::success(
        media.open_record_store(
            worth_store::physical_runtime::PhysicalRecordOpen::new(format, access)
                .with_physical_work_profile(profile),
        ),
    );
    let command = PhysicalExecutorCommand::exact_write(
        admitted_write(&serving, request),
        b"closing!".as_slice(),
    )
    .unwrap();
    let execution = serving.physical_work_execution();
    let close = serving.close_plan();
    let progress = close.observation();

    let (settled, closed) = std::thread::scope(|scope| {
        let effect = scope.spawn(move || execution.execute_physical_work(command));
        let deadline = std::time::Instant::now() + std::time::Duration::from_secs(5);
        while gate.reached_context().is_none() && std::time::Instant::now() < deadline {
            std::thread::yield_now();
        }
        assert!(
            gate.reached_context().is_some(),
            "effect never reached pause gate"
        );
        let closing = scope.spawn(move || close.execute());
        let deadline = std::time::Instant::now() + std::time::Duration::from_secs(5);
        while !progress.reached(PhysicalStoreClosePhase::AdmissionStopped)
            && std::time::Instant::now() < deadline
        {
            std::thread::yield_now();
        }
        assert!(progress.reached(PhysicalStoreClosePhase::AdmissionStopped));
        assert!(!progress.reached(PhysicalStoreClosePhase::DispatchSettlementComplete));
        assert!(!progress.reached(PhysicalStoreClosePhase::SignalDisposed));
        gate.release();
        (effect.join().unwrap().unwrap(), closing.join().unwrap())
    });

    assert_eq!(
        settled.settled().evidence().fate(),
        PhysicalWorkEffectFate::WriteCompleted
    );
    assert!(matches!(closed, PhysicalStoreCloseOutcome::Closed { .. }));
    assert!(progress.reached(PhysicalStoreClosePhase::MediaReleased));
}

#[test]
fn overlapping_exact_writes_settle_as_whole_artifact_coordinated_effects() {
    let root = tempdir().unwrap();
    let (profile, _, request) = work_fixture();
    let serving = serving_from_initialization_with_work_profile(root.path(), profile);

    let (first, second) = execute_two(
        &serving,
        admitted_write(&serving, request.clone()),
        admitted_write(&serving, request),
        *b"winner01",
        *b"winner02",
    );

    assert_completed_distinct(&first, &second);
    let catalog = std::fs::read(root.path().join("families/records/bootstrap.catalog")).unwrap();
    assert!(
        &catalog[8..16] == b"winner01" || &catalog[8..16] == b"winner02",
        "the backend coordination boundary must not expose a torn overlapping write"
    );
    serving.close();
}

#[test]
fn cancellation_and_dispatch_have_one_atomic_physical_winner() {
    let root = tempdir().unwrap();
    let (profile, _, request) = work_fixture();
    let serving = serving_from_initialization_with_work_profile(root.path(), profile);
    for ordinal in 0..32_u8 {
        let admitted = admitted_write(&serving, request.clone());
        let consumer = admitted.consumer_handle();
        let command =
            PhysicalExecutorCommand::exact_write(admitted, [ordinal; 8].as_slice()).unwrap();
        let barrier = std::sync::Barrier::new(3);
        let before = serving.media_counters();
        let (execution, cancellation) = std::thread::scope(|scope| {
            let execute_barrier = &barrier;
            let cancel_barrier = &barrier;
            let execution = scope.spawn(|| {
                execute_barrier.wait();
                serving.execute_physical_work(command)
            });
            let cancellation = scope.spawn(|| {
                cancel_barrier.wait();
                serving.cancel_physical_work(consumer)
            });
            barrier.wait();
            (
                execution.join().unwrap(),
                cancellation.join().unwrap().unwrap(),
            )
        });
        let effects = serving
            .media_counters()
            .attempts_for(MediaOperationRole::PositionedWrite)
            - before.attempts_for(MediaOperationRole::PositionedWrite);
        match execution {
            Ok(settled) => {
                assert_eq!(
                    settled.settled().evidence().fate(),
                    PhysicalWorkEffectFate::WriteCompleted
                );
                assert_eq!(effects, 1);
                assert_eq!(
                    cancellation.obligation(),
                    PhysicalEffectObligation::SettlementContinues
                );
            }
            Err(PhysicalWorkPreEffectDenial::ConsumerCancelled) => {
                assert_eq!(effects, 0);
                assert_eq!(
                    cancellation.obligation(),
                    PhysicalEffectObligation::NotDispatched
                );
            }
            Err(other) => panic!("unexpected race denial: {other:?}"),
        }
    }
    assert!(matches!(
        serving.close_plan().execute(),
        PhysicalStoreCloseOutcome::Closed { .. }
    ));
}

fn execute_two(
    serving: &worth_store::physical_runtime::ServingPhysicalRuntime,
    first: worth_store::physical_runtime::ResourceAdmittedPhysicalWork,
    second: worth_store::physical_runtime::ResourceAdmittedPhysicalWork,
    first_bytes: [u8; 8],
    second_bytes: [u8; 8],
) -> (PhysicalWorkExecutionOutcome, PhysicalWorkExecutionOutcome) {
    let first = PhysicalExecutorCommand::exact_write(first, first_bytes.as_slice()).unwrap();
    let second = PhysicalExecutorCommand::exact_write(second, second_bytes.as_slice()).unwrap();
    let barrier = std::sync::Barrier::new(3);
    std::thread::scope(|scope| {
        let first_barrier = &barrier;
        let second_barrier = &barrier;
        let first = scope.spawn(move || {
            first_barrier.wait();
            serving.execute_physical_work(first)
        });
        let second = scope.spawn(move || {
            second_barrier.wait();
            serving.execute_physical_work(second)
        });
        barrier.wait();
        (
            first.join().unwrap().unwrap(),
            second.join().unwrap().unwrap(),
        )
    })
}

fn assert_completed_distinct(
    first: &PhysicalWorkExecutionOutcome,
    second: &PhysicalWorkExecutionOutcome,
) {
    assert_eq!(
        first.settled().evidence().fate(),
        PhysicalWorkEffectFate::WriteCompleted
    );
    assert_eq!(
        second.settled().evidence().fate(),
        PhysicalWorkEffectFate::WriteCompleted
    );
    assert_ne!(
        first.settled().intent().identity(),
        second.settled().intent().identity()
    );
}
