use super::super::support::*;

fn bridge_projection_for(
    kind: ForgeQueryRuntimeAsyncResultStateKind,
) -> ForgeQueryRuntimeAsyncResultProjection {
    match kind {
        ForgeQueryRuntimeAsyncResultStateKind::Pending => {
            ForgeQueryRuntimeAsyncResultProjection::pending("async:pending")
        }
        ForgeQueryRuntimeAsyncResultStateKind::Current => {
            ForgeQueryRuntimeAsyncResultProjection::completion_state(
                BridgeAsyncCompletionState::Admitted(BridgeAsyncCompletionClass::Fulfilled),
                "async:current",
            )
        }
        ForgeQueryRuntimeAsyncResultStateKind::Failed => {
            ForgeQueryRuntimeAsyncResultProjection::completion_state(
                BridgeAsyncCompletionState::Denied(BridgeAsyncCompletionDenialClass::Rejected),
                "async:failed",
            )
        }
        ForgeQueryRuntimeAsyncResultStateKind::Stale => {
            ForgeQueryRuntimeAsyncResultProjection::completion_state(
                BridgeAsyncCompletionState::Denied(BridgeAsyncCompletionDenialClass::StaleDenied),
                "async:stale",
            )
        }
        ForgeQueryRuntimeAsyncResultStateKind::Cancelled => {
            ForgeQueryRuntimeAsyncResultProjection::completion_state(
                BridgeAsyncCompletionState::Denied(BridgeAsyncCompletionDenialClass::Cancelled),
                "async:cancelled",
            )
        }
        ForgeQueryRuntimeAsyncResultStateKind::Retried => {
            ForgeQueryRuntimeAsyncResultProjection::forward_causality(
                BridgeAsyncForwardCausalityClass::RetryAfterTimeout,
                "async:retried",
            )
        }
        ForgeQueryRuntimeAsyncResultStateKind::Revalidating => {
            ForgeQueryRuntimeAsyncResultProjection::forward_causality(
                BridgeAsyncForwardCausalityClass::RevalidationAfterTruthBasisDrift,
                "async:revalidating",
            )
        }
        ForgeQueryRuntimeAsyncResultStateKind::Superseded => {
            ForgeQueryRuntimeAsyncResultProjection::supersession("async:superseded")
        }
        ForgeQueryRuntimeAsyncResultStateKind::Denied => {
            ForgeQueryRuntimeAsyncResultProjection::completion_state(
                BridgeAsyncCompletionState::Denied(
                    BridgeAsyncCompletionDenialClass::SignalLifecycleDenied,
                ),
                "async:denied",
            )
        }
    }
}

#[test]
fn runtime_state_and_inspection_project_async_result_state_parity() {
    let mut runtime = stateful_bridge_task_runtime();
    let view: ForgeQueryLiveView<Value> = runtime
        .declare_live_view("tasks.async-state", task_live_request(), task_schema())
        .expect("live view should declare");
    let (basis_digest, generation_digest) = live_subscription_async_identity(&runtime, view.name());
    let cases = [
        (
            ForgeQueryRuntimeAsyncResultStateKind::Pending,
            ForgeQueryRuntimeStateKind::Pending,
        ),
        (
            ForgeQueryRuntimeAsyncResultStateKind::Current,
            ForgeQueryRuntimeStateKind::Ready,
        ),
        (
            ForgeQueryRuntimeAsyncResultStateKind::Failed,
            ForgeQueryRuntimeStateKind::Failed,
        ),
        (
            ForgeQueryRuntimeAsyncResultStateKind::Stale,
            ForgeQueryRuntimeStateKind::Stale,
        ),
        (
            ForgeQueryRuntimeAsyncResultStateKind::Cancelled,
            ForgeQueryRuntimeStateKind::Cancelled,
        ),
        (
            ForgeQueryRuntimeAsyncResultStateKind::Retried,
            ForgeQueryRuntimeStateKind::Retried,
        ),
        (
            ForgeQueryRuntimeAsyncResultStateKind::Revalidating,
            ForgeQueryRuntimeStateKind::Revalidating,
        ),
        (
            ForgeQueryRuntimeAsyncResultStateKind::Superseded,
            ForgeQueryRuntimeStateKind::Superseded,
        ),
        (
            ForgeQueryRuntimeAsyncResultStateKind::Denied,
            ForgeQueryRuntimeStateKind::Denied,
        ),
    ];

    for (kind, expected_state_kind) in cases {
        let projection = bridge_projection_for(kind);
        let projected = runtime
            .project_async_result_state(view.name(), &projection, &basis_digest, &generation_digest)
            .expect("async result state should project");
        let state =
            <&ForgeQueryLiveView<Value> as ForgeQueryRuntimeStateTarget>::into_state_snapshot(
                &view, &runtime,
            )
            .expect("state should snapshot");
        let inspection = runtime
            .inspect_live_view_explanation(&view)
            .expect("inspection should succeed");

        assert_eq!(state.kind(), expected_state_kind);
        assert_eq!(state.async_result_state(), Some(&projected));
        assert!(state.explanation().contains(kind.as_str()));
        assert_eq!(inspection.async_result_state(), Some(&projected));
    }
}

#[test]
fn runtime_async_result_state_preserves_runtime_and_replay_parity() {
    let mut runtime_a = stateful_bridge_task_runtime();
    let view_a: ForgeQueryLiveView<Value> = runtime_a
        .declare_live_view("tasks.async-replay", task_live_request(), task_schema())
        .expect("first live view should declare");
    let mut runtime_b = stateful_bridge_task_runtime();
    let view_b: ForgeQueryLiveView<Value> = runtime_b
        .declare_live_view("tasks.async-replay", task_live_request(), task_schema())
        .expect("second live view should declare");

    let (basis_a, generation_a) = live_subscription_async_identity(&runtime_a, view_a.name());
    let (basis_b, generation_b) = live_subscription_async_identity(&runtime_b, view_b.name());
    assert_eq!(basis_a, basis_b);
    assert_eq!(generation_a, generation_b);

    let projection = bridge_projection_for(ForgeQueryRuntimeAsyncResultStateKind::Current);
    let runtime_state = runtime_a
        .project_async_result_state(view_a.name(), &projection, &basis_a, &generation_a)
        .expect("runtime-backed async projection should succeed");
    let replay_state = runtime_b
        .project_async_result_state(view_b.name(), &projection, &basis_b, &generation_b)
        .expect("replayed async projection should succeed");

    assert_eq!(
        runtime_state.result_state_digest(),
        replay_state.result_state_digest()
    );
    assert_eq!(
        <&ForgeQueryLiveView<Value> as ForgeQueryRuntimeStateTarget>::into_state_snapshot(
            &view_a, &runtime_a,
        )
        .expect("runtime-backed state should snapshot")
        .async_result_state()
        .expect("runtime-backed async result state should exist")
        .result_state_digest(),
        <&ForgeQueryLiveView<Value> as ForgeQueryRuntimeStateTarget>::into_state_snapshot(
            &view_b, &runtime_b,
        )
        .expect("replayed state should snapshot")
        .async_result_state()
        .expect("replayed async result state should exist")
        .result_state_digest()
    );
}

#[test]
fn runtime_async_result_state_fails_closed_for_generation_drift_and_preview_mismatch() {
    let mut runtime = stateful_bridge_task_runtime();
    let view: ForgeQueryLiveView<Value> = runtime
        .declare_live_view("tasks.async-drift", task_live_request(), task_schema())
        .expect("live view should declare");
    let (basis_digest, generation_digest) = live_subscription_async_identity(&runtime, view.name());

    let generation_drift = runtime
        .project_async_result_state(
            view.name(),
            &ForgeQueryRuntimeAsyncResultProjection::completion_state(
                BridgeAsyncCompletionState::Admitted(BridgeAsyncCompletionClass::Fulfilled),
                "async:generation-drift",
            ),
            &basis_digest,
            "generation:drifted",
        )
        .expect_err("current generation drift should deny");
    let preview_mismatch = runtime
        .project_async_result_state(
            view.name(),
            &ForgeQueryRuntimeAsyncResultProjection::completion_state(
                BridgeAsyncCompletionState::Admitted(BridgeAsyncCompletionClass::Fulfilled),
                "async:preview-mismatch",
            ),
            "basis:drifted",
            &generation_digest,
        )
        .expect_err("current preview mismatch should deny");
    let superseded = runtime
        .project_async_result_state(
            view.name(),
            &ForgeQueryRuntimeAsyncResultProjection::supersession("async:superseded-drift"),
            &basis_digest,
            "generation:drifted",
        )
        .expect("superseded generation drift should stay typed");
    let denied = runtime
        .project_async_result_state(
            view.name(),
            &ForgeQueryRuntimeAsyncResultProjection::completion_state(
                BridgeAsyncCompletionState::Denied(
                    BridgeAsyncCompletionDenialClass::SignalLifecycleDenied,
                ),
                "async:denied-preview",
            ),
            "basis:drifted",
            &generation_digest,
        )
        .expect("denied preview mismatch should stay typed");

    match generation_drift {
        ForgeQueryRuntimeError::LiveSubscriptionInstallation { stage, message, .. } => {
            assert_eq!(stage, "async-result-state");
            assert!(message.contains("GenerationDriftRequiresTypedState"));
        }
        other => panic!("expected generation drift denial, got {other:?}"),
    }
    match preview_mismatch {
        ForgeQueryRuntimeError::LiveSubscriptionInstallation { stage, message, .. } => {
            assert_eq!(stage, "async-result-state");
            assert!(message.contains("PreviewBasisMismatchRequiresTypedState"));
        }
        other => panic!("expected preview mismatch denial, got {other:?}"),
    }

    let state = <&ForgeQueryLiveView<Value> as ForgeQueryRuntimeStateTarget>::into_state_snapshot(
        &view, &runtime,
    )
    .expect("state should snapshot");
    let inspection = runtime
        .inspect_live_view_explanation(&view)
        .expect("inspection should succeed");
    assert_eq!(state.kind(), ForgeQueryRuntimeStateKind::Denied);
    assert_eq!(state.async_result_state(), Some(&denied));
    assert_eq!(inspection.async_result_state(), Some(&denied));
    assert_eq!(superseded.generation_digest(), "generation:drifted");
    assert_eq!(denied.basis_digest(), "basis:drifted");
}
