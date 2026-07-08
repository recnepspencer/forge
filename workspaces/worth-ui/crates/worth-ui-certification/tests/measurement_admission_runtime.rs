#[path = "fixtures/obligation_dispatch_prerequisite_support/mod.rs"]
mod obligation_dispatch_prerequisite_support;

use worth_ui::facade::admission::{
    UiAdmissionFamily, UiAdmissionTarget, UiAdmissionWorld, UiMeasurementAdmissionPosture,
    UiMeasurementCapabilityGateReason, UiMeasurementUnsupportedReason, UiSupportPosture,
    UiSupportReason,
};
use worth_ui::facade::graph::{
    ForgeQuerySessionLabel, UiGraphTouchAspectPosture, UiGraphTouchAspects, UiGraphTouchTiming,
    UiGraphWorldProfile,
};
use worth_ui::facade::obligations::{UiObligationDispatchStopPosture, UiObligationFamily};
use worth_ui_host_contract::{
    WorthUiHostCapabilityObservationGeneration, WorthUiHostCapabilityReport, WorthUiHostContract,
};
use worth_ui_test_support::{
    runtime_origin_fixture, WorthUiTouchOriginFixtureVariant,
};

use self::obligation_dispatch_prerequisite_support::{
    available_host_capability_target, focus_touch_app, missing_host_capability_target,
};

#[test]
fn identical_measurement_world_and_capability_profiles_admit_identically() {
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
    let target = available_measurement_target(&touch);

    let first_selected = fixture
        .app
        .admission()
        .select_obligations_for_target(&touch, target.clone());
    let second_selected = fixture
        .app
        .admission()
        .select_obligations_for_target(&touch, target);
    let first = fixture
        .app
        .admission()
        .admit_measurement_requirement(&first_selected)
        .expect("measurement touch should produce measurement admission");
    let second = fixture
        .app
        .admission()
        .admit_measurement_requirement(&second_selected)
        .expect("measurement touch should produce measurement admission");
    let dispatch = fixture
        .app
        .admission()
        .lower_obligation_dispatch(&first_selected);
    let report = fixture
        .app
        .admission()
        .admit_selected_obligations(&first_selected);

    assert_eq!(
        first_selected
            .obligation_for_family(UiObligationFamily::MeasurementRequirement)
            .map(|obligation| obligation.family()),
        Some(UiObligationFamily::MeasurementRequirement)
    );
    assert_eq!(first, second);
    assert!(matches!(
        first.posture(),
        UiMeasurementAdmissionPosture::Admitted { .. }
    ));
    assert_eq!(dispatch.measurement_admission(), Some(&first));
    assert_eq!(report.measurement_admission(), Some(&first));
    assert_eq!(
        dispatch.plan_stop_posture(),
        UiObligationDispatchStopPosture::Deferred
    );
    assert!(first.declaration_identity().is_some());
    assert!(first
        .selected_measurement_obligation_identity_digest()
        .is_some());
    assert!(first.host_capability_profile_digest().is_some());
    assert_eq!(
        first.host_capability_observation_generation(),
        Some(WorthUiHostCapabilityObservationGeneration::new(0))
    );
}

#[test]
fn measurement_admission_keeps_wrong_world_and_capability_gated_denials_distinct() {
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

    let wrong_world_target = UiAdmissionTarget::graph_node(
        touch.target().graph_node_identity(),
        UiAdmissionWorld::from_graph_world_profile(UiGraphWorldProfile::preview_session_label(
            ForgeQuerySessionLabel::scoped_strs("worth-ui", ["measurement", "preview"])
                .expect("preview label should admit"),
        )),
    )
    .with_host_capability_report(WorthUiHostCapabilityReport::from_contract(
        WorthUiHostContract::egui(),
    ));
    let wrong_world_selected = fixture
        .app
        .admission()
        .select_obligations_for_target(&touch, wrong_world_target);
    let wrong_world = fixture
        .app
        .admission()
        .admit_measurement_requirement(&wrong_world_selected)
        .expect("measurement lane should admit into typed wrong-world denial");
    let wrong_world_dispatch = fixture
        .app
        .admission()
        .lower_obligation_dispatch(&wrong_world_selected);
    let wrong_world_report = fixture
        .app
        .admission()
        .admit_selected_obligations(&wrong_world_selected);

    assert!(matches!(
        wrong_world.posture(),
        UiMeasurementAdmissionPosture::WrongWorld { .. }
    ));
    assert_eq!(
        wrong_world_dispatch.measurement_admission(),
        Some(&wrong_world)
    );
    assert_eq!(
        wrong_world_report.measurement_admission(),
        Some(&wrong_world)
    );
    assert_eq!(
        wrong_world_dispatch.plan_stop_posture(),
        UiObligationDispatchStopPosture::WrongWorld
    );
    assert_eq!(
        wrong_world_dispatch.support_snapshot().posture(),
        &UiSupportPosture::WrongWorld {
            family: UiAdmissionFamily::MeasurementRequirement,
            expected: UiAdmissionWorld::from_graph_world_profile(
                touch.world().world_profile().clone(),
            ),
            observed: UiAdmissionWorld::from_graph_world_profile(
                UiGraphWorldProfile::preview_session_label(
                    ForgeQuerySessionLabel::scoped_strs("worth-ui", ["measurement", "preview"],)
                        .expect("preview label should admit"),
                ),
            ),
        }
    );
    assert_eq!(
        wrong_world_report.support_snapshot().posture(),
        wrong_world_dispatch.support_snapshot().posture()
    );

    let gated_selected = fixture
        .app
        .admission()
        .select_obligations_for_target(&touch, missing_host_capability_target(&touch));
    let gated = fixture
        .app
        .admission()
        .admit_measurement_requirement(&gated_selected)
        .expect("measurement lane should admit into typed capability denial");
    let gated_dispatch = fixture
        .app
        .admission()
        .lower_obligation_dispatch(&gated_selected);
    let gated_report = fixture
        .app
        .admission()
        .admit_selected_obligations(&gated_selected);

    assert_eq!(
        gated.posture(),
        &UiMeasurementAdmissionPosture::CapabilityGated {
            world: UiAdmissionWorld::from_graph_world_profile(
                touch.world().world_profile().clone(),
            ),
            reason: UiMeasurementCapabilityGateReason::MissingHostCapability,
        }
    );
    assert_eq!(gated_dispatch.measurement_admission(), Some(&gated));
    assert_eq!(gated_report.measurement_admission(), Some(&gated));
    assert_eq!(
        gated_dispatch.plan_stop_posture(),
        UiObligationDispatchStopPosture::WrongHostCapability {
            required: worth_ui::facade::admission::UiAdmissionHostCapability::Available,
            observed: worth_ui::facade::admission::UiAdmissionHostCapability::Missing,
        }
    );
    assert_eq!(
        gated_dispatch.support_snapshot().posture(),
        &UiSupportPosture::Unsupported {
            family: UiAdmissionFamily::MeasurementRequirement,
            reason: UiSupportReason::MissingDeclarationSupportEvidence,
            world: UiAdmissionWorld::from_graph_world_profile(
                touch.world().world_profile().clone(),
            ),
        }
    );
    assert_eq!(
        gated_report.support_snapshot().posture(),
        gated_dispatch.support_snapshot().posture()
    );
}

#[test]
fn unsupported_measurement_touch_without_selected_requirement_denies_structurally() {
    let app = focus_touch_app();
    let artifact = obligation_dispatch_prerequisite_support::artifact_from_module_path(
        &app,
        "app/obligation_dispatch_focus_runtime.wui",
    );
    let touch = app
        .graph()
        .touches()
        .from_node(
            app.graph()
                .touches()
                .declaration_change_receipt(artifact)
                .expect("declaration change should admit"),
            UiGraphTouchTiming::PostMutation,
            obligation_dispatch_prerequisite_support::graph_node_identity(&app, artifact),
            UiGraphTouchAspects::new().measurement(UiGraphTouchAspectPosture::Read),
        )
        .expect("measurement touch should admit");
    let selected = app
        .admission()
        .select_obligations_for_target(&touch, available_host_capability_target(&touch));
    let admission = app
        .admission()
        .admit_measurement_requirement(&selected)
        .expect("measurement lane should produce typed unsupported denial");
    let dispatch = app.admission().lower_obligation_dispatch(&selected);
    let report = app.admission().admit_selected_obligations(&selected);

    assert!(selected
        .obligation_for_family(UiObligationFamily::MeasurementRequirement)
        .is_none());
    assert_eq!(
        admission.posture(),
        &UiMeasurementAdmissionPosture::Unsupported {
            world: UiAdmissionWorld::from_graph_world_profile(
                touch.world().world_profile().clone(),
            ),
            reason: UiMeasurementUnsupportedReason::SelectionDidNotYieldMeasurementRequirement,
        }
    );
    assert_eq!(dispatch.measurement_admission(), Some(&admission));
    assert_eq!(report.measurement_admission(), Some(&admission));
    assert_eq!(
        dispatch.plan_stop_posture(),
        UiObligationDispatchStopPosture::Unsupported
    );
    assert_eq!(
        dispatch.support_snapshot().posture(),
        &UiSupportPosture::Unsupported {
            family: UiAdmissionFamily::MeasurementRequirement,
            reason: UiSupportReason::MissingDeclarationSupportEvidence,
            world: UiAdmissionWorld::from_graph_world_profile(
                touch.world().world_profile().clone(),
            ),
        }
    );
    assert_eq!(
        report.support_snapshot().posture(),
        dispatch.support_snapshot().posture()
    );
}

#[test]
fn measurement_admission_rejects_stale_selected_support_authority() {
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
        UiAdmissionTarget::graph_node(
            fixture.region_graph_node_identity(),
            UiAdmissionWorld::from_graph_world_profile(touch.world().world_profile().clone()),
        )
        .with_host_capability_report(WorthUiHostCapabilityReport::from_contract(
            WorthUiHostContract::egui(),
        )),
    );
    let stale = fixture
        .app
        .admission()
        .admit_measurement_requirement(&selected)
        .expect("measurement lane should produce typed stale denial");
    let stale_dispatch = fixture.app.admission().lower_obligation_dispatch(&selected);
    let stale_report = fixture
        .app
        .admission()
        .admit_selected_obligations(&selected);

    assert!(
        matches!(
            stale.posture(),
            UiMeasurementAdmissionPosture::StaleSupportPosture { .. }
        ),
        "expected stale support posture, got {:?}",
        stale.posture()
    );
    assert_eq!(stale_dispatch.measurement_admission(), Some(&stale));
    assert_eq!(stale_report.measurement_admission(), Some(&stale));
    assert!(matches!(
        stale_dispatch.plan_stop_posture(),
        UiObligationDispatchStopPosture::Stale { .. }
    ));
    assert_eq!(
        stale_dispatch.support_snapshot().posture(),
        &UiSupportPosture::Unsupported {
            family: UiAdmissionFamily::MeasurementRequirement,
            reason: UiSupportReason::MissingDeclarationSupportEvidence,
            world: UiAdmissionWorld::from_graph_world_profile(
                touch.world().world_profile().clone(),
            ),
        }
    );
    assert_eq!(
        stale_report.support_snapshot().posture(),
        stale_dispatch.support_snapshot().posture()
    );
}

#[test]
fn measurement_admission_retains_host_observation_generation_even_for_same_profile() {
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
    let first_target = UiAdmissionTarget::graph_node(
        touch.target().graph_node_identity(),
        UiAdmissionWorld::from_graph_world_profile(touch.world().world_profile().clone()),
    )
    .with_host_capability_report(
        WorthUiHostCapabilityReport::from_contract(WorthUiHostContract::egui())
            .with_observation_generation(WorthUiHostCapabilityObservationGeneration::new(7)),
    );
    let second_target = UiAdmissionTarget::graph_node(
        touch.target().graph_node_identity(),
        UiAdmissionWorld::from_graph_world_profile(touch.world().world_profile().clone()),
    )
    .with_host_capability_report(
        WorthUiHostCapabilityReport::from_contract(WorthUiHostContract::egui())
            .with_observation_generation(WorthUiHostCapabilityObservationGeneration::new(11)),
    );

    let first = fixture
        .app
        .admission()
        .admit_measurement_requirement(
            &fixture
                .app
                .admission()
                .select_obligations_for_target(&touch, first_target),
        )
        .expect("measurement lane should admit");
    let second = fixture
        .app
        .admission()
        .admit_measurement_requirement(
            &fixture
                .app
                .admission()
                .select_obligations_for_target(&touch, second_target),
        )
        .expect("measurement lane should admit");

    assert_eq!(
        first.host_capability_profile_digest(),
        second.host_capability_profile_digest()
    );
    assert_ne!(
        first.host_capability_observation_generation(),
        second.host_capability_observation_generation()
    );
}

fn available_measurement_target(
    touch: &worth_ui::facade::graph::UiGraphTouchDescriptor,
) -> UiAdmissionTarget {
    UiAdmissionTarget::graph_node(
        touch.target().graph_node_identity(),
        UiAdmissionWorld::from_graph_world_profile(touch.world().world_profile().clone()),
    )
    .with_host_capability_report(WorthUiHostCapabilityReport::from_contract(
        WorthUiHostContract::egui(),
    ))
}
