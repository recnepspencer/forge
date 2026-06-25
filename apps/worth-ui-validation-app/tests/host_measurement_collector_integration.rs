use worth_ui::facade::{
    WorthUiHostMeasurementReadinessPosture, WorthUiHostObservationAdmissionDenialCode,
    WorthUiMountedCompositionTreeReceipt, WorthUiRuntimeFactFamily,
};
use worth_ui_validation_app::{
    collect_live_view_host_observations_from_input, reload::ValidationLiveViewSource,
    ValidationHostFrameObservationOutcome, ValidationHostFrameObservationUnavailable,
    ValidationHostObservationInput, ValidationWorkbenchApp, ValidationWorkbenchAuthoredInputs,
    ValidationWorkbenchLaunch,
};

#[path = "support/live_view_product_fixtures.rs"]
#[allow(dead_code)]
mod live_view_product_fixtures;

#[test]
fn collector_admits_measures_and_counts_host_observation_families() {
    let app = prepared_app();
    let proof = app
        .live_view_projection_proof()
        .expect("projection admits through runtime");
    let node_id = first_mounted_node_id(proof.mounted_product_view().composition_tree());

    let measurement = collect_live_view_host_observations_from_input(
        app.workbench().runtime(),
        Ok(&proof),
        ValidationHostObservationInput::new(640.0, 420.0, 12)
            .with_dpi_scale(1.25)
            .with_scroll_viewport(node_id.clone(), 0.0, 12.0, 620.0, 380.0)
            .with_text_metric(node_id.clone(), 17, 120.0, 24.0, 18.0)
            .with_icon_metric(node_id, 29, 20.0, 20.0, 14.0)
            .with_elapsed_time("animation", 16_000),
    );

    let admitted = measurement
        .admitted()
        .expect("collector should admit valid host observations");
    assert_eq!(
        admitted.readiness(),
        WorthUiHostMeasurementReadinessPosture::Ready
    );
    assert_eq!(admitted.counters().available_bounds_count(), 1);
    assert_eq!(admitted.counters().viewport_count(), 1);
    assert_eq!(admitted.counters().scroll_viewport_count(), 1);
    assert_eq!(admitted.counters().text_metric_count(), 1);
    assert_eq!(admitted.counters().icon_metric_count(), 1);
    assert_eq!(admitted.counters().dpi_count(), 1);
    assert_eq!(admitted.counters().elapsed_time_count(), 1);
    assert_eq!(admitted.counters().source_reparse_count(), 0);
    assert_eq!(admitted.counters().renderer_parse_count(), 0);
    assert!(measurement.measured_product_view().is_some());
    assert!(measurement.measurement_denial().is_none());
    assert!(admitted
        .consumed_facts()
        .iter()
        .any(|fact| fact.family() == WorthUiRuntimeFactFamily::HostMeasurementObservation));
}

#[test]
fn collector_propagates_denials_instead_of_dropping_them() {
    let app = prepared_app();
    let proof = app
        .live_view_projection_proof()
        .expect("projection admits through runtime");

    let measurement = collect_live_view_host_observations_from_input(
        app.workbench().runtime(),
        Ok(&proof),
        ValidationHostObservationInput::new(f32::NAN, 420.0, 13),
    );

    let ValidationHostFrameObservationOutcome::Denied(denials) = measurement.outcome() else {
        panic!("invalid host observation must be reported as a typed denial");
    };
    assert!(denials.iter().any(|denial| {
        denial.code() == WorthUiHostObservationAdmissionDenialCode::InvalidMetricBasis
    }));
    assert!(measurement.measured_product_view().is_none());
}

#[test]
fn collector_reports_projection_unavailable_as_typed_posture() {
    let app = prepared_app();
    let measurement = collect_live_view_host_observations_from_input(
        app.workbench().runtime(),
        Err("projection denied"),
        ValidationHostObservationInput::new(640.0, 420.0, 14),
    );

    assert_eq!(
        measurement.outcome(),
        &ValidationHostFrameObservationOutcome::Unavailable(
            ValidationHostFrameObservationUnavailable::ProjectionUnavailable
        )
    );
}

fn prepared_app() -> ValidationWorkbenchApp {
    let launch = ValidationWorkbenchLaunch::new()
        .prepare_from_authored_inputs(
            ValidationWorkbenchAuthoredInputs::sample().with_live_view_source(
                ValidationLiveViewSource::new(live_view_product_fixtures::contact_form_source()),
            ),
        )
        .expect("validation app should prepare");
    ValidationWorkbenchApp::new(launch)
}

fn first_mounted_node_id(tree: &WorthUiMountedCompositionTreeReceipt) -> String {
    tree.root_children()
        .first()
        .expect("mounted tree has a root child")
        .node_id()
        .to_owned()
}
