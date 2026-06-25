use worth_ui::facade::{
    WorthUiLiveViewControlEditabilityPosture, WorthUiLiveViewControlHostFrameKind,
    WorthUiLiveViewControlHostFrameWidthPolicy, WorthUiLiveViewProjectionAdmissionDenial,
    WorthUiLiveViewReadinessPosture, WorthUiRuntimeFactFamily,
};

#[path = "support/live_view_control_host_frame_fixtures.rs"]
mod live_view_control_host_frame_fixtures;

use live_view_control_host_frame_fixtures::{
    apply_text, contact_source, mounted_submit_interaction, prepared_app_with_live_view_source,
};
use worth_ui_validation_app::{ValidationWorkbenchAuthoredInputs, ValidationWorkbenchLaunch};

#[test]
fn text_and_dropdown_controls_render_from_sealed_host_frame_receipts() {
    let app = prepared_app_with_live_view_source(contact_source());
    let proof = app
        .live_view_projection_proof()
        .expect("live-view projection admits");

    let first_name = proof
        .render_plan()
        .controls()
        .iter()
        .find(|row| row.control().control_id() == "first_name_input")
        .expect("first name control renders");
    let contact_mode = proof
        .render_plan()
        .controls()
        .iter()
        .find(|row| row.control().control_id() == "contact_mode_input")
        .expect("contact mode control renders");

    assert_eq!(
        first_name.host_frame().kind(),
        WorthUiLiveViewControlHostFrameKind::TextInput
    );
    assert_eq!(
        first_name.host_frame().subject().component_id().as_str(),
        "worth.component.text_input"
    );
    assert_eq!(
        first_name.host_frame().editability(),
        WorthUiLiveViewControlEditabilityPosture::Editable
    );
    assert_eq!(first_name.host_frame().style().padding_left_points(), 16.0);
    assert_eq!(first_name.host_frame().style().padding_top_points(), 16.0);
    assert_eq!(first_name.host_frame().style().radius_points(), 8.0);
    assert_eq!(
        first_name.host_frame().style().width_policy(),
        WorthUiLiveViewControlHostFrameWidthPolicy::Hug
    );
    assert_eq!(
        first_name
            .host_frame()
            .style()
            .background_color()
            .hex_triplet(),
        "#1f2937"
    );
    assert_consumes_primitive_families(first_name.host_frame().consumed_facts());

    assert_eq!(
        contact_mode.host_frame().kind(),
        WorthUiLiveViewControlHostFrameKind::DropdownInput
    );
    assert_eq!(
        contact_mode.host_frame().subject().component_id().as_str(),
        "worth.component.dropdown_input"
    );
    assert_eq!(contact_mode.host_frame().options().len(), 2);
    assert_consumes_primitive_families(contact_mode.host_frame().consumed_facts());
}

#[test]
fn default_sample_is_two_field_card_submit_scenario() {
    let mut app = worth_ui_validation_app::ValidationWorkbenchApp::new(
        ValidationWorkbenchLaunch::new()
            .prepare_from_authored_inputs(ValidationWorkbenchAuthoredInputs::sample())
            .expect("default validation app should prepare"),
    );
    let initial = app
        .live_view_projection_proof()
        .expect("default live-view admits");

    assert_eq!(initial.render_plan().controls().len(), 2);
    assert!(initial
        .render_plan()
        .controls()
        .iter()
        .all(|row| row.host_frame().kind() == WorthUiLiveViewControlHostFrameKind::TextInput));
    assert_eq!(
        initial
            .interactions()
            .first()
            .expect("submit interaction exists")
            .readiness()
            .posture(),
        WorthUiLiveViewReadinessPosture::DeniedMissingRequired
    );

    apply_text(&mut app, "title", "Runtime-owned form");
    apply_text(&mut app, "details", "Two bound text inputs");
    let ready = app
        .live_view_projection_proof()
        .expect("filled live-view admits");
    let interaction = ready.interactions().first().expect("submit exists");
    assert_eq!(
        interaction.readiness().posture(),
        WorthUiLiveViewReadinessPosture::Enabled
    );
    assert!(
        interaction
            .readiness()
            .consumed_facts()
            .iter()
            .any(|fact| fact.family() == WorthUiRuntimeFactFamily::LiveViewExpressionOutput),
        "readiness must consume the expression output fact, not only local presence rows"
    );
    assert!(
        interaction
            .payload_projection()
            .consumed_facts()
            .iter()
            .any(|fact| fact.family() == WorthUiRuntimeFactFamily::LiveViewExpressionOutput),
        "payload projection must consume the expression output fact that owns payload shape"
    );
    assert_eq!(
        interaction.payload_projection().shape().token(),
        "data_payload_values"
    );

    let receipt = app
        .workbench()
        .runtime()
        .activate_mounted_live_view_interaction(&mounted_submit_interaction(&ready))
        .map(|eligible| {
            app.workbench()
                .runtime()
                .submit_live_view_interaction(eligible)
        })
        .expect("enabled submit emits payload");
    assert_eq!(receipt.emitted_payload().shape_token(), "data_payload");
    let field_names = receipt
        .emitted_payload()
        .fields()
        .iter()
        .map(|field| field.name())
        .collect::<Vec<_>>();
    assert_eq!(field_names, vec!["title", "details"]);
}

#[test]
fn control_primitive_edits_change_host_frame_without_rebinding_state() {
    let mut app = prepared_app_with_live_view_source(contact_source());
    let prior = app
        .live_view_projection_proof()
        .expect("initial projection admits");
    let prior_frame = prior
        .render_plan()
        .controls()
        .iter()
        .find(|row| row.control().control_id() == "first_name_input")
        .expect("first name control renders")
        .host_frame()
        .frame_digest();
    let prior_state_digest = prior
        .controls()
        .iter()
        .find(|control| control.control_id() == "first_name_input")
        .expect("control admits")
        .binding()
        .binding_digest();

    let next = app
        .hot_reload_live_view_source(contact_source().replace(
            "flow_padding validation.density.primitive.flow.padding.compact",
            "flow_padding validation.density.primitive.flow.padding.fat",
        ))
        .expect("control primitive edit admits");
    let next_frame = next
        .render_plan()
        .controls()
        .iter()
        .find(|row| row.control().control_id() == "first_name_input")
        .expect("first name control renders")
        .host_frame()
        .frame_digest();
    let next_state_digest = next
        .controls()
        .iter()
        .find(|control| control.control_id() == "first_name_input")
        .expect("control admits")
        .binding()
        .binding_digest();

    assert_ne!(prior_frame, next_frame);
    assert_eq!(prior_state_digest, next_state_digest);
    assert_eq!(
        next.render_plan()
            .controls()
            .iter()
            .find(|row| row.control().control_id() == "first_name_input")
            .expect("first name control renders")
            .host_frame()
            .style()
            .padding_left_points(),
        48.0
    );
}

#[test]
fn live_view_card_primitives_drive_mounted_surface_without_rebinding_state() {
    let mut app = prepared_app_with_live_view_source(contact_source());
    let surface_digest =
        |proof: &worth_ui_validation_app::app::ValidationLiveViewProjectionProof| {
            proof
                .mounted_product_view()
                .composition_tree()
                .root_children()
                .iter()
                .find_map(|child| match child.mounted_node() {
                    worth_ui::facade::WorthUiMountedNodeReceipt::Surface(surface) => {
                        Some(surface.receipt_digest())
                    }
                    _ => None,
                })
                .expect("mounted view includes surface")
        };
    let flow_digest = |proof: &worth_ui_validation_app::app::ValidationLiveViewProjectionProof| {
        let tree = proof.mounted_product_view().composition_tree();
        let surface = tree.root_children()[0].node_id();
        tree.ordered_children(surface)
            .iter()
            .find_map(|child| match child.mounted_node() {
                worth_ui::facade::WorthUiMountedNodeReceipt::FlowContainer(flow) => {
                    Some(flow.receipt_digest())
                }
                _ => None,
            })
            .expect("mounted view includes flow container")
    };
    let prior = app
        .live_view_projection_proof()
        .expect("initial projection admits");
    let prior_surface = surface_digest(&prior);
    let prior_flow = flow_digest(&prior);
    let prior_state_digest = prior
        .controls()
        .iter()
        .find(|control| control.control_id() == "first_name_input")
        .expect("control admits")
        .binding()
        .binding_digest();

    let next = app
        .hot_reload_live_view_source(
            contact_source()
                .replace(
                    "flow_padding validation.density.primitive.flow.padding.default",
                    "flow_padding validation.density.primitive.flow.padding.fat",
                )
                .replace(
                    "flow_gap validation.density.primitive.flow.gap.default",
                    "flow_gap validation.density.primitive.flow.gap.compact",
                )
                .replace(
                    "appearance_rest_background \"#ffffff\"",
                    "appearance_rest_background \"#101828\"",
                ),
        )
        .expect("live-view primitive edit admits");
    let next_state_digest = next
        .controls()
        .iter()
        .find(|control| control.control_id() == "first_name_input")
        .expect("control admits")
        .binding()
        .binding_digest();

    assert_ne!(prior_surface, surface_digest(&next));
    assert_ne!(prior_flow, flow_digest(&next));
    assert_eq!(prior_state_digest, next_state_digest);
}

#[test]
fn invalid_control_primitive_props_deny_projection_admission() {
    let app = prepared_app_with_live_view_source(contact_source());
    let live_view = app.live_view_state_proof().expect("state binding admits");
    let source = contact_source()
        .replacen(
            "flow_padding validation.density.primitive.flow.padding.compact",
            "flow_padding 32",
            1,
        )
        .replacen(
            "appearance_rest_radius validation.density.primitive.radius\n        event_cursor text",
            "appearance_rest_radius missing.radius.token\n        event_cursor text",
            1,
        )
        .replacen("event_cursor text", "event_cursor banana", 1);
    let document = worth_ui::facade::WorthUiAuthoredLiveViewDocument::parse(&source)
        .expect("source still parses");
    let declaration = document
        .declaration("validation.live_view.primitive_proof")
        .expect("live view declaration exists");
    let report = app
        .workbench()
        .runtime()
        .admit_authored_live_view_projections(&live_view, declaration)
        .expect_err("invalid control primitive values deny");
    let codes = report
        .denials()
        .iter()
        .map(WorthUiLiveViewProjectionAdmissionDenial::code)
        .collect::<Vec<_>>();

    assert_eq!(
        codes,
        vec![
            "live_view_control.primitive_flow_layout_denied",
            "live_view_control.primitive_appearance_state_denied",
            "live_view_control.primitive_event_geometry_denied",
        ]
    );
    assert_eq!(report.counters().control_count(), 2);
    assert_eq!(report.counters().denial_count(), 3);
    assert!(report.denial_set_digest() > 0);
}

fn assert_consumes_primitive_families(facts: &[worth_ui::facade::WorthUiRuntimeFactId]) {
    assert!(facts
        .iter()
        .any(|fact| fact.family() == WorthUiRuntimeFactFamily::PrimitiveFlowLayout));
    assert!(facts
        .iter()
        .any(|fact| fact.family() == WorthUiRuntimeFactFamily::PrimitiveAppearanceState));
    assert!(facts
        .iter()
        .any(|fact| fact.family() == WorthUiRuntimeFactFamily::PrimitiveEventGeometry));
}
