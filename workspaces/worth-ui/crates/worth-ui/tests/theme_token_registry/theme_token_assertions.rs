use worth_ui::facade::diagnostics::{CapabilityDiagnosticCode, CapabilityRegistrationReport};
use worth_ui_runtime::facade::registry::snapshot::FrozenThemeTokenCapabilities;

pub(crate) fn assert_registered_theme_token_ids(
    registry: &FrozenThemeTokenCapabilities,
    expected_ids: &[&str],
) {
    let actual_ids = registry
        .entries()
        .iter()
        .map(|entry| entry.descriptor().id().as_str())
        .collect::<Vec<_>>();
    assert_eq!(actual_ids, expected_ids);
}

pub(crate) fn assert_diagnostic_codes(
    report: &CapabilityRegistrationReport,
    expected_codes: &[CapabilityDiagnosticCode],
) {
    let actual_codes = report
        .registration_diagnostics()
        .iter()
        .map(|diagnostic| diagnostic.code())
        .collect::<Vec<_>>();
    assert_eq!(actual_codes, expected_codes);
}
