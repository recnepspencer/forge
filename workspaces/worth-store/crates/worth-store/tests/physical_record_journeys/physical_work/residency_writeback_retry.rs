use worth_store::physical_runtime::{
    AdmittedDirtyFrame, PhysicalResidencyCertification, PhysicalWorkEffectFate,
    PhysicalWorkRecoveryDisposition, PhysicalWorkRetryScheduleOutcome, PhysicalWritebackExecution,
};
use worth_store_physical_backend::{
    ArtifactRangeWriteDurabilityRequirement, MediaFaultDirective, MediaOperationRole,
};
use worth_store_physical_format::{RecordArtifactFile, RecordFrameCoordinate};

use super::{
    fault_fixture::serving_from_open_with_positioned_write_fault,
    serving_from_initialization_with_work_profile, work_fixture,
};

const EXPECTED_WRITEBACK: [u8; 8] = [0xc6, 0x51, 0xfa, 0x17, 0x90, 0x2d, 0x33, 0x74];

#[test]
fn no_effect_writeback_retains_dirty_ownership_through_signal_retry() {
    let root = tempfile::tempdir().unwrap();
    let (profile, _, _) = work_fixture();
    serving_from_initialization_with_work_profile(root.path(), profile.clone()).close();
    let serving = serving_from_open_with_positioned_write_fault(
        root.path(),
        profile,
        MediaFaultDirective::FailBefore {
            kind: std::io::ErrorKind::Other,
            raw_os_error: None,
        },
    );
    let residency = serving.certification_physical_residency();
    let coordinate =
        RecordFrameCoordinate::new(RecordArtifactFile::BootstrapCatalog, 8, 8).unwrap();
    let before_bytes = artifact_range(root.path(), coordinate);
    let writeback_before = serving.residency_observation().writebacks();
    let dirty = dirty_frame(&residency, coordinate);
    let ready = residency
        .request_writeback(
            residency
                .prepare_writeback(
                    dirty,
                    ArtifactRangeWriteDurabilityRequirement::BufferedWrite,
                )
                .unwrap(),
        )
        .unwrap();
    let consumer = ready.consumer_handle();
    let identity = ready.identity();
    let admitted = residency.admit_writeback(ready).unwrap();
    let before = serving.media_counters();
    let writeback_after_admission = serving.residency_observation().writebacks();
    assert_eq!(
        writeback_after_admission.attempts(),
        writeback_before.attempts() + 1
    );

    let retryable = match residency.execute_writeback(admitted).unwrap() {
        PhysicalWritebackExecution::Retryable(retryable) => retryable,
        PhysicalWritebackExecution::Clean(_) => {
            panic!("faulted residency writeback must retain a safe retry")
        }
        PhysicalWritebackExecution::InspectionRequired(_) => {
            panic!("a proven no-effect writeback must not require inspection")
        }
    };

    assert_eq!(retryable.settled().intent().identity(), identity);
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
    let retry_observation = serving.residency_observation().writebacks();
    assert_eq!(
        retry_observation.attempts(),
        writeback_after_admission.attempts()
    );
    assert_eq!(
        retry_observation.retryable(),
        writeback_before.retryable() + 1
    );
    assert_eq!(
        retry_observation.exact_receipts(),
        writeback_before.exact_receipts()
    );
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

    serving
        .advance_physical_signal_clock(consumer, clock_advance(1_000))
        .unwrap();
    serving.timeout_physical_work(consumer).unwrap();
    let retry = match serving
        .schedule_physical_work_retry(retryable.settled())
        .unwrap()
    {
        PhysicalWorkRetryScheduleOutcome::Scheduled(retry) => retry,
        PhysicalWorkRetryScheduleOutcome::Denied(report) => {
            panic!("no-effect residency retry should schedule: {report:?}")
        }
    };
    serving
        .advance_physical_signal_clock(consumer, clock_advance(1_001))
        .unwrap();
    let (settled, dirty) = retryable.into_parts();
    let retry = serving.admit_physical_work_retry(&retry, settled).unwrap();
    let retry_consumer = retry
        .consumer_handle()
        .expect("Signal admitted the residency retry generation");
    assert_ne!(retry_consumer.signal_request(), consumer.signal_request());
    let (ready, retry_command, _) = retry.into_parts();
    assert_eq!(ready.intent().identity(), identity);
    let ready = residency.bind_writeback_retry(ready, dirty).unwrap();
    assert_eq!(
        serving.residency_observation().writebacks().attempts(),
        writeback_before.attempts() + 2
    );
    let admitted = residency
        .admit_writeback_retry(ready, retry_command)
        .unwrap_or_else(|failure| {
            panic!(
                "C5_PREDICATE:local-physical-work-scheduler: canonical retry identity was rejected by local scheduler state: {:?}",
                failure.cause()
            )
        });
    let settlement = match residency.execute_writeback(admitted).unwrap() {
        PhysicalWritebackExecution::Clean(settlement) => settlement,
        PhysicalWritebackExecution::Retryable(_) => {
            panic!("unfaulted residency retry must settle clean")
        }
        PhysicalWritebackExecution::InspectionRequired(_) => {
            panic!("unfaulted residency retry must not require inspection")
        }
    };

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
    let final_observation = serving.residency_observation().writebacks();
    assert_eq!(
        final_observation.attempts(),
        writeback_before.attempts() + 2
    );
    assert_eq!(
        final_observation.exact_receipts(),
        writeback_before.exact_receipts() + 1
    );
    assert_eq!(
        final_observation.retryable(),
        writeback_before.retryable() + 1
    );
    assert_eq!(
        final_observation.indeterminate(),
        writeback_before.indeterminate()
    );
    assert_eq!(
        final_observation.inspection_required(),
        writeback_before.inspection_required()
    );
    assert!(!serving.close().residency().requires_inspection());
}

fn dirty_frame(
    residency: &PhysicalResidencyCertification,
    coordinate: RecordFrameCoordinate,
) -> AdmittedDirtyFrame {
    let lease = residency.pin_exact(coordinate).unwrap();
    residency
        .admit_dirty_frame(lease, |_, target| {
            target.copy_from_slice(&EXPECTED_WRITEBACK);
        })
        .unwrap()
}

fn artifact_range(root: &std::path::Path, coordinate: RecordFrameCoordinate) -> Vec<u8> {
    let bytes = std::fs::read(root.join("families/records/bootstrap.catalog")).unwrap();
    let start = coordinate.offset() as usize;
    bytes[start..start + coordinate.length() as usize].to_vec()
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
