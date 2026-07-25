use worth_foundational::FoundationalPerformanceWorkClass;
use worth_proof::TransitionOutcome;
use worth_store::physical_runtime::{
    PhysicalExecutorCommand, PhysicalMutationWorkRequest, PhysicalReadWorkRequest,
    PhysicalSchedulerDemand, PhysicalSchedulerDenial, PhysicalSignalSettlementOutcome,
    PhysicalWorkEffectFate, PhysicalWorkReadiness, PhysicalWorkRecoveryDisposition,
    PhysicalWorkRetryScheduleOutcome, ReadyPhysicalWork, ResourceAdmittedPhysicalWork,
    ServingPhysicalRuntime,
};
use worth_store_contracts::QueueProducerResourceShape;
use worth_store_io_scheduler::QueueExecutionAdmissionDenial;
use worth_store_physical_format::{RecordArtifactFile, RecordFrameCoordinate};

pub(super) fn ready_read(
    serving: &ServingPhysicalRuntime,
    request: PhysicalReadWorkRequest,
) -> ReadyPhysicalWork {
    let receipt = match serving
        .physical_read_submission()
        .submit(request)
        .into_raw()
    {
        TransitionOutcome::Success(receipt) => receipt,
        outcome => panic!("Phase 16 read submission should succeed: {outcome:?}"),
    };
    let admitted = serving.admit_physical_work(receipt).unwrap();
    expect_ready(serving.request_physical_work(admitted).unwrap())
}

pub(super) fn ready_write(
    serving: &ServingPhysicalRuntime,
    request: PhysicalMutationWorkRequest,
) -> ReadyPhysicalWork {
    let receipt = match serving
        .physical_mutation_submission()
        .submit(request)
        .into_raw()
    {
        TransitionOutcome::Success(receipt) => receipt,
        outcome => panic!("Phase 16 write submission should succeed: {outcome:?}"),
    };
    let admitted = serving.admit_physical_work(receipt).unwrap();
    expect_ready(serving.request_physical_work(admitted).unwrap())
}

pub(super) fn admit_read(
    serving: &ServingPhysicalRuntime,
    ready: ReadyPhysicalWork,
) -> ResourceAdmittedPhysicalWork {
    let demand = PhysicalSchedulerDemand::foreground(
        ready,
        super::super::reserved_buffered_file_read(serving),
        None,
    )
    .unwrap();
    let work = demand.queue_work();
    let backend = serving
        .admit_physical_scheduler_capability(work.backend_requirement())
        .unwrap();
    serving
        .admit_physical_scheduler_demand(
            demand,
            &backend,
            super::super::scheduler::policy_receipt_for(
                work.requested_budget(),
                0,
                FoundationalPerformanceWorkClass::AuthoritativeRead,
            ),
        )
        .unwrap()
}

pub(super) fn deny_scheduler_breadth(
    serving: &ServingPhysicalRuntime,
    request: PhysicalReadWorkRequest,
) -> worth_store::physical_runtime::PhysicalWorkIdentity {
    let ready = ready_read(serving, request);
    let identity = ready.intent().identity();
    let demand = PhysicalSchedulerDemand::foreground(
        ready,
        super::super::reserved_buffered_file_read(serving),
        None,
    )
    .unwrap();
    let work = demand.queue_work();
    let backend = serving
        .admit_physical_scheduler_capability(work.backend_requirement())
        .unwrap();
    assert!(matches!(
        serving.admit_physical_scheduler_demand(
            demand,
            &backend,
            super::super::scheduler::exhausted_policy_receipt(
                work.requested_budget(),
                FoundationalPerformanceWorkClass::AuthoritativeRead,
            ),
        ),
        Err(PhysicalSchedulerDenial::Queue(
            QueueExecutionAdmissionDenial::PolicyReceiptBudgetMismatch { .. }
        ))
    ));
    identity
}

pub(super) fn retry_write_after_clock_wake(
    serving: &ServingPhysicalRuntime,
    ready: ReadyPhysicalWork,
    payload: &[u8],
) -> worth_store::physical_runtime::PhysicalWorkIdentity {
    let admitted = admit_write(serving, ready);
    let consumer = admitted.consumer_handle();
    let identity = admitted.intent().identity();
    let command = PhysicalExecutorCommand::exact_write(admitted, payload).unwrap();
    let outcome = serving.execute_physical_work(command).unwrap();
    assert_eq!(
        outcome.settled().evidence().fate(),
        PhysicalWorkEffectFate::ProvenNoEffect
    );
    serving
        .advance_physical_signal_clock(
            consumer,
            worth_signal::facade::ClockAdvanceRequest::new(
                worth_signal::facade::ClockDomain::MonotonicExecution,
                worth_signal::facade::ClockTick::new(1_000),
            ),
        )
        .unwrap();
    serving.timeout_physical_work(consumer).unwrap();
    let settled = outcome.into_settled();
    let retry = match serving.schedule_physical_work_retry(&settled).unwrap() {
        PhysicalWorkRetryScheduleOutcome::Scheduled(retry) => retry,
        PhysicalWorkRetryScheduleOutcome::Denied(report) => {
            panic!("Phase 16 pre-effect retry should schedule: {report:?}")
        }
    };
    serving
        .advance_physical_signal_clock(
            consumer,
            worth_signal::facade::ClockAdvanceRequest::new(
                worth_signal::facade::ClockDomain::MonotonicExecution,
                worth_signal::facade::ClockTick::new(1_001),
            ),
        )
        .unwrap();
    let retry = serving.admit_physical_work_retry(&retry, settled).unwrap();
    let (ready, command, _) = retry.into_parts();
    let admitted = admit_write(serving, ready);
    let outcome = serving
        .execute_physical_work(command.bind(admitted).unwrap())
        .unwrap();
    assert_eq!(
        outcome.settled().evidence().fate(),
        PhysicalWorkEffectFate::WriteCompleted
    );
    identity
}

pub(super) fn execute_exact_writeback(
    serving: &ServingPhysicalRuntime,
    request: PhysicalMutationWorkRequest,
    bytes: Vec<u8>,
) -> worth_store::physical_runtime::PhysicalWorkIdentity {
    let coordinate =
        RecordFrameCoordinate::new(RecordArtifactFile::BootstrapCatalog, 8, 8).unwrap();
    let handoff = serving.c6_physical_work_handoff();
    let receipt = match handoff.mutation_submission().submit(request).into_raw() {
        TransitionOutcome::Success(receipt) => receipt,
        outcome => panic!("Phase 16 writeback submission should succeed: {outcome:?}"),
    };
    let admitted = handoff.admit_submitted_work(receipt).unwrap();
    let ready = expect_ready(handoff.request_work(admitted).unwrap());
    let residency = handoff.residency_work();
    let lease = residency.pin_exact(coordinate).unwrap();
    let dirty = residency.admit_dirty_frame(&ready, lease, bytes).unwrap();
    let reservation = residency.reserve_writeback(&ready, &dirty).unwrap();
    let prepared = residency
        .prepare_writeback(ready, reservation, 7, writeback_shape())
        .unwrap();
    let identity = prepared.identity();
    let admitted = residency.admit_writeback(prepared, dirty).unwrap();
    let outcome = residency
        .execute_writeback(admitted)
        .unwrap()
        .settled()
        .expect("unfaulted Phase 16 writeback must settle");
    assert_eq!(
        outcome.effect_fate(),
        PhysicalWorkEffectFate::WriteCompleted
    );
    assert_eq!(
        outcome.recovery(),
        PhysicalWorkRecoveryDisposition::ContinueSettlement
    );
    assert_eq!(outcome.signal(), PhysicalSignalSettlementOutcome::Committed);
    assert_eq!(residency.counters().dirty_frames(), 0);
    identity
}

fn admit_write(
    serving: &ServingPhysicalRuntime,
    ready: ReadyPhysicalWork,
) -> ResourceAdmittedPhysicalWork {
    let demand = super::super::scheduler::write_demand(serving, ready);
    let work = demand.queue_work();
    let backend = serving
        .admit_physical_scheduler_capability(work.backend_requirement())
        .unwrap();
    serving
        .admit_physical_scheduler_demand(
            demand,
            &backend,
            super::super::policy_receipt(work.requested_budget()),
        )
        .unwrap()
}

pub(super) fn expect_ready(readiness: PhysicalWorkReadiness) -> ReadyPhysicalWork {
    match readiness {
        PhysicalWorkReadiness::Ready(ready) => ready,
        PhysicalWorkReadiness::Blocked(blocked) => {
            panic!(
                "Phase 16 work unexpectedly blocked: {:?}",
                blocked.condition()
            )
        }
    }
}

fn writeback_shape() -> QueueProducerResourceShape {
    QueueProducerResourceShape::new()
        .with_queue_slots(1)
        .with_bandwidth_tokens(8)
        .with_write_back_windows(1)
        .with_worker_permits(1)
}
