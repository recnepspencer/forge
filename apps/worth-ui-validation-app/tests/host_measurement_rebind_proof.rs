use worth_ui::facade::{WorthUiProjectionFamily, WorthUiRuntimeFactFamily};
use worth_ui_validation_app::{
    collect_live_view_host_observations_from_input, reload::ValidationLiveViewSource,
    ValidationHostObservationInput, ValidationWorkbenchApp, ValidationWorkbenchAuthoredInputs,
    ValidationWorkbenchLaunch,
};

#[path = "support/live_view_product_fixtures.rs"]
#[allow(dead_code)]
mod live_view_product_fixtures;

#[test]
fn host_measurement_rebind_names_changed_facts_and_graph_consumers() {
    let app = prepared_app();
    let proof = app
        .live_view_projection_proof()
        .expect("projection admits through runtime");
    let node_id = proof
        .mounted_product_view()
        .composition_tree()
        .root_children()
        .first()
        .expect("mounted tree has a root child")
        .node_id()
        .to_owned();

    let first = collect_live_view_host_observations_from_input(
        app.workbench().runtime(),
        Ok(&proof),
        ValidationHostObservationInput::new(640.0, 420.0, 20).with_text_metric(
            node_id.clone(),
            17,
            120.0,
            24.0,
            18.0,
        ),
    );
    let second = collect_live_view_host_observations_from_input(
        app.workbench().runtime(),
        Ok(&proof),
        ValidationHostObservationInput::new(640.0, 420.0, 20)
            .with_text_metric(node_id, 17, 144.0, 24.0, 18.0),
    );

    let rebind = app
        .workbench()
        .runtime()
        .rebind_host_measurement_observations(
            first.admitted().expect("first observation admits"),
            second.admitted().expect("second observation admits"),
        );

    assert!(rebind
        .changed_facts()
        .iter()
        .any(|fact| fact.family() == WorthUiRuntimeFactFamily::HostMeasurementObservation));
    assert!(rebind
        .consuming_projection_families()
        .contains(&WorthUiProjectionFamily::PageHost));
    assert_eq!(rebind.counters().source_reparse_count(), 0);
    assert_eq!(rebind.counters().artifact_scan_count(), 0);
    assert_eq!(rebind.counters().renderer_parse_count(), 0);
    assert_eq!(
        rebind.counters().changed_fact_count(),
        rebind.changed_facts().len()
    );
    assert_eq!(
        rebind.counters().consuming_projection_count(),
        rebind.consuming_projection_families().len()
    );
}

#[test]
fn equivalent_host_measurements_preserve_measurement_facts() {
    let app = prepared_app();
    let proof = app
        .live_view_projection_proof()
        .expect("projection admits through runtime");
    let node_id = proof
        .mounted_product_view()
        .composition_tree()
        .root_children()
        .first()
        .expect("mounted tree has a root child")
        .node_id()
        .to_owned();

    let input = || {
        ValidationHostObservationInput::new(640.0, 420.0, 21).with_text_metric(
            node_id.clone(),
            17,
            120.0,
            24.0,
            18.0,
        )
    };
    let first = collect_live_view_host_observations_from_input(
        app.workbench().runtime(),
        Ok(&proof),
        input(),
    );
    let second = collect_live_view_host_observations_from_input(
        app.workbench().runtime(),
        Ok(&proof),
        input(),
    );

    let rebind = app
        .workbench()
        .runtime()
        .rebind_host_measurement_observations(
            first.admitted().expect("first observation admits"),
            second.admitted().expect("second observation admits"),
        );

    assert!(rebind.changed_facts().is_empty());
    assert!(!rebind.preserved_facts().is_empty());
    assert_eq!(rebind.counters().consuming_projection_count(), 0);
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
