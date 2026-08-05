use super::super::super::application_attempt::authenticated_principal;
use super::super::super::fixture::{CapabilityElevationStatus, ElevatedCapabilityTouchOperation};
use super::super::capability_progression::time;
use super::admit;
use crate::domain_computation::primary_graph::{
    WorthQueryApplicationAuthorizationExplanationCause, WorthQueryOperationAuthorizationDenialKind,
};

#[test]
fn explicitly_expired_elevation_has_a_typed_denial() {
    let (mut world, request, approved) = super::approval_transition::exact_approved_world();
    world.application.script_authorization_time([time(100)]);
    let principal = authenticated_principal(&world, &request);
    super::mutation::set_status(&world, "elevation-2", CapabilityElevationStatus::Expired);

    let Err(denial) = admit(&world, &approved, &principal, &request, Some("elevation-2")) else {
        panic!("explicit expired posture must not mint active-use authority");
    };

    assert_eq!(
        denial.kind(),
        WorthQueryOperationAuthorizationDenialKind::ElevationExpired
    );
    assert_eq!(
        denial.explanation_cause(),
        Some(WorthQueryApplicationAuthorizationExplanationCause::ElevationExpired)
    );
}

#[test]
fn trusted_time_denies_active_status_past_the_elevation_expiry() {
    let (mut world, request, approved) = super::approval_transition::exact_approved_world();
    world.application.script_authorization_time([time(106)]);
    let principal = authenticated_principal(&world, &request);

    let Err(denial) = admit(&world, &approved, &principal, &request, Some("elevation-2")) else {
        panic!("trusted time must terminate active-use authority at elevation expiry");
    };

    assert_eq!(
        denial.kind(),
        WorthQueryOperationAuthorizationDenialKind::ElevationExpired
    );
}

#[test]
fn active_status_before_elevation_issue_is_inactive_not_expired() {
    let (mut world, request, approved) = super::approval_transition::exact_approved_world();
    world.application.script_authorization_time([time(99)]);
    let principal = authenticated_principal(&world, &request);

    let Err(denial) = admit(&world, &approved, &principal, &request, Some("elevation-2")) else {
        panic!("an elevation cannot open authority before its installed issue boundary");
    };

    assert_eq!(
        denial.kind(),
        WorthQueryOperationAuthorizationDenialKind::ElevationInactive
    );
}

#[test]
fn expiry_between_access_and_operation_denies_fresh_progression() {
    let (mut world, request, approved) = super::approval_transition::exact_approved_world();
    world
        .application
        .script_authorization_time([time(100), time(106)]);
    let principal = authenticated_principal(&world, &request);
    let access = admit(&world, &approved, &principal, &request, Some("elevation-2")).unwrap();
    let operation = world
        .application
        .installed_schema()
        .installed_operation(ElevatedCapabilityTouchOperation::reference())
        .unwrap();

    let Err(denial) =
        world
            .application
            .authorize_capability_operation(access, &operation, Default::default())
    else {
        panic!("an access proof must not progress after the exact elevation expires");
    };

    assert_eq!(
        denial.kind(),
        WorthQueryOperationAuthorizationDenialKind::ElevationExpired
    );
}
