use worth_ui::facade::{
    diagnostics::{
        CapabilityDiagnosticCode, CapabilityRegistrationDiagnostic, CapabilityRegistrationReport,
    },
    registry::FrozenIconCapabilities,
};

pub(crate) fn assert_registered_icon_ids(registry: &FrozenIconCapabilities, expected_ids: &[&str]) {
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
        .map(CapabilityRegistrationDiagnostic::code)
        .collect::<Vec<_>>();
    assert_eq!(actual_codes, expected_codes);
}

pub(crate) fn assert_dependency_diagnostics(
    diagnostics: &[CapabilityRegistrationDiagnostic],
    expected: &[(&str, &str, &str)],
) {
    let actual = diagnostics
        .iter()
        .filter(|diagnostic| diagnostic.code() == CapabilityDiagnosticCode::MissingDependency)
        .map(|diagnostic| {
            (
                diagnostic.identity_text().expect("candidate identity"),
                diagnostic.related_family_name().expect("related family"),
                diagnostic
                    .related_identity_text()
                    .expect("related identity"),
            )
        })
        .collect::<Vec<_>>();
    assert_eq!(actual, expected);
}
