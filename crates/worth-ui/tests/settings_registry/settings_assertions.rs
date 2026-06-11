use worth_ui::facade::{
    CapabilityDiagnosticCode, CapabilityRegistrationDiagnostic, FrozenSettingCapabilities,
};

pub(crate) fn assert_diagnostic_codes(
    diagnostics: &[CapabilityRegistrationDiagnostic],
    expected: &[CapabilityDiagnosticCode],
) {
    let actual = diagnostics
        .iter()
        .map(CapabilityRegistrationDiagnostic::code)
        .collect::<Vec<_>>();
    assert_eq!(actual, expected);
}

pub(crate) fn assert_registered_setting_ids(
    settings: &FrozenSettingCapabilities,
    expected: &[&str],
) {
    let actual = settings
        .entries()
        .iter()
        .map(|entry| entry.descriptor().id().as_str())
        .collect::<Vec<_>>();
    assert_eq!(actual, expected);
}
