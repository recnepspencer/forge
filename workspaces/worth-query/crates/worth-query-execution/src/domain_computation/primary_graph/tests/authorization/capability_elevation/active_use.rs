use super::super::super::application_attempt::authenticated_principal;
use super::super::super::fixture::CapabilityAction;
use super::super::capability_progression::time;
use crate::domain_computation::primary_graph::WorthQueryOperationAuthorizationDenialKind;

#[test]
fn approved_receipt_rejects_another_elevation_and_a_narrowed_amount() {
    let (mut world, request, approved) = super::approval_transition::exact_approved_world();
    world
        .application
        .script_authorization_time([time(100), time(100)]);
    let principal = authenticated_principal(&world, &request);
    let capability = super::installed_capability(&world);
    let wrong_elevation = super::elevated_input(Some("elevation-1"));
    let mut narrowed_amount = super::elevated_input(Some("elevation-2"));
    narrowed_amount.amount = 49;

    for input in [wrong_elevation, narrowed_amount] {
        let denial = world
            .application
            .admit_approved_elevation_access(&approved, &principal, &capability, input, &request)
            .err()
            .expect("active use must remain inside the exact requested upper bound");
        assert_eq!(
            denial.kind(),
            WorthQueryOperationAuthorizationDenialKind::ElevationApprovalRejected
        );
    }
}

#[test]
fn approved_receipt_from_another_runtime_cannot_open_active_use() {
    let (_source, _source_request, foreign) = super::approval_transition::exact_approved_world();
    let (mut world, request, _local) = super::approval_transition::exact_approved_world();
    world.application.script_authorization_time([time(100)]);
    let principal = authenticated_principal(&world, &request);
    let capability = super::installed_capability(&world);

    let denial = world
        .application
        .admit_approved_elevation_access(
            &foreign,
            &principal,
            &capability,
            super::elevated_input(Some("elevation-2")),
            &request,
        )
        .err()
        .expect("foreign runtime lifecycle authority must not compose locally");
    assert_eq!(
        denial.kind(),
        WorthQueryOperationAuthorizationDenialKind::ElevationApprovalRejected
    );
}

#[test]
fn approved_touch_elevation_cannot_authorize_a_disbursement_request() {
    let (mut world, request, approved) = super::approval_transition::exact_approved_world();
    world.application.script_authorization_time([time(100)]);
    let principal = authenticated_principal(&world, &request);
    let capability = super::installed_capability(&world);
    let mut disbursement = super::elevated_input(Some("elevation-2"));
    disbursement.action = CapabilityAction::Disburse;

    let denial = world
        .application
        .admit_approved_elevation_access(&approved, &principal, &capability, disbursement, &request)
        .err()
        .expect("the touch upper bound must reject a disbursement before operation authority");

    assert_eq!(
        denial.kind(),
        WorthQueryOperationAuthorizationDenialKind::CapabilityProjectionRejected
    );
}
