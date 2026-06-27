use worth_ui::facade::{
    CapabilityDiagnosticCode, CapabilityRegistrationDiagnostic, FrozenViewBindingCapabilities,
};

pub(crate) fn assert_diagnostic_codes(
    diagnostics: &[CapabilityRegistrationDiagnostic],
    expected_codes: &[CapabilityDiagnosticCode],
) {
    let actual_codes = diagnostics
        .iter()
        .map(|diagnostic| diagnostic.code())
        .collect::<Vec<_>>();
    assert_eq!(actual_codes, expected_codes);
}

pub(crate) fn assert_registered_view_binding_ids(
    view_bindings: &FrozenViewBindingCapabilities,
    expected_binding_ids: &[&str],
) {
    let actual_binding_ids = view_bindings
        .entries()
        .iter()
        .map(|entry| entry.descriptor().id().as_str())
        .collect::<Vec<_>>();
    assert_eq!(actual_binding_ids, expected_binding_ids);
}
