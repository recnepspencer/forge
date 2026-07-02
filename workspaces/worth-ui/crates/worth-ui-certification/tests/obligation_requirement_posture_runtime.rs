#[path = "fixtures/obligation_dispatch_prerequisite_support/mod.rs"]
mod obligation_dispatch_prerequisite_support;

use worth_ui::facade::graph::{
    UiGraphTouchAspectPosture, UiGraphTouchAspects, UiGraphTouchRuntimeLane, UiGraphTouchTiming,
};
use worth_ui::facade::obligations::{
    UiObligationDispatchStopPosture, UiObligationFamily, UiObligationVerdictClass,
};
use worth_ui_host_contract::{WorthUiHostCapabilityReport, WorthUiHostContract};
use worth_ui_runtime::facade::{runtime_origin_fixture, WorthUiTouchOriginFixtureVariant};

use self::obligation_dispatch_prerequisite_support::{
    available_host_capability_target, diagnostic_only_host_capability_target, execute_for_target,
    focus_touch, focus_touch_app, motion_touch, motion_touch_app, selection_target, service_touch,
    service_touch_app,
};

#[test]
fn portal_requirement_without_host_report_fails_closed_as_unsupported() {
    let app = service_touch_app();
    let touch = service_touch(&app);
    let bundle = execute_for_target(&app, &touch, selection_target(&touch));

    assert_eq!(
        bundle.dispatch.plan_stop_posture(),
        UiObligationDispatchStopPosture::Unsupported
    );
    assert!(bundle.verdicts.iter().all(|verdict| {
        verdict.class() == UiObligationVerdictClass::Violation
            && verdict.stop_posture() == UiObligationDispatchStopPosture::Unsupported
    }));
}

#[test]
fn focus_requirement_stays_a_deferred_requirement_check_even_with_host_posture() {
    let app = focus_touch_app();
    let touch = focus_touch(&app);
    let bundle = execute_for_target(&app, &touch, available_host_capability_target(&touch));

    assert_eq!(
        bundle
            .dispatch
            .entries()
            .iter()
            .map(|entry| entry.selected().family())
            .collect::<Vec<_>>(),
        vec![
            UiObligationFamily::StructuralLegality,
            UiObligationFamily::FocusRouteRequirement,
        ]
    );
    assert_eq!(
        bundle
            .verdicts
            .iter()
            .find(|verdict| verdict.family() == Some(UiObligationFamily::FocusRouteRequirement))
            .expect("focus route verdict should exist")
            .stop_posture(),
        UiObligationDispatchStopPosture::Deferred
    );
}

#[test]
fn motion_requirement_stays_a_deferred_requirement_check_even_with_host_posture() {
    let app = motion_touch_app();
    let touch = motion_touch(&app);
    let bundle = execute_for_target(&app, &touch, available_host_capability_target(&touch));

    assert_eq!(
        bundle
            .dispatch
            .entries()
            .iter()
            .map(|entry| entry.selected().family())
            .collect::<Vec<_>>(),
        vec![
            UiObligationFamily::StructuralLegality,
            UiObligationFamily::MotionSupportRequirement,
        ]
    );
    assert_eq!(
        bundle
            .verdicts
            .iter()
            .find(|verdict| verdict.family() == Some(UiObligationFamily::MotionSupportRequirement))
            .expect("motion support verdict should exist")
            .stop_posture(),
        UiObligationDispatchStopPosture::Deferred
    );
}

#[test]
fn portal_requirement_can_lower_to_diagnostic_only_without_runtime_execution() {
    let app = service_touch_app();
    let touch = service_touch(&app);
    let bundle = execute_for_target(&app, &touch, diagnostic_only_host_capability_target(&touch));

    assert_eq!(
        bundle
            .dispatch
            .entries()
            .iter()
            .map(|entry| entry.selected().family())
            .collect::<Vec<_>>(),
        vec![
            UiObligationFamily::StructuralLegality,
            UiObligationFamily::PortalHostRequirement,
        ]
    );
    assert_eq!(
        bundle.dispatch.plan_stop_posture(),
        UiObligationDispatchStopPosture::DiagnosticOnly
    );
    assert_eq!(
        bundle
            .verdicts
            .iter()
            .map(|verdict| (verdict.family(), verdict.class(), verdict.stop_posture()))
            .collect::<Vec<_>>(),
        vec![
            (
                Some(UiObligationFamily::StructuralLegality),
                UiObligationVerdictClass::Advisory,
                UiObligationDispatchStopPosture::DiagnosticOnly,
            ),
            (
                Some(UiObligationFamily::PortalHostRequirement),
                UiObligationVerdictClass::Advisory,
                UiObligationDispatchStopPosture::DiagnosticOnly,
            ),
        ]
    );
}

#[test]
fn measurement_requirement_remains_prerequisite_only_under_host_observation() {
    let fixture = runtime_origin_fixture(WorthUiTouchOriginFixtureVariant::Baseline);
    let touch = fixture
        .app
        .graph()
        .touches()
        .from_node(
            fixture
                .app
                .graph()
                .touches()
                .host_observation_receipt(fixture.runtime.inspect_active(), &fixture.inspection)
                .expect("host observation should admit"),
            UiGraphTouchTiming::ReactiveObservation,
            fixture.control_graph_node_identity(),
            UiGraphTouchAspects::new().measurement(UiGraphTouchAspectPosture::Read),
        )
        .expect("host measurement touch should admit");

    let selected = fixture.app.admission().select_obligations_for_target(
        &touch,
        worth_ui::facade::admission::UiAdmissionTarget::graph_node(
            touch.target().graph_node_identity(),
            worth_ui::facade::admission::UiAdmissionWorld::from_graph_world_profile(
                touch.world().world_profile().clone(),
            ),
        )
        .with_host_capability_report(WorthUiHostCapabilityReport::from_contract(
            WorthUiHostContract::egui(),
        )),
    );
    let dispatch = fixture.app.admission().lower_obligation_dispatch(&selected);
    let verdicts = dispatch.execute();

    assert_eq!(
        dispatch
            .entries()
            .iter()
            .map(|entry| (
                entry.selected().family(),
                entry.selected().identity().aspect_scope().to_vec(),
            ))
            .collect::<Vec<_>>(),
        vec![(
            UiObligationFamily::MeasurementRequirement,
            vec![UiGraphTouchRuntimeLane::Measurement],
        )]
    );
    assert_eq!(
        dispatch.plan_stop_posture(),
        UiObligationDispatchStopPosture::Deferred
    );
    assert_eq!(
        verdicts
            .iter()
            .map(|verdict| (verdict.family(), verdict.class(), verdict.stop_posture()))
            .collect::<Vec<_>>(),
        vec![(
            Some(UiObligationFamily::MeasurementRequirement),
            UiObligationVerdictClass::Violation,
            UiObligationDispatchStopPosture::Deferred,
        )]
    );
}
