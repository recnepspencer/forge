use worth_runtime_bridge::facade::{
    BridgeMixedCauseOrderingInput, BridgeMixedCauseOrderingRequest,
};
use worth_signal::facade::NodeId;

use crate::{
    scalar_projection_async_fixture::{
        admitted_async_completion_for_request, admitted_async_request_and_completion,
        async_ordering, authoritative_async_basis, projection_bridge, scalar_async_view,
    },
    scalar_text_projection_fixture::{insert_status, projection_workspace, update_status},
    UiPresentProjection, UiProjectionAvailability, UiProjectionConsumptionBudget,
    UiProjectionFactStopKind, UiProjectionFieldRequirement, UiProjectionRetainedActivityKind,
    UiProjectionTransitionPosture, UiScalarProjectionBatchOutcome, UiScalarProjectionBinding,
    UiScalarProjectionBindingAdmission, UiScalarProjectionFactReceipt,
    UiScalarProjectionRegistration, WorthUiQueryWorkspaceExt,
};

#[test]
fn bridge_current_stale_revalidating_current_consumes_native_query_text() {
    let bridge = projection_bridge();
    let (request, first_completion) = admitted_async_request_and_completion(
        &bridge,
        NodeId::new(313, 0),
        authoritative_async_basis("commit-a", "snapshot-a"),
        64,
    );
    let mut workspace = projection_workspace(true);
    let entity = insert_status(&mut workspace, "Ready");
    let view = scalar_async_view(&mut workspace, &request);
    let mut binding = scalar_binding(&workspace);

    let current_batch = admit_input(
        &bridge,
        &mut workspace,
        &view,
        BridgeMixedCauseOrderingInput::AsyncCompletion(first_completion),
    );
    let current_batch_identity = current_batch.binding_identity().clone();
    let current = advanced(binding.consume_async_result_batch(
        &mut workspace,
        current_batch,
        None,
        UiProjectionConsumptionBudget::platform_pulse(),
    ));
    assert_current(current.fact(), "Ready");
    assert_eq!(
        current.fact().core().binding_identity_for_reporting(),
        current_batch_identity.terminal_projection_for_reporting()
    );
    assert!(current.retained_predecessor().is_none());
    assert_eq!(current.work().native_key_declaration_checks(), 1);
    assert_eq!(current.work().native_key_indexed_slot_lookups(), 2);
    assert_eq!(current.work().native_key_scan_work(), 0);
    assert_eq!(current.work().native_indexed_accesses(), 1);
    assert_eq!(current.work().native_access_scan_work(), 0);
    let (current_fact, retained) = current.into_fact_and_predecessor();
    assert!(retained.is_none());

    let revalidation = bridge
        .revalidate_async_request(
            &request,
            authoritative_async_basis("commit-b", "snapshot-b"),
        )
        .expect("Bridge must issue revalidation lineage");
    let next_request = revalidation.newer_request().clone();
    let refresh_batch = admit_input(
        &bridge,
        &mut workspace,
        &view,
        BridgeMixedCauseOrderingInput::AsyncRevalidationLineage(revalidation),
    );
    let refresh = advanced(binding.consume_async_result_batch(
        &mut workspace,
        refresh_batch,
        Some(current_fact),
        UiProjectionConsumptionBudget::platform_pulse(),
    ));
    assert_eq!(
        refresh.posture_trace().postures(),
        [
            UiProjectionTransitionPosture::RetainedStale(UiProjectionRetainedActivityKind::Idle,),
            UiProjectionTransitionPosture::RetainedStale(
                UiProjectionRetainedActivityKind::Revalidating,
            ),
        ]
    );
    assert_retained_revalidating(refresh.fact(), "Ready");
    assert!(refresh.retained_predecessor().is_none());
    assert_eq!(refresh.work().native_indexed_accesses(), 0);
    let (refresh_fact, retained) = refresh.into_fact_and_predecessor();
    assert!(retained.is_none());

    update_status(&mut workspace, entity, "Updated");
    let refreshed_completion = admitted_async_completion_for_request(&bridge, &next_request, 72);
    let refreshed_batch = admit_input(
        &bridge,
        &mut workspace,
        &view,
        BridgeMixedCauseOrderingInput::AsyncCompletion(refreshed_completion),
    );
    let refreshed = advanced(binding.consume_async_result_batch(
        &mut workspace,
        refreshed_batch,
        Some(refresh_fact),
        UiProjectionConsumptionBudget::platform_pulse(),
    ));
    assert_current(refreshed.fact(), "Updated");
    assert_eq!(
        refreshed.fact().core().binding_identity_for_reporting(),
        current_batch_identity.terminal_projection_for_reporting()
    );
    assert_eq!(refreshed.work().native_indexed_accesses(), 1);
}

#[test]
fn foreign_query_batch_stops_before_native_projection_consumption() {
    let foreign_bridge = projection_bridge();
    let (foreign_request, foreign_completion) = admitted_async_request_and_completion(
        &foreign_bridge,
        NodeId::new(314, 0),
        authoritative_async_basis("commit-a", "snapshot-a"),
        64,
    );
    let mut source = projection_workspace(true);
    insert_status(&mut source, "Source");
    let mut binding = scalar_binding(&source);
    let mut foreign = projection_workspace(true);
    insert_status(&mut foreign, "Source");
    let foreign_view = scalar_async_view(&mut foreign, &foreign_request);
    let foreign_batch = admit_input(
        &foreign_bridge,
        &mut foreign,
        &foreign_view,
        BridgeMixedCauseOrderingInput::AsyncCompletion(foreign_completion),
    );

    let stopped = advanced(binding.consume_async_result_batch(
        &mut source,
        foreign_batch,
        None,
        UiProjectionConsumptionBudget::platform_pulse(),
    ));
    match stopped.fact().availability() {
        UiProjectionAvailability::Stopped(stop) => {
            assert_eq!(stop.kind(), UiProjectionFactStopKind::WrongWorld)
        }
        other => panic!("foreign Query batch must stop, got {other:?}"),
    }
    assert_eq!(stopped.work().native_indexed_accesses(), 0);
}

#[test]
fn revalidation_without_a_predecessor_value_is_a_basis_stop() {
    let bridge = projection_bridge();
    let (request, first_completion) = admitted_async_request_and_completion(
        &bridge,
        NodeId::new(315, 0),
        authoritative_async_basis("commit-a", "snapshot-a"),
        64,
    );
    let mut workspace = projection_workspace(true);
    insert_status(&mut workspace, "Ready");
    let view = scalar_async_view(&mut workspace, &request);
    let mut binding = scalar_binding(&workspace);
    let _current_batch = admit_input(
        &bridge,
        &mut workspace,
        &view,
        BridgeMixedCauseOrderingInput::AsyncCompletion(first_completion),
    );
    let revalidation = bridge
        .revalidate_async_request(
            &request,
            authoritative_async_basis("commit-b", "snapshot-b"),
        )
        .expect("Bridge must issue revalidation lineage");
    let refresh_batch = admit_input(
        &bridge,
        &mut workspace,
        &view,
        BridgeMixedCauseOrderingInput::AsyncRevalidationLineage(revalidation),
    );

    let stopped = advanced(binding.consume_async_result_batch(
        &mut workspace,
        refresh_batch,
        None,
        UiProjectionConsumptionBudget::platform_pulse(),
    ));
    match stopped.fact().availability() {
        UiProjectionAvailability::Stopped(stop) => {
            assert_eq!(stop.kind(), UiProjectionFactStopKind::BasisMismatch)
        }
        other => panic!("missing predecessor must stop, got {other:?}"),
    }
}

fn scalar_binding(
    workspace: &worth_query::facade::runtime::WorthQueryWorkspace,
) -> UiScalarProjectionBinding {
    let view = workspace
        .worth_ui()
        .expect("Worth UI domain installed")
        .projection_view("platform.pulse.status")
        .expect("valid Platform Pulse view");
    let registration = UiScalarProjectionRegistration::text(
        view,
        UiProjectionFieldRequirement::declared("status").expect("valid selected field"),
    );
    match registration.admit(workspace) {
        UiScalarProjectionBindingAdmission::Ready(binding) => binding,
        UiScalarProjectionBindingAdmission::Unavailable(unavailable) => {
            panic!("scalar binding must be supported: {unavailable:?}")
        }
        UiScalarProjectionBindingAdmission::Stopped(stop) => {
            panic!("scalar binding must admit: {}", stop.summary())
        }
    }
}

fn admit_input(
    bridge: &worth_runtime_bridge::facade::RuntimeBridge,
    workspace: &mut worth_query::facade::runtime::WorthQueryWorkspace,
    view: &worth_query::facade::runtime::WorthQueryLiveView<
        worth_query::facade::runtime::WorthQueryUnrefinedLiveShape,
    >,
    input: BridgeMixedCauseOrderingInput,
) -> worth_query::facade::runtime::WorthQueryAsyncResultTransitionBatch {
    let request: BridgeMixedCauseOrderingRequest = async_ordering(input);
    let ordering = bridge.order_mixed_causes(&request);
    workspace
        .admit_bridge_async_result_transitions(view, &ordering)
        .expect("Bridge async transition must reach Query")
}

fn advanced(outcome: UiScalarProjectionBatchOutcome) -> crate::UiScalarProjectionTransitionReceipt {
    match outcome {
        UiScalarProjectionBatchOutcome::Advanced(receipt) => receipt,
        UiScalarProjectionBatchOutcome::Unchanged(_) => {
            panic!("the lifecycle stimulus must advance")
        }
    }
}

fn assert_current(fact: &UiScalarProjectionFactReceipt, expected: &str) {
    match fact.availability() {
        UiProjectionAvailability::Present(UiPresentProjection::Current(value)) => {
            assert_eq!(value.as_str(), expected)
        }
        other => panic!("expected current scalar value, got {other:?}"),
    }
}

fn assert_retained_revalidating(fact: &UiScalarProjectionFactReceipt, expected: &str) {
    match fact.availability() {
        UiProjectionAvailability::Present(UiPresentProjection::RetainedStale {
            value,
            activity,
        }) => {
            assert_eq!(value.as_str(), expected);
            assert_eq!(
                activity.kind(),
                UiProjectionRetainedActivityKind::Revalidating
            );
        }
        other => panic!("expected retained revalidating scalar value, got {other:?}"),
    }
}
