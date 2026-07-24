use super::installed_operation_fixture::{
    artifact_lease_workspace, artifact_move_workspace, bind_artifact_workflow, lease_intent,
    move_intent,
};
use worth_proof::TransitionOutcome;

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
fn preparation_and_registered_owner_panics_each_dispose_exactly_once() {
    for mode in ["panic-during-projection", "panic-after-production"] {
        let (mut workspace, probe) =
            artifact_move_workspace(&format!("artifact-unwind-{mode}")).unwrap();
        let unwind = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
            let _ = bind_artifact_workflow(&workspace).reexecute(move_intent(mode), &mut workspace);
        }));

        assert!(unwind.is_err(), "{mode} did not unwind");
        assert_eq!(probe.allocations(), 1, "{mode}");
        assert_eq!(probe.projection_calls(), 1, "{mode}");
        assert_eq!(probe.disposals(), 1, "{mode}");
    }
}
