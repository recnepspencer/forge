mod primitive_appearance_state_basis_support;
mod primitive_appearance_state_denial_support;

use primitive_appearance_state_denial_support::{
    appearance_state_denial_for_edits, PRIMITIVE_SURFACE,
};
use worth_ui::facade::{
    WorthUiAppearanceStateAdmissionReport, WorthUiAppearanceStateTokenDenialReason,
    WorthUiAppearanceStateValueDenialCode, WorthUiPrimitiveDenialPresentation,
    WorthUiPrimitiveProofDenial, WorthUiSemanticSliceId,
};
use worth_ui_validation_app::reload::ValidationAuthoredReloadEdit;

#[test]
fn invalid_appearance_state_values_reject_with_schema_owned_rows() {
    let report = appearance_denial_report(&[ValidationAuthoredReloadEdit::set_surface_prop(
        PRIMITIVE_SURFACE,
        "appearance_pressed_opacity",
        "2",
    )]);
    let denial_set = report
        .status()
        .denial_set()
        .expect("appearance-state denial report carries denial set");
    let receipt = denial_set
        .denials()
        .first()
        .expect("appearance-state denial set is non-empty");

    assert_eq!(report.counters().denials_emitted(), 1);
    assert_eq!(receipt.prop_key(), "appearance_pressed_opacity");
    assert_eq!(receipt.raw_value(), "2");
    assert_eq!(
        receipt.denial_code(),
        WorthUiAppearanceStateValueDenialCode::InvalidOpacity
    );
    assert!(receipt.source_span().is_some());
    assert_ne!(receipt.denial_digest(), 0);
    assert_presentation_row(
        &receipt.presentation(),
        "expected",
        "a number from `0` through `1`",
    );
    assert_presentation_row(
        &receipt.presentation(),
        "slice",
        &format!("{:?}", WorthUiSemanticSliceId::PrimitiveAppearanceState),
    );
    assert_presentation_row(&receipt.presentation(), "source_span", "..");
    assert_presentation_row(
        &receipt.presentation(),
        "digest",
        &receipt.denial_digest().to_string(),
    );
}

#[test]
fn multiple_invalid_values_report_one_stable_denial_set() {
    let edits = [
        ValidationAuthoredReloadEdit::set_surface_prop(
            PRIMITIVE_SURFACE,
            "appearance_hover_background",
            "validation.theme.header.missing",
        ),
        ValidationAuthoredReloadEdit::set_surface_prop(
            PRIMITIVE_SURFACE,
            "appearance_pressed_radius",
            "\"12px\"",
        ),
        ValidationAuthoredReloadEdit::set_surface_prop(
            PRIMITIVE_SURFACE,
            "appearance_selected_opacity",
            "2",
        ),
        ValidationAuthoredReloadEdit::set_surface_prop(
            PRIMITIVE_SURFACE,
            "appearance_selected_typography",
            "validation.theme.header.text",
        ),
    ];
    let first = appearance_denial_report(&edits);
    let second = appearance_denial_report(&edits);
    let denial_set = first.status().denial_set().expect("denial set");

    assert_eq!(first.counters().denials_emitted(), 4);
    assert_eq!(
        denial_set
            .denials()
            .iter()
            .map(|receipt| receipt.prop_key())
            .collect::<Vec<_>>(),
        vec![
            "appearance_hover_background",
            "appearance_pressed_radius",
            "appearance_selected_opacity",
            "appearance_selected_typography",
        ]
    );
    assert_eq!(
        denial_set.denial_set_digest(),
        second
            .status()
            .denial_set()
            .expect("second denial set")
            .denial_set_digest()
    );
    assert!(denial_set
        .denials()
        .iter()
        .all(|receipt| receipt.source_span().is_some()));
}

#[test]
fn token_resolution_denials_are_machine_readable() {
    let report = appearance_denial_report(&[
        ValidationAuthoredReloadEdit::set_surface_prop(
            PRIMITIVE_SURFACE,
            "appearance_hover_background",
            "validation.theme.header.missing",
        ),
        ValidationAuthoredReloadEdit::set_surface_prop(
            PRIMITIVE_SURFACE,
            "appearance_pressed_radius",
            "validation.density.primitive.missing",
        ),
        ValidationAuthoredReloadEdit::set_surface_prop(
            PRIMITIVE_SURFACE,
            "appearance_selected_typography",
            "validation.theme.header.text",
        ),
    ]);
    let denials = report.status().denial_set().expect("denial set").denials();

    assert_eq!(
        denials[0].token_denial_reason(),
        Some(WorthUiAppearanceStateTokenDenialReason::MissingThemeToken)
    );
    assert_eq!(
        denials[1].token_denial_reason(),
        Some(WorthUiAppearanceStateTokenDenialReason::MissingDensityToken)
    );
    assert_eq!(
        denials[2].token_denial_reason(),
        Some(WorthUiAppearanceStateTokenDenialReason::MissingAppearanceToken)
    );
    assert_presentation_row(
        &denials[0].presentation(),
        "token_reason",
        "MissingThemeToken",
    );
}

#[test]
fn unknown_appearance_state_prop_rejects_through_same_denial_set() {
    let report = appearance_denial_report(&[ValidationAuthoredReloadEdit::set_surface_prop(
        PRIMITIVE_SURFACE,
        "appearance_drag_background",
        "\"#123456\"",
    )]);
    let denial = &report.status().denial_set().expect("denial set").denials()[0];

    assert_eq!(report.counters().denials_emitted(), 1);
    assert_eq!(denial.prop_key(), "appearance_drag_background");
    assert_eq!(
        denial.denial_code(),
        WorthUiAppearanceStateValueDenialCode::UnknownAppearanceStateProp
    );
    assert_presentation_row(
        &denial.presentation(),
        "expected",
        "a declared appearance state prop",
    );
}

fn appearance_denial_report(
    edits: &[ValidationAuthoredReloadEdit],
) -> WorthUiAppearanceStateAdmissionReport {
    let denial = appearance_state_denial_for_edits(edits);
    let WorthUiPrimitiveProofDenial::InvalidAppearanceStateValues { report } = denial else {
        panic!("expected appearance-state denial report");
    };
    report
}

fn assert_presentation_row(
    presentation: &WorthUiPrimitiveDenialPresentation,
    label: &str,
    expected: &str,
) {
    let row = presentation
        .rows()
        .iter()
        .find(|row| row.label() == label)
        .expect("presentation row should exist");
    assert!(
        row.value().contains(expected),
        "presentation row `{label}` expected `{expected}`, got `{}`",
        row.value()
    );
}
