use super::super::super::application_attempt::{authenticated_principal, idempotency};
use super::super::super::fixture::{
    installed_elevated_capability_world, live_scope, revoke_current_capability,
    CapabilityElevationScenario,
};
use super::super::capability_progression::time;
use crate::domain_computation::primary_graph::{
    WorthQueryApplicationCommitDenialKind, WorthQueryApplicationCommitDenialStage,
    WorthQueryElevationRequestOutcome,
};

#[test]
fn governed_upper_bound_support_is_revalidated_at_request_commit() {
    let mut world = installed_elevated_capability_world(CapabilityElevationScenario::Active);
    world
        .application
        .script_authorization_time(std::iter::repeat_n(time(100), 8));
    let request = live_scope();
    let principal = authenticated_principal(&world, &request);
    let program = super::request_transition::request_reads(
        &world,
        &principal,
        &request,
        super::request_transition::honest_input(),
    )
    .materialize_elevation_request_program()
    .unwrap();

    revoke_current_capability(&world);

    let WorthQueryElevationRequestOutcome::Denied(denial) = world
        .application
        .compare_and_commit_elevation_request(program, idempotency(74, 74))
    else {
        panic!("revoked upper-bound support must deny before lifecycle effects");
    };
    assert_eq!(
        denial.kind(),
        WorthQueryApplicationCommitDenialKind::ProviderRejected
    );
    assert_eq!(
        denial.stage(),
        WorthQueryApplicationCommitDenialStage::DecisionReadSet
    );
}
