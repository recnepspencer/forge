use worth_store::physical_runtime::{
    certification::CertificationPhysicalExecutionCheckpoint, PhysicalOperationAllocationScope,
    PhysicalPrefetchIntent, PhysicalPrefetchOutcome, PhysicalReadAheadFrameOutcome,
    PhysicalReadAheadIntent, PhysicalReadAheadOutcome, PhysicalResidencyDimension,
    PhysicalResidencyRetryPosture, PhysicalSpeculativeWorkKind,
};

use super::fixture::{causal_record, coordinate, initialize_store, open_store, positioned_reads};

#[test]
fn overlapping_prefetch_coalesces_without_duplicate_work() {
    let parent = tempfile::tempdir().unwrap();
    let root = parent.path().join("speculative-coalescing");
    initialize_store(&root);
    let serving = open_store(&root, 2, 2);
    let residency = serving.certification_physical_residency();
    residency.drain_unpinned_clean_frames();
    let shared_coordinate = coordinate(0);
    let before = residency.counters();
    let causal_before_owner = serving.physical_work_observer().causal().records().len();
    let media_before_owner = positioned_reads(&serving);
    let gate = serving.certification_pause_physical_execution_at(
        CertificationPhysicalExecutionCheckpoint::BeforeBackendDispatch,
    );
    let owner_residency = residency.clone();
    let owner = std::thread::spawn(move || {
        owner_residency.prefetch(PhysicalPrefetchIntent::new(shared_coordinate))
    });
    assert!(gate.await_arrival());

    let work_before_waiter = serving.physical_work_counters();
    let media_before_waiter = serving.media_counters();
    let waiter_residency = residency.clone();
    let waiter = std::thread::spawn(move || {
        waiter_residency.prefetch(PhysicalPrefetchIntent::new(shared_coordinate))
    });
    await_coalesced_waiter(&residency, before.coalesced_waiters());
    assert_eq!(serving.physical_work_counters(), work_before_waiter);
    assert_eq!(serving.media_counters(), media_before_waiter);
    gate.release();

    let owner_work = match owner.join().unwrap() {
        PhysicalPrefetchOutcome::Loaded { work, .. } => work,
        outcome => panic!("cold prefetch owner did not load canonically: {outcome:?}"),
    };
    assert_eq!(
        waiter.join().unwrap(),
        PhysicalPrefetchOutcome::Coalesced {
            coordinate: shared_coordinate
        }
    );
    let after = residency.counters();
    assert_eq!(after.source_loads(), before.source_loads() + 1);
    assert_eq!(after.coalesced_waiters(), before.coalesced_waiters() + 1);
    assert_eq!(
        after.speculative_attempts(PhysicalSpeculativeWorkKind::Prefetch),
        before.speculative_attempts(PhysicalSpeculativeWorkKind::Prefetch) + 2
    );
    assert_eq!(
        after.speculative_completions(PhysicalSpeculativeWorkKind::Prefetch),
        before.speculative_completions(PhysicalSpeculativeWorkKind::Prefetch) + 2
    );
    assert_eq!(
        after.active_speculative_frames(PhysicalSpeculativeWorkKind::Prefetch),
        0
    );
    assert_eq!(
        serving.physical_work_observer().causal().records().len(),
        causal_before_owner + 1
    );
    assert_eq!(positioned_reads(&serving), media_before_owner + 1);
    assert_eq!(causal_record(&serving, owner_work).identity(), owner_work);
    assert!(!serving.close().residency().requires_inspection());
}

#[test]
fn saturated_prefetch_is_dropped_with_exact_pressure_and_zero_work() {
    let parent = tempfile::tempdir().unwrap();
    let root = parent.path().join("speculative-drop");
    initialize_store(&root);
    let serving = open_store(&root, 1, 2);
    let residency = serving.certification_physical_residency();
    residency.drain_unpinned_clean_frames();
    let held_coordinate = coordinate(0);
    let gate = serving.certification_pause_physical_execution_at(
        CertificationPhysicalExecutionCheckpoint::BeforeBackendDispatch,
    );
    let owner_residency = residency.clone();
    let owner = std::thread::spawn(move || {
        owner_residency.prefetch(PhysicalPrefetchIntent::new(held_coordinate))
    });
    assert!(gate.await_arrival());
    let work_before = serving.physical_work_counters();
    let media_before = serving.media_counters();
    let causal_before = serving.physical_work_observer().causal().records().len();

    let dropped = match residency.prefetch(PhysicalPrefetchIntent::new(coordinate(1))) {
        PhysicalPrefetchOutcome::Dropped(dropped) => dropped,
        outcome => panic!("one-past prefetch was not dropped: {outcome:?}"),
    };
    let pressure = dropped
        .pressure()
        .expect("kind saturation must expose Store pressure evidence");
    assert_eq!(
        pressure.scope(),
        PhysicalOperationAllocationScope::ForegroundRead
    );
    assert_eq!(
        pressure.dimension(),
        PhysicalResidencyDimension::SpeculativeFrames(PhysicalSpeculativeWorkKind::Prefetch)
    );
    assert_eq!(
        (pressure.requested(), pressure.admitted(), pressure.limit()),
        (1, 1, 1)
    );
    assert_eq!(
        pressure.retry_posture(),
        PhysicalResidencyRetryPosture::AfterLeaseRelease
    );
    assert!(!pressure.effect_may_have_started());
    assert_eq!(serving.physical_work_counters(), work_before);
    assert_eq!(serving.media_counters(), media_before);
    assert_eq!(
        serving.physical_work_observer().causal().records().len(),
        causal_before
    );

    gate.release();
    assert!(matches!(
        owner.join().unwrap(),
        PhysicalPrefetchOutcome::Loaded { .. }
    ));
    let after = residency.counters();
    assert_eq!(
        after.speculative_denials(PhysicalSpeculativeWorkKind::Prefetch),
        1
    );
    assert_eq!(
        after.active_speculative_frames(PhysicalSpeculativeWorkKind::Prefetch),
        0
    );
    assert_eq!(after.active_operation_bytes(), 0);
    assert!(!serving.close().residency().requires_inspection());
}

#[test]
fn saturated_read_ahead_is_dropped_with_exact_pressure_and_zero_work() {
    let parent = tempfile::tempdir().unwrap();
    let root = parent.path().join("read-ahead-drop");
    initialize_store(&root);
    let serving = open_store(&root, 2, 1);
    let residency = serving.certification_physical_residency();
    residency.drain_unpinned_clean_frames();
    let before = residency.counters();
    let held_coordinate = coordinate(0);
    let gate = serving.certification_pause_physical_execution_at(
        CertificationPhysicalExecutionCheckpoint::BeforeBackendDispatch,
    );
    let owner_residency = residency.clone();
    let owner = std::thread::spawn(move || {
        let coordinates = [held_coordinate];
        owner_residency.read_ahead(PhysicalReadAheadIntent::new(&coordinates).unwrap())
    });
    assert!(gate.await_arrival());
    let work_before = serving.physical_work_counters();
    let media_before = serving.media_counters();
    let causal_before = serving.physical_work_observer().causal().records().len();

    let denied_coordinates = [coordinate(1)];
    let dropped =
        match residency.read_ahead(PhysicalReadAheadIntent::new(&denied_coordinates).unwrap()) {
            PhysicalReadAheadOutcome::Dropped(dropped) => dropped,
            outcome => panic!("one-past read-ahead was not dropped: {outcome:?}"),
        };
    let pressure = dropped
        .pressure()
        .expect("kind saturation must expose Store pressure evidence");
    assert_eq!(
        pressure.scope(),
        PhysicalOperationAllocationScope::ForegroundRead
    );
    assert_eq!(
        pressure.dimension(),
        PhysicalResidencyDimension::SpeculativeFrames(PhysicalSpeculativeWorkKind::ReadAhead)
    );
    assert_eq!(
        (pressure.requested(), pressure.admitted(), pressure.limit()),
        (1, 1, 1)
    );
    assert_eq!(
        pressure.retry_posture(),
        PhysicalResidencyRetryPosture::AfterLeaseRelease
    );
    assert!(!pressure.effect_may_have_started());
    assert_eq!(serving.physical_work_counters(), work_before);
    assert_eq!(serving.media_counters(), media_before);
    assert_eq!(
        serving.physical_work_observer().causal().records().len(),
        causal_before
    );

    gate.release();
    let owner_batch = match owner.join().unwrap() {
        PhysicalReadAheadOutcome::Complete(batch) => batch,
        outcome => panic!("held read-ahead owner did not settle canonically: {outcome:?}"),
    };
    assert_eq!(owner_batch.hits(), 0);
    assert_eq!(owner_batch.coalesced(), 0);
    assert_eq!(owner_batch.loaded(), 1);
    assert_eq!(owner_batch.failed(), 0);
    assert!(matches!(
        owner_batch.frames(),
        [PhysicalReadAheadFrameOutcome::Loaded { coordinate, .. }]
            if *coordinate == held_coordinate
    ));
    let after = residency.counters();
    assert_eq!(
        after.speculative_attempts(PhysicalSpeculativeWorkKind::ReadAhead),
        before.speculative_attempts(PhysicalSpeculativeWorkKind::ReadAhead) + 2
    );
    assert_eq!(
        after.speculative_admissions(PhysicalSpeculativeWorkKind::ReadAhead),
        before.speculative_admissions(PhysicalSpeculativeWorkKind::ReadAhead) + 1
    );
    assert_eq!(
        after.speculative_completions(PhysicalSpeculativeWorkKind::ReadAhead),
        before.speculative_completions(PhysicalSpeculativeWorkKind::ReadAhead) + 1
    );
    assert_eq!(
        after.speculative_denials(PhysicalSpeculativeWorkKind::ReadAhead),
        before.speculative_denials(PhysicalSpeculativeWorkKind::ReadAhead) + 1
    );
    assert_eq!(
        after.active_speculative_frames(PhysicalSpeculativeWorkKind::ReadAhead),
        0
    );
    assert_eq!(after.active_operation_bytes(), 0);
    assert!(!serving.close().residency().requires_inspection());
}

fn await_coalesced_waiter(
    residency: &worth_store::physical_runtime::PhysicalResidencyCertification,
    before: u64,
) {
    let deadline = std::time::Instant::now() + std::time::Duration::from_secs(5);
    while residency.counters().coalesced_waiters() == before {
        assert!(std::time::Instant::now() < deadline);
        std::thread::yield_now();
    }
}
