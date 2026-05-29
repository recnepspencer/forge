use crate::application::{
    ForgeQueryDeclarationBridgeRoutingChecked, ForgeQueryDeclarationBridgeRoutingDenialCause,
    ForgeQueryDeclarationEnvelopeInput,
};

use super::support::{
    domain::{admitted_handle, RoutingInput, RuntimeRouteFamily, SignalOnlyFamily},
    proof::checked_from_progressed,
};

#[test]
fn explicit_routing_rejects_envelopes_from_the_wrong_handle() {
    let first = admitted_handle("alpha");
    let second = admitted_handle("beta");

    let progressed = first
        .declare_review_and_progress(RoutingInput::<RuntimeRouteFamily>::new("edge:42"))
        .unwrap_or_else(|_| panic!("progression should admit"));
    let envelope = first
        .envelope_routes_from_progressed(progressed)
        .unwrap_or_else(|_| panic!("envelope should admit"));

    match second.route_bridge_continuation_checked(
        crate::application::ForgeQueryDeclarationBridgeRoutingInput::enveloped(envelope),
    ) {
        ForgeQueryDeclarationBridgeRoutingChecked::Denied(routing) => {
            assert_eq!(
                routing.cause(),
                ForgeQueryDeclarationBridgeRoutingDenialCause::BridgeEnvelopeMismatch
            );
        }
        _ => panic!("wrong-handle bridge routing should deny"),
    }
}

#[test]
fn signal_only_declarations_do_not_reach_bridge_continuation() {
    let handle = admitted_handle("signal-only");

    match checked_from_progressed(&handle, RoutingInput::<SignalOnlyFamily>::new("edge:42")) {
        ForgeQueryDeclarationBridgeRoutingChecked::Denied(routing) => {
            assert_eq!(
                routing.cause(),
                ForgeQueryDeclarationBridgeRoutingDenialCause::EnvelopeNotCoveredForBridgeRouting
            );
        }
        _ => panic!("signal-only route plans should deny bridge continuation"),
    }
}

#[test]
fn denied_receipt_envelopes_stay_denied_for_bridge_routing() {
    let handle = admitted_handle("denied");

    let route_checked = handle.plan_routes_checked(
        crate::application::ForgeQueryDeclarationRoutePlanInput::with_intent(
            handle
                .declare_review_and_progress(RoutingInput::<RuntimeRouteFamily>::new("edge:42"))
                .unwrap_or_else(|_| panic!("progression should admit")),
            handle
                .describe_foundational(
                    crate::application::ForgeQueryDeclarationFoundationalEvidenceInput::admitted_progression(
                        handle
                            .declare_review_and_progress(RoutingInput::<RuntimeRouteFamily>::new("edge:42"))
                            .unwrap_or_else(|_| panic!("progression should admit")),
                    ),
                )
                .unwrap_or_else(|_| panic!("evidence should materialize")),
            crate::application::ForgeQueryDeclarationRouteIntent::DeferredRouting,
        ),
    );
    let receipt_checked = handle.receipt_routes_checked(
        crate::application::ForgeQueryDeclarationReceiptInput::route_checked(route_checked),
    );
    let envelope_checked = handle.envelope_routes_checked(
        ForgeQueryDeclarationEnvelopeInput::receipt_checked(receipt_checked),
    );

    match handle.route_bridge_continuation_checked(
        crate::application::ForgeQueryDeclarationBridgeRoutingInput::envelope_checked(
            envelope_checked,
        ),
    ) {
        ForgeQueryDeclarationBridgeRoutingChecked::Denied(_)
        | ForgeQueryDeclarationBridgeRoutingChecked::Deferred(_) => {}
        _ => panic!("non-success envelope input should not route"),
    }
}
