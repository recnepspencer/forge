use worth_store::physical_runtime::{
    PhysicalFrameFaultCause, PhysicalFrameReadFailure, PhysicalFrameWorkFailure,
    PhysicalReadAheadFrameOutcome, PhysicalReadAheadIntent, PhysicalReadAheadOutcome,
    PhysicalSpeculativeReadFailure, PhysicalSpeculativeWorkKind,
};
use worth_store_physical_backend::{MediaFaultDirective, MediaOperationRole};

use super::fixture::{
    coordinate, initialize_store, open_store, positioned_reads, residency_policy,
};

#[test]
fn second_read_ahead_fault_reports_partial_and_releases_every_grant() {
    let parent = tempfile::tempdir().unwrap();
    let root = parent.path().join("speculative-partial");
    initialize_store(&root);
    let calibration = open_store(&root, 4, 4);
    let bootstrap_reads = positioned_reads(&calibration);
    assert!(!calibration.close().residency().requires_inspection());
    let (format, _, _) = super::super::configuration();
    let serving =
        super::super::fault_fixture::serving_from_open_with_positioned_read_fault_and_policy(
            &root,
            bootstrap_reads + 2,
            MediaFaultDirective::FailBefore {
                kind: std::io::ErrorKind::Other,
                raw_os_error: None,
            },
            residency_policy(format, 4, 4),
        );
    let residency = serving.certification_physical_residency();
    residency.drain_unpinned_clean_frames();
    let coordinates = [coordinate(0), coordinate(1)];
    let media_before = positioned_reads(&serving);
    let counters_before = residency.counters();

    let batch = match residency.read_ahead(PhysicalReadAheadIntent::new(&coordinates).unwrap()) {
        PhysicalReadAheadOutcome::Partial(batch) => batch,
        outcome => panic!("second-frame backend failure was not partial: {outcome:?}"),
    };
    assert_eq!((batch.loaded(), batch.failed()), (1, 1));
    assert!(matches!(
        batch.frames()[0],
        PhysicalReadAheadFrameOutcome::Loaded { coordinate, .. }
            if coordinate == coordinates[0]
    ));
    assert!(matches!(
        batch.frames()[1].failure(),
        Some(PhysicalSpeculativeReadFailure::Frame(
            PhysicalFrameReadFailure::FaultTerminated {
                cause: PhysicalFrameFaultCause::PhysicalWork(PhysicalFrameWorkFailure::Backend(_)),
                ..
            }
        ))
    ));
    assert_eq!(positioned_reads(&serving), media_before + 2);
    let after = residency.counters();
    assert_eq!(
        after.speculative_attempts(PhysicalSpeculativeWorkKind::ReadAhead),
        counters_before.speculative_attempts(PhysicalSpeculativeWorkKind::ReadAhead) + 1
    );
    assert_eq!(
        after.speculative_admissions(PhysicalSpeculativeWorkKind::ReadAhead),
        counters_before.speculative_admissions(PhysicalSpeculativeWorkKind::ReadAhead) + 1
    );
    assert_eq!(
        after.speculative_completions(PhysicalSpeculativeWorkKind::ReadAhead),
        counters_before.speculative_completions(PhysicalSpeculativeWorkKind::ReadAhead) + 1
    );
    assert_eq!(
        after.active_speculative_frames(PhysicalSpeculativeWorkKind::ReadAhead),
        0
    );
    assert_eq!(after.active_operation_bytes(), 0);
    assert_eq!(
        serving
            .media_counters()
            .attempts_for(MediaOperationRole::PositionedRead),
        media_before + 2
    );
    assert!(!serving.close().residency().requires_inspection());
}
