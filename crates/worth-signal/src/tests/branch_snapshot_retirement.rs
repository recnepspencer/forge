use worth_proof::TransitionOutcome;

use crate::facade::branch::{AdmittedSignalBranchBasis, AdmittedSignalBranchSnapshot};
use crate::facade::*;

type TestRuntime = SignalRuntime<(), (), (), (), ()>;

fn runtime() -> TestRuntime {
    SignalRuntime::builder(SignalGraph::new())
        .with_kernel_defaults()
        .build()
}

fn fork(runtime: &mut TestRuntime, name: &str, parent: SignalBranchId) -> SignalBranchHandle {
    match runtime.fork_branch(SignalBranchForkRequest::from_parent_branch_head(
        name, parent,
    )) {
        TransitionOutcome::Success(receipt) => receipt.created_branch().clone(),
        other => panic!("expected branch fork, got {other:?}"),
    }
}

fn capture(
    runtime: &mut TestRuntime,
    branch: &SignalBranchHandle,
) -> (AdmittedSignalBranchSnapshot, AdmittedSignalBranchBasis) {
    let basis = runtime
        .observe_signal_branch_basis(branch.clone())
        .expect("branch should be observable");
    runtime
        .capture_signal_branch_snapshot(&basis)
        .expect("canonical snapshot capture should succeed")
        .into_parts()
}

#[test]
fn declared_snapshot_release_permits_retirement_only_after_authority_is_dropped() {
    let mut runtime = runtime();
    let canonical = runtime.current_branch();
    let branch = fork(&mut runtime, "snapshot-release", canonical.id);
    let (snapshot, captured_basis) = capture(&mut runtime, &branch);

    assert!(matches!(
        runtime.plan_signal_branch_retirement(
            branch.clone(),
            captured_basis,
            SignalBranchRetirementReason::Rejected,
        ),
        TransitionOutcome::Denied(SignalBranchRetirementDenial::RetainedAdmittedBasis {
            active_leases: 2,
            ..
        })
    ));

    let basis = runtime
        .observe_signal_branch_basis(branch.clone())
        .expect("branch should remain observable");
    let snapshot_clone = snapshot.clone();
    let plan = match runtime.plan_signal_branch_retirement_releasing_snapshots(
        branch.clone(),
        basis,
        &[&snapshot, &snapshot_clone],
        SignalBranchRetirementReason::Rejected,
    ) {
        TransitionOutcome::Success(plan) => plan,
        other => panic!("expected snapshot-aware retirement plan, got {other:?}"),
    };
    drop(snapshot_clone);
    drop(snapshot.into_snapshot());

    assert!(matches!(
        runtime.retire_signal_branch(plan),
        TransitionOutcome::Success(_)
    ));
    assert!(runtime.branch_handle(branch.id).is_none());
}

#[test]
fn denied_snapshot_release_plan_preserves_restore_authority() {
    let mut runtime = runtime();
    let canonical = runtime.current_branch();
    let parent = fork(&mut runtime, "snapshot-parent", canonical.id);
    let child = fork(&mut runtime, "live-child", parent.id);
    let (snapshot, captured_basis) = capture(&mut runtime, &parent);

    assert!(matches!(
        runtime.plan_signal_branch_retirement_releasing_snapshots(
            parent.clone(),
            captured_basis,
            &[&snapshot],
            SignalBranchRetirementReason::DependencyCancellation,
        ),
        TransitionOutcome::Denied(SignalBranchRetirementDenial::LiveChildren {
            child_branch_ids,
            ..
        }) if child_branch_ids == vec![child.id]
    ));

    let basis = runtime
        .observe_signal_branch_basis(parent)
        .expect("denied retirement should preserve the parent");
    runtime
        .restore_signal_branch(&basis, &snapshot)
        .expect("denied planning must leave snapshot authority usable");
}

#[test]
fn snapshot_release_allowance_rejects_cross_branch_authority() {
    let mut runtime = runtime();
    let canonical = runtime.current_branch();
    let snapshot_branch = fork(&mut runtime, "snapshot-owner", canonical.id);
    let retirement_branch = fork(&mut runtime, "retirement-target", canonical.id);
    let (snapshot, _) = capture(&mut runtime, &snapshot_branch);
    let retirement_basis = runtime
        .observe_signal_branch_basis(retirement_branch.clone())
        .expect("retirement branch should be observable");

    assert!(matches!(
        runtime.plan_signal_branch_retirement_releasing_snapshots(
            retirement_branch.clone(),
            retirement_basis,
            &[&snapshot],
            SignalBranchRetirementReason::Rejected,
        ),
        TransitionOutcome::Denied(
            SignalBranchRetirementDenial::RetirementSnapshotBranchMismatch {
                branch_id,
                snapshot_branch_id,
            }
        ) if branch_id == retirement_branch.id && snapshot_branch_id == snapshot_branch.id
    ));
}

#[test]
fn snapshot_release_allowance_rejects_foreign_runtime_authority() {
    let mut target_runtime = runtime();
    let target_canonical = target_runtime.current_branch();
    let target_branch = fork(&mut target_runtime, "same-id", target_canonical.id);

    let mut source_runtime = runtime();
    let source_canonical = source_runtime.current_branch();
    let source_branch = fork(&mut source_runtime, "same-id", source_canonical.id);
    assert_eq!(target_branch.id, source_branch.id);
    let (source_snapshot, _) = capture(&mut source_runtime, &source_branch);

    let target_basis = target_runtime
        .observe_signal_branch_basis(target_branch.clone())
        .expect("target branch should be observable");
    assert!(matches!(
        target_runtime.plan_signal_branch_retirement_releasing_snapshots(
            target_branch.clone(),
            target_basis,
            &[&source_snapshot],
            SignalBranchRetirementReason::Rejected,
        ),
        TransitionOutcome::Denied(SignalBranchRetirementDenial::ForeignRetirementSnapshot { .. })
    ));
    assert!(target_runtime.branch_handle(target_branch.id).is_some());

    let source_basis = source_runtime
        .observe_signal_branch_basis(source_branch)
        .expect("source branch should retain authority");
    source_runtime
        .restore_signal_branch(&source_basis, &source_snapshot)
        .expect("foreign denial must preserve source snapshot authority");
}

#[test]
fn snapshot_release_batch_retires_child_before_parent() {
    let mut runtime = runtime();
    let canonical = runtime.current_branch();
    let parent = fork(&mut runtime, "batch-parent", canonical.id);
    let child = fork(&mut runtime, "batch-child", parent.id);
    let (child_snapshot, child_basis) = capture(&mut runtime, &child);
    let (parent_snapshot, parent_basis) = capture(&mut runtime, &parent);
    let child_snapshot_clone = child_snapshot.clone();

    let plan = match runtime.plan_signal_branch_retirement_batch_releasing_snapshots(vec![
        (
            child.clone(),
            child_basis,
            vec![&child_snapshot, &child_snapshot_clone],
            SignalBranchRetirementReason::Rejected,
        ),
        (
            parent.clone(),
            parent_basis,
            vec![&parent_snapshot],
            SignalBranchRetirementReason::DependencyCancellation,
        ),
    ]) {
        TransitionOutcome::Success(plan) => plan,
        other => panic!("expected snapshot-aware batch plan, got {other:?}"),
    };
    drop(child_snapshot_clone);
    drop(child_snapshot.into_snapshot());
    drop(parent_snapshot.into_snapshot());

    let receipt = match runtime.retire_signal_branch_batch(plan) {
        TransitionOutcome::Success(receipt) => receipt,
        other => panic!("expected snapshot-aware batch retirement, got {other:?}"),
    };
    assert_eq!(receipt.receipts()[0].retired_branch().id, child.id);
    assert_eq!(receipt.receipts()[1].retired_branch().id, parent.id);
}
