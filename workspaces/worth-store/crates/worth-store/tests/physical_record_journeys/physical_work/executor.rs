use tempfile::tempdir;
use worth_foundational::FoundationalPerformanceWorkClass;
use worth_store::physical_runtime::{
    PhysicalEffectObligation, PhysicalExecutorCommand, PhysicalSchedulerDemand,
    PhysicalSignalSettlementOutcome, PhysicalStoreCloseOutcome, PhysicalWorkCounterStage,
    PhysicalWorkEffectFate, PhysicalWorkOperationFamily, PhysicalWorkPreEffectDenial,
    PhysicalWorkPressureClass,
};
use worth_store_physical_backend::MediaOperationRole;

use super::fixture::{
    disjoint_mutation_fixture, family_locality_fixture,
    serving_from_initialization_with_work_profile, work_fixture,
};
use super::scheduler::{
    policy_receipt, policy_receipt_for, ready_read_work, ready_work, secure_demand, write_demand,
};

#[test]
fn write_settles_physical_truth_before_signal_completion() {
    let root = tempdir().unwrap();
    let (profile, _, mutation_request) = work_fixture();
    let serving = serving_from_initialization_with_work_profile(root.path(), profile);
    let demand = write_demand(&serving, ready_work(&serving, mutation_request));
    let work = demand.queue_work();
    let backend = serving
        .admit_physical_scheduler_capability(work.backend_requirement())
        .unwrap();
    let demand = secure_demand(demand, &backend);
    let admitted = serving
        .admit_physical_scheduler_demand(demand, &backend, policy_receipt(work.requested_budget()))
        .unwrap();
    let consumer = admitted.consumer_handle();
    let before = serving.media_counters();
    let command = PhysicalExecutorCommand::exact_write(admitted, b"executed".as_slice()).unwrap();

    let outcome = serving.execute_physical_work(command).unwrap();

    assert_eq!(
        outcome.settled().evidence().fate(),
        PhysicalWorkEffectFate::WriteCompleted
    );
    assert_eq!(outcome.signal(), PhysicalSignalSettlementOutcome::Committed);
    assert_eq!(outcome.settled().intent().identity(), consumer.identity());
    let after = serving.media_counters();
    assert_eq!(
        after.attempts_for(MediaOperationRole::PositionedWrite)
            - before.attempts_for(MediaOperationRole::PositionedWrite),
        1
    );
    serving.close();
}

#[test]
fn exact_write_checkpoint_precedes_scheduler_and_signal_settlement() {
    use worth_store::physical_runtime::certification::CertificationPhysicalExecutionCheckpoint;

    let root = tempdir().unwrap();
    let (profile, _, mutation_request) = work_fixture();
    let serving = serving_from_initialization_with_work_profile(root.path(), profile);
    let admitted = admitted_write(&serving, mutation_request);
    let command = PhysicalExecutorCommand::exact_write(admitted, b"executed".as_slice()).unwrap();
    let checkpoint = serving.certification_pause_physical_execution_at(
        CertificationPhysicalExecutionCheckpoint::AfterExactWriteBeforeSchedulerSettlement,
    );
    let settled_before = serving.physical_work_observer().causal().records().len();
    let writes_before = serving
        .media_counters()
        .attempts_for(MediaOperationRole::PositionedWrite);

    std::thread::scope(|scope| {
        let execution = scope.spawn(|| serving.execute_physical_work(command));
        assert!(checkpoint.await_arrival());
        assert_eq!(
            serving
                .media_counters()
                .attempts_for(MediaOperationRole::PositionedWrite),
            writes_before + 1
        );
        assert_eq!(
            serving.physical_work_observer().causal().records().len(),
            settled_before
        );
        checkpoint.release();
        assert!(execution.join().unwrap().is_ok());
    });
    assert_eq!(
        serving.physical_work_observer().causal().records().len(),
        settled_before + 1
    );
    serving.close();
}

#[test]
fn read_returns_the_exact_bounded_destination() {
    let root = tempdir().unwrap();
    let (profile, read_request, _) = work_fixture();
    let serving = serving_from_initialization_with_work_profile(root.path(), profile);
    let ready = ready_read_work(&serving, read_request);
    let demand = PhysicalSchedulerDemand::foreground(
        ready,
        super::reserved_buffered_file_read(&serving),
        None,
    )
    .unwrap();
    let work = demand.queue_work();
    let backend = serving
        .admit_physical_scheduler_capability(work.backend_requirement())
        .unwrap();
    let admitted = serving
        .admit_physical_scheduler_demand(
            demand,
            &backend,
            policy_receipt_for(
                work.requested_budget(),
                0,
                FoundationalPerformanceWorkClass::AuthoritativeRead,
            ),
        )
        .unwrap();
    let command = PhysicalExecutorCommand::read(admitted).unwrap();

    let outcome = serving.execute_physical_work(command).unwrap();

    assert_eq!(
        outcome.settled().evidence().fate(),
        PhysicalWorkEffectFate::ReadCompleted
    );
    assert_eq!(outcome.settled().evidence().completed_payload_bytes(), 8);
    assert_eq!(outcome.signal(), PhysicalSignalSettlementOutcome::Committed);
    serving.close();
}

#[test]
fn cancellation_before_dispatch_revokes_the_command_without_an_effect() {
    let root = tempdir().unwrap();
    let (profile, _, mutation_request) = work_fixture();
    let serving = serving_from_initialization_with_work_profile(root.path(), profile);
    let admitted = admitted_write(&serving, mutation_request);
    let consumer = admitted.consumer_handle();
    let command = PhysicalExecutorCommand::exact_write(admitted, b"cancelld".as_slice()).unwrap();
    let before = serving.media_counters();

    let cancellation = serving.cancel_physical_work(consumer).unwrap();

    assert!(cancellation.signal().cancelled_request().is_some());
    assert_eq!(
        cancellation.obligation(),
        PhysicalEffectObligation::NotDispatched
    );
    assert!(matches!(
        serving.execute_physical_work(command),
        Err(PhysicalWorkPreEffectDenial::ConsumerCancelled)
    ));
    assert_eq!(serving.media_counters(), before);
    let closed = serving.close();
    assert_eq!(
        closed.work().drain().cancelled_before_dispatch(),
        &[consumer.identity()]
    );
}

#[test]
fn signal_timeout_uses_deterministic_clock_and_proves_no_dispatch() {
    let root = tempdir().unwrap();
    let (profile, _, mutation_request) = work_fixture();
    let serving = serving_from_initialization_with_work_profile(root.path(), profile);
    let admitted = admitted_write(&serving, mutation_request);
    let consumer = admitted.consumer_handle();
    let command = PhysicalExecutorCommand::exact_write(admitted, b"timedout".as_slice()).unwrap();
    let before = serving.media_counters();
    serving
        .advance_physical_signal_clock(
            consumer,
            worth_signal::facade::ClockAdvanceRequest::new(
                worth_signal::facade::ClockDomain::MonotonicExecution,
                worth_signal::facade::ClockTick::new(1_000),
            ),
        )
        .unwrap();

    let timeout = serving.timeout_physical_work(consumer).unwrap();

    assert!(timeout.signal().timed_out_request().is_some());
    assert_eq!(
        timeout.obligation(),
        PhysicalEffectObligation::NotDispatched,
        "C5_PREDICATE:store-local-async-registry: local timeout state overrode Signal's effect obligation"
    );
    assert!(matches!(
        serving.execute_physical_work(command),
        Err(PhysicalWorkPreEffectDenial::ConsumerCancelled)
    ));
    assert_eq!(serving.media_counters(), before);
    let closed = serving.close();
    assert_eq!(
        closed.work().drain().cancelled_before_dispatch(),
        &[consumer.identity()]
    );
}

#[test]
fn bounded_completion_batch_preserves_each_physical_identity() {
    let root = tempdir().unwrap();
    let (profile, first_request, second_request) = disjoint_mutation_fixture();
    let serving = serving_from_initialization_with_work_profile(root.path(), profile);
    let first = admitted_write(&serving, first_request);
    let second = admitted_write(&serving, second_request);
    let identities = [first.intent().identity(), second.intent().identity()];
    let before = serving.media_counters();
    let commands = vec![
        PhysicalExecutorCommand::exact_write(first, b"first001".as_slice()).unwrap(),
        PhysicalExecutorCommand::exact_write(second, b"second02".as_slice()).unwrap(),
    ]
    .into_boxed_slice();

    let batch = serving.execute_physical_work_batch(commands);

    assert!(batch.denied_before_effect().is_empty());
    assert_eq!(batch.executions().len(), 2);
    let signal_outcomes = batch
        .executions()
        .iter()
        .map(|execution| execution.signal())
        .collect::<Vec<_>>();
    assert!(
        signal_outcomes
            .iter()
            .all(|outcome| *outcome != PhysicalSignalSettlementOutcome::DerivedStateUnavailable),
        "batch Signal outcomes: {signal_outcomes:?}"
    );
    assert_eq!(
        signal_outcomes
            .iter()
            .filter(|outcome| **outcome == PhysicalSignalSettlementOutcome::Committed)
            .count(),
        1
    );
    assert_eq!(
        signal_outcomes
            .iter()
            .filter(|outcome| {
                **outcome == PhysicalSignalSettlementOutcome::ReconciledFromPhysicalTruth
            })
            .count(),
        1
    );
    assert_eq!(
        batch
            .executions()
            .iter()
            .map(|execution| execution.settled().intent().identity())
            .collect::<Vec<_>>(),
        identities
    );
    assert_eq!(
        serving
            .media_counters()
            .attempts_for(MediaOperationRole::PositionedWrite)
            - before.attempts_for(MediaOperationRole::PositionedWrite),
        2
    );
    assert_eq!(
        serving.physical_work_counters().count(
            PhysicalWorkOperationFamily::ArtifactRangeWrite,
            PhysicalWorkCounterStage::Terminal,
        ),
        2
    );
    assert_eq!(
        serving.physical_work_counters().count_under_pressure(
            PhysicalWorkOperationFamily::ArtifactRangeWrite,
            PhysicalWorkPressureClass::ForegroundMutation,
            PhysicalWorkCounterStage::Terminal,
        ),
        2
    );

    let closed = serving.close_plan().execute();
    assert!(
        matches!(closed, PhysicalStoreCloseOutcome::Closed { .. }),
        "close inspection: records={:?}, residency={}, drain={:?}, signal={:?}, cancellations={}, summary={:?}, media={:?}",
        closed.shutdown().records().posture(),
        closed.shutdown().residency().requires_inspection(),
        closed.shutdown().work().drain(),
        closed.shutdown().signal(),
        closed.shutdown().signal_cancellation_failures(),
        closed.shutdown().signal_summary(),
        closed.shutdown().media().release(),
    );
    assert_eq!(closed.shutdown().work().drain().settled(), identities);
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
fn mixed_route_batch_completes_each_settlement_on_its_admitted_signal_route() {
    let root = tempdir().unwrap();
    let (profile, read_request, write_request, _) = family_locality_fixture();
    let serving = serving_from_initialization_with_work_profile(root.path(), profile);
    let read = ready_read_work(&serving, read_request);
    let read_demand = PhysicalSchedulerDemand::foreground(
        read,
        super::reserved_buffered_file_read(&serving),
        None,
    )
    .unwrap();
    let read_work = read_demand.queue_work();
    let read_backend = serving
        .admit_physical_scheduler_capability(read_work.backend_requirement())
        .unwrap();
    let read_demand = secure_demand(read_demand, &read_backend);
    let read = serving
        .admit_physical_scheduler_demand(
            read_demand,
            &read_backend,
            policy_receipt_for(
                read_work.requested_budget(),
                0,
                FoundationalPerformanceWorkClass::AuthoritativeRead,
            ),
        )
        .unwrap();
    let write = admitted_write(&serving, write_request);
    let identities = [read.intent().identity(), write.intent().identity()];

    let batch = serving.execute_physical_work_batch(
        vec![
            PhysicalExecutorCommand::read(read).unwrap(),
            PhysicalExecutorCommand::exact_write(write, b"routes!!".as_slice()).unwrap(),
        ]
        .into_boxed_slice(),
    );

    assert!(batch.denied_before_effect().is_empty());
    assert_eq!(batch.executions().len(), 2);
    assert_eq!(
        batch
            .executions()
            .iter()
            .map(|execution| execution.settled().intent().identity())
            .collect::<Vec<_>>(),
        identities
    );
    assert!(batch
        .executions()
        .iter()
        .all(|execution| execution.signal() == PhysicalSignalSettlementOutcome::Committed));
    let closed = serving.close_plan().execute();
    assert!(matches!(closed, PhysicalStoreCloseOutcome::Closed { .. }));
    let signal = closed.shutdown().signal_summary().unwrap();
    assert_eq!(signal.active_in_flight_node_count(), 0);
}

pub(super) fn admitted_write(
    serving: &worth_store::physical_runtime::ServingPhysicalRuntime,
    request: worth_store::physical_runtime::PhysicalMutationWorkRequest,
) -> worth_store::physical_runtime::ResourceAdmittedPhysicalWork {
    let demand = write_demand(serving, ready_work(serving, request));
    let work = demand.queue_work();
    let backend = serving
        .admit_physical_scheduler_capability(work.backend_requirement())
        .unwrap();
    let demand = secure_demand(demand, &backend);
    serving
        .admit_physical_scheduler_demand(demand, &backend, policy_receipt(work.requested_budget()))
        .unwrap()
}
