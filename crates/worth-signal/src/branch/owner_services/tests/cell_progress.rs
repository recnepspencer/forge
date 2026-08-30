use std::panic::{catch_unwind, AssertUnwindSafe};
use std::sync::{Arc, Barrier};
use std::thread;

use crate::state::SignalBranchId;

use super::super::{SignalBranchRegistry, SignalOwnerLifecycleState, SignalOwnerServiceCounters};

#[test]
fn parked_branch_does_not_delay_unrelated_branch() {
    let (counters, lifecycle, registry) = kernel(2);
    let installation = lifecycle.admit(61).expect("owner admits installation");
    let branch_a = install(&registry, &installation, SignalBranchId(1), 0_u64);
    let branch_b = install(&registry, &installation, SignalBranchId(2), 0_u64);
    drop(installation);

    let entered_a = Arc::new(Barrier::new(2));
    let release_a = Arc::new(Barrier::new(2));
    let a_lifecycle = Arc::clone(&lifecycle);
    let a_entered = Arc::clone(&entered_a);
    let a_release = Arc::clone(&release_a);
    let branch_a_thread = Arc::clone(&branch_a);
    let a_worker = thread::spawn(move || {
        let admission = a_lifecycle.admit(61).expect("branch A work is admitted");
        branch_a_thread
            .with_state(&admission, |state, work| {
                a_entered.wait();
                a_release.wait();
                *state += 1;
                work.record_canonical_movement();
            })
            .expect("branch A cell accepts its admission");
    });

    entered_a.wait();
    let before_b = counters.snapshot();
    let b_admission = lifecycle.admit(61).expect("branch B work is admitted");
    branch_b
        .with_state(&b_admission, |state, work| {
            *state += 1;
            work.record_canonical_movement();
        })
        .expect("branch B completes while branch A is parked");
    let after_b = counters.snapshot();
    assert_eq!(
        after_b.target_cell_contacts(),
        before_b.target_cell_contacts() + 1
    );
    assert_eq!(after_b.target_cell_waits(), before_b.target_cell_waits());
    assert_eq!(
        after_b.canonical_movements(),
        before_b.canonical_movements() + 1
    );
    assert_eq!(read(&branch_b, &b_admission), 1);

    release_a.wait();
    a_worker.join().expect("branch A worker remains healthy");
    assert_eq!(read(&branch_a, &b_admission), 1);
    let final_snapshot = counters.snapshot();
    assert_eq!(final_snapshot.target_cell_waits(), 0);
    assert_eq!(final_snapshot.branch_registry_entries_scanned(), 0);
}

#[test]
fn same_cell_serializes_with_exact_contact_and_wait_deltas() {
    let (counters, lifecycle, registry) = kernel(1);
    let installation = lifecycle.admit(61).expect("owner admits installation");
    let cell = install(&registry, &installation, SignalBranchId(1), 0_u64);
    drop(installation);
    let baseline = counters.snapshot();
    let entered_first = Arc::new(Barrier::new(2));
    let release_first = Arc::new(Barrier::new(2));

    let first_cell = Arc::clone(&cell);
    let first_lifecycle = Arc::clone(&lifecycle);
    let first_entered = Arc::clone(&entered_first);
    let first_release = Arc::clone(&release_first);
    let first = thread::spawn(move || {
        let admission = first_lifecycle.admit(61).expect("first work is admitted");
        first_cell
            .with_state(&admission, |state, work| {
                first_entered.wait();
                first_release.wait();
                *state += 1;
                work.record_canonical_movement();
            })
            .expect("first operation completes");
    });
    entered_first.wait();

    let second_cell = Arc::clone(&cell);
    let second_lifecycle = Arc::clone(&lifecycle);
    let second = thread::spawn(move || {
        let admission = second_lifecycle.admit(61).expect("second work is admitted");
        second_cell
            .with_state(&admission, |state, work| {
                *state += 1;
                work.record_canonical_movement();
            })
            .expect("second operation completes after serialization");
    });

    while counters.snapshot().target_cell_waits() == baseline.target_cell_waits() {
        thread::yield_now();
    }
    let blocked = counters.snapshot();
    assert_eq!(
        blocked.target_cell_contacts(),
        baseline.target_cell_contacts() + 2
    );
    assert_eq!(
        blocked.target_cell_waits(),
        baseline.target_cell_waits() + 1
    );
    assert_eq!(
        blocked.canonical_movements(),
        baseline.canonical_movements()
    );

    release_first.wait();
    first.join().expect("first worker remains healthy");
    second.join().expect("second worker remains healthy");
    let observation = lifecycle.admit(61).expect("follow-up read is admitted");
    assert_eq!(read(&cell, &observation), 2);
    let final_snapshot = counters.snapshot();
    assert_eq!(
        final_snapshot.target_cell_contacts(),
        baseline.target_cell_contacts() + 3
    );
    assert_eq!(
        final_snapshot.target_cell_waits(),
        baseline.target_cell_waits() + 1
    );
    assert_eq!(
        final_snapshot.canonical_movements(),
        baseline.canonical_movements() + 2
    );
}

#[test]
fn panic_poison_is_recovered_without_harming_other_or_later_cell_work() {
    let (counters, lifecycle, registry) = kernel(2);
    let installation = lifecycle.admit(61).expect("owner admits installation");
    let branch_a = install(&registry, &installation, SignalBranchId(1), 0_u64);
    let branch_b = install(&registry, &installation, SignalBranchId(2), 0_u64);
    drop(installation);
    let baseline = counters.snapshot();

    let panic_admission = lifecycle.admit(61).expect("panic path is admitted");
    let panic_result = catch_unwind(AssertUnwindSafe(|| {
        branch_a
            .with_state(&panic_admission, |state, work| {
                *state += 1;
                work.record_canonical_movement();
                panic!("exercise branch-local poison recovery");
            })
            .expect("admission is valid before the injected panic");
    }));
    assert!(panic_result.is_err());
    drop(panic_admission);

    let healthy_admission = lifecycle.admit(61).expect("owner stays open");
    branch_b
        .with_state(&healthy_admission, |state, work| {
            *state += 1;
            work.record_canonical_movement();
        })
        .expect("unrelated branch remains healthy");
    branch_a
        .with_state(&healthy_admission, |state, work| {
            *state += 1;
            work.record_canonical_movement();
        })
        .expect("poisoned cell explicitly recovers");

    assert_eq!(read(&branch_a, &healthy_admission), 2);
    assert_eq!(read(&branch_b, &healthy_admission), 1);
    let final_snapshot = counters.snapshot();
    assert_eq!(
        final_snapshot.canonical_movements(),
        baseline.canonical_movements() + 3
    );
    assert_eq!(
        final_snapshot.target_cell_contacts(),
        baseline.target_cell_contacts() + 5
    );
    assert_eq!(
        final_snapshot.target_cell_waits(),
        baseline.target_cell_waits()
    );
}

#[test]
fn cell_work_records_exact_semantic_deltas_without_reconstruction() {
    let (counters, lifecycle, registry) = kernel(1);
    let admission = lifecycle.admit(61).expect("owner admits work");
    let cell = install(&registry, &admission, SignalBranchId(1), 0_u64);
    let baseline = counters.snapshot();

    cell.with_state(&admission, |state, work| {
        *state = 9;
        work.record_canonical_movement();
        work.record_retention_registry_contact();
        work.record_fork_source_capture();
        work.record_diagnostic_event();
        work.record_dropped_diagnostic_event();
    })
    .expect("cell operation completes");

    let snapshot = counters.snapshot();
    assert_eq!(
        snapshot.target_cell_contacts(),
        baseline.target_cell_contacts() + 1
    );
    assert_eq!(snapshot.target_cell_waits(), baseline.target_cell_waits());
    assert_eq!(
        snapshot.canonical_movements(),
        baseline.canonical_movements() + 1
    );
    assert_eq!(
        snapshot.retention_registry_contacts(),
        baseline.retention_registry_contacts() + 1
    );
    assert_eq!(
        snapshot.fork_source_captures(),
        baseline.fork_source_captures() + 1
    );
    assert_eq!(
        snapshot.diagnostic_events_recorded(),
        baseline.diagnostic_events_recorded() + 1
    );
    assert_eq!(
        snapshot.diagnostic_events_dropped(),
        baseline.diagnostic_events_dropped() + 1
    );
    assert_eq!(snapshot.forked_mutable_graph_nodes_copied(), 0);
    assert_eq!(snapshot.branch_registry_entries_scanned(), 0);
}

fn kernel(
    maximum_live_branches: usize,
) -> (
    Arc<SignalOwnerServiceCounters>,
    Arc<SignalOwnerLifecycleState>,
    Arc<SignalBranchRegistry<u64>>,
) {
    let counters = Arc::new(SignalOwnerServiceCounters::default());
    let lifecycle = SignalOwnerLifecycleState::new(61, Arc::clone(&counters));
    let registry = Arc::new(SignalBranchRegistry::new(
        &lifecycle,
        maximum_live_branches,
        maximum_live_branches,
    ));
    (counters, lifecycle, registry)
}

fn install(
    registry: &SignalBranchRegistry<u64>,
    admission: &super::super::SignalOwnerOperationAdmission,
    branch_id: SignalBranchId,
    state: u64,
) -> Arc<super::super::SignalBranchExecutionCell<u64>> {
    registry
        .reserve(admission, branch_id)
        .expect("branch identity reserves")
        .install(state)
        .expect("branch state installs")
}

fn read(
    cell: &super::super::SignalBranchExecutionCell<u64>,
    admission: &super::super::SignalOwnerOperationAdmission,
) -> u64 {
    cell.with_state(admission, |state, _| *state)
        .expect("cell observation completes")
}
