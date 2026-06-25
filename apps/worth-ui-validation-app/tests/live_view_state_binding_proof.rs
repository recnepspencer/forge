use worth_ui::facade::{
    WorthUiAuthoredLiveViewDocument, WorthUiLiveViewDeclaration, WorthUiLiveViewDenial,
    WorthUiLiveViewStateAccess, WorthUiLiveViewStateBindingDeclaration, WorthUiLiveViewStateValue,
    WorthUiLiveViewStateValueKind, WorthUiQueryGraphObligationSemantic, WorthUiRuntimeChangeFamily,
    WorthUiRuntimeFactFamily,
};
use worth_ui_validation_app::reload::{
    ValidationLiveViewSource, ValidationSourcePackage, VALIDATION_SAMPLE_LIVE_VIEW_SOURCE,
};
use worth_ui_validation_app::sample_source::{
    VALIDATION_SAMPLE_MODULE_PATH, VALIDATION_SAMPLE_SOURCE,
};
use worth_ui_validation_app::{ValidationWorkbenchAuthoredInputs, ValidationWorkbenchLaunch};

#[test]
fn live_view_state_bindings_admit_with_query_graph_evidence() {
    let app = prepared_app();
    let receipt = app
        .live_view_state_proof()
        .expect("live view proof should admit authored state bindings");

    assert_eq!(
        receipt.live_view_id(),
        "validation.live_view.primitive_proof"
    );
    assert_eq!(receipt.bindings().len(), 2);
    assert!(receipt.binding("title").is_some());
    assert_eq!(receipt.target_binding().slot_name(), "button_proof");
    assert_eq!(receipt.counters().binding_count(), 2);
    assert_eq!(receipt.counters().source_reparse_count(), 0);
    assert_eq!(receipt.counters().renderer_parse_count(), 0);
    assert_eq!(
        receipt.query_graph_execution().selected_obligation_count(),
        7
    );
    for expected in WorthUiQueryGraphObligationSemantic::LIVE_VIEW_STATE_BINDING {
        assert!(
            receipt
                .query_graph_execution()
                .rows()
                .iter()
                .any(|row| row.semantic() == expected),
            "missing live-view graph semantic {expected:?}"
        );
    }
}

#[test]
fn state_edit_mutates_runtime_owned_state_fact() {
    let mut app = prepared_app();
    let intent = app
        .live_view_control_edit_intent("title", WorthUiLiveViewStateValue::text("Ada"))
        .expect("control edit intent should be derived from admitted live-view binding");

    let edit = app
        .workbench_mut()
        .runtime_mut()
        .apply_live_view_state_edit(intent)
        .expect("state edit should admit");
    assert_eq!(
        edit.changed_fact().family(),
        WorthUiRuntimeFactFamily::LiveViewStateValue
    );
    assert_eq!(
        edit.changed_fact().identity(),
        "validation.state.form.title"
    );
    assert_eq!(
        app.workbench()
            .runtime()
            .live_view_state_value(edit.binding())
            .map(WorthUiLiveViewStateValue::as_display_text),
        Some("Ada".to_owned())
    );

    let admitted = app
        .workbench()
        .runtime()
        .admit_live_view_state_runtime_change(&edit)
        .expect("live view edit should admit as runtime evidence");
    let row = admitted.family_rows().first().expect("row exists");
    assert_eq!(row.family(), WorthUiRuntimeChangeFamily::DurableState);
    assert!(row
        .changed_facts()
        .contains_family(WorthUiRuntimeFactFamily::LiveViewStateValue));
}

#[test]
fn binding_identity_rebind_names_only_affected_live_view_binding_facts() {
    let app = prepared_app();
    let prior = app.live_view_state_proof().expect("prior live view admits");
    let next_app = prepared_app_with_live_view_source(
        VALIDATION_SAMPLE_LIVE_VIEW_SOURCE.replace("state details", "state summary"),
    );
    let next = next_app
        .live_view_state_proof()
        .expect("next live view admits");

    let rebind = app
        .workbench()
        .runtime()
        .compare_live_view_declaration_rebind(&prior, &next);
    let changed = rebind.changed_facts();

    assert!(changed.contains_exact(
        &worth_ui::facade::WorthUiRuntimeFactId::live_view_state_binding(
            "validation.live_view.primitive_proof:details",
        )
    ));
    assert!(changed.contains_exact(
        &worth_ui::facade::WorthUiRuntimeFactId::live_view_state_binding(
            "validation.live_view.primitive_proof:summary",
        )
    ));
    assert!(!changed.contains_exact(
        &worth_ui::facade::WorthUiRuntimeFactId::live_view_state_binding(
            "validation.live_view.primitive_proof:title",
        )
    ));
    assert!(!changed.contains_family(WorthUiRuntimeFactFamily::LiveViewStateValue));
    assert_eq!(rebind.counters().changed_binding_count(), 2);
    assert_eq!(rebind.counters().source_reparse_count(), 0);
    assert_eq!(rebind.counters().renderer_parse_count(), 0);
}

#[test]
fn equivalent_live_view_declaration_rebind_is_noop() {
    let app = prepared_app();
    let prior = app.live_view_state_proof().expect("prior live view admits");
    let next = app.live_view_state_proof().expect("next live view admits");

    let rebind = app
        .workbench()
        .runtime()
        .compare_live_view_declaration_rebind(&prior, &next);

    assert!(!rebind
        .changed_facts()
        .contains_family(WorthUiRuntimeFactFamily::LiveViewStateBinding));
    assert_eq!(rebind.counters().changed_binding_count(), 0);
}

#[test]
fn binding_fact_edit_rebind_names_only_that_binding_fact() {
    let app = prepared_app();
    let prior = app.live_view_state_proof().expect("prior live view admits");
    let next_app = prepared_app_with_live_view_source(VALIDATION_SAMPLE_LIVE_VIEW_SOURCE.replace(
        "fact validation.state.form.details",
        "fact validation.state.form.details.v2",
    ));
    let next = next_app
        .live_view_state_proof()
        .expect("next live view admits");

    let rebind = app
        .workbench()
        .runtime()
        .compare_live_view_declaration_rebind(&prior, &next);

    assert!(rebind.changed_facts().contains_exact(
        &worth_ui::facade::WorthUiRuntimeFactId::live_view_state_binding(
            "validation.live_view.primitive_proof:details",
        )
    ));
    assert!(!rebind.changed_facts().contains_exact(
        &worth_ui::facade::WorthUiRuntimeFactId::live_view_state_binding(
            "validation.live_view.primitive_proof:title",
        )
    ));
    assert_eq!(rebind.counters().changed_binding_count(), 1);
}

#[test]
fn declaration_denials_batch_in_canonical_declaration_order() {
    let stale_target = prepared_app()
        .live_view_state_proof()
        .expect("source target admits before component identity changes")
        .target_binding()
        .clone();
    let app_with_current_target = prepared_app_with_source(source_with_button_component());
    let document = WorthUiAuthoredLiveViewDocument::parse(
        r#"live_view validation.live_view.bad {
    target button_proof
    state bad binding {
        fact bad fact
        kind blob
        access read_only
    }
    state bad binding {
        fact validation.state.test.first_name
        kind text
        access read_write
    }
}"#,
    )
    .expect("raw authored live-view source parses before semantic admission");
    let declaration = document
        .declaration("validation.live_view.bad")
        .expect("authored declaration exists");

    let report = app_with_current_target
        .workbench()
        .runtime()
        .admit_authored_live_view_declaration(declaration, stale_target)
        .expect_err("invalid declaration should deny as one report");
    let codes = report
        .denials()
        .iter()
        .map(WorthUiLiveViewDenial::code)
        .collect::<Vec<_>>();

    assert_eq!(
        codes,
        vec![
            "live_view.stale_target_binding",
            "live_view.invalid_binding_id",
            "live_view.invalid_state_fact",
            "live_view.unsupported_value_kind",
            "live_view.unsupported_write_posture",
            "live_view.invalid_binding_id",
            "live_view.duplicate_binding_id",
        ]
    );
    assert_eq!(report.counters().binding_count(), 2);
    assert_eq!(report.counters().denial_count(), 7);
    assert!(report.denial_set_digest() > 0);
}

#[test]
fn edit_value_kind_mismatch_rejects_before_state_mutation() {
    let mut app = prepared_app();
    let receipt = app.live_view_state_proof().expect("live view admits");
    let binding = receipt.binding("title").expect("binding exists").clone();

    let denial = app
        .workbench_mut()
        .runtime_mut()
        .apply_live_view_state_edit(binding.edit(WorthUiLiveViewStateValue::Boolean(true)))
        .expect_err("wrong value kind should deny");

    assert!(matches!(
        denial,
        worth_ui::facade::WorthUiLiveViewStateEditDenial::ValueKindMismatch { .. }
    ));
    assert!(app
        .workbench()
        .runtime()
        .live_view_state_value(&binding)
        .is_none());
}

#[test]
fn stale_target_binding_rejects_declaration_and_edit_before_state_mutation() {
    let app_with_stale_target = prepared_app();
    let admitted = app_with_stale_target
        .live_view_state_proof()
        .expect("live view admits");
    let stale_target = admitted.target_binding().clone();
    let stale_binding = admitted.binding("title").expect("binding exists").clone();
    let mut app_with_current_target = prepared_app_with_source(source_with_button_component());

    let stale_declaration =
        WorthUiLiveViewDeclaration::new("validation.live_view.primitive_proof", stale_target)
            .with_state_binding(test_binding(
                "title",
                "validation.state.form.title",
                WorthUiLiveViewStateValueKind::Text,
                WorthUiLiveViewStateAccess::ReadWrite,
            ));
    let report = app_with_current_target
        .workbench()
        .runtime()
        .admit_live_view_declaration(stale_declaration)
        .expect_err("stale target binding should deny declaration admission");
    assert_eq!(
        report
            .denials()
            .iter()
            .map(WorthUiLiveViewDenial::code)
            .collect::<Vec<_>>(),
        vec!["live_view.stale_target_binding"]
    );

    let denial = app_with_current_target
        .workbench_mut()
        .runtime_mut()
        .apply_live_view_state_edit(stale_binding.edit(WorthUiLiveViewStateValue::text("Grace")))
        .expect_err("stale binding cannot mutate runtime state");
    assert!(matches!(
        denial,
        worth_ui::facade::WorthUiLiveViewStateEditDenial::StaleTargetBinding { .. }
    ));
    assert!(app_with_current_target
        .workbench()
        .runtime()
        .live_view_state_value(&stale_binding)
        .is_none());
}

fn prepared_app() -> worth_ui_validation_app::ValidationWorkbenchApp {
    prepared_app_from_inputs(ValidationWorkbenchAuthoredInputs::sample())
}

fn prepared_app_with_source(
    source_text: String,
) -> worth_ui_validation_app::ValidationWorkbenchApp {
    prepared_app_from_inputs(ValidationWorkbenchAuthoredInputs::sample().with_source(
        ValidationSourcePackage::new(VALIDATION_SAMPLE_MODULE_PATH, source_text),
    ))
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

fn test_binding(
    binding_id: &str,
    state_fact: &str,
    value_kind: WorthUiLiveViewStateValueKind,
    access: WorthUiLiveViewStateAccess,
) -> WorthUiLiveViewStateBindingDeclaration {
    WorthUiLiveViewStateBindingDeclaration::new(
        binding_id,
        worth_ui::facade::WorthUiLiveViewStateFactId::new(state_fact)
            .expect("state fact id is valid"),
        value_kind,
        access,
    )
}

fn source_with_button_component() -> String {
    VALIDATION_SAMPLE_SOURCE.replace(
        "surface worth.surface.preview.primitive.proof {\n    component worth.component.primitive_proof",
        "surface worth.surface.preview.primitive.proof {\n    component worth.component.button",
    )
}
