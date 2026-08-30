use std::panic::{catch_unwind, AssertUnwindSafe};
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Arc;

use crate::state::SignalBranchId;

use super::super::{
    SignalBranchCellAdmissionDenial, SignalBranchExecutionCell, SignalBranchRegistry,
    SignalBranchRegistryDenial, SignalBranchRegistryPoisonRecovery, SignalOwnerLifecycleState,
    SignalOwnerServiceCounters,
};

#[test]
fn registry_preserves_one_cell_per_id_and_direct_unknown_denial() {
    let counters = Arc::new(SignalOwnerServiceCounters::default());
    let lifecycle = SignalOwnerLifecycleState::new(11, Arc::clone(&counters));
    let admission = lifecycle.admit(11).expect("owner admits registry work");
    let registry = SignalBranchRegistry::new(&lifecycle, 2, 2);
    let branch_id = SignalBranchId(1);
    let installed = registry
        .reserve(&admission, branch_id)
        .expect("identity reserves")
        .install(17_u64)
        .expect("reserved state installs");

    let first_lookup = registry
        .lookup(&admission, branch_id)
        .expect("installed branch is found");
    let second_lookup = registry
        .lookup(&admission, branch_id)
        .expect("same branch is found again");
    assert!(Arc::ptr_eq(&installed, &first_lookup));
    assert!(Arc::ptr_eq(&first_lookup, &second_lookup));
    assert_eq!(first_lookup.branch_id(), branch_id);
    assert_eq!(
        registry.reserve(&admission, branch_id).unwrap_err(),
        SignalBranchRegistryDenial::DuplicateBranch(branch_id)
    );
    assert_eq!(
        registry.lookup(&admission, SignalBranchId(99)).unwrap_err(),
        SignalBranchRegistryDenial::UnknownBranch(SignalBranchId(99))
    );

    let snapshot = counters.snapshot();
    assert_eq!(snapshot.branch_registry_lookups(), 3);
    assert_eq!(snapshot.branch_registry_reservations(), 1);
    assert_eq!(snapshot.branch_registry_entries_scanned(), 0);
}

#[test]
fn explicit_registry_scan_records_each_visited_entry_while_direct_paths_stay_zero_scan() {
    let counters = Arc::new(SignalOwnerServiceCounters::default());
    let lifecycle = SignalOwnerLifecycleState::new(12, Arc::clone(&counters));
    let admission = lifecycle.admit(12).expect("owner admits registry work");
    let registry = SignalBranchRegistry::new(&lifecycle, 2, 2);
    for branch_id in [SignalBranchId(2), SignalBranchId(1)] {
        registry
            .reserve(&admission, branch_id)
            .expect("identity reserves")
            .install(branch_id.0)
            .expect("state installs");
    }
    assert_eq!(counters.snapshot().branch_registry_entries_scanned(), 0);

    let scanned = registry
        .live_cells_in_identity_order(&admission)
        .expect("explicit owner scan succeeds");
    assert_eq!(
        scanned
            .iter()
            .map(|cell| cell.branch_id())
            .collect::<Vec<_>>(),
        vec![SignalBranchId(1), SignalBranchId(2)]
    );
    assert_eq!(counters.snapshot().branch_registry_entries_scanned(), 2);

    registry
        .lookup(&admission, SignalBranchId(1))
        .expect("direct keyed lookup stays healthy");
    assert_eq!(counters.snapshot().branch_registry_entries_scanned(), 2);
}

#[test]
fn live_and_reservation_capacity_deny_independently_before_installation() {
    let live_counters = Arc::new(SignalOwnerServiceCounters::default());
    let live_lifecycle = SignalOwnerLifecycleState::new(21, live_counters);
    let live_admission = live_lifecycle.admit(21).expect("owner admits work");
    let live_registry = SignalBranchRegistry::<u64>::new(&live_lifecycle, 1, 2);
    let held_live_slot = live_registry
        .reserve(&live_admission, SignalBranchId(1))
        .expect("first reservation claims the live slot");
    assert_eq!(
        live_registry
            .reserve(&live_admission, SignalBranchId(2))
            .unwrap_err(),
        SignalBranchRegistryDenial::LiveCapacityExhausted {
            maximum_live_branches: 1
        }
    );
    assert_eq!(live_registry.live_count(), 0);
    assert_eq!(live_registry.reservation_count(), 1);
    drop(held_live_slot);

    let reservation_counters = Arc::new(SignalOwnerServiceCounters::default());
    let reservation_lifecycle = SignalOwnerLifecycleState::new(22, reservation_counters);
    let reservation_admission = reservation_lifecycle.admit(22).expect("owner admits work");
    let reservation_registry = SignalBranchRegistry::<u64>::new(&reservation_lifecycle, 3, 1);
    let held_reservation = reservation_registry
        .reserve(&reservation_admission, SignalBranchId(1))
        .expect("first reservation claims reservation capacity");
    assert_eq!(
        reservation_registry
            .reserve(&reservation_admission, SignalBranchId(2))
            .unwrap_err(),
        SignalBranchRegistryDenial::ReservationCapacityExhausted {
            maximum_reservations: 1
        }
    );
    assert_eq!(reservation_registry.live_count(), 0);
    assert_eq!(reservation_registry.reservation_count(), 1);
    assert_eq!(reservation_registry.maximum_live_branches(), 3);
    assert_eq!(reservation_registry.maximum_reservations(), 1);
    drop(held_reservation);
}

#[test]
fn reservation_drop_and_caught_unwind_restore_capacity_exactly() {
    let counters = Arc::new(SignalOwnerServiceCounters::default());
    let lifecycle = SignalOwnerLifecycleState::new(31, Arc::clone(&counters));
    let admission = lifecycle.admit(31).expect("owner admits work");
    let registry = SignalBranchRegistry::<u64>::new(&lifecycle, 1, 1);

    {
        let reservation = registry
            .reserve(&admission, SignalBranchId(1))
            .expect("capacity is initially available");
        assert_eq!(registry.reservation_count(), 1);
        drop(reservation);
    }
    assert_eq!(registry.reservation_count(), 0);

    let panic_result = catch_unwind(AssertUnwindSafe(|| {
        let _reservation = registry
            .reserve(&admission, SignalBranchId(2))
            .expect("capacity is available after ordinary drop");
        panic!("exercise reservation unwind cleanup");
    }));
    assert!(panic_result.is_err());
    assert_eq!(registry.reservation_count(), 0);

    registry
        .reserve(&admission, SignalBranchId(3))
        .expect("capacity is available after unwind")
        .install_fork_destination(3)
        .expect("healthy installation follows unwind");
    assert_eq!(registry.live_count(), 1);
    let snapshot = counters.snapshot();
    assert_eq!(snapshot.branch_registry_reservations(), 3);
    assert_eq!(snapshot.fork_destination_installations(), 1);
    assert_eq!(snapshot.branch_registry_entries_scanned(), 0);
}

#[test]
fn retirement_busy_guard_stale_handle_and_reinstall_restore_exact_live_capacity() {
    let counters = Arc::new(SignalOwnerServiceCounters::default());
    let lifecycle = SignalOwnerLifecycleState::new(32, Arc::clone(&counters));
    let admission = lifecycle.admit(32).expect("owner admits work");
    let registry = SignalBranchRegistry::new(&lifecycle, 1, 1);
    let branch_id = SignalBranchId(1);
    let old_cell = registry
        .reserve(&admission, branch_id)
        .expect("initial identity reserves")
        .install(7_u64)
        .expect("initial cell installs");

    let pending = registry
        .begin_retirement(&admission, branch_id)
        .expect("owner marks the canonical cell retiring");
    assert_eq!(
        registry.lookup(&admission, branch_id).unwrap_err(),
        SignalBranchRegistryDenial::RetirementInProgress(branch_id)
    );
    assert_eq!(
        registry.reserve(&admission, branch_id).unwrap_err(),
        SignalBranchRegistryDenial::RetirementInProgress(branch_id)
    );
    let busy_mutation_executed = AtomicBool::new(false);
    assert_eq!(
        old_cell
            .with_state(&admission, |state, _| {
                busy_mutation_executed.store(true, Ordering::Release);
                *state += 1;
            })
            .unwrap_err(),
        SignalBranchCellAdmissionDenial::RetirementInProgress
    );
    assert!(!busy_mutation_executed.load(Ordering::Acquire));
    drop(pending);
    assert!(Arc::ptr_eq(
        &old_cell,
        &registry
            .lookup(&admission, branch_id)
            .expect("abandoned retirement restores canonical membership")
    ));

    registry
        .begin_retirement(&admission, branch_id)
        .expect("healthy retirement restarts")
        .complete()
        .expect("cell becomes inert before membership is removed");
    assert_eq!(registry.live_count(), 0);
    let stale_mutation_executed = AtomicBool::new(false);
    assert_eq!(
        old_cell
            .with_state(&admission, |state, _| {
                stale_mutation_executed.store(true, Ordering::Release);
                *state += 1;
            })
            .unwrap_err(),
        SignalBranchCellAdmissionDenial::RetiredIncarnation
    );
    assert!(!stale_mutation_executed.load(Ordering::Acquire));

    let reinstalled = registry
        .reserve(&admission, branch_id)
        .expect("retirement restores the sole live slot")
        .install(11_u64)
        .expect("same semantic identity receives a new cell incarnation");
    assert!(!Arc::ptr_eq(&old_cell, &reinstalled));
    reinstalled
        .with_state(&admission, |state, _| *state += 1)
        .expect("new incarnation remains healthy");
    assert_eq!(
        reinstalled
            .with_state(&admission, |state, _| *state)
            .expect("new incarnation is observable"),
        12
    );
    assert_eq!(registry.live_count(), 1);
    assert_eq!(counters.snapshot().branch_registry_reservations(), 2);
    assert_eq!(counters.snapshot().branch_registry_entries_scanned(), 0);
}

#[test]
fn registry_poison_policy_preserves_canonical_membership() {
    let counters = Arc::new(SignalOwnerServiceCounters::default());
    let lifecycle = SignalOwnerLifecycleState::new(33, counters);
    let admission = lifecycle.admit(33).expect("owner admits work");
    let registry = SignalBranchRegistry::new(&lifecycle, 1, 1);
    let installed = registry
        .reserve(&admission, SignalBranchId(1))
        .expect("identity reserves")
        .install(5_u64)
        .expect("cell installs");

    let poisoned = catch_unwind(AssertUnwindSafe(|| registry.poison_state_for_test()));
    assert!(poisoned.is_err());
    let recovered = registry
        .lookup(&admission, SignalBranchId(1))
        .expect("registry follows its recoverable poison policy");
    assert!(Arc::ptr_eq(&installed, &recovered));
    assert_eq!(
        registry.poison_recovery(),
        Some(SignalBranchRegistryPoisonRecovery::PreservedCanonicalMembership)
    );
}

#[test]
fn signal_kernel_owners_are_send_and_sync() {
    fn assert_send_sync<T: Send + Sync>() {}

    assert_send_sync::<SignalOwnerServiceCounters>();
    assert_send_sync::<SignalOwnerLifecycleState>();
    assert_send_sync::<super::super::SignalOwnerOperationAdmission>();
    assert_send_sync::<SignalBranchRegistry<u64>>();
    assert_send_sync::<SignalBranchExecutionCell<u64>>();
}
