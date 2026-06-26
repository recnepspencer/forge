use worth_ui::facade::{
    CapabilityDiagnosticCode, ThemeTokenAlias, ThemeTokenDescriptor, ThemeTokenFamily,
    ThemeTokenSource, WorthUi,
};

use super::theme_token_assertions::{assert_diagnostic_codes, assert_registered_theme_token_ids};
use super::theme_token_fixtures::{
    color_value, platform_color_theme_token, plugin_alias_theme_token, plugin_custom_theme_token,
    theme_token_id,
};

#[test]
fn plugin_cannot_silently_override_platform_token_meaning() {
    let report = WorthUi::app()
        .register_theme_token(ThemeTokenDescriptor::define(
            theme_token_id("theme.platform.text"),
            ThemeTokenFamily::text(),
            ThemeTokenSource::plugin_platform_override_for_diagnostics(),
            color_value("#010203"),
        ))
        .freeze_with_registration_report();

    assert_diagnostic_codes(
        &report,
        &[CapabilityDiagnosticCode::PluginThemeTokenOverridesPlatformMeaning],
    );
    assert!(report.accepted_snapshot().theme_tokens().is_empty());
}

#[test]
fn plugin_custom_token_cannot_claim_platform_identity_segment() {
    let report = WorthUi::app()
        .register_theme_token(ThemeTokenDescriptor::define(
            theme_token_id("theme.platform.accent"),
            ThemeTokenFamily::accent(),
            ThemeTokenSource::plugin_custom(),
            color_value("#010203"),
        ))
        .freeze_with_registration_report();

    assert_diagnostic_codes(
        &report,
        &[CapabilityDiagnosticCode::PluginThemeTokenOverridesPlatformMeaning],
    );
    assert!(report.accepted_snapshot().theme_tokens().is_empty());
}

#[test]
fn plugin_alias_token_cannot_claim_platform_identity_segment() {
    let report = WorthUi::app()
        .register_theme_token(ThemeTokenDescriptor::alias(
            theme_token_id("theme.platform.accent_alias"),
            ThemeTokenFamily::accent(),
            ThemeTokenSource::plugin_alias(),
            ThemeTokenAlias::to(theme_token_id("theme.text.primary")),
        ))
        .freeze_with_registration_report();

    assert_diagnostic_codes(
        &report,
        &[
            CapabilityDiagnosticCode::PluginThemeTokenOverridesPlatformMeaning,
            CapabilityDiagnosticCode::MissingDependency,
        ],
    );
    assert!(report.accepted_snapshot().theme_tokens().is_empty());
}

#[test]
fn plugin_alias_platform_identity_rejection_does_not_depend_on_missing_target() {
    let report = WorthUi::app()
        .register_theme_token(platform_color_theme_token("theme.text.primary", "#101820"))
        .register_theme_token(ThemeTokenDescriptor::alias(
            theme_token_id("theme.platform.accent_alias"),
            ThemeTokenFamily::accent(),
            ThemeTokenSource::plugin_alias(),
            ThemeTokenAlias::to(theme_token_id("theme.text.primary")),
        ))
        .freeze_with_registration_report();

    assert_diagnostic_codes(
        &report,
        &[CapabilityDiagnosticCode::PluginThemeTokenOverridesPlatformMeaning],
    );
    assert_registered_theme_token_ids(
        report.accepted_snapshot().theme_tokens(),
        &["theme.text.primary"],
    );
}

#[test]
fn plugin_custom_and_alias_tokens_stay_inside_runtime_registry() {
    let app = WorthUi::app()
        .register_theme_token(platform_color_theme_token(
            "theme.platform.accent",
            "#224466",
        ))
        .register_theme_token(plugin_custom_theme_token("plugin.panel.accent", "#336699"))
        .register_theme_token(plugin_alias_theme_token(
            "plugin.panel.accent.alias",
            "theme.platform.accent",
        ))
        .freeze();

    assert_registered_theme_token_ids(
        app.capabilities().theme_tokens(),
        &[
            "plugin.panel.accent",
            "plugin.panel.accent.alias",
            "theme.platform.accent",
        ],
    );
}

#[test]
fn plugin_alias_source_cannot_define_custom_value() {
    let report = WorthUi::app()
        .register_theme_token(ThemeTokenDescriptor::define(
            theme_token_id("plugin.bad.alias"),
            ThemeTokenFamily::accent(),
            ThemeTokenSource::plugin_alias(),
            color_value("#112233"),
        ))
        .freeze_with_registration_report();

    assert_diagnostic_codes(
        &report,
        &[CapabilityDiagnosticCode::PluginThemeTokenContributionKindMismatch],
    );
    assert!(report.accepted_snapshot().theme_tokens().is_empty());
}

#[test]
fn plugin_custom_source_cannot_define_alias() {
    let report = WorthUi::app()
        .register_theme_token(ThemeTokenDescriptor::alias(
            theme_token_id("plugin.bad.custom"),
            ThemeTokenFamily::accent(),
            ThemeTokenSource::plugin_custom(),
            ThemeTokenAlias::to(theme_token_id("theme.platform.accent")),
        ))
        .freeze_with_registration_report();

    assert_diagnostic_codes(
        &report,
        &[
            CapabilityDiagnosticCode::PluginThemeTokenContributionKindMismatch,
            CapabilityDiagnosticCode::MissingDependency,
        ],
    );
    assert!(report.accepted_snapshot().theme_tokens().is_empty());
}
