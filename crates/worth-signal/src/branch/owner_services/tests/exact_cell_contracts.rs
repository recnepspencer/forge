use crate::branch::{
    admit_runtime_signal_branch_observation, AdmittedSignalBranchSnapshot,
    SignalBranchRetirementReason,
};
use worth_proof::TransitionOutcome;

use super::super::SignalOwnerCancellationSource;
use super::runtime_root::runtime_with_two_branches;

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
        .capture_snapshot_exact(
            &admission,
            &starting_basis,
            reservation,
            &cancellation.token(),
        )
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
        .capture_snapshot_exact(
            &admission,
            &captured_basis,
            reservation,
            &cancellation.token(),
        )
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
