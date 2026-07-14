use crate::application::{
    WorthQueryDeclarationRelationalRoutingChecked, WorthQueryDeclarationRelationalRoutingInput,
};

use super::support::{
    domain::{admitted_handle, RoutingInput, RuntimeFamily, SignalOnlyFamily},
    proof::checked_from_progressed,
};

#[test]
fn non_relational_route_plans_fail_closed_before_binding() {
    match checked_from_progressed(
        &admitted_handle("primary"),
        RoutingInput::<SignalOnlyFamily>::new("edge:42"),
    ) {
        WorthQueryDeclarationRelationalRoutingChecked::Denied(denial) => {
            assert_eq!(
                denial.cause(),
                crate::application::WorthQueryDeclarationRelationalRoutingDenialCause::EnvelopeNotCoveredForRelationalRouting
            );
        }
        _ => panic!("signal-only route plans should not route relational truth"),
    }
}

#[test]
fn explicit_routing_rejects_envelopes_from_the_wrong_handle() {
    let primary = admitted_handle("primary");
    let alternate = admitted_handle("alternate");
    let progressed = primary
        .declare_review_and_progress(RoutingInput::<RuntimeFamily>::new("edge:42"))
        .unwrap_or_else(|_| panic!("progression should admit"));
    let envelope = primary
        .envelope_routes_from_progressed(progressed)
        .unwrap_or_else(|_| panic!("envelope should succeed"));

    match alternate.route_relational_truth_checked(
        WorthQueryDeclarationRelationalRoutingInput::enveloped(envelope),
    ) {
        WorthQueryDeclarationRelationalRoutingChecked::Denied(denial) => {
            assert_eq!(
                denial.cause(),
                crate::application::WorthQueryDeclarationRelationalRoutingDenialCause::RelationalEnvelopeMismatch
            );
        }
        _ => panic!("cross-handle routing should deny on handle mismatch"),
    }
}
