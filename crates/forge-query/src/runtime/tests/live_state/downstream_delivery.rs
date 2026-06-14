use super::super::support::*;
use crate::runtime::evidence_identities::{
    runtime_state_snapshot_basis_label_identity, runtime_state_snapshot_test_subject_identity,
};
use crate::subscription::QuerySubscriptionDeliveryCauseKind;
use forge_runtime_bridge::facade::{
    BridgeAsyncCompletionClass, BridgeAsyncCompletionState, BridgeAsyncRequestTruthViewBasis,
    BridgeMixedCauseOrderingInput, BridgeMixedCauseOrderingLaneKind,
    BridgeMixedCauseOrderingRequest, BridgeSubscriptionDeliveryFamilyKind,
};

fn remasked_runtime(projection: ForgeQueryRuntimeRemaskProjection) -> ForgeQueryRuntime {
    ForgeQueryRuntime::builder()
        .runtime_bridge(test_bridge())
        .schema_adapter(TestSchemaAdapter)
        .source_adapter(TestSourceAdapter::default())
        .snapshot_identity(TestSnapshotIdentityAdapter)
        .write_authority(TestWriteAuthority)
        .signal_sink(TestSignalSink)
        .subscription_activation(RemaskingSubscriptionActivation { projection })
        .preview_basis(TestPreviewBasis)
        .inspector_evidence(TestInspectorEvidence)
        .build_backend_from_parts()
        .build()
        .expect("bridge-backed runtime should build")
}

#[test]
fn runtime_downstream_delivery_projects_time_only_contract_with_explicit_resume_posture() {
    let mut runtime = stateful_bridge_task_runtime();
    let view: ForgeQueryLiveView<Value> = runtime
        .declare_live_view(
            "tasks.downstream-time-only",
            task_live_request(),
            task_schema(),
        )
        .expect("live view should declare");
    runtime
        .emit_time_only_delivery(
            view.name(),
            QuerySubscriptionDeliveryCauseKind::WindowEntry,
            "tick:downstream-window-entry",
            false,
            true,
        )
        .expect("time-only delivery should emit");
    let _ = runtime.drain_patches(&view);

    let delivery = runtime
        .downstream_delivery(&view)
        .expect("downstream delivery should project")
        .expect("retained time-only delivery should exist");
    let basis_digest = view.subscription_installation().basis_binding_for_reporting();

    assert_eq!(
        delivery.delivery_class(),
        ForgeQueryRuntimeDownstreamDeliveryClass::TimeOnly
    );
    assert_eq!(
        delivery.support_posture(),
        ForgeQueryRuntimeDownstreamSupportPosture::Supported
    );
    assert_eq!(
        delivery.runtime_resume_support_posture(),
        ForgeQueryLowerRuntimeSupportPosture::Admitted
    );
    assert_eq!(
        delivery.durable_resume_support_posture(),
        ForgeQueryLowerRuntimeSupportPosture::Deferred
    );
    assert_eq!(
        delivery.delivery_cause_kind(),
        QuerySubscriptionDeliveryCauseKind::WindowEntry
    );
    assert_eq!(delivery.basis_for_reporting(), basis_digest);
    assert_eq!(
        delivery
            .negotiate_runtime_resume(Some(
                view.subscription_installation().basis_binding_identity()
            ))
            .kind(),
        ForgeQueryRuntimeDownstreamResumePostureKind::RuntimeBackedAdmitted
    );
    assert_eq!(
        delivery.negotiate_runtime_resume(None).kind(),
        ForgeQueryRuntimeDownstreamResumePostureKind::MissingBasisDenied
    );
    assert_eq!(
        delivery.durable_resume_posture().kind(),
        ForgeQueryRuntimeDownstreamResumePostureKind::DurableDeferredDebt
    );
    assert_eq!(
        delivery.durable_resume_posture().support_posture(),
        ForgeQueryLowerRuntimeSupportPosture::Deferred
    );
}

#[test]
fn runtime_downstream_delivery_projects_mixed_cause_and_async_truth_without_reclassification() {
    let bridge = test_bridge();
    let truth_patch = canonical_truth_patch("truth-main", "snapshot-a", "commit-a", "patch-a");
    let truth_plus_time = authoritative_truth_plus_time_cause(&bridge, &truth_patch);
    let async_completion = admitted_async_completion(
        &bridge,
        forge_signal::facade::NodeId::new(301, 0),
        BridgeAsyncRequestTruthViewBasis::authoritative(
            TruthBranchIdentity::from_bridge_harness_label("truth-main"),
            TruthCommitIdentity::from_bridge_harness_label("commit-a"),
            TruthSnapshotIdentity::from_bridge_harness_label("snapshot-a"),
        ),
        64,
    );
    let ordering = bridge.order_mixed_causes(&BridgeMixedCauseOrderingRequest::new(
        BridgeMixedCauseOrderingLaneKind::Authoritative,
        vec![
            BridgeMixedCauseOrderingInput::TruthPatch(truth_patch),
            BridgeMixedCauseOrderingInput::Temporal(truth_plus_time),
            BridgeMixedCauseOrderingInput::AsyncCompletion(async_completion),
        ],
    ));
    let window = bridge
        .plan_mixed_cause_delivery_window(
            &ordering,
            BridgeSubscriptionDeliveryFamilyKind::AdmittedCoalesced,
        )
        .expect("coalesced mixed-cause delivery should plan");

    let mut runtime = stateful_bridge_task_runtime();
    let view: ForgeQueryLiveView<Value> = runtime
        .declare_live_view(
            "tasks.downstream-mixed-cause",
            task_live_request(),
            task_schema(),
        )
        .expect("live view should declare");
    let (basis_digest, generation_digest) = live_subscription_async_identity(&runtime, view.name());
    runtime
        .project_async_result_state(
            view.name(),
            &ForgeQueryRuntimeAsyncResultProjection::completion_state(
                BridgeAsyncCompletionState::Admitted(BridgeAsyncCompletionClass::Fulfilled),
                "async:downstream-current",
            ),
            &basis_digest,
            &generation_digest,
        )
        .expect("async current state should project");
    runtime
        .emit_mixed_cause_delivery(view.name(), &ordering, &window)
        .expect("mixed-cause delivery should emit");
    let _ = runtime.drain_patches(&view);

    let delivery = runtime
        .downstream_delivery(&view)
        .expect("downstream delivery should project")
        .expect("retained mixed-cause delivery should exist");

    assert_eq!(
        delivery.delivery_class(),
        ForgeQueryRuntimeDownstreamDeliveryClass::MixedCause
    );
    assert!(delivery.mixed_cause_for_reporting().is_some());
    assert!(delivery.async_result_state_for_reporting().is_some());
    assert_eq!(
        delivery
            .negotiate_runtime_resume(Some(
                view.subscription_installation().basis_binding_identity()
            ))
            .kind(),
        ForgeQueryRuntimeDownstreamResumePostureKind::RuntimeBackedAdmitted
    );
}

#[test]
fn runtime_downstream_delivery_fails_closed_for_stale_basis_and_preserves_remask_denial() {
    let mut runtime = remasked_runtime(ForgeQueryRuntimeRemaskProjection::denied(
        ForgeQueryRuntimeRemaskReasonKind::SchemaContextDrift,
        "policy:stable",
        "tenant-truth:stable",
        "tenant-schema:stable",
        "relationship-proof:stable",
        "schema-context:drifted",
    ));
    let view: ForgeQueryLiveView<Value> = runtime
        .declare_live_view(
            "tasks.downstream-remask-denied",
            task_live_request(),
            task_schema(),
        )
        .expect("live view should declare");
    runtime
        .emit_time_only_delivery(
            view.name(),
            QuerySubscriptionDeliveryCauseKind::Deadline,
            "tick:downstream-remask-denied",
            false,
            true,
        )
        .expect("time-only delivery should emit");
    let _ = runtime.drain_patches(&view);

    let delivery = runtime
        .downstream_delivery(&view)
        .expect("downstream delivery should project")
        .expect("retained delivery should exist");

    assert_eq!(
        delivery.support_posture(),
        ForgeQueryRuntimeDownstreamSupportPosture::Denied
    );
    assert!(delivery.remask_for_reporting().is_some());
    assert_eq!(
        delivery
            .negotiate_runtime_resume(Some(
                &runtime_state_snapshot_basis_label_identity(
                    &runtime_state_snapshot_test_subject_identity("basis:drifted"),
                )
            ))
            .kind(),
        ForgeQueryRuntimeDownstreamResumePostureKind::StaleBasisDenied
    );
}
