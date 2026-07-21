use worth_ui::facade::{
    diagnostics::{CapabilityDiagnosticCode, CapabilityRegistrationDiagnostic},
    registry::FrozenNativeCapabilities,
};

pub(crate) fn assert_registered_native_capability_ids(
    native_capabilities: &FrozenNativeCapabilities,
    expected_ids: &[&str],
) {
    let actual_ids = native_capabilities
        .entries()
        .iter()
        .map(|entry| entry.descriptor().id().as_str())
        .collect::<Vec<_>>();
    assert_eq!(actual_ids, expected_ids);
}

pub(crate) fn assert_diagnostic_codes(
    diagnostics: &[CapabilityRegistrationDiagnostic],
    expected_codes: &[CapabilityDiagnosticCode],
) {
    let actual_codes = diagnostics
        .iter()
        .map(CapabilityRegistrationDiagnostic::code)
        .collect::<Vec<_>>();
    assert_eq!(actual_codes, expected_codes);
}
