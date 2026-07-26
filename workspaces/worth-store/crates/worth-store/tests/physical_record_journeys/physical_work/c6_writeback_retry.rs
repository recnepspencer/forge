use worth_proof::TransitionOutcome;
use worth_store::physical_runtime::{
    C6AdmittedDirtyFrame, C6PhysicalResidencyWork, C6PhysicalWorkHandoff,
    C6PhysicalWritebackExecution, PhysicalMutationWorkRequest, PhysicalWorkEffectFate,
    PhysicalWorkReadiness, PhysicalWorkRecoveryDisposition, PhysicalWorkRetryScheduleOutcome,
    PhysicalWorkSubmissionOutcome, PhysicalWorkSubmissionReceipt, ReadyPhysicalWork,
};
use worth_store_contracts::QueueProducerResourceShape;
use worth_store_physical_backend::{MediaFaultDirective, MediaOperationRole};
use worth_store_physical_format::{RecordArtifactFile, RecordFrameCoordinate};

use super::{
    fault_fixture::serving_from_open_with_positioned_write_fault,
    serving_from_initialization_with_work_profile, work_fixture,
};

const EXPECTED_WRITEBACK: [u8; 8] = [0xc6, 0x51, 0xfa, 0x17, 0x90, 0x2d, 0x33, 0x74];

#[test]
fn c6_no_effect_writeback_retains_dirty_ownership_through_signal_retry() {
    let root = tempfile::tempdir().unwrap();
    let (profile, _, mutation) = work_fixture();
    serving_from_initialization_with_work_profile(root.path(), profile.clone()).close();
    let serving = serving_from_open_with_positioned_write_fault(
        root.path(),
        profile,
        MediaFaultDirective::FailBefore {
            kind: std::io::ErrorKind::Other,
            raw_os_error: None,
        },
    );
    let handoff = serving.c6_physical_work_handoff();
    let residency = handoff.residency_work();
    let residency_certification = serving.certification_physical_residency();
    let ready = ready_mutation(&handoff, mutation);
    let consumer = ready.consumer_handle();
    let identity = ready.intent().identity();
    let coordinate =
        RecordFrameCoordinate::new(RecordArtifactFile::BootstrapCatalog, 8, 8).unwrap();
    let before_bytes = artifact_range(root.path(), coordinate);
    let dirty = dirty_frame(&residency, &residency_certification, &ready, coordinate);
    let admitted = initial_writeback(&residency, ready, dirty);
    let before = serving.media_counters();

    let retryable = match residency.execute_writeback(admitted).unwrap() {
        C6PhysicalWritebackExecution::Retryable(retryable) => retryable,
        C6PhysicalWritebackExecution::Settled(_) => {
            panic!("faulted C.6 writeback must retain a safe retry")
        }
    };

    assert_eq!(retryable.identity(), identity);
    assert_eq!(
        retryable.settled().evidence().fate(),
        PhysicalWorkEffectFate::ProvenNoEffect
    );
    assert_eq!(
        retryable.settled().recovery_disposition(),
        PhysicalWorkRecoveryDisposition::RetryExact
    );
    assert_eq!(artifact_range(root.path(), coordinate), before_bytes);
    assert_eq!(residency.counters().dirty_frames(), 1);
    assert_eq!(
        completed_writes(&serving)
            - before.completed_operations_for(MediaOperationRole::PositionedWrite),
        0
    );
    assert_eq!(
        denied_writes(&serving)
            - before.denied_before_effect_for(MediaOperationRole::PositionedWrite),
        1
    );

    handoff
        .advance_signal_clock(consumer, clock_advance(1_000))
        .unwrap();
    handoff.timeout_work(consumer).unwrap();
    let retry = match handoff.schedule_work_retry(retryable.settled()).unwrap() {
        PhysicalWorkRetryScheduleOutcome::Scheduled(retry) => retry,
        PhysicalWorkRetryScheduleOutcome::Denied(report) => {
            panic!("C.6 no-effect retry should schedule: {report:?}")
        }
    };
    handoff
        .advance_signal_clock(consumer, clock_advance(1_001))
        .unwrap();
    let (settled, dirty) = retryable.into_parts();
    let retry = handoff.admit_work_retry(&retry, settled).unwrap();
    let retry_consumer = retry
        .consumer_handle()
        .expect("Signal admitted the C.6 retry generation");
    assert_ne!(retry_consumer.signal_request(), consumer.signal_request());
    let (ready, retry_command, _) = retry.into_parts();
    assert_eq!(ready.intent().identity(), identity);
    let reservation = residency.reserve_writeback(&ready, &dirty).unwrap();
    let prepared = residency
        .prepare_writeback(ready, reservation, 2, writeback_shape())
        .unwrap();
    let admitted = residency
        .admit_writeback_retry(prepared, dirty, retry_command)
        .unwrap();
    let settlement = residency
        .execute_writeback(admitted)
        .unwrap()
        .settled()
        .expect("unfaulted C.6 retry must settle");

    assert_eq!(settlement.identity(), identity);
    assert_eq!(
        settlement.effect_fate(),
        PhysicalWorkEffectFate::WriteCompleted
    );
    assert_eq!(
        completed_writes(&serving),
        before.completed_operations_for(MediaOperationRole::PositionedWrite) + 1
    );
    assert_eq!(
        denied_writes(&serving),
        before.denied_before_effect_for(MediaOperationRole::PositionedWrite) + 1
    );
    assert_eq!(artifact_range(root.path(), coordinate), EXPECTED_WRITEBACK);
    assert_eq!(residency.counters().dirty_frames(), 0);
    assert!(!serving.close().residency().requires_inspection());
}

fn ready_mutation(
    handoff: &C6PhysicalWorkHandoff,
    request: PhysicalMutationWorkRequest,
) -> ReadyPhysicalWork {
    let receipt = successful_receipt(handoff.mutation_submission().submit(request));
    let admitted = handoff.admit_submitted_work(receipt).unwrap();
    match handoff.request_work(admitted).unwrap() {
        PhysicalWorkReadiness::Ready(ready) => ready,
        PhysicalWorkReadiness::Blocked(blocked) => {
            panic!(
                "C.6 retry work unexpectedly blocked: {:?}",
                blocked.condition()
            )
        }
    }
}

fn successful_receipt(outcome: PhysicalWorkSubmissionOutcome) -> PhysicalWorkSubmissionReceipt {
    match outcome.into_raw() {
        TransitionOutcome::Success(receipt) => receipt,
        other => panic!("C.6 retry submission should succeed: {other:?}"),
    }
}

fn dirty_frame(
    residency: &C6PhysicalResidencyWork,
    residency_certification: &worth_store::physical_runtime::PhysicalResidencyCertification,
    ready: &ReadyPhysicalWork,
    coordinate: RecordFrameCoordinate,
) -> C6AdmittedDirtyFrame {
    let lease = residency_certification.pin_exact(coordinate).unwrap();
    residency
        .admit_dirty_frame(ready, lease, |_, target| {
            target.copy_from_slice(&EXPECTED_WRITEBACK);
        })
        .unwrap()
}

fn initial_writeback(
    residency: &C6PhysicalResidencyWork,
    ready: ReadyPhysicalWork,
    dirty: C6AdmittedDirtyFrame,
) -> worth_store::physical_runtime::C6AdmittedPhysicalWriteback {
    let reservation = residency.reserve_writeback(&ready, &dirty).unwrap();
    let prepared = residency
        .prepare_writeback(ready, reservation, 1, writeback_shape())
        .unwrap();
    residency.admit_writeback(prepared, dirty).unwrap()
}

fn artifact_range(root: &std::path::Path, coordinate: RecordFrameCoordinate) -> Vec<u8> {
    let bytes = std::fs::read(root.join("families/records/bootstrap.catalog")).unwrap();
    let start = coordinate.offset() as usize;
    bytes[start..start + coordinate.length() as usize].to_vec()
}

fn writeback_shape() -> QueueProducerResourceShape {
    QueueProducerResourceShape::new()
        .with_queue_slots(1)
        .with_bandwidth_tokens(8)
        .with_write_back_windows(1)
        .with_worker_permits(1)
}

fn clock_advance(tick: u64) -> worth_signal::facade::ClockAdvanceRequest {
    worth_signal::facade::ClockAdvanceRequest::new(
        worth_signal::facade::ClockDomain::MonotonicExecution,
        worth_signal::facade::ClockTick::new(tick),
    )
}

fn completed_writes(serving: &worth_store::physical_runtime::ServingPhysicalRuntime) -> u64 {
    serving
        .media_counters()
        .completed_operations_for(MediaOperationRole::PositionedWrite)
}

fn denied_writes(serving: &worth_store::physical_runtime::ServingPhysicalRuntime) -> u64 {
    serving
        .media_counters()
        .denied_before_effect_for(MediaOperationRole::PositionedWrite)
}
