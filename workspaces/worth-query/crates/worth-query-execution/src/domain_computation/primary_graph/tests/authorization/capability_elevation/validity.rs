use super::super::super::application_attempt::authenticated_principal;
use super::super::super::fixture::{
    installed_elevated_capability_world, live_scope, CapabilityElevationScenario,
    ElevatedCapabilityTouchOperation,
};
use super::super::capability_progression::time;
use super::admit;
use crate::domain_computation::primary_graph::WorthQueryOperationAuthorizationDenialKind;

#[test]
fn explicitly_expired_elevation_has_a_typed_denial() {
    let mut world = installed_elevated_capability_world(CapabilityElevationScenario::Expired);
    world.application.script_authorization_time([time(100)]);
    let request = live_scope();
    let principal = authenticated_principal(&world, &request);

    let Err(denial) = admit(&world, &principal, &request, Some("elevation-1")) else {
        panic!("explicit expired posture must not mint active-use authority");
    };

    assert_eq!(
        denial.kind(),
        WorthQueryOperationAuthorizationDenialKind::ElevationExpired
    );
}

#[test]
fn trusted_time_denies_active_status_past_the_elevation_expiry() {
    let mut world = installed_elevated_capability_world(CapabilityElevationScenario::Active);
    world.application.script_authorization_time([time(108)]);
    let request = live_scope();
    let principal = authenticated_principal(&world, &request);

    let Err(denial) = admit(&world, &principal, &request, Some("elevation-1")) else {
        panic!("trusted time must terminate active-use authority at elevation expiry");
    };

    assert_eq!(
        denial.kind(),
        WorthQueryOperationAuthorizationDenialKind::ElevationExpired
    );
}

#[test]
fn active_status_before_elevation_issue_is_inactive_not_expired() {
    let mut world = installed_elevated_capability_world(CapabilityElevationScenario::Active);
    world.application.script_authorization_time([time(92)]);
    let request = live_scope();
    let principal = authenticated_principal(&world, &request);

    let Err(denial) = admit(&world, &principal, &request, Some("elevation-1")) else {
        panic!("an elevation cannot open authority before its installed issue boundary");
    };

    assert_eq!(
        denial.kind(),
        WorthQueryOperationAuthorizationDenialKind::ElevationInactive
    );
}

#[test]
fn expiry_between_access_and_operation_denies_fresh_progression() {
    let mut world = installed_elevated_capability_world(CapabilityElevationScenario::Active);
    world
        .application
        .script_authorization_time([time(100), time(108)]);
    let request = live_scope();
    let principal = authenticated_principal(&world, &request);
    let access = admit(&world, &principal, &request, Some("elevation-1")).unwrap();
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
