use std::time::{Duration, Instant};

use worth_store::physical_runtime::{
    certification::CertificationPhysicalExecutionCheckpoint, PhysicalPrefetchIntent,
    PhysicalPrefetchOutcome, PhysicalReadAheadFrameOutcome, PhysicalReadAheadIntent,
    PhysicalReadAheadOutcome, PhysicalSpeculativeWorkKind, PhysicalStoreCloseObservation,
    PhysicalStoreClosePhase, PhysicalWritebackExecution,
};
use worth_store_physical_backend::ArtifactRangeWriteDurabilityRequirement;

use super::fixture::{coordinate, initialize_store, open_store, open_store_with_writebehind};

#[test]
fn close_joins_paused_speculation_and_reports_no_residue() {
    let parent = tempfile::tempdir().unwrap();
    let root = parent.path().join("speculative-close");
    initialize_store(&root);
    let serving = open_store(&root, 2, 2);
    let residency = serving.certification_physical_residency();
    residency.drain_unpinned_clean_frames();
    let gate = serving.certification_pause_physical_execution_at(
        CertificationPhysicalExecutionCheckpoint::AfterReadBeforeSchedulerSettlement,
    );
    let owner_residency = residency.clone();
    let owner = std::thread::spawn(move || {
        owner_residency.prefetch(PhysicalPrefetchIntent::new(coordinate(0)))
    });
    assert!(gate.await_arrival());

    let close_plan = serving.close_plan();
    let close_progress = close_plan.observation();
    let close = std::thread::spawn(move || close_plan.execute());
    await_close_blocked_on_possible_effect(&close_progress);
    gate.release();
    let owner_outcome = owner.join().unwrap();
    assert!(
        matches!(owner_outcome, PhysicalPrefetchOutcome::Loaded { .. }),
        "paused speculative owner did not settle after dispatch: {owner_outcome:?}"
    );
    let closed = close.join().unwrap().into_shutdown();

    assert!(!closed.residency().requires_inspection());
    let counters = closed.residency().counters();
    assert_eq!(
        counters.active_speculative_frames(PhysicalSpeculativeWorkKind::Prefetch),
        0
    );
    assert_eq!(counters.active_operation_bytes(), 0);
}

#[test]
fn close_joins_paused_read_ahead_possible_effect_and_reports_no_residue() {
    let parent = tempfile::tempdir().unwrap();
    let root = parent.path().join("read-ahead-close");
    initialize_store(&root);
    let serving = open_store(&root, 2, 2);
    let residency = serving.certification_physical_residency();
    residency.drain_unpinned_clean_frames();
    let gate = serving.certification_pause_physical_execution_at(
        CertificationPhysicalExecutionCheckpoint::AfterReadBeforeSchedulerSettlement,
    );
    let coordinates = [coordinate(0)];
    let owner_residency = residency.clone();
    let owner = std::thread::spawn(move || {
        owner_residency.read_ahead(PhysicalReadAheadIntent::new(&coordinates).unwrap())
    });
    assert!(gate.await_arrival());

    let close_plan = serving.close_plan();
    let close_progress = close_plan.observation();
    let close = std::thread::spawn(move || close_plan.execute());
    await_close_blocked_on_possible_effect(&close_progress);
    gate.release();
    let batch = match owner.join().unwrap() {
        PhysicalReadAheadOutcome::Complete(batch) => batch,
        outcome => panic!("paused read-ahead did not settle after dispatch: {outcome:?}"),
    };
    assert!(matches!(
        batch.frames(),
        [PhysicalReadAheadFrameOutcome::Loaded { .. }]
    ));
    let closed = close.join().unwrap().into_shutdown();

    assert!(!closed.residency().requires_inspection());
    let counters = closed.residency().counters();
    assert_eq!(
        counters.active_speculative_frames(PhysicalSpeculativeWorkKind::ReadAhead),
        0
    );
    assert_eq!(counters.active_operation_bytes(), 0);
}

#[test]
fn close_joins_paused_writebehind_possible_effect_and_reports_no_residue() {
    let parent = tempfile::tempdir().unwrap();
    let root = parent.path().join("writebehind-close");
    initialize_store(&root);
    let serving = open_store_with_writebehind(&root, 2, 2, 1);
    let residency = serving.certification_physical_residency();
    let dirty = residency
        .admit_dirty_frame(residency.pin_exact(coordinate(0)).unwrap(), |_, target| {
            target.fill(0x73);
        })
        .unwrap();
    let admitted = residency
        .admit_writeback(
            residency
                .request_writeback(
                    residency
                        .prepare_writeback(
                            dirty,
                            ArtifactRangeWriteDurabilityRequirement::BufferedWrite,
                        )
                        .unwrap(),
                )
                .unwrap(),
        )
        .unwrap();
    let gate = serving.certification_pause_physical_execution_at(
        CertificationPhysicalExecutionCheckpoint::AfterResidencyWriteBeforeSchedulerSettlement,
    );
    let owner_residency = residency.clone();
    let owner = std::thread::spawn(move || owner_residency.execute_writeback(admitted));
    assert!(gate.await_arrival());

    let close_plan = serving.close_plan();
    let close_progress = close_plan.observation();
    let close = std::thread::spawn(move || close_plan.execute());
    await_close_blocked_on_possible_effect(&close_progress);
    gate.release();
    assert!(matches!(
        owner.join().unwrap().unwrap(),
        PhysicalWritebackExecution::Clean(_)
    ));
    let closed = close.join().unwrap().into_shutdown();

    assert!(!closed.residency().requires_inspection());
    let counters = closed.residency().counters();
    assert_eq!(
        counters.active_speculative_frames(PhysicalSpeculativeWorkKind::WriteBehind),
        0
    );
    assert_eq!(counters.dirty_frames(), 0);
    assert_eq!(counters.active_operation_bytes(), 0);
}

fn await_close_blocked_on_possible_effect(progress: &PhysicalStoreCloseObservation) {
    let deadline = Instant::now() + Duration::from_secs(5);
    while !progress.reached(PhysicalStoreClosePhase::AdmissionStopped) && Instant::now() < deadline
    {
        std::thread::yield_now();
    }
    assert!(
        progress.reached(PhysicalStoreClosePhase::AdmissionStopped),
        "close never entered the shutdown protocol"
    );
    assert!(
        !progress.reached(PhysicalStoreClosePhase::SafeCancellationComplete),
        "close classified safe cancellation before the live possible effect settled"
    );
}
