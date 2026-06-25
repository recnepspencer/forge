use worth_ui::facade::{
    WorthUiHostFrameObservationDraft, WorthUiHostMeasurementReadinessPosture,
    WorthUiHostObservationAdmissionDenialCode, WorthUiMountedCompositionTreeReceipt,
    WorthUiRuntimeFactFamily,
};
use worth_ui_validation_app::{
    reload::ValidationLiveViewSource, ValidationWorkbenchAuthoredInputs,
};
use worth_ui_validation_app::{ValidationWorkbenchApp, ValidationWorkbenchLaunch};

#[path = "support/live_view_product_fixtures.rs"]
#[allow(dead_code)]
mod live_view_product_fixtures;

#[test]
fn identical_host_observations_are_replay_equivalent() {
    let app = prepared_app();
    let proof = app
        .live_view_projection_proof()
        .expect("projection admits through runtime");
    let mounted = proof.mounted_product_view();
    let node_id = first_mounted_node_id(mounted.composition_tree());
    let runtime = app.workbench().runtime();

    let first = runtime
        .admit_host_frame_observations(
            mounted,
            WorthUiHostFrameObservationDraft::for_mounted_product_view(mounted.receipt_digest(), 7)
                .observe_available_bounds(node_id.clone(), 640.0, 420.0)
                .observe_viewport(node_id, 640.0, 420.0)
                .observe_dpi(1.25),
        )
        .expect("host observations admit");
    let second = runtime
        .admit_host_frame_observations(
            mounted,
            WorthUiHostFrameObservationDraft::for_mounted_product_view(mounted.receipt_digest(), 7)
                .observe_available_bounds(
                    first_mounted_node_id(mounted.composition_tree()),
                    640.0,
                    420.0,
                )
                .observe_viewport(
                    first_mounted_node_id(mounted.composition_tree()),
                    640.0,
                    420.0,
                )
                .observe_dpi(1.25),
        )
        .expect("same host observations admit");

    assert_eq!(first, second);
    assert_eq!(
        first.readiness(),
        WorthUiHostMeasurementReadinessPosture::Ready
    );
    assert_eq!(first.counters().source_reparse_count(), 0);
    assert_eq!(first.counters().renderer_parse_count(), 0);
    assert!(first
        .consumed_facts()
        .iter()
        .any(|fact| { fact.family() == WorthUiRuntimeFactFamily::HostMeasurementObservation }));
}

#[test]
fn reordered_host_observation_rows_are_replay_equivalent() {
    let app = prepared_app();
    let proof = app
        .live_view_projection_proof()
        .expect("projection admits through runtime");
    let mounted = proof.mounted_product_view();
    let node_id = first_mounted_node_id(mounted.composition_tree());
    let runtime = app.workbench().runtime();

    let first = runtime
        .admit_host_frame_observations(
            mounted,
            WorthUiHostFrameObservationDraft::for_mounted_product_view(mounted.receipt_digest(), 8)
                .observe_available_bounds(node_id.clone(), 640.0, 420.0)
                .observe_viewport(node_id.clone(), 640.0, 420.0)
                .observe_scroll_viewport(node_id.clone(), 0.0, 4.0, 600.0, 360.0)
                .observe_text_metric(node_id.clone(), 17, 120.0, 24.0, 18.0)
                .observe_icon_metric(node_id.clone(), 29, 20.0, 20.0, 14.0)
                .observe_elapsed_time("animation", 16000)
                .observe_dpi(1.25),
        )
        .expect("host observations admit");
    let second = runtime
        .admit_host_frame_observations(
            mounted,
            WorthUiHostFrameObservationDraft::for_mounted_product_view(mounted.receipt_digest(), 8)
                .observe_icon_metric(node_id.clone(), 29, 20.0, 20.0, 14.0)
                .observe_elapsed_time("animation", 16000)
                .observe_text_metric(node_id.clone(), 17, 120.0, 24.0, 18.0)
                .observe_scroll_viewport(node_id.clone(), 0.0, 4.0, 600.0, 360.0)
                .observe_viewport(node_id.clone(), 640.0, 420.0)
                .observe_available_bounds(node_id, 640.0, 420.0)
                .observe_dpi(1.25),
        )
        .expect("same host observations admit");

    assert_eq!(first.receipt_digest(), second.receipt_digest());
    assert_eq!(first.consumed_facts(), second.consumed_facts());
}

#[test]
fn changed_host_measurement_values_change_observation_fact_identity() {
    let app = prepared_app();
    let proof = app
        .live_view_projection_proof()
        .expect("projection admits through runtime");
    let mounted = proof.mounted_product_view();
    let node_id = first_mounted_node_id(mounted.composition_tree());
    let runtime = app.workbench().runtime();

    let first = runtime
        .admit_host_frame_observations(
            mounted,
            WorthUiHostFrameObservationDraft::for_mounted_product_view(mounted.receipt_digest(), 9)
                .observe_available_bounds(node_id.clone(), 640.0, 420.0)
                .observe_text_metric(node_id.clone(), 17, 120.0, 24.0, 18.0),
        )
        .expect("host observations admit");
    let second = runtime
        .admit_host_frame_observations(
            mounted,
            WorthUiHostFrameObservationDraft::for_mounted_product_view(mounted.receipt_digest(), 9)
                .observe_available_bounds(node_id.clone(), 640.0, 420.0)
                .observe_text_metric(node_id, 17, 144.0, 24.0, 18.0),
        )
        .expect("changed text metric admits");

    assert_ne!(first.receipt_digest(), second.receipt_digest());
    assert_ne!(first.consumed_facts(), second.consumed_facts());
}

#[test]
fn measured_product_view_rejects_observations_from_another_mounted_view() {
    let first_app = prepared_app();
    let second_app = prepared_app_with_source(
        live_view_product_fixtures::contact_form_source_with_action_before_inputs(),
    );
    let first_proof = first_app
        .live_view_projection_proof()
        .expect("first projection admits through runtime");
    let second_proof = second_app
        .live_view_projection_proof()
        .expect("second projection admits through runtime");
    let first_mounted = first_proof.mounted_product_view();
    let second_mounted = second_proof.mounted_product_view();
    assert_ne!(
        first_mounted.receipt_digest(),
        second_mounted.receipt_digest(),
        "fixture must produce a distinct mounted view basis"
    );

    let first_node_id = first_mounted
        .composition_tree()
        .root_children()
        .first()
        .expect("mounted tree has root child")
        .node_id()
        .to_owned();
    let observations = first_app
        .workbench()
        .runtime()
        .admit_host_frame_observations(
            first_mounted,
            WorthUiHostFrameObservationDraft::for_mounted_product_view(
                first_mounted.receipt_digest(),
                10,
            )
            .observe_available_bounds(first_node_id, 640.0, 420.0),
        )
        .expect("first mounted view observations admit");

    let denial = second_app
        .workbench()
        .runtime()
        .measure_mounted_product_view(second_mounted, observations)
        .expect_err("mismatched host observations must not measure another mounted view");

    assert_eq!(
        denial.code(),
        WorthUiHostObservationAdmissionDenialCode::StaleMountedProductView
    );
}

#[test]
fn stale_mounted_view_digest_rejects_host_observations() {
    let app = prepared_app();
    let proof = app
        .live_view_projection_proof()
        .expect("projection admits through runtime");
    let mounted = proof.mounted_product_view();
    let node_id = first_mounted_node_id(mounted.composition_tree());

    let denials = app
        .workbench()
        .runtime()
        .admit_host_frame_observations(
            mounted,
            WorthUiHostFrameObservationDraft::for_mounted_product_view(
                mounted.receipt_digest() + 1,
                1,
            )
            .observe_available_bounds(node_id, 640.0, 420.0),
        )
        .expect_err("stale mounted view digest must reject");

    assert_eq!(
        denials[0].code(),
        WorthUiHostObservationAdmissionDenialCode::StaleMountedProductView
    );
}

#[test]
fn invalid_host_measurement_values_reject_before_receipt_construction() {
    let app = prepared_app();
    let proof = app
        .live_view_projection_proof()
        .expect("projection admits through runtime");
    let mounted = proof.mounted_product_view();
    let node_id = first_mounted_node_id(mounted.composition_tree());

    let denials = app
        .workbench()
        .runtime()
        .admit_host_frame_observations(
            mounted,
            WorthUiHostFrameObservationDraft::for_mounted_product_view(mounted.receipt_digest(), 4)
                .observe_available_bounds(node_id.clone(), f32::NAN, 420.0)
                .observe_viewport(node_id.clone(), 640.0, -1.0)
                .observe_scroll_viewport(node_id.clone(), 0.0, f32::INFINITY, 640.0, 420.0)
                .observe_text_metric(node_id.clone(), 17, 120.0, 24.0, 30.0)
                .observe_icon_metric(node_id, 29, 20.0, 20.0, f32::NAN)
                .observe_dpi(0.0),
        )
        .expect_err("invalid metric basis must reject");

    let invalid_denial_count = denials
        .iter()
        .filter(|denial| {
            denial.code() == WorthUiHostObservationAdmissionDenialCode::InvalidMetricBasis
        })
        .count();
    assert!(
        invalid_denial_count >= 6,
        "every invalid host metric family should report a denial: {denials:?}"
    );
}

#[test]
fn unknown_host_measurement_node_rejects() {
    let app = prepared_app();
    let proof = app
        .live_view_projection_proof()
        .expect("projection admits through runtime");
    let mounted = proof.mounted_product_view();

    let denials = app
        .workbench()
        .runtime()
        .admit_host_frame_observations(
            mounted,
            WorthUiHostFrameObservationDraft::for_mounted_product_view(mounted.receipt_digest(), 2)
                .observe_available_bounds("missing.node", 640.0, 420.0),
        )
        .expect_err("unknown observed node must reject");

    assert!(denials.iter().any(|denial| {
        denial.code() == WorthUiHostObservationAdmissionDenialCode::UnknownMountedNode
    }));
}

#[test]
fn missing_available_bounds_is_typed_readiness_not_renderer_fallback() {
    let app = prepared_app();
    let proof = app
        .live_view_projection_proof()
        .expect("projection admits through runtime");
    let mounted = proof.mounted_product_view();

    let receipt = app
        .workbench()
        .runtime()
        .admit_host_frame_observations(
            mounted,
            WorthUiHostFrameObservationDraft::for_mounted_product_view(mounted.receipt_digest(), 3)
                .observe_dpi(1.0),
        )
        .expect("missing bounds remains admitted with readiness posture");

    assert_eq!(
        receipt.readiness(),
        WorthUiHostMeasurementReadinessPosture::MissingAvailableBounds
    );
}

fn prepared_app() -> ValidationWorkbenchApp {
    prepared_app_with_source(live_view_product_fixtures::contact_form_source())
}

fn prepared_app_with_source(source: String) -> ValidationWorkbenchApp {
    let launch = ValidationWorkbenchLaunch::new()
        .prepare_from_authored_inputs(
            ValidationWorkbenchAuthoredInputs::sample()
                .with_live_view_source(ValidationLiveViewSource::new(source)),
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
