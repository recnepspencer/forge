use super::installed_operation_fixture::{
    artifact_lease_workspace, artifact_move_workspace, bind_artifact_workflow, lease_intent,
    lease_intent_with_mode, move_intent,
};
use worth_proof::TransitionOutcome;
use worth_query::facade::domain;

#[test]
fn moved_artifact_borrows_once_and_disposes_exactly_once() {
    let (mut workspace, probe) = artifact_move_workspace("artifact-move").unwrap();
    let trace = bind_artifact_workflow(&workspace)
        .reexecute(move_intent("produce"), &mut workspace)
        .unwrap();

    assert_eq!(trace.stage_receipts().len(), 2);
    assert_eq!(probe.allocations(), 1);
    assert_eq!(probe.projection_calls(), 1);
    assert_eq!(probe.borrow_observations(), 1);
    assert_eq!(probe.disposals(), 1);
    let snapshots = probe.lifecycle_snapshots();
    assert_eq!(snapshots.len(), 3);
    assert_owner_counts(snapshots[0], 1, 0, 0);
    assert_owner_counts(snapshots[1], 1, 1, 0);
    assert_owner_counts(snapshots[2], 1, 0, 0);
    let counters = snapshots[2].counters();
    assert_eq!(counters.production_admissions, 1);
    assert_eq!(counters.owner_registrations, 1);
    assert_eq!(counters.transfer_admissions, 1);
    assert_eq!(counters.borrow_admissions, 1);
    assert_eq!(counters.lease_admissions, 0);
    assert_eq!(counters.provider_disposals, 0);
}

#[test]
fn distinct_retained_leases_borrow_and_release_the_same_owner_generation_safely() {
    let (mut workspace, probe) = artifact_lease_workspace("artifact-leases").unwrap();
    let trace = bind_artifact_workflow(&workspace)
        .reexecute(lease_intent(), &mut workspace)
        .unwrap();

    assert_eq!(trace.stage_receipts().len(), 3);
    assert_eq!(probe.allocations(), 1);
    assert_eq!(probe.projection_calls(), 1);
    assert_eq!(probe.borrow_observations(), 2);
    assert_eq!(probe.disposals(), 1);
    let snapshots = probe.lifecycle_snapshots();
    assert_eq!(snapshots.len(), 6);
    for window in snapshots.chunks_exact(3) {
        assert_owner_counts(window[0], 1, 0, 1);
        assert_owner_counts(window[1], 1, 1, 1);
        assert_owner_counts(window[2], 1, 0, 1);
    }
    let counters = snapshots[5].counters();
    assert_eq!(counters.production_admissions, 1);
    assert_eq!(counters.owner_registrations, 1);
    assert_eq!(counters.transfer_admissions, 0);
    assert_eq!(counters.borrow_admissions, 2);
    assert_eq!(counters.lease_admissions, 2);
    assert_eq!(counters.provider_disposals, 0);
}

#[test]
fn replacement_disposes_the_prior_owner_and_eventually_the_successor_once_each() {
    let (mut workspace, probe) = artifact_move_workspace("artifact-replacement").unwrap();
    bind_artifact_workflow(&workspace)
        .reexecute(move_intent("replace"), &mut workspace)
        .unwrap();

    assert_eq!(probe.allocations(), 2);
    assert_eq!(probe.projection_calls(), 2);
    assert_eq!(probe.replacements(), 1);
    assert_eq!(probe.disposals(), 2);
}

#[test]
fn explicit_cancellation_returns_a_cancelled_receipt_and_disposes_once() {
    let (mut workspace, probe) = artifact_move_workspace("artifact-cancel").unwrap();
    let outcome =
        bind_artifact_workflow(&workspace).reexecute(move_intent("cancel"), &mut workspace);

    assert!(matches!(
        outcome,
        TransitionOutcome::Denied(_) | TransitionOutcome::Failed(_)
    ));
    assert_eq!(probe.cancellations(), 1);
    assert_eq!(probe.allocations(), 1);
    assert_eq!(probe.disposals(), 1);
}

#[test]
fn declared_failure_after_registration_releases_the_owned_resource() {
    let (mut workspace, probe) = artifact_move_workspace("artifact-failure-cleanup").unwrap();
    let outcome = bind_artifact_workflow(&workspace)
        .reexecute(move_intent("fail-after-production"), &mut workspace);

    assert!(matches!(
        outcome,
        TransitionOutcome::Denied(_) | TransitionOutcome::Failed(_)
    ));
    assert_eq!(probe.allocations(), 1);
    assert_eq!(probe.projection_calls(), 1);
    assert_eq!(probe.disposals(), 1);
}

#[test]
fn declared_failure_after_transfer_releases_the_owned_resource() {
    let (mut workspace, probe) = artifact_move_workspace("artifact-transfer-failure").unwrap();
    let outcome = bind_artifact_workflow(&workspace)
        .reexecute(move_intent("fail-after-transfer"), &mut workspace);

    assert!(matches!(
        outcome,
        TransitionOutcome::Denied(_) | TransitionOutcome::Failed(_)
    ));
    assert_eq!(probe.allocations(), 1);
    assert_eq!(probe.projection_calls(), 1);
    assert_eq!(probe.borrow_observations(), 1);
    assert_eq!(probe.disposals(), 1);
}

#[test]
fn declared_failure_after_lease_transfer_releases_lease_and_owner_exactly_once() {
    let (mut workspace, probe) =
        artifact_lease_workspace("artifact-lease-transfer-failure").unwrap();
    let outcome = bind_artifact_workflow(&workspace).reexecute(
        lease_intent_with_mode("fail-after-lease-transfer"),
        &mut workspace,
    );

    assert!(matches!(
        outcome,
        TransitionOutcome::Denied(_) | TransitionOutcome::Failed(_)
    ));
    assert_eq!(probe.allocations(), 1);
    assert_eq!(probe.projection_calls(), 1);
    assert_eq!(probe.borrow_observations(), 1);
    assert_eq!(probe.disposals(), 1);
    let snapshots = probe.lifecycle_snapshots();
    assert_eq!(snapshots.len(), 3);
    assert_owner_counts(snapshots[0], 1, 0, 1);
    assert_owner_counts(snapshots[1], 1, 1, 1);
    assert_owner_counts(snapshots[2], 1, 0, 1);
}

#[test]
fn escaped_transferred_handle_is_revoked_and_disposed_by_the_run_registry() {
    let (mut workspace, probe) = artifact_move_workspace("artifact-transfer-escape").unwrap();
    let outcome = bind_artifact_workflow(&workspace)
        .reexecute(move_intent("escape-after-transfer"), &mut workspace);

    assert!(matches!(
        outcome,
        TransitionOutcome::Denied(_) | TransitionOutcome::Failed(_)
    ));
    assert_eq!(probe.disposals(), 1);
    let escaped = probe
        .take_escaped_handle()
        .expect("consumer retained the transferred handle for sabotage");
    let snapshot = escaped.owner_snapshot();
    assert_owner_counts(snapshot, 0, 0, 0);
    assert!(snapshot.is_disposed());
    assert_eq!(snapshot.counters().provider_disposals, 1);
    let denial = match escaped.borrow("post-run-sabotage") {
        Err(denial) => denial,
        Ok(_) => panic!("run-closed escaped handle admitted a borrow"),
    };
    assert_eq!(
        denial.kind(),
        domain::WorthQueryArtifactDenialKind::AlreadyDisposed
    );
    drop(escaped);
    assert_eq!(probe.disposals(), 1);
}

#[test]
fn preparation_producer_and_consumer_panics_each_dispose_exactly_once() {
    for (mode, expected_resources, expected_borrows) in [
        ("panic-during-projection", 1, 0),
        ("panic-after-production", 1, 0),
        ("panic-after-transfer", 1, 1),
        ("panic-during-replacement", 2, 0),
    ] {
        let (mut workspace, probe) =
            artifact_move_workspace(&format!("artifact-unwind-{mode}")).unwrap();
        let unwind = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
            let _ = bind_artifact_workflow(&workspace).reexecute(move_intent(mode), &mut workspace);
        }));

        assert!(unwind.is_err(), "{mode} did not unwind");
        assert_eq!(probe.allocations(), expected_resources, "{mode}");
        assert_eq!(probe.projection_calls(), expected_resources, "{mode}");
        assert_eq!(probe.borrow_observations(), expected_borrows, "{mode}");
        assert_eq!(probe.disposals(), expected_resources, "{mode}");
    }
}

fn assert_owner_counts(
    snapshot: domain::WorthQueryArtifactOwnerSnapshot,
    owners: usize,
    borrows: usize,
    leases: usize,
) {
    assert_eq!(snapshot.owner_count(), owners);
    assert_eq!(snapshot.borrow_count(), borrows);
    assert_eq!(snapshot.lease_count(), leases);
}
