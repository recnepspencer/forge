#[path = "fixtures/obligation_dispatch_prerequisite_support/mod.rs"]
pub mod obligation_dispatch_prerequisite_support;

use worth_ui::facade::obligations::UiObligationDispatchStopPosture;

use self::obligation_dispatch_prerequisite_support::{
    apps::{query_touch_app, service_touch_app, structural_touch_app},
    targets::{
        ambiguous_host_capability_target, ambiguous_query_basis_target, budget_exceeded_target,
        execute_for_target, missing_host_capability_target, wrong_query_basis_target,
    },
    touches::{query_touch, service_touch, structural_touch},
};

#[test]
fn blocked_prerequisite_dispatch_stays_deterministic_for_query_basis_denials() {
    let app = query_touch_app();
    let touch = query_touch(&app);

    assert_deterministic_bundle(
        execute_for_target(&app, &touch, wrong_query_basis_target(&touch)),
        execute_for_target(&app, &touch, wrong_query_basis_target(&touch)),
        UiObligationDispatchStopPosture::WrongQueryBasis {
            required: worth_ui::facade::admission::UiAdmissionQueryBasis::GraphAligned,
            observed: worth_ui::facade::admission::UiAdmissionQueryBasis::WrongWorldProjection,
        },
    );
    assert_deterministic_bundle(
        execute_for_target(&app, &touch, ambiguous_query_basis_target(&touch)),
        execute_for_target(&app, &touch, ambiguous_query_basis_target(&touch)),
        UiObligationDispatchStopPosture::Ambiguous {
            required_query_basis: Some(
                worth_ui::facade::admission::UiAdmissionQueryBasis::GraphAligned,
            ),
            observed_query_basis: Some(
                worth_ui::facade::admission::UiAdmissionQueryBasis::AmbiguousSources,
            ),
            required_host_capability: None,
            observed_host_capability: None,
        },
    );
}

#[test]
fn blocked_prerequisite_dispatch_stays_deterministic_for_host_and_budget_denials() {
    let service_app = service_touch_app();
    let service_touch = service_touch(&service_app);
    assert_deterministic_bundle(
        execute_for_target(
            &service_app,
            &service_touch,
            missing_host_capability_target(&service_touch),
        ),
        execute_for_target(
            &service_app,
            &service_touch,
            missing_host_capability_target(&service_touch),
        ),
        UiObligationDispatchStopPosture::WrongHostCapability {
            required: worth_ui::facade::admission::UiAdmissionHostCapability::Available,
            observed: worth_ui::facade::admission::UiAdmissionHostCapability::Missing,
        },
    );
    assert_deterministic_bundle(
        execute_for_target(
            &service_app,
            &service_touch,
            ambiguous_host_capability_target(&service_touch),
        ),
        execute_for_target(
            &service_app,
            &service_touch,
            ambiguous_host_capability_target(&service_touch),
        ),
        UiObligationDispatchStopPosture::Ambiguous {
            required_query_basis: None,
            observed_query_basis: None,
            required_host_capability: Some(
                worth_ui::facade::admission::UiAdmissionHostCapability::Available,
            ),
            observed_host_capability: Some(
                worth_ui::facade::admission::UiAdmissionHostCapability::Ambiguous,
            ),
        },
    );

    let structural_app = structural_touch_app();
    let structural_touch = structural_touch(&structural_app);
    assert_deterministic_bundle(
        execute_for_target(
            &structural_app,
            &structural_touch,
            budget_exceeded_target(&structural_touch),
        ),
        execute_for_target(
            &structural_app,
            &structural_touch,
            budget_exceeded_target(&structural_touch),
        ),
        UiObligationDispatchStopPosture::BudgetExceeded {
            budget: worth_ui::facade::admission::UiAdmissionSelectionBudget::ordinary_lane_budget(
                0,
            ),
            attempted_lane_cost: 1,
        },
    );
}

fn assert_deterministic_bundle(
    left: obligation_dispatch_prerequisite_support::targets::DispatchExecutionBundle,
    right: obligation_dispatch_prerequisite_support::targets::DispatchExecutionBundle,
    expected_stop: UiObligationDispatchStopPosture,
) {
    assert_eq!(left.selected, right.selected);
    assert_eq!(left.dispatch, right.dispatch);
    assert_eq!(left.dispatch.shape_digest(), right.dispatch.shape_digest());
    assert_eq!(left.verdicts, right.verdicts);
    assert_eq!(left.dispatch.plan_stop_posture(), expected_stop);
}
