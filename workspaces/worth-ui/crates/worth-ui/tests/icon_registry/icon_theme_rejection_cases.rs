use worth_ui::facade::{
    CapabilityDiagnosticCode, IconAccessibilityPosture, IconColorSupport, IconDescriptor,
    IconFamily, IconSourceDescriptor, IconThemePosture, WorthUi,
};

use super::icon_assertions::assert_dependency_diagnostics;
use super::icon_assertions::assert_diagnostic_codes;
use super::icon_fixtures::{color_theme_token, icon_id, theme_token_id};

#[test]
fn theme_incompatible_icon_descriptor_rejected() {
    let report = WorthUi::app()
        .register_icon(
            IconDescriptor::new(
                icon_id("workspace.icon.save"),
                IconFamily::command(),
                IconSourceDescriptor::symbol("save")
                    .with_color_support(IconColorSupport::fixed_color()),
            )
            .with_theme_posture(IconThemePosture::theme_token_driven()),
        )
        .freeze_with_registration_report();

    assert!(report.has_errors());
    assert!(report.accepted_snapshot().icons().is_empty());
    assert_diagnostic_codes(
        &report,
        &[CapabilityDiagnosticCode::ThemeIncompatibleIconDescriptor],
    );
}

#[test]
fn missing_public_icon_postures_are_rejected() {
    let report = WorthUi::app()
        .register_icon(
            IconDescriptor::new(
                icon_id("workspace.icon.save"),
                IconFamily::command(),
                IconSourceDescriptor::symbol("save"),
            )
            .with_theme_posture(IconThemePosture::missing_for_diagnostics())
            .with_accessibility_posture(IconAccessibilityPosture::missing_for_diagnostics()),
        )
        .freeze_with_registration_report();

    assert!(report.has_errors());
    assert!(report.accepted_snapshot().icons().is_empty());
    assert_diagnostic_codes(
        &report,
        &[
            CapabilityDiagnosticCode::MissingIconThemePosture,
            CapabilityDiagnosticCode::MissingIconAccessibilityPosture,
        ],
    );
}

#[test]
fn theme_token_driven_icon_without_token_reference_rejected() {
    let report = WorthUi::app()
        .register_icon(
            IconDescriptor::new(
                icon_id("workspace.icon.save"),
                IconFamily::command(),
                IconSourceDescriptor::symbol("save")
                    .with_color_support(IconColorSupport::theme_token_driven()),
            )
            .with_theme_posture(IconThemePosture::theme_token_driven()),
        )
        .freeze_with_registration_report();

    assert!(report.has_errors());
    assert!(report.accepted_snapshot().icons().is_empty());
    assert_diagnostic_codes(
        &report,
        &[CapabilityDiagnosticCode::MissingIconThemeTokenReference],
    );
}

#[test]
fn theme_token_driven_icon_references_missing_token_rejected() {
    let report = WorthUi::app()
        .register_icon(
            IconDescriptor::new(
                icon_id("workspace.icon.save"),
                IconFamily::command(),
                IconSourceDescriptor::symbol("save")
                    .with_color_support(IconColorSupport::theme_token_driven())
                    .with_theme_token(theme_token_id("theme.text.missing")),
            )
            .with_theme_posture(IconThemePosture::theme_token_driven()),
        )
        .freeze_with_registration_report();

    assert!(report.has_errors());
    assert!(report.accepted_snapshot().icons().is_empty());
    assert_dependency_diagnostics(
        report.registration_diagnostics(),
        &[("workspace.icon.save", "theme_token", "theme.text.missing")],
    );
}

#[test]
fn non_theme_token_driven_icon_cannot_attach_token_reference() {
    let report = WorthUi::app()
        .register_icon(
            IconDescriptor::new(
                icon_id("workspace.icon.save"),
                IconFamily::command(),
                IconSourceDescriptor::symbol("save")
                    .with_theme_token(theme_token_id("theme.text.primary")),
            )
            .with_theme_posture(IconThemePosture::inherits_text_color()),
        )
        .freeze_with_registration_report();

    assert!(report.has_errors());
    assert!(report.accepted_snapshot().icons().is_empty());
    assert_diagnostic_codes(
        &report,
        &[CapabilityDiagnosticCode::UnexpectedIconThemeTokenReference],
    );
}

#[test]
fn theme_token_driven_icon_accepts_registered_token_reference() {
    let app = WorthUi::app()
        .register_theme_token(color_theme_token("theme.text.primary", "#101820"))
        .register_icon(
            IconDescriptor::new(
                icon_id("workspace.icon.save"),
                IconFamily::command(),
                IconSourceDescriptor::symbol("save")
                    .with_color_support(IconColorSupport::theme_token_driven())
                    .with_theme_token(theme_token_id("theme.text.primary")),
            )
            .with_theme_posture(IconThemePosture::theme_token_driven()),
        )
        .freeze()
        .expect("application preparation should succeed");

    assert_eq!(app.capabilities().icons().len(), 1);
}
