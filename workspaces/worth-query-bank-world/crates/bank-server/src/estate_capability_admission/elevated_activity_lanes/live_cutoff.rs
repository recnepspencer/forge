use worth_query_host::facade::primary_graph::{
    WorthQueryApplicationLiveCloseOutcome, WorthQueryApplicationLiveControls,
    WorthQueryOperationAuthorizationDenialKind,
};

use super::support::{
    activity_request, activity_world, approve_first, assert_exact_revoked_alternate_active,
    assert_resources_released, controls, revoke_exact_support, take_first_requested,
};
use crate::BankEstateEmergencyAccessActivityLiveOutcome;

#[test]
fn queued_real_lifecycle_cause_is_cut_off_before_live_payload_and_closes() {
    let mut world = activity_world("estate-emergency-activity-live-cutoff");
    let live_controls = WorthQueryApplicationLiveControls::bounded(
        super::super::fixture::request_scope(),
        4,
        8,
        2_048,
    )
    .unwrap();
    let first_requested = take_first_requested(&mut world);
    let mut live = world
        .fixture
        .runtime
        .query(activity_request())
        .as_principal(&world.requester)
        .controls(controls(8))
        .subscribe_with_approved_elevation(&world.approved, live_controls)
        .expect("the exact approved elevation should open the live activity lane");

    approve_first(&world, first_requested);
    revoke_exact_support(&world, 153);
    let BankEstateEmergencyAccessActivityLiveOutcome::AuthorizationDenied(denial) = live.poll()
    else {
        panic!("revoked live authority must terminate with its exact denial");
    };
    assert_eq!(
        denial.kind(),
        WorthQueryOperationAuthorizationDenialKind::StaleAuthorization
    );
    assert!(denial.identity().is_some());
    assert!(matches!(
        live.poll(),
        BankEstateEmergencyAccessActivityLiveOutcome::Closed
    ));
    assert_eq!(live.buffered_cause_count(), 0);
    let WorthQueryApplicationLiveCloseOutcome::Completed(completion) = live.close() else {
        panic!("the denied live lane must release its opening graph-read session");
    };
    assert_eq!(completion.release().released_reservation_count(), 1);
    assert_exact_revoked_alternate_active(&world);
    assert_resources_released(&world);
}
