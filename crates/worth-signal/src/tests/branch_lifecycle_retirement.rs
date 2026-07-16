use worth_proof::TransitionOutcome;

use crate::facade::*;

fn fork(
    runtime: &mut SignalRuntime<(), (), (), (), ()>,
    name: impl Into<String>,
    parent: SignalBranchId,
) -> SignalBranchHandle {
    match runtime.fork_branch(SignalBranchForkRequest::from_parent_branch_head(
        name, parent,
    )) {
        TransitionOutcome::Success(receipt) => receipt.created_branch().clone(),
        other => panic!("expected branch fork, got {other:?}"),
    }
}

fn retirement_plan(
    runtime: &mut SignalRuntime<(), (), (), (), ()>,
    branch: SignalBranchHandle,
    reason: SignalBranchRetirementReason,
) -> PlannedSignalBranchRetirement {
    let head = match runtime.branch_transaction_head(branch.clone()) {
        TransitionOutcome::Success(head) => head,
        other => panic!("expected branch head, got {other:?}"),
    };
    match runtime.plan_branch_retirement(SignalBranchRetirementRequest::new(branch, head, reason)) {
        TransitionOutcome::Success(plan) => plan,
        other => panic!("expected retirement plan, got {other:?}"),
    }
}

fn retirement_request(
    runtime: &mut SignalRuntime<(), (), (), (), ()>,
    branch: SignalBranchHandle,
    reason: SignalBranchRetirementReason,
) -> SignalBranchRetirementRequest {
    let head = match runtime.branch_transaction_head(branch.clone()) {
        TransitionOutcome::Success(head) => head,
        other => panic!("expected branch head, got {other:?}"),
    };
    SignalBranchRetirementRequest::new(branch, head, reason)
}

#[test]
fn retirement_reclaims_heavy_state_and_retains_compact_closeout_proof() {
    let mut runtime = SignalRuntime::builder(SignalGraph::new())
        .with_kernel_defaults()
        .build();
    let canonical = runtime.current_branch();
    let branch = fork(&mut runtime, "retire-with-snapshots", canonical.id);
    runtime.switch_branch(branch.clone()).unwrap();
    runtime.capture_snapshot();
    runtime.capture_snapshot();
    runtime.switch_branch(canonical.clone()).unwrap();

    let plan = retirement_plan(
        &mut runtime,
        branch.clone(),
        SignalBranchRetirementReason::Rejected,
    );
    let receipt = match runtime.retire_branch(plan) {
        TransitionOutcome::Success(receipt) => receipt,
        other => panic!("expected retirement success, got {other:?}"),
    };

    assert_eq!(receipt.retired_branch(), &branch);
    assert_eq!(receipt.parent_branch_id(), canonical.id);
    assert_eq!(receipt.reclaimed_branch_state_count(), 1);
    assert_eq!(receipt.reclaimed_snapshot_state_count(), 2);
    assert_eq!(receipt.reclaimed_runtime_meta_count(), 1);
    assert_eq!(receipt.retained_proof_record_count(), 1);
    assert!(!receipt.closeout_digest().is_empty());
    assert!(runtime.branch_handle(branch.id).is_none());
    assert_eq!(runtime.known_branches(), vec![canonical]);
    assert_eq!(
        runtime
            .branch_retirement_receipt(branch.id)
            .expect("retirement proof must remain readable")
            .closeout_digest(),
        receipt.closeout_digest()
    );
    assert!(runtime
        .replay_for_branch(runtime.current_branch().id)
        .frames
        .iter()
        .any(|event| event.kind == ReplayEventKind::BranchRetired));
}

#[test]
fn retirement_denies_current_and_parent_branches_with_live_native_children() {
    let mut runtime = SignalRuntime::builder(SignalGraph::new())
        .with_kernel_defaults()
        .build();
    let canonical = runtime.current_branch();
    let canonical_head = match runtime.branch_transaction_head(canonical.clone()) {
        TransitionOutcome::Success(head) => head,
        other => panic!("expected canonical head, got {other:?}"),
    };
    let current_denial = runtime.plan_branch_retirement(SignalBranchRetirementRequest::new(
        canonical.clone(),
        canonical_head,
        SignalBranchRetirementReason::Superseded,
    ));
    assert!(matches!(
        current_denial,
        TransitionOutcome::Denied(SignalBranchRetirementDenial::CurrentBranch { .. })
    ));

    let parent = fork(&mut runtime, "parent", canonical.id);
    let child = fork(&mut runtime, "child", parent.id);
    let parent_head = match runtime.branch_transaction_head(parent.clone()) {
        TransitionOutcome::Success(head) => head,
        other => panic!("expected parent head, got {other:?}"),
    };
    let denial = runtime.plan_branch_retirement(SignalBranchRetirementRequest::new(
        parent.clone(),
        parent_head,
        SignalBranchRetirementReason::DependencyCancellation,
    ));
    assert!(matches!(
        denial,
        TransitionOutcome::Denied(SignalBranchRetirementDenial::LiveChildren {
            child_branch_ids,
            ..
        }) if child_branch_ids == vec![child.id]
    ));
    let child_plan = retirement_plan(
        &mut runtime,
        child,
        SignalBranchRetirementReason::DependencyCancellation,
    );
    assert!(matches!(
        runtime.retire_branch(child_plan),
        TransitionOutcome::Success(_)
    ));
    let parent_plan = retirement_plan(
        &mut runtime,
        parent,
        SignalBranchRetirementReason::DependencyCancellation,
    );
    assert!(matches!(
        runtime.retire_branch(parent_plan),
        TransitionOutcome::Success(_)
    ));
}

#[test]
fn one_thousand_retired_siblings_leave_no_live_branch_residue() {
    let mut runtime = SignalRuntime::builder(SignalGraph::new())
        .with_kernel_defaults()
        .build();
    let canonical = runtime.current_branch();
    let siblings = (0..1_000)
        .map(|ordinal| fork(&mut runtime, format!("sibling-{ordinal}"), canonical.id))
        .collect::<Vec<_>>();

    for sibling in &siblings {
        let plan = retirement_plan(
            &mut runtime,
            sibling.clone(),
            SignalBranchRetirementReason::ProjectionRebuild,
        );
        assert!(matches!(
            runtime.retire_branch(plan),
            TransitionOutcome::Success(_)
        ));
    }

    assert_eq!(runtime.known_branches(), vec![canonical]);
    assert!(siblings
        .iter()
        .all(|branch| runtime.branch_handle(branch.id).is_none()));
    assert!(siblings
        .iter()
        .all(|branch| runtime.branch_retirement_receipt(branch.id).is_some()));
    assert_eq!(
        runtime
            .telemetry()
            .transaction
            .branch_retirement_execution_count,
        1_000
    );
}

#[test]
fn retirement_plan_is_invalidated_by_a_snapshot_free_branch_transaction() {
    let mut graph = SignalGraph::new();
    let node = graph.node().build();
    let mut runtime = SignalRuntime::builder(graph).with_kernel_defaults().build();
    let canonical = runtime.current_branch();
    let branch = fork(&mut runtime, "moving-head", canonical.id);
    let retirement = retirement_plan(
        &mut runtime,
        branch.clone(),
        SignalBranchRetirementReason::Superseded,
    );
    let head = match runtime.branch_transaction_head(branch.clone()) {
        TransitionOutcome::Success(head) => head,
        other => panic!("expected branch head, got {other:?}"),
    };
    let transaction = match runtime
        .plan_branch_targeted_transaction(BranchTargetedTransactionRequest::new(branch, head))
    {
        TransitionOutcome::Success(plan) => plan,
        other => panic!("expected targeted transaction plan, got {other:?}"),
    };
    assert!(matches!(
        runtime.execute_branch_targeted_transaction(&mut (), transaction, |tx| {
            tx.mark_dirty(node, Aspect::new(0))
        }),
        TransitionOutcome::Success(_)
    ));

    assert!(matches!(
        runtime.retire_branch(retirement),
        TransitionOutcome::Denied(SignalBranchRetirementDenial::StaleBranchHead { .. })
    ));
}

#[test]
fn ordered_retirement_batch_closes_child_then_parent_as_one_plan() {
    let mut runtime = SignalRuntime::builder(SignalGraph::new())
        .with_kernel_defaults()
        .build();
    let canonical = runtime.current_branch();
    let parent = fork(&mut runtime, "derived-basis", canonical.id);
    let child = fork(&mut runtime, "effect", parent.id);
    let child_request = retirement_request(
        &mut runtime,
        child.clone(),
        SignalBranchRetirementReason::Merged,
    );
    let parent_request = retirement_request(
        &mut runtime,
        parent.clone(),
        SignalBranchRetirementReason::DependencyCancellation,
    );
    let plan =
        match runtime.plan_branch_retirement_batch(SignalBranchRetirementBatchRequest::new(vec![
            child_request,
            parent_request,
        ])) {
            TransitionOutcome::Success(plan) => plan,
            other => panic!("expected ordered retirement plan, got {other:?}"),
        };
    assert_eq!(plan.breadth(), 2);
    let receipt = match runtime.retire_branch_batch(plan) {
        TransitionOutcome::Success(receipt) => receipt,
        other => panic!("expected ordered retirement, got {other:?}"),
    };
    assert_eq!(receipt.receipts().len(), 2);
    assert_eq!(receipt.receipts()[0].retired_branch().id, child.id);
    assert_eq!(receipt.receipts()[1].retired_branch().id, parent.id);
    assert_eq!(runtime.known_branches(), vec![canonical]);
}

#[test]
fn retirement_batch_denial_is_side_effect_free() {
    let mut runtime = SignalRuntime::builder(SignalGraph::new())
        .with_kernel_defaults()
        .build();
    let canonical = runtime.current_branch();
    let parent = fork(&mut runtime, "parent-first", canonical.id);
    let child = fork(&mut runtime, "child-second", parent.id);
    let parent_request = retirement_request(
        &mut runtime,
        parent.clone(),
        SignalBranchRetirementReason::DependencyCancellation,
    );
    let child_request = retirement_request(
        &mut runtime,
        child.clone(),
        SignalBranchRetirementReason::Rejected,
    );
    assert!(matches!(
        runtime.plan_branch_retirement_batch(SignalBranchRetirementBatchRequest::new(vec![
            parent_request,
            child_request,
        ])),
        TransitionOutcome::Denied(SignalBranchRetirementBatchDenial::Retirement {
            denial: SignalBranchRetirementDenial::LiveChildren { .. },
            ..
        })
    ));
    assert!(runtime.branch_handle(parent.id).is_some());
    assert!(runtime.branch_handle(child.id).is_some());
    assert!(runtime.branch_retirement_receipt(parent.id).is_none());
    assert!(runtime.branch_retirement_receipt(child.id).is_none());
}
