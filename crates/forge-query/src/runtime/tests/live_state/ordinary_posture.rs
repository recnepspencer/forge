use super::super::support::*;
use crate::ordinary_outcome::{
    ForgeQueryOrdinaryRuntimeAsyncPostureKind, ForgeQueryOrdinaryRuntimeBasisPostureKind,
    ForgeQueryOrdinaryRuntimeCausePostureKind, ForgeQueryOrdinaryRuntimePosture,
    ForgeQueryOrdinaryRuntimePostureKind,
};
use crate::subscription::QuerySubscriptionDeliveryCauseKind;
use forge_runtime_bridge::facade::{
    BridgeAsyncCompletionClass, BridgeAsyncCompletionDenialClass, BridgeAsyncCompletionState,
    BridgeAsyncRequestTruthViewBasis, BridgeMixedCauseOrderingInput,
    BridgeMixedCauseOrderingLaneKind, BridgeMixedCauseOrderingRequest,
    BridgeSubscriptionDeliveryFamilyKind,
};

fn inspect_live_view_posture<T>(
    runtime: &ForgeQueryRuntime,
    view: &ForgeQueryLiveView<T>,
) -> ForgeQueryOrdinaryRuntimePosture {
    let inspection = runtime.inspect(view).expect("inspection should succeed");
    let ForgeQueryInspection::LiveView(inspection) = inspection else {
        panic!("inspection should target the live-view surface");
    };
    inspection.ordinary_runtime_posture().clone()
}

#[test]
fn runtime_state_and_inspection_share_time_only_compact_posture() {
    let mut runtime = stateful_bridge_task_runtime();
    let view: ForgeQueryLiveView<Value> = runtime
        .declare_live_view(
            "tasks.compact-time-only",
            task_live_request(),
            task_schema(),
        )
        .expect("live view should declare");
    runtime
        .emit_time_only_delivery(
            view.name(),
            QuerySubscriptionDeliveryCauseKind::WindowEntry,
            "tick:compact-window-entry",
            false,
            true,
        )
        .expect("time-only delivery should emit");
    let _ = runtime.drain_patches(&view);

    let state = <&ForgeQueryLiveView<Value> as ForgeQueryRuntimeStateTarget>::into_state_snapshot(
        &view, &runtime,
    )
    .expect("state should snapshot");
    let posture = state
        .ordinary_runtime_posture()
        .expect("compact posture should project on state");

    assert_eq!(
        posture.kind(),
        ForgeQueryOrdinaryRuntimePostureKind::Current
    );
    assert_eq!(
        posture.cause_posture(),
        ForgeQueryOrdinaryRuntimeCausePostureKind::TimeOnly
    );
    assert_eq!(posture.async_posture(), None);
    assert_eq!(
        posture.basis_posture(),
        ForgeQueryOrdinaryRuntimeBasisPostureKind::Stable
    );
    assert_eq!(
        posture.support_evidence_digest(),
        view.subscription_installation().support_evidence()
    );
    assert_eq!(posture, &inspect_live_view_posture(&runtime, &view));
}

#[test]
fn runtime_state_and_inspection_share_mixed_async_compact_posture() {
    let bridge = test_bridge();
    let truth_patch = canonical_truth_patch("truth-main", "snapshot-a", "commit-a", "patch-a");
    let truth_plus_time = authoritative_truth_plus_time_cause(&bridge, &truth_patch);
    let async_completion = admitted_async_completion(
        &bridge,
        forge_signal::facade::NodeId::new(243, 0),
        BridgeAsyncRequestTruthViewBasis::authoritative(
            TruthBranchIdentity::new("truth-main"),
            TruthCommitIdentity::new("commit-a"),
            TruthSnapshotIdentity::new("snapshot-a"),
        ),
        64,
    );
    let ordering = bridge.order_mixed_causes(&BridgeMixedCauseOrderingRequest::new(
        BridgeMixedCauseOrderingLaneKind::Authoritative,
        vec![
            BridgeMixedCauseOrderingInput::AsyncCompletion(async_completion),
            BridgeMixedCauseOrderingInput::TruthPatch(truth_patch),
            BridgeMixedCauseOrderingInput::Temporal(truth_plus_time),
        ],
    ));
    let window = bridge
        .plan_mixed_cause_delivery_window(
            &ordering,
            BridgeSubscriptionDeliveryFamilyKind::AdmittedCoalesced,
        )
        .expect("mixed-cause delivery window should plan");

    let mut runtime = stateful_bridge_task_runtime();
    let view: ForgeQueryLiveView<Value> = runtime
        .declare_live_view(
            "tasks.compact-mixed-async",
            task_live_request(),
            task_schema(),
        )
        .expect("live view should declare");
    runtime
        .emit_mixed_cause_delivery(view.name(), &ordering, &window)
        .expect("mixed-cause delivery should emit");
    let (basis_digest, generation_digest) = live_subscription_async_identity(&runtime, view.name());
    runtime
        .project_async_result_state(
            view.name(),
            &ForgeQueryRuntimeAsyncResultProjection::completion_state(
                BridgeAsyncCompletionState::Admitted(BridgeAsyncCompletionClass::Fulfilled),
                "async:compact-current",
            ),
            &basis_digest,
            &generation_digest,
        )
        .expect("async result state should project");
    let _ = runtime.drain_patches(&view);

    let state = <&ForgeQueryLiveView<Value> as ForgeQueryRuntimeStateTarget>::into_state_snapshot(
        &view, &runtime,
    )
    .expect("state should snapshot");
    let posture = state
        .ordinary_runtime_posture()
        .expect("compact posture should project on state");

    assert_eq!(
        posture.kind(),
        ForgeQueryOrdinaryRuntimePostureKind::Current
    );
    assert_eq!(
        posture.cause_posture(),
        ForgeQueryOrdinaryRuntimeCausePostureKind::MixedCause
    );
    assert_eq!(
        posture.async_posture(),
        Some(ForgeQueryOrdinaryRuntimeAsyncPostureKind::Current)
    );
    assert_eq!(
        posture.basis_posture(),
        ForgeQueryOrdinaryRuntimeBasisPostureKind::Stable
    );
    assert_eq!(
        posture.support_evidence_digest(),
        view.subscription_installation().support_evidence()
    );
    assert_eq!(posture, &inspect_live_view_posture(&runtime, &view));
}

#[test]
fn runtime_compact_posture_keeps_basis_sensitive_denied_async_state_typed_on_scalar_surfaces() {
    let mut runtime = stateful_bridge_task_runtime();
    let view: ForgeQueryLiveView<Value> = runtime
        .declare_live_view(
            "tasks.compact-async-denied",
            task_live_request(),
            task_schema(),
        )
        .expect("live view should declare");
    let (basis_digest, generation_digest) = live_subscription_async_identity(&runtime, view.name());
    runtime
        .project_async_result_state(
            view.name(),
            &ForgeQueryRuntimeAsyncResultProjection::completion_state(
                BridgeAsyncCompletionState::Denied(
                    BridgeAsyncCompletionDenialClass::SignalLifecycleDenied,
                ),
                "async:compact-denied",
            ),
            "basis:drifted",
            &generation_digest,
        )
        .expect("denied async state should project");

    let state = <&ForgeQueryLiveView<Value> as ForgeQueryRuntimeStateTarget>::into_state_snapshot(
        &view, &runtime,
    )
    .expect("state should snapshot");
    let posture = state
        .ordinary_runtime_posture()
        .expect("compact posture should project on state");

    assert_eq!(state.kind(), ForgeQueryRuntimeStateKind::Denied);
    assert_eq!(posture.kind(), ForgeQueryOrdinaryRuntimePostureKind::Denied);
    assert_eq!(
        posture.cause_posture(),
        ForgeQueryOrdinaryRuntimeCausePostureKind::Ordinary
    );
    assert_eq!(
        posture.async_posture(),
        Some(ForgeQueryOrdinaryRuntimeAsyncPostureKind::Denied)
    );
    assert_eq!(
        posture.basis_posture(),
        ForgeQueryOrdinaryRuntimeBasisPostureKind::BasisDrift
    );
    assert_ne!(basis_digest, "basis:drifted");
    assert_eq!(
        posture.support_evidence_digest(),
        view.subscription_installation().support_evidence()
    );
    assert_eq!(posture, &inspect_live_view_posture(&runtime, &view));
}
