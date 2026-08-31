use crate::branch::{
    admit_runtime_signal_branch_observation, AdmittedSignalBranchSnapshot,
    SignalBranchRetirementReason, SignalBranchSnapshotCaptureDenial,
};
use crate::data::graph::SignalGraph;
use crate::logic::transaction::SignalRuntime;
use worth_proof::TransitionOutcome;

use super::super::SignalOwnerCancellationSource;
use super::runtime_root::runtime_with_two_branches;

type SnapshotCapacityRuntime = SignalRuntime<(), (), (), (), ()>;

fn runtime_with_snapshot_capacity(
    maximum_stored_snapshots: usize,
) -> (
    SnapshotCapacityRuntime,
    crate::state::SignalBranchHandle,
    crate::branch::AdmittedSignalBranchBasis,
) {
    let runtime = SignalRuntime::builder(SignalGraph::new())
        .with_kernel_defaults()
        .maximum_stored_branch_snapshots(maximum_stored_snapshots)
        .build();
    let branch = runtime.current_branch();
    let basis = runtime
        .observe_signal_branch_basis(branch.clone())
        .expect("the real runtime admits its initial branch");
    (runtime, branch, basis)
}

fn caught_panic_message(panic: &(dyn std::any::Any + Send)) -> Option<&str> {
    panic
        .downcast_ref::<&str>()
        .copied()
        .or_else(|| panic.downcast_ref::<String>().map(String::as_str))
}

#[test]
fn exact_snapshot_and_restore_contracts_move_one_cell_and_install_metadata_between_locks() {
    let (mut runtime, _, branch, starting_basis) = runtime_with_two_branches();
    let (_, mutation, _) = runtime.owner_port_slots().expect("runtime seals");
    let owner = mutation.upgrade_owner().expect("owner remains live");
    let admission = owner.admit().expect("operation admits");
    let cell = owner
        .lookup_cell(&admission, branch.id)
        .expect("target cell is live");
    let cancellation = SignalOwnerCancellationSource::new();
    let reservation = owner
        .metadata
        .reserve_snapshot(&admission)
        .expect("snapshot capacity reserves before cell work");
    let capture = cell
        .capture_snapshot_exact(&starting_basis, reservation, &cancellation.token())
        .expect("exact cell snapshot performs");
    let snapshot_a_id = capture.snapshot.meta.snapshot_id;
    assert_eq!(capture.observation.generation().get(), 1);
    assert_eq!(
        capture
            .observation
            .target()
            .as_basis()
            .and_then(|target| target.snapshot_id()),
        Some(snapshot_a_id.0)
    );
    let captured_basis = admit_runtime_signal_branch_observation(
        capture.observation,
        branch.id,
        owner
            .acquire_admitted_retention(branch.id)
            .expect("captured basis retains its branch"),
    );
    let admitted_snapshot_a = AdmittedSignalBranchSnapshot::owner_issued(
        owner.runtime_instance_id(),
        capture.snapshot,
        owner
            .acquire_admitted_retention(branch.id)
            .expect("snapshot authority retains its branch"),
    );
    let snapshot_state = owner
        .metadata
        .snapshot_state(&admission, &admitted_snapshot_a)
        .expect("snapshot lookup is owner-admitted")
        .expect("snapshot semantic state is installed");
    let reservation = owner
        .metadata
        .reserve_snapshot(&admission)
        .expect("second snapshot capacity reserves");
    let capture_b = cell
        .capture_snapshot_exact(&captured_basis, reservation, &cancellation.token())
        .expect("second exact snapshot performs");
    let snapshot_b_id = capture_b.snapshot.meta.snapshot_id;
    assert_ne!(snapshot_b_id, snapshot_a_id);
    let basis_b = admit_runtime_signal_branch_observation(
        capture_b.observation,
        branch.id,
        owner
            .acquire_admitted_retention(branch.id)
            .expect("second captured basis retains its branch"),
    );
    let admitted_snapshot_b = AdmittedSignalBranchSnapshot::owner_issued(
        owner.runtime_instance_id(),
        capture_b.snapshot,
        owner
            .acquire_admitted_retention(branch.id)
            .expect("second snapshot authority retains its branch"),
    );
    let snapshot_state_b = owner
        .metadata
        .snapshot_state(&admission, &admitted_snapshot_b)
        .expect("second snapshot lookup is owner-admitted")
        .expect("second snapshot semantic state is installed");
    let before_mismatch = cell.cost_snapshot();
    let mismatch = cell.restore_exact(
        &admission,
        &basis_b,
        &admitted_snapshot_b,
        snapshot_state,
        &cancellation.token(),
    );
    assert!(matches!(
        mismatch,
        Err(crate::branch::SignalBranchRestoreDenial::UnavailableSnapshot {
            branch_id,
            snapshot_id,
        }) if branch_id == branch.id && snapshot_id == snapshot_b_id
    ));
    assert_eq!(cell.cost_snapshot(), before_mismatch);
    let restore = cell
        .restore_exact(
            &admission,
            &basis_b,
            &admitted_snapshot_b,
            snapshot_state_b,
            &cancellation.token(),
        )
        .expect("exact cell restore performs");
    assert_eq!(restore.observation.generation().get(), 3);
    assert_eq!(
        restore
            .observation
            .target()
            .as_basis()
            .and_then(|target| target.restore_snapshot_id()),
        Some(snapshot_b_id.0)
    );
    assert_eq!(cell.cost_snapshot().movements(), 3);
}

#[test]
fn snapshot_reservation_restores_capacity_after_denial_drop_and_unwind() {
    let (mut runtime, branch, starting_basis) = runtime_with_snapshot_capacity(1);
    let (_, mutation, _) = runtime.owner_port_slots().expect("runtime seals");
    let owner = mutation.upgrade_owner().expect("owner remains live");
    let admission = owner.admit().expect("snapshot operation admits");
    let cell = owner
        .lookup_cell(&admission, branch.id)
        .expect("target cell is live");

    let reservation = owner
        .metadata
        .reserve_snapshot(&admission)
        .expect("one snapshot slot reserves");
    assert!(matches!(
        owner.metadata.reserve_snapshot(&admission),
        Err(
            SignalBranchSnapshotCaptureDenial::SnapshotCapacityExhausted {
                maximum_stored_snapshots: 1,
            }
        )
    ));
    assert_eq!(cell.cost_snapshot(), Default::default());
    drop(reservation);

    let unwind = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
        let _reservation = owner
            .metadata
            .reserve_snapshot(&admission)
            .expect("the dropped reservation restored capacity");
        panic!("exercise snapshot reservation unwind cleanup");
    }));
    let unwind = unwind.expect_err("the reservation scope must unwind");
    assert_eq!(
        caught_panic_message(unwind.as_ref()),
        Some("exercise snapshot reservation unwind cleanup")
    );

    let cancellation = SignalOwnerCancellationSource::new();
    let mut runtime_context = ();
    let advanced = cell
        .advance_exact::<(), (), _>(
            &admission,
            &starting_basis,
            &mut runtime_context,
            &cancellation.token(),
            |_| Ok(()),
        )
        .expect("the real owner cell advances before the stale capture twin");
    let (advanced_observation, _) = advanced.into_parts();
    let advanced_basis = admit_runtime_signal_branch_observation(
        advanced_observation,
        branch.id,
        owner
            .acquire_admitted_retention(branch.id)
            .expect("the advanced basis retains its branch"),
    );
    let denied_reservation = owner
        .metadata
        .reserve_snapshot(&admission)
        .expect("the stale capture reserves before exact comparison");
    let before_stale_capture = cell.cost_snapshot();
    assert!(matches!(
        cell.capture_snapshot_exact(&starting_basis, denied_reservation, &cancellation.token(),),
        Err(SignalBranchSnapshotCaptureDenial::BasisMismatch { .. })
    ));
    let after_stale_capture = cell.cost_snapshot();
    assert_eq!(
        after_stale_capture.contacts(),
        before_stale_capture.contacts() + 1
    );
    assert_eq!(
        after_stale_capture.movements(),
        before_stale_capture.movements()
    );

    let reservation = owner
        .metadata
        .reserve_snapshot(&admission)
        .expect("stale denial restored snapshot capacity after cell release");
    let capture = cell
        .capture_snapshot_exact(&advanced_basis, reservation, &cancellation.token())
        .expect("the reserved snapshot installs through the real owner cell");
    assert_eq!(capture.observation.generation().get(), 2);
    assert!(matches!(
        owner.metadata.reserve_snapshot(&admission),
        Err(
            SignalBranchSnapshotCaptureDenial::SnapshotCapacityExhausted {
                maximum_stored_snapshots: 1,
            }
        )
    ));
}

#[test]
fn snapshot_reservation_rejects_a_different_owner_before_target_contact() {
    let (mut runtime_a, _, _) = runtime_with_snapshot_capacity(1);
    let (mut runtime_b, branch_b, basis_b) = runtime_with_snapshot_capacity(1);
    let (_, mutation_a, _) = runtime_a.owner_port_slots().expect("owner A seals");
    let (_, mutation_b, _) = runtime_b.owner_port_slots().expect("owner B seals");
    let owner_a = mutation_a.upgrade_owner().expect("owner A remains live");
    let owner_b = mutation_b.upgrade_owner().expect("owner B remains live");
    let admission_a = owner_a.admit().expect("owner A admits reservation");
    let admission_b = owner_b.admit().expect("owner B admits capture");
    let cell_b = owner_b
        .lookup_cell(&admission_b, branch_b.id)
        .expect("owner B target cell is live");
    let reservation_a = owner_a
        .metadata
        .reserve_snapshot(&admission_a)
        .expect("owner A reserves its snapshot capacity");
    let before_b = cell_b.cost_snapshot();
    let cancellation = SignalOwnerCancellationSource::new();

    assert!(matches!(
        cell_b.capture_snapshot_exact(&basis_b, reservation_a, &cancellation.token(),),
        Err(SignalBranchSnapshotCaptureDenial::OwnerUnavailable(_))
    ));
    assert_eq!(cell_b.cost_snapshot(), before_b);
    let reusable_a = owner_a
        .metadata
        .reserve_snapshot(&admission_a)
        .expect("cross-owner denial restores owner A capacity");
    drop(reusable_a);
    let reservation_b = owner_b
        .metadata
        .reserve_snapshot(&admission_b)
        .expect("owner B reserves after the foreign attempt");
    let healthy_b = cell_b
        .capture_snapshot_exact(&basis_b, reservation_b, &cancellation.token())
        .expect("owner B captures through its own originating admission");
    assert_eq!(healthy_b.observation.generation().get(), 1);
}

#[cfg(debug_assertions)]
#[test]
fn snapshot_reservation_cleanup_guard_detects_out_of_order_cell_cleanup() {
    let (mut runtime, branch, _) = runtime_with_snapshot_capacity(1);
    let (_, mutation, _) = runtime.owner_port_slots().expect("runtime seals");
    let owner = mutation.upgrade_owner().expect("owner remains live");
    let admission = owner.admit().expect("snapshot operation admits");
    let cell = owner
        .lookup_cell(&admission, branch.id)
        .expect("target cell is live");
    let reservation = owner
        .metadata
        .reserve_snapshot(&admission)
        .expect("snapshot capacity reserves before cell admission");

    let out_of_order = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
        cell.with_state(&admission, |_, _| drop(reservation))
            .expect("the real owner cell admits before the sensitivity fault");
    }));
    let out_of_order = out_of_order
        .expect_err("dropping snapshot capacity under a cell hold must trip the ordering guard");
    assert_eq!(
        caught_panic_message(out_of_order.as_ref()),
        Some("snapshot reservation cleanup must run after target-cell release")
    );
}

#[test]
fn exact_retirement_contract_consumes_a_linear_plan_before_registry_removal() {
    let (mut runtime, _, branch, basis) = runtime_with_two_branches();
    let plan = match runtime.plan_signal_branch_retirement(
        branch.clone(),
        basis,
        SignalBranchRetirementReason::Rejected,
    ) {
        TransitionOutcome::Success(plan) => plan,
        other => panic!("retirement plan should be issued before sealing: {other:?}"),
    };
    let (_, _, lifecycle) = runtime.owner_port_slots().expect("runtime seals");
    let owner = lifecycle.upgrade_owner().expect("owner remains live");
    let admission = owner.admit().expect("retirement admits");
    let retirement = owner
        .begin_retirement(&admission, branch.id)
        .expect("registry reserves the exact target incarnation");
    let cancellation = SignalOwnerCancellationSource::new();
    let outcome = retirement
        .execute_exact(plan, &cancellation.token())
        .expect("registry admission remains valid")
        .expect("exact cell retirement performs");
    assert_eq!(outcome.retired_branch, branch);
    assert_eq!(outcome.reason, SignalBranchRetirementReason::Rejected);
    assert!(!outcome.terminal_basis_digest.is_empty());
    assert_eq!(owner.live_count(), 1);
}
