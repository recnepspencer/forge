use worth_store::physical_runtime::{
    C6PhysicalFrameReadFailure, PhysicalRecordInitialization, PhysicalRecordOpen,
    PhysicalRecordResidencyPolicy,
};
use worth_store_buffer_pool::PhysicalResidencyDenial;
use worth_store_physical_backend::MediaOperationRole;
use worth_store_physical_format::{RecordArtifactFile, RecordFrameCoordinate};

use super::{configuration, media, success};

const FRAME_BYTES: u32 = 8;

#[test]
fn c6_pins_distinguish_faults_hits_overpin_and_refault_without_another_runtime() {
    let parent = tempfile::tempdir().unwrap();
    let root = parent.path().join("c6-pin-inheritance");
    let (format, placement, access) = configuration();
    let seeded = success(
        media(&root)
            .initialize_record_store(PhysicalRecordInitialization::new(format, placement, access)),
    );
    assert!(!seeded.close().residency().requires_inspection());
    let policy = PhysicalRecordResidencyPolicy::new_with_metadata_budget(
        64 * 1024,
        16 * 1024,
        8,
        2,
        4 * 1024 * 1024,
        8,
    )
    .unwrap()
    .with_pin_lease_limit(2)
    .unwrap();
    let serving =
        success(media(&root).open_record_store(
            PhysicalRecordOpen::new(format, access).with_residency_policy(policy),
        ));
    serving.drain_clean_residency();
    let residency = serving.c6_physical_work_handoff().residency_work();

    let first_coordinate = coordinate(0);
    let reads_before = positioned_reads(&serving);
    let first = residency.pin_exact(first_coordinate).unwrap();
    assert_eq!(first.coordinate(), first_coordinate);
    assert_eq!(first.physical_work_count(), 1);
    assert!(first.first_physical_work().is_some());
    assert_eq!(positioned_reads(&serving), reads_before + 1);

    let hot = residency.pin_exact(first_coordinate).unwrap();
    assert_eq!(hot.physical_work_count(), 0);
    assert_eq!(positioned_reads(&serving), reads_before + 1);
    assert_eq!(residency.counters().pinned_frames(), 1);
    assert_eq!(residency.counters().pin_leases(), 2);
    assert!(matches!(
        residency.pin_exact(first_coordinate),
        Err(C6PhysicalFrameReadFailure::Residency(
            PhysicalResidencyDenial::PinLeaseBudgetExceeded,
        ))
    ));

    drop(hot);
    drop(first);
    assert_eq!(residency.counters().pinned_frames(), 0);
    assert_eq!(residency.counters().pin_leases(), 0);

    for ordinal in 1..=8 {
        pin_and_release(&residency, coordinate(ordinal * u64::from(FRAME_BYTES)));
    }
    let after_pressure = residency.counters();
    assert!(after_pressure.evictions() > 0);
    let reads_before_refault = positioned_reads(&serving);
    let refault = residency.pin_exact(first_coordinate).unwrap();
    assert_eq!(refault.physical_work_count(), 1);
    assert_eq!(positioned_reads(&serving), reads_before_refault + 1);
    assert!(residency.counters().peak_resident_bytes() <= policy.resident_bytes());
    drop(refault);

    let shutdown = serving.close();
    assert!(!shutdown.residency().requires_inspection());
}

fn coordinate(offset: u64) -> RecordFrameCoordinate {
    RecordFrameCoordinate::new(RecordArtifactFile::BootstrapCatalog, offset, FRAME_BYTES).unwrap()
}

fn pin_and_release(
    residency: &worth_store::physical_runtime::C6PhysicalResidencyWork,
    coordinate: RecordFrameCoordinate,
) {
    let frame = residency.pin_exact(coordinate).unwrap();
    assert_eq!(frame.physical_work_count(), 1);
    drop(frame);
}

fn positioned_reads(serving: &worth_store::physical_runtime::ServingPhysicalRuntime) -> u64 {
    serving
        .media_counters()
        .attempts_for(MediaOperationRole::PositionedRead)
}
