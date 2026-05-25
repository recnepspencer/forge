use crate::application::{
    ForgeQueryDeclarationBridgeBinding, ForgeQueryDeclarationBridgeContinuationFamily,
    ForgeQueryDeclarationBridgeRoutingChecked, ForgeQueryDeclarationBridgeRoutingClass,
};

use super::support::{
    domain::{
        admitted_handle, MixedAuthorityFamily, PreviewPromotionFamily, PreviewSessionFamily,
        RoutingInput, RuntimeRouteFamily, TruthViewCurrentFamily, TruthViewHistoricalFamily,
    },
    proof::{checked_from_progressed, routed_from_progressed},
};

#[test]
fn common_lane_routes_bridge_continuation() {
    let handle = admitted_handle("common");

    let routing = handle
        .declare_review_progress_describe_plan_receipt_envelope_and_route_bridge_continuation(
            RoutingInput::<PreviewSessionFamily>::new("edge:42"),
        )
        .unwrap_or_else(|_| panic!("common bridge lane should admit"));

    assert_eq!(
        routing.continuation_family(),
        ForgeQueryDeclarationBridgeContinuationFamily::PreviewSession
    );
    assert_eq!(
        routing.continuation_request().truth_context(),
        crate::application::ForgeQueryDeclarationBridgeTruthContext::Preview
    );
    assert_eq!(
        routing.binding().surface(),
        "forge_runtime_bridge::facade::BridgeSpeculativeSessionRequest"
    );
}

#[test]
fn equivalent_retained_truth_yields_identical_bridge_digest() {
    let handle = admitted_handle("digest");

    let left = handle
        .route_bridge_continuation_from_progressed(
            handle
                .declare_review_and_progress(RoutingInput::<RuntimeRouteFamily>::new("edge:42"))
                .unwrap_or_else(|_| panic!("progression should admit")),
        )
        .unwrap_or_else(|_| panic!("bridge routing should admit"));
    let right = handle
        .route_bridge_continuation_from_progressed(
            handle
                .declare_review_and_progress(RoutingInput::<RuntimeRouteFamily>::new("edge:42"))
                .unwrap_or_else(|_| panic!("progression should admit")),
        )
        .unwrap_or_else(|_| panic!("bridge routing should admit"));

    assert_eq!(left.bridge_routing_digest(), right.bridge_routing_digest());
}

#[test]
fn mixed_authority_families_keep_common_bridge_lane() {
    let handle = admitted_handle("mixed");

    let routing = handle
        .declare_review_progress_describe_plan_receipt_envelope_and_route_bridge_continuation(
            RoutingInput::<MixedAuthorityFamily>::new("edge:42"),
        )
        .unwrap_or_else(|_| panic!("mixed authority bridge lane should admit"));

    assert_eq!(
        routing.class(),
        ForgeQueryDeclarationBridgeRoutingClass::MixedAuthorityBridgeContinuation
    );
}

#[test]
fn bridge_bindings_remain_distinct_by_family() {
    let handle = admitted_handle("binding");

    let historical = handle
        .declare_review_progress_describe_plan_receipt_envelope_and_route_bridge_continuation(
            RoutingInput::<TruthViewHistoricalFamily>::new("edge:42"),
        )
        .unwrap_or_else(|_| panic!("historical truth view should admit"));
    let promotion = handle
        .declare_review_progress_describe_plan_receipt_envelope_and_route_bridge_continuation(
            RoutingInput::<PreviewPromotionFamily>::new("edge:42"),
        )
        .unwrap_or_else(|_| panic!("preview promotion should admit"));

    assert_ne!(
        historical.bridge_routing_digest(),
        promotion.bridge_routing_digest()
    );
    assert!(matches!(
        historical.binding(),
        ForgeQueryDeclarationBridgeBinding::TruthView(_)
    ));
    assert!(matches!(
        promotion.binding(),
        ForgeQueryDeclarationBridgeBinding::PreviewPromotion(_)
    ));
}

#[test]
fn truth_context_sensitive_truth_view_routes_diverge_in_digest() {
    let handle = admitted_handle("truth-context");

    let current = handle
        .declare_review_progress_describe_plan_receipt_envelope_and_route_bridge_continuation(
            RoutingInput::<TruthViewCurrentFamily>::new("edge:42"),
        )
        .unwrap_or_else(|_| panic!("current truth view should admit"));
    let historical = handle
        .declare_review_progress_describe_plan_receipt_envelope_and_route_bridge_continuation(
            RoutingInput::<TruthViewHistoricalFamily>::new("edge:42"),
        )
        .unwrap_or_else(|_| panic!("historical truth view should admit"));

    assert_ne!(
        current.bridge_routing_digest(),
        historical.bridge_routing_digest()
    );
    assert_eq!(
        current.truth_context(),
        crate::application::ForgeQueryDeclarationBridgeTruthContext::Current
    );
    assert_eq!(
        historical.truth_context(),
        crate::application::ForgeQueryDeclarationBridgeTruthContext::Historical
    );
}

#[test]
fn advanced_lane_envelope_checked_input_routes() {
    let handle = admitted_handle("advanced");

    match checked_from_progressed(
        &handle,
        RoutingInput::<PreviewSessionFamily>::new("edge:42"),
    ) {
        ForgeQueryDeclarationBridgeRoutingChecked::Routed(routing) => {
            assert_eq!(
                routing.continuation_family(),
                ForgeQueryDeclarationBridgeContinuationFamily::PreviewSession
            );
        }
        _ => panic!("advanced bridge lane should route"),
    }
}

#[test]
fn advanced_lane_envelope_input_routes_without_checked_wrapper_loss() {
    let handle = admitted_handle("advanced-direct");

    let routing = routed_from_progressed(
        &handle,
        RoutingInput::<PreviewSessionFamily>::new("edge:42"),
    );

    assert_eq!(
        routing.continuation_family(),
        ForgeQueryDeclarationBridgeContinuationFamily::PreviewSession
    );
    assert_eq!(
        routing.binding().surface(),
        "forge_runtime_bridge::facade::BridgeSpeculativeSessionRequest"
    );
}
