mod primitive_interaction_support;
mod validation_app_reload_fixture;

use worth_ui::facade::{WorthUiInteractionValueDenialCode, WorthUiPrimitiveProofDenial};
use worth_ui_validation_app::reload::ValidationAuthoredReloadEdit;

use primitive_interaction_support::{
    launch_interaction_workbench, prepare_interaction_reload, primitive_surface_id,
    PRIMITIVE_SURFACE,
};

#[test]
fn invalid_interaction_values_report_one_denial_set() {
    let mut workbench = launch_interaction_workbench();
    let prepared = prepare_interaction_reload(
        &workbench,
        &[
            ValidationAuthoredReloadEdit::set_surface_prop(
                PRIMITIVE_SURFACE,
                "interaction_kind",
                "command",
            ),
            ValidationAuthoredReloadEdit::set_surface_prop(
                PRIMITIVE_SURFACE,
                "interaction_readiness",
                "sometimes",
            ),
            ValidationAuthoredReloadEdit::set_surface_prop(
                PRIMITIVE_SURFACE,
                "interaction_payload",
                "\"\"",
            ),
            ValidationAuthoredReloadEdit::set_surface_prop(
                PRIMITIVE_SURFACE,
                "interaction_surprise",
                "nope",
            ),
        ],
    );
    workbench
        .activate_reload(prepared)
        .expect("invalid interaction declarations can activate before projection denial");

    let denial = workbench
        .runtime()
        .resolve_primitive_proof(&primitive_surface_id())
        .expect_err("invalid interaction declarations reject the primitive projection");
    let WorthUiPrimitiveProofDenial::InvalidInteractionValues { report } = denial else {
        panic!("expected interaction admission report");
    };
    let denial_set = report.status().denial_set().expect("denial set");

    assert_eq!(report.counters().denials_emitted(), 4);
    assert_eq!(
        denial_set
            .denials()
            .iter()
            .map(|denial| denial.prop_key())
            .collect::<Vec<_>>(),
        vec![
            "interaction_payload",
            "interaction_command",
            "interaction_readiness",
            "interaction_surprise",
        ]
    );
    assert_eq!(
        denial_set.denials()[0].denial_code(),
        WorthUiInteractionValueDenialCode::InvalidPayload
    );
    assert_eq!(
        denial_set.denials()[1].denial_code(),
        WorthUiInteractionValueDenialCode::MissingRequiredValue
    );
    let presentation = denial_set.denials()[0].presentation();
    assert_eq!(presentation.title(), "Interaction value rejected");
    assert!(presentation
        .rows()
        .iter()
        .any(|row| row.label() == "expected"
            && row.value() == "a text, number, or identifier payload value"));
    assert!(presentation
        .rows()
        .iter()
        .any(|row| row.label() == "source_span"));
    assert_ne!(denial_set.denial_set_digest(), 0);
}

#[test]
fn unknown_command_target_rejects_through_interaction_admission() {
    let mut workbench = launch_interaction_workbench();
    let prepared = prepare_interaction_reload(
        &workbench,
        &[
            ValidationAuthoredReloadEdit::set_surface_prop(
                PRIMITIVE_SURFACE,
                "interaction_kind",
                "command",
            ),
            ValidationAuthoredReloadEdit::set_surface_prop(
                PRIMITIVE_SURFACE,
                "interaction_command",
                "validation.command.missing",
            ),
        ],
    );
    workbench
        .activate_reload(prepared)
        .expect("invalid interaction target can activate before projection denial");

    let denial = workbench
        .runtime()
        .resolve_primitive_proof(&primitive_surface_id())
        .expect_err("unknown command target rejects interaction admission");
    let WorthUiPrimitiveProofDenial::InvalidInteractionValues { report } = denial else {
        panic!("expected interaction admission report");
    };
    let denial_set = report.status().denial_set().expect("denial set");
    assert_eq!(denial_set.denials().len(), 1);
    assert_eq!(denial_set.denials()[0].prop_key(), "interaction_command");
    assert_eq!(
        denial_set.denials()[0].denial_code(),
        WorthUiInteractionValueDenialCode::InvalidTargetReference
    );
}
