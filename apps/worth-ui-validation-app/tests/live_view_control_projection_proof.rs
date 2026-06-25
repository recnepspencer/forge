use worth_ui::facade::{
    ComponentExecutionLane, ComponentFocusSupport, ComponentId, WorthUiAuthoredLiveViewDocument,
    WorthUiLiveViewConditionalProjectionDenial,
    WorthUiLiveViewControlProjectionCompatibilityReceipt, WorthUiLiveViewControlProjectionKind,
    WorthUiLiveViewProjectionAdmissionDenial, WorthUiLiveViewProjectionConsumerKind,
    WorthUiLiveViewStateValue, WorthUiQueryGraphObligationSemantic, WorthUiRuntimeFactFamily,
};
use worth_ui_validation_app::reload::ValidationLiveViewSource;
use worth_ui_validation_app::{ValidationWorkbenchAuthoredInputs, ValidationWorkbenchLaunch};

#[test]
fn text_to_select_control_projection_preserves_state_binding() {
    let mut app = prepared_app_with_live_view_source(text_contact_mode_source());
    assert_registered_control_component(&app, "worth.component.text_input");
    assert_registered_control_component(&app, "worth.component.dropdown_input");
    let edit = app
        .live_view_control_edit_intent("contact_mode", WorthUiLiveViewStateValue::text("maybe"))
        .expect("contact mode edit derives from state binding");
    app.workbench_mut()
        .runtime_mut()
        .apply_live_view_state_edit(edit)
        .expect("contact mode edit applies before projection reload");
    let next = app
        .hot_reload_live_view_source(select_contact_mode_source())
        .expect("select source reload admits through one runtime");

    let contact = next
        .controls()
        .iter()
        .find(|control| control.control_id() == "contact_mode_input")
        .expect("contact mode control exists");
    assert_eq!(contact.binding().binding_id(), "contact_mode");
    assert_eq!(
        contact.kind(),
        &WorthUiLiveViewControlProjectionKind::Select
    );
    assert_eq!(
        contact.component_id().as_str(),
        "worth.component.dropdown_input"
    );
    assert!(contact.options().is_some());
    assert!(contact
        .query_graph_execution()
        .rows()
        .iter()
        .any(|row| row.semantic()
            == WorthUiQueryGraphObligationSemantic::LiveViewControlProjectionKind));

    let rebind = next
        .last_rebind()
        .expect("runtime source reload produces projection rebind receipt")
        .control_rebind();
    assert!(rebind
        .changed_facts()
        .contains_family(WorthUiRuntimeFactFamily::LiveViewControlProjection));
    assert!(!rebind
        .changed_facts()
        .contains_family(WorthUiRuntimeFactFamily::LiveViewStateValue));
    assert_eq!(rebind.counters().renderer_parse_count(), 0);
    let row = rebind
        .compatibility_rows()
        .iter()
        .find(|row| row.control_id() == "contact_mode_input")
        .expect("contact mode compatibility row exists");
    assert_eq!(row.prior_kind(), Some("text_input"));
    assert_eq!(row.next_kind(), Some("select"));
    assert_eq!(
        row.compatibility(),
        WorthUiLiveViewControlProjectionCompatibilityReceipt::Preserved
    );
    assert_eq!(
        app.workbench()
            .runtime()
            .live_view_state_value(contact.binding())
            .map(WorthUiLiveViewStateValue::as_display_text),
        Some("maybe".to_owned())
    );
    assert!(next.render_plan().controls().iter().any(|row| {
        row.control().control_id() == "contact_mode_input"
            && row.component_id().as_str() == "worth.component.dropdown_input"
            && row.control().kind() == &WorthUiLiveViewControlProjectionKind::Select
    }));
}

fn assert_registered_control_component(
    app: &worth_ui_validation_app::ValidationWorkbenchApp,
    component_id: &str,
) {
    let component_id = ComponentId::new(component_id).expect("valid component id");
    let descriptor = app
        .workbench()
        .runtime()
        .inspect_active_component_descriptor(&component_id)
        .expect("live-view control component is registered in frozen capabilities");
    assert_eq!(descriptor.focus(), ComponentFocusSupport::Focusable);
    assert_eq!(
        descriptor.execution_lane(),
        ComponentExecutionLane::Interactive
    );
}

#[test]
fn dropdown_yes_controls_conditional_layout_event_accessibility_participation() {
    let mut app = prepared_app_with_live_view_source(select_contact_mode_source());
    let initial = app
        .live_view_projection_proof()
        .expect("initial conditional proof admits");
    let company = initial
        .conditionals()
        .iter()
        .find(|conditional| conditional.control().control_id() == "company_name_input")
        .expect("company conditional exists");
    assert!(!company.participation().participates_in_layout());
    assert!(!company.participation().participates_in_events());
    assert!(!company.participation().participates_in_accessibility());
    assert_eq!(company.participation().retained_state().token(), "retained");
    let initial_company_render_row = initial
        .render_plan()
        .controls()
        .iter()
        .find(|row| row.control().control_id() == "company_name_input")
        .expect("retained absent controls keep mounted render identity");
    assert!(!initial_company_render_row
        .participation()
        .expect("conditional row carries participation receipt")
        .participates_in_layout());

    let intent = app
        .live_view_control_edit_intent("contact_mode", WorthUiLiveViewStateValue::text("yes"))
        .expect("contact mode edit intent derives from state binding");
    app.workbench_mut()
        .runtime_mut()
        .apply_live_view_state_edit(intent)
        .expect("contact mode edit admits");

    let after_yes = app
        .live_view_projection_proof()
        .expect("conditional proof admits after edit");
    let company = after_yes
        .conditionals()
        .iter()
        .find(|conditional| conditional.control().control_id() == "company_name_input")
        .expect("company conditional exists");
    assert!(company.participation().participates_in_layout());
    assert!(company.participation().participates_in_events());
    assert!(company.participation().participates_in_accessibility());
    assert!(company
        .query_graph_execution()
        .rows()
        .iter()
        .any(|row| row.semantic()
            == WorthUiQueryGraphObligationSemantic::LiveViewConditionalParticipation));
    let company_render_row = after_yes
        .render_plan()
        .controls()
        .iter()
        .find(|row| row.control().control_id() == "company_name_input")
        .expect("runtime render plan includes company row when participation is present");
    assert_eq!(
        company_render_row.component_id().as_str(),
        "worth.component.text_input"
    );
    assert!(company_render_row
        .participation()
        .expect("conditional row carries participation receipt")
        .participates_in_events());
    assert!(after_yes.render_plan().consumers().iter().any(|consumer| {
        consumer.control_id() == "company_name_input"
            && consumer.kind() == WorthUiLiveViewProjectionConsumerKind::EventGeometry
    }));

    let company_intent = app
        .live_view_control_edit_intent("company_name", WorthUiLiveViewStateValue::text("Acme"))
        .expect("company edit intent derives from retained state binding");
    app.workbench_mut()
        .runtime_mut()
        .apply_live_view_state_edit(company_intent)
        .expect("company edit admits while control participates");

    let no_intent = app
        .live_view_control_edit_intent("contact_mode", WorthUiLiveViewStateValue::text("no"))
        .expect("contact mode edit intent derives from state binding");
    app.workbench_mut()
        .runtime_mut()
        .apply_live_view_state_edit(no_intent)
        .expect("contact mode edit admits");

    let after_no = app
        .live_view_projection_proof()
        .expect("conditional proof admits after edit away from yes");
    let company = after_no
        .conditionals()
        .iter()
        .find(|conditional| conditional.control().control_id() == "company_name_input")
        .expect("company conditional exists");
    assert!(!company.participation().participates_in_layout());
    assert!(!company.participation().participates_in_events());
    assert!(!company.participation().participates_in_accessibility());
    assert_eq!(company.participation().retained_state().token(), "retained");
    let after_no_company_render_row = after_no
        .render_plan()
        .controls()
        .iter()
        .find(|row| row.control().control_id() == "company_name_input")
        .expect("retained absent controls keep mounted render identity");
    assert!(!after_no_company_render_row
        .participation()
        .expect("conditional row carries participation receipt")
        .participates_in_layout());
    let company_binding = after_no
        .declaration()
        .binding("company_name")
        .expect("company binding is retained");
    assert_eq!(
        app.workbench()
            .runtime()
            .live_view_state_value(company_binding)
            .map(WorthUiLiveViewStateValue::as_display_text),
        Some("Acme".to_owned())
    );
}

#[test]
fn invalid_projection_values_batch_in_authored_declaration_order() {
    let source = r#"live_view validation.live_view.primitive_proof {
    target button_proof
    state first_name {
        fact validation.state.contact.first_name
        kind text
        access read_write
    }
    control bad control {
        binding missing_binding
        projection slider
        label "Bad"
        options_source remote.source
    }
    control first_name_input {
        binding first_name
        projection text_input
        label "First name"
    }
    condition first_name_input {
        when missing weird "yes"
        true unsupported
        false absent_retaining_state
    }
}"#;
    let app = prepared_app_with_live_view_source(source.to_owned());
    let live_view = app.live_view_state_proof().expect("state binding admits");
    let document =
        WorthUiAuthoredLiveViewDocument::parse(source).expect("authored projection source parses");
    let declaration = document
        .declaration("validation.live_view.primitive_proof")
        .expect("live view declaration exists");

    let report = app
        .workbench()
        .runtime()
        .admit_authored_live_view_projections(&live_view, declaration)
        .expect_err("invalid projections deny as one report");
    let codes = report
        .denials()
        .iter()
        .map(WorthUiLiveViewProjectionAdmissionDenial::code)
        .collect::<Vec<_>>();

    assert_eq!(
        codes,
        vec![
            "live_view_control.invalid_id",
            "live_view_control.unknown_binding",
            "live_view_control.unsupported_kind",
            "live_view_control.unsupported_option_source",
            "live_view_condition.unsupported_condition",
            "live_view_condition.unsupported_participation",
        ]
    );
    assert_eq!(report.counters().control_count(), 2);
    assert_eq!(report.counters().conditional_count(), 1);
    assert_eq!(report.counters().denial_count(), 6);
    assert!(report.denial_set_digest() > 0);
}

#[test]
fn invalid_conditional_values_batch_in_declaration_order() {
    let source = r#"live_view validation.live_view.primitive_proof {
    target button_proof
    state first_name {
        fact validation.state.contact.first_name
        kind text
        access read_write
    }
    control first_name_input {
        binding first_name
        projection text_input
        label "First name"
    }
    condition first_name_input {
        when missing weird "yes"
        true unsupported
        false absent_retaining_state
    }
}"#;
    let app = prepared_app_with_live_view_source(source.to_owned());
    let live_view = app.live_view_state_proof().expect("state binding admits");
    let document =
        WorthUiAuthoredLiveViewDocument::parse(source).expect("authored conditional source parses");
    let declaration = document
        .declaration("validation.live_view.primitive_proof")
        .expect("live view declaration exists");
    let controls = app
        .workbench()
        .runtime()
        .admit_live_view_control_projections(&live_view, declaration.controls())
        .expect("control admits");

    let report = app
        .workbench()
        .runtime()
        .admit_live_view_conditional_projections(&live_view, &controls, declaration.conditionals())
        .expect_err("invalid conditional projection denies");
    let codes = report
        .denials()
        .iter()
        .map(WorthUiLiveViewConditionalProjectionDenial::code)
        .collect::<Vec<_>>();

    assert_eq!(
        codes,
        vec![
            "live_view_condition.unsupported_condition",
            "live_view_condition.unsupported_participation",
        ]
    );
}

fn prepared_app_with_live_view_source(
    source_text: String,
) -> worth_ui_validation_app::ValidationWorkbenchApp {
    prepared_app_from_inputs(
        ValidationWorkbenchAuthoredInputs::sample()
            .with_live_view_source(ValidationLiveViewSource::new(source_text)),
    )
}

fn prepared_app_from_inputs(
    inputs: ValidationWorkbenchAuthoredInputs,
) -> worth_ui_validation_app::ValidationWorkbenchApp {
    let launch = ValidationWorkbenchLaunch::new()
        .prepare_from_authored_inputs(inputs)
        .expect("validation app should prepare");
    worth_ui_validation_app::ValidationWorkbenchApp::new(launch)
}

fn select_contact_mode_source() -> String {
    r#"live_view validation.live_view.primitive_proof {
    target button_proof
    state first_name {
        fact validation.state.contact.first_name
        kind text
        access read_write
    }
    state contact_mode {
        fact validation.state.contact.mode
        kind text
        access read_write
    }
    state company_name {
        fact validation.state.contact.company_name
        kind text
        access read_write
    }
    control first_name_input {
        binding first_name
        projection text_input
        label "First name"
    }
    control contact_mode_input {
        binding contact_mode
        projection select
        label "Contact mode"
        options yes:Yes,no:No
    }
    control company_name_input {
        binding company_name
        projection text_input
        label "Company"
    }
    condition company_name_input {
        when contact_mode equals "yes"
        true present
        false absent_retaining_state
    }
}"#
    .to_owned()
}

fn text_contact_mode_source() -> String {
    select_contact_mode_source().replace(
        "projection select\n        label \"Contact mode\"\n        options yes:Yes,no:No",
        "projection text_input\n        label \"Contact mode\"",
    )
}
