use worth_ui::facade::{CapabilityDiagnosticCode, WorthUi};

use super::theme_token_assertions::assert_diagnostic_codes;
use super::theme_token_fixtures::alias_theme_token;

#[test]
fn two_token_alias_cycle_is_rejected() {
    let report = WorthUi::app()
        .register_theme_token(alias_theme_token("theme.a", "theme.b"))
        .register_theme_token(alias_theme_token("theme.b", "theme.a"))
        .freeze_with_registration_report();

    assert_diagnostic_codes(
        &report,
        &[
            CapabilityDiagnosticCode::ThemeTokenAliasCycle,
            CapabilityDiagnosticCode::ThemeTokenAliasCycle,
        ],
    );
    assert!(report.accepted_snapshot().theme_tokens().is_empty());
}

#[test]
fn self_alias_cycle_is_rejected() {
    let report = WorthUi::app()
        .register_theme_token(alias_theme_token("theme.self", "theme.self"))
        .freeze_with_registration_report();

    assert_diagnostic_codes(&report, &[CapabilityDiagnosticCode::ThemeTokenAliasCycle]);
    assert!(report.accepted_snapshot().theme_tokens().is_empty());
}
