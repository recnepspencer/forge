use std::{
    collections::HashSet,
    sync::mpsc,
    time::{Duration, Instant},
};

use worth_store::physical_runtime::{
    PhysicalEffectObligation, PhysicalExecutorCommand, PhysicalSignalSettlementOutcome,
    PhysicalWorkConcurrencyRelation, PhysicalWorkConsumerHandle, PhysicalWorkEffectFate,
    PhysicalWorkIdentity, PhysicalWorkPreEffectDenial, ReadyPhysicalWork,
    ResourceAdmittedPhysicalWork, ServingPhysicalRuntime,
};
use worth_store_physical_backend::{MediaOperationRole, MediaPauseGate};

use super::{fixture::MaelstromPauseGates, world::MaelstromWorld, LifecycleMaelstromModel};

pub(super) struct JoinedTrace {
    pub denial: PhysicalWorkIdentity,
    pub pre_dispatch_cancellation: PhysicalWorkIdentity,
    pub post_dispatch_cancellation: PhysicalWorkIdentity,
}

struct ReorderedReadCommands {
    first: PhysicalExecutorCommand,
    second: PhysicalExecutorCommand,
    first_consumer: PhysicalWorkConsumerHandle,
    first_identity: PhysicalWorkIdentity,
}

pub(super) fn execute(world: &MaelstromWorld, model: &LifecycleMaelstromModel) -> JoinedTrace {
    let (first, second, write) = apply_one_slice_delta(world);
    let denial =
        super::workflows::deny_scheduler_breadth(&world.serving, world.fixture.reads[0].clone());
    let pre_dispatch_cancellation = prove_pre_dispatch_cancellation(
        &world.serving,
        super::workflows::ready_read(&world.serving, world.fixture.reads[1].clone()),
    );
    let post_dispatch_cancellation = prove_reordered_reads(world, first, second, model);
    let retry = prove_clock_retry(world, write, model);
    let writeback = prove_exact_writeback(world, model);
    let append =
        super::append_preparation::prepare_and_publish_independently(&world.serving, &world.gates);
    assert_eq!(append.generations, model.append_generations);
    assert_causal_evidence(world, &append.work, retry, writeback);
    JoinedTrace {
        denial,
        pre_dispatch_cancellation,
        post_dispatch_cancellation,
    }
}

fn apply_one_slice_delta(
    world: &MaelstromWorld,
) -> (ReadyPhysicalWork, ReadyPhysicalWork, ReadyPhysicalWork) {
    let first = super::workflows::ready_read(&world.serving, world.fixture.reads[0].clone());
    let second = super::workflows::ready_read(&world.serving, world.fixture.reads[1].clone());
    let write = super::workflows::ready_write(&world.serving, world.fixture.write.clone());
    let lineages = [
        first.signal_request(),
        second.signal_request(),
        write.signal_request(),
    ];
    let before = world.serving.media_counters();
    world
        .serving
        .certification_apply_physical_aspect_delta(world.fixture.read_delta.clone())
        .unwrap();
    let first = revalidate(&world.serving, first);
    let second = revalidate(&world.serving, second);
    let write = revalidate(&world.serving, write);
    assert_ne!(first.signal_request(), lineages[0]);
    assert_eq!(second.signal_request(), lineages[1]);
    assert_eq!(write.signal_request(), lineages[2]);
    assert_eq!(world.serving.media_counters(), before);
    (first, second, write)
}

fn revalidate(serving: &ServingPhysicalRuntime, ready: ReadyPhysicalWork) -> ReadyPhysicalWork {
    super::workflows::expect_ready(serving.revalidate_physical_work(ready).unwrap())
}

fn prove_pre_dispatch_cancellation(
    serving: &ServingPhysicalRuntime,
    ready: ReadyPhysicalWork,
) -> PhysicalWorkIdentity {
    let admitted = super::workflows::admit_read(serving, ready);
    let consumer = admitted.consumer_handle();
    let identity = admitted.intent().identity();
    let command = PhysicalExecutorCommand::read(admitted).unwrap();
    let before = serving.media_counters();
    let cancelled = serving.cancel_physical_work(consumer).unwrap();
    assert_eq!(
        cancelled.obligation(),
        PhysicalEffectObligation::NotDispatched
    );
    assert!(matches!(
        serving.execute_physical_work(command),
        Err(PhysicalWorkPreEffectDenial::ConsumerCancelled)
    ));
    assert_eq!(serving.media_counters(), before);
    identity
}

fn prove_reordered_reads(
    world: &MaelstromWorld,
    first: ReadyPhysicalWork,
    second: ReadyPhysicalWork,
    model: &LifecycleMaelstromModel,
) -> PhysicalWorkIdentity {
    let before = world.serving.media_counters();
    let (first, second, cancelled) =
        execute_reordered_reads(&world.serving, first, second, &world.gates);
    assert_eq!(
        [
            first.settled().evidence().fate(),
            second.settled().evidence().fate(),
        ],
        [
            PhysicalWorkEffectFate::ReadCompleted,
            PhysicalWorkEffectFate::ReadCompleted,
        ]
    );
    let after = world.serving.media_counters();
    assert_eq!(
        after.attempts_for(MediaOperationRole::PositionedRead)
            - before.attempts_for(MediaOperationRole::PositionedRead),
        model.disjoint_read_effects
    );
    cancelled
}

fn prove_clock_retry(
    world: &MaelstromWorld,
    write: ReadyPhysicalWork,
    model: &LifecycleMaelstromModel,
) -> PhysicalWorkIdentity {
    let before = world.serving.media_counters();
    let identity = super::workflows::retry_write_after_clock_wake(
        &world.serving,
        write,
        &world.original_writeback,
    );
    let after = world.serving.media_counters();
    assert_eq!(
        after.attempts_for(MediaOperationRole::PositionedWrite)
            - before.attempts_for(MediaOperationRole::PositionedWrite),
        model.retry_write_attempts
    );
    assert_eq!(
        after.completed_operations_for(MediaOperationRole::PositionedWrite)
            - before.completed_operations_for(MediaOperationRole::PositionedWrite),
        model.retry_write_completions
    );
    identity
}

fn prove_exact_writeback(
    world: &MaelstromWorld,
    model: &LifecycleMaelstromModel,
) -> PhysicalWorkIdentity {
    let before = world.serving.media_counters();
    let identity = super::workflows::execute_exact_writeback(
        &world.serving,
        world.fixture.write.clone(),
        world.original_writeback.clone(),
    );
    let after = world.serving.media_counters();
    assert_eq!(
        after.attempts_for(MediaOperationRole::PositionedWrite)
            - before.attempts_for(MediaOperationRole::PositionedWrite),
        model.exact_writeback_effects
    );
    assert_eq!(
        &std::fs::read(&world.catalog_path).unwrap()[8..16],
        world.original_writeback
    );
    identity
}

fn assert_causal_evidence(
    world: &MaelstromWorld,
    append_work: &[PhysicalWorkIdentity],
    retry: PhysicalWorkIdentity,
    writeback: PhysicalWorkIdentity,
) {
    let causal = world.observer.causal().records();
    for identity in append_work {
        assert_eq!(
            causal
                .iter()
                .filter(|record| record.identity() == *identity)
                .count(),
            1,
            "each append effect must have one causal settlement"
        );
    }
    assert!(causal.iter().any(|record| record.identity() == retry));
    assert!(causal.iter().any(|record| record.identity() == writeback));
    assert!(causal.iter().all(|record| {
        record.derived_completion()
            != Some(PhysicalSignalSettlementOutcome::DerivedStateUnavailable)
    }));
    let backend = causal
        .iter()
        .filter_map(|record| record.backend_operation())
        .collect::<Vec<_>>();
    assert_eq!(
        backend.iter().copied().collect::<HashSet<_>>().len(),
        backend.len(),
        "one backend effect cannot settle two physical attempts"
    );
}

fn execute_reordered_reads(
    serving: &ServingPhysicalRuntime,
    first: ReadyPhysicalWork,
    second: ReadyPhysicalWork,
    gates: &MaelstromPauseGates,
) -> (
    worth_store::physical_runtime::PhysicalWorkExecutionOutcome,
    worth_store::physical_runtime::PhysicalWorkExecutionOutcome,
    PhysicalWorkIdentity,
) {
    let commands = prepare_reordered_reads(serving, first, second);
    let first_execution = serving.physical_work_execution();
    let second_execution = serving.physical_work_execution();
    let (completed, observed) = mpsc::sync_channel(2);
    std::thread::scope(|scope| {
        let first_completed = completed.clone();
        scope.spawn(move || {
            first_completed
                .send((1, first_execution.execute_physical_work(commands.first)))
                .unwrap();
        });
        require_gate(&gates.first_read, &gates.second_read, "first disjoint read");
        scope.spawn(move || {
            completed
                .send((2, second_execution.execute_physical_work(commands.second)))
                .unwrap();
        });
        require_gate(
            &gates.second_read,
            &gates.first_read,
            "second disjoint read",
        );
        let cancellation = serving
            .cancel_physical_work(commands.first_consumer)
            .unwrap();
        assert_eq!(
            cancellation.obligation(),
            PhysicalEffectObligation::SettlementContinues
        );
        gates.second_read.release();
        let second = observed.recv_timeout(Duration::from_secs(3)).unwrap();
        if second.0 != 2 {
            gates.first_read.release();
            panic!("the released disjoint read must complete first");
        }
        gates.first_read.release();
        let first = observed.recv_timeout(Duration::from_secs(3)).unwrap();
        assert_eq!(first.0, 1);
        (first.1.unwrap(), second.1.unwrap(), commands.first_identity)
    })
}

fn prepare_reordered_reads(
    serving: &ServingPhysicalRuntime,
    first: ReadyPhysicalWork,
    second: ReadyPhysicalWork,
) -> ReorderedReadCommands {
    let first = super::workflows::admit_read(serving, first);
    let second = super::workflows::admit_read(serving, second);
    assert_disjoint(&first, &second);
    ReorderedReadCommands {
        first_consumer: first.consumer_handle(),
        first_identity: first.intent().identity(),
        first: PhysicalExecutorCommand::read(first).unwrap(),
        second: PhysicalExecutorCommand::read(second).unwrap(),
    }
}

fn assert_disjoint(first: &ResourceAdmittedPhysicalWork, second: &ResourceAdmittedPhysicalWork) {
    assert_eq!(
        first
            .concurrency_scope()
            .relation(&second.concurrency_scope()),
        PhysicalWorkConcurrencyRelation::DisjointArtifacts
    );
}

fn require_gate(gate: &MediaPauseGate, peer: &MediaPauseGate, label: &str) {
    if !gate_reached_within(gate) {
        gate.release();
        peer.release();
        panic!("{label} did not reach its independent backend gate");
    }
}

fn gate_reached_within(gate: &MediaPauseGate) -> bool {
    let deadline = Instant::now() + Duration::from_secs(5);
    while gate.reached_context().is_none() && Instant::now() < deadline {
        std::thread::yield_now();
    }
    gate.reached_context().is_some()
}
