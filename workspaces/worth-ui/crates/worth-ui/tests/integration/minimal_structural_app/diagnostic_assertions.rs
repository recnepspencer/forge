use worth_ui::facade::diagnostics::{CapabilityDiagnosticCode, CapabilityRegistrationDiagnostic};

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

pub(crate) fn assert_diagnostic_codes_and_identities(
    diagnostics: &[CapabilityRegistrationDiagnostic],
    expected_codes_and_identities: &[(CapabilityDiagnosticCode, &str)],
) {
    let actual_codes_and_identities = diagnostics
        .iter()
        .map(|diagnostic| {
            (
                diagnostic.code(),
                diagnostic
                    .identity_text()
                    .expect("diagnostic should name the rejected capability"),
            )
        })
        .collect::<Vec<_>>();

    assert_eq!(actual_codes_and_identities, expected_codes_and_identities);
}
