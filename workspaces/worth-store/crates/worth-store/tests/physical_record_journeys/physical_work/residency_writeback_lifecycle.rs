use worth_store::physical_runtime::{
    AdmittedDirtyFrame, PhysicalEffectObligation, PhysicalResidencyCertification,
    PhysicalWorkEffectFate, PhysicalWorkPreEffectDenial, PhysicalWorkRecoveryDisposition,
    PhysicalWritebackExecution, PhysicalWritebackFailureCause,
};
use worth_store_physical_backend::{
    ArtifactRangeWriteDurabilityRequirement, MediaFaultDirective, MediaOperationRole,
};
use worth_store_physical_format::{RecordArtifactFile, RecordFrameCoordinate};

use super::{
    fault_fixture::serving_from_open_with_positioned_write_fault,
    fixture::serving_from_open_with_work_profile, serving_from_initialization_with_work_profile,
    work_fixture,
};

const WRITEBACK: [u8; 8] = [0x51, 0xc6, 0x7a, 0x19, 0x84, 0x2f, 0xd0, 0x33];

#[test]
fn predispatch_timeout_returns_dirty_authority_without_a_media_attempt() {
    let root = tempfile::tempdir().unwrap();
    let (profile, _, _) = work_fixture();
    serving_from_initialization_with_work_profile(root.path(), profile.clone()).close();
    let serving = serving_from_open_with_work_profile(root.path(), profile);
    let residency = serving.certification_physical_residency();
    let coordinate = writeback_coordinate();
    let ready = residency
        .request_writeback(
            residency
                .prepare_writeback(
                    dirty_frame(&residency, coordinate),
                    ArtifactRangeWriteDurabilityRequirement::BufferedWrite,
                )
                .unwrap(),
        )
        .unwrap();
    let consumer = ready.consumer_handle();
    let media_before = serving.media_counters();
    let scheduler_before = serving.physical_scheduler_capacity();
    let writeback_before = serving.residency_observation().writebacks();
    serving
        .advance_physical_signal_clock(consumer, clock_advance(1_000))
        .unwrap();

    let timeout = serving.timeout_physical_work(consumer).unwrap();

    assert_eq!(
        timeout.obligation(),
        PhysicalEffectObligation::NotDispatched
    );
    let failure = match residency.admit_writeback(ready) {
        Err(failure) => failure,
        Ok(_) => panic!("a timed-out writeback must not enter scheduler admission"),
    };
    assert!(matches!(
        failure.cause(),
        PhysicalWritebackFailureCause::PreEffect(PhysicalWorkPreEffectDenial::ConsumerCancelled)
    ));
    let dirty = failure.into_dirty();
    assert_eq!(dirty.coordinate(), coordinate);
    assert_eq!(residency.counters().dirty_frames(), 1);
    assert_eq!(serving.media_counters(), media_before);
    assert_eq!(serving.physical_scheduler_capacity(), scheduler_before);
    assert_eq!(
        serving.residency_observation().writebacks(),
        writeback_before
    );
    dirty.discard().unwrap();
    assert!(!serving.close().residency().requires_inspection());
}

#[test]
fn partial_writeback_retains_dirty_truth_and_exact_indeterminate_observation() {
    let root = tempfile::tempdir().unwrap();
    let (profile, _, _) = work_fixture();
    serving_from_initialization_with_work_profile(root.path(), profile.clone()).close();
    let coordinate = writeback_coordinate();
    let before_bytes = artifact_range(root.path(), coordinate);
    let serving = serving_from_open_with_positioned_write_fault(
        root.path(),
        profile,
        MediaFaultDirective::AllowPrefix { bytes: 3 },
    );
    let residency = serving.certification_physical_residency();
    let writeback_before = serving.residency_observation().writebacks();
    let admitted = residency
        .admit_writeback(
            residency
                .request_writeback(
                    residency
                        .prepare_writeback(
                            dirty_frame(&residency, coordinate),
                            ArtifactRangeWriteDurabilityRequirement::BufferedWrite,
                        )
                        .unwrap(),
                )
                .unwrap(),
        )
        .unwrap();
    let media_before = serving.media_counters();
    let writeback_after_admission = serving.residency_observation().writebacks();
    assert_eq!(
        writeback_after_admission.attempts(),
        writeback_before.attempts() + 1
    );

    let inspection = match residency.execute_writeback(admitted).unwrap() {
        PhysicalWritebackExecution::InspectionRequired(inspection) => inspection,
        PhysicalWritebackExecution::Clean(_) => {
            panic!("a partial physical writeback cannot publish clean")
        }
        PhysicalWritebackExecution::Retryable(_) => {
            panic!("a partial physical writeback is not safe to retry")
        }
    };

    let settlement = inspection.settlement();
    assert_eq!(
        settlement.effect_fate(),
        PhysicalWorkEffectFate::Indeterminate
    );
    assert_eq!(
        settlement.recovery(),
        PhysicalWorkRecoveryDisposition::InspectionRequired
    );
    assert_eq!(residency.counters().dirty_frames(), 1);
    let observed = artifact_range(root.path(), coordinate);
    assert_eq!(&observed[..3], &WRITEBACK[..3]);
    assert_eq!(&observed[3..], &before_bytes[3..]);
    assert_eq!(
        serving
            .media_counters()
            .attempts_for(MediaOperationRole::PositionedWrite),
        media_before.attempts_for(MediaOperationRole::PositionedWrite) + 1
    );
    let writeback_after = serving.residency_observation().writebacks();
    assert_eq!(
        writeback_after.attempts(),
        writeback_after_admission.attempts()
    );
    assert_eq!(
        writeback_after.indeterminate(),
        writeback_before.indeterminate() + 1
    );
    assert_eq!(
        writeback_after.inspection_required(),
        writeback_before.inspection_required() + 1
    );
    assert_eq!(
        writeback_after.exact_receipts(),
        writeback_before.exact_receipts()
    );
    assert_eq!(writeback_after.retryable(), writeback_before.retryable());
    assert!(serving.close().residency().requires_inspection());
}

fn dirty_frame(
    residency: &PhysicalResidencyCertification,
    coordinate: RecordFrameCoordinate,
) -> AdmittedDirtyFrame {
    let lease = residency.pin_exact(coordinate).unwrap();
    residency
        .admit_dirty_frame(lease, |_, target| target.copy_from_slice(&WRITEBACK))
        .unwrap()
}

fn writeback_coordinate() -> RecordFrameCoordinate {
    RecordFrameCoordinate::new(RecordArtifactFile::BootstrapCatalog, 8, 8).unwrap()
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
