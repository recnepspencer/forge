use crate::application::{
    WorthQueryDeclarationRouteIntent, WorthQueryDeclarationRoutePlanChecked,
    WorthQueryDeclarationRoutePlanDenialCause, WorthQueryDeclarationRoutePlanInput,
};

use super::domain::{
    admitted_handle, progressed, route_input, DeferredRouteFamily, FailedRouteFamily,
    ForbiddenIntentFamily, RelationalRouteFamily, RequiredIntentFamily, RouteInput,
};

#[test]
fn required_intent_is_a_typed_denial() {
    let handle = admitted_handle("primary");

    match handle.plan_routes_checked(route_input(
        &handle,
        RouteInput::<RequiredIntentFamily>::new("edge:42"),
    )) {
        WorthQueryDeclarationRoutePlanChecked::Denied(denial) => {
            assert_eq!(
                denial.cause(),
                WorthQueryDeclarationRoutePlanDenialCause::IntentRequired
            );
            assert!(denial
                .reason()
                .contains("requires explicit caller route intent"));
        }
        _ => panic!("required intent should deny without explicit route intent"),
    }
}

#[test]
fn forbidden_intent_is_a_typed_denial() {
    let handle = admitted_handle("primary");
    let progressed = progressed(&handle, RouteInput::<ForbiddenIntentFamily>::new("edge:42"));
    let evidence = handle
        .describe_foundational(
            crate::application::WorthQueryDeclarationFoundationalEvidenceInput::admitted_progression(
                progressed.clone(),
            ),
        )
        .unwrap_or_else(|_| panic!("same-handle foundational evidence should materialize"));

    match handle.plan_routes_checked(WorthQueryDeclarationRoutePlanInput::with_intent(
        progressed,
        evidence,
        WorthQueryDeclarationRouteIntent::RelationalOnly,
    )) {
        WorthQueryDeclarationRoutePlanChecked::Denied(denial) => {
            assert_eq!(
                denial.cause(),
                WorthQueryDeclarationRoutePlanDenialCause::IntentForbidden
            );
            assert!(denial
                .reason()
                .contains("forbids caller-owned route narrowing"));
        }
        _ => panic!("forbidden intent should deny explicit route intent"),
    }
}

#[test]
fn route_planning_rejects_mismatched_admitted_world_inputs() {
    let primary = admitted_handle("primary");
    let alternate = admitted_handle("alternate");
    let primary_progressed = progressed(
        &primary,
        RouteInput::<RelationalRouteFamily>::new("edge:42"),
    );
    let alternate_evidence = alternate
        .describe_foundational(
            crate::application::WorthQueryDeclarationFoundationalEvidenceInput::admitted_progression(progressed(
                &alternate,
                RouteInput::<RelationalRouteFamily>::new("edge:42"),
            )),
        )
        .unwrap_or_else(|_| panic!("alternate foundational evidence should materialize"));

    match primary.plan_routes_checked(WorthQueryDeclarationRoutePlanInput::admitted(
        primary_progressed,
        alternate_evidence,
    )) {
        WorthQueryDeclarationRoutePlanChecked::Denied(denial) => {
            assert_eq!(
                denial.cause(),
                WorthQueryDeclarationRoutePlanDenialCause::WrongAdmittedWorld
            );
        }
        _ => panic!("mismatched admitted worlds should deny route planning"),
    }
}

#[test]
fn deferred_and_failed_paths_remain_typed() {
    let handle = admitted_handle("primary");

    assert!(matches!(
        handle.plan_routes_checked(route_input(
            &handle,
            RouteInput::<DeferredRouteFamily>::new("edge:42"),
        )),
        WorthQueryDeclarationRoutePlanChecked::Deferred(_)
    ));

    assert!(matches!(
        handle.plan_routes_checked(route_input(
            &handle,
            RouteInput::<FailedRouteFamily>::new("edge:42"),
        )),
        WorthQueryDeclarationRoutePlanChecked::Failed(_)
    ));
}
