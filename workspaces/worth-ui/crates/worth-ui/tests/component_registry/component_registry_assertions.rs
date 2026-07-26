use worth_ui::facade::diagnostics::{CapabilityDiagnosticCode, CapabilityRegistrationDiagnostic};
use worth_ui_runtime::facade::registry::snapshot::FrozenComponentCapabilities;

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
                    .expect("diagnostic should identify component registration"),
            )
        })
        .collect::<Vec<_>>();
    assert_eq!(actual_codes_and_identities, expected_codes_and_identities);
}

pub(crate) fn assert_dependency_diagnostics(
    diagnostics: &[CapabilityRegistrationDiagnostic],
    expected_diagnostics: &[(CapabilityDiagnosticCode, &str, &str, &str)],
) {
    let actual_diagnostics = diagnostics
        .iter()
        .map(|diagnostic| {
            (
                diagnostic.code(),
                diagnostic
                    .identity_text()
                    .expect("diagnostic should identify component registration"),
                diagnostic
                    .related_family_name()
                    .expect("dependency diagnostic should identify target family"),
                diagnostic
                    .related_identity_text()
                    .expect("dependency diagnostic should identify target identity"),
            )
        })
        .collect::<Vec<_>>();
    let expected_diagnostics = expected_diagnostics
        .iter()
        .map(
            |(code, identity_text, related_family_name, related_identity_text)| {
                (
                    *code,
                    *identity_text,
                    *related_family_name,
                    *related_identity_text,
                )
            },
        )
        .collect::<Vec<_>>();

    assert_eq!(actual_diagnostics, expected_diagnostics);
}

pub(crate) fn assert_registered_component_ids(
    components: &FrozenComponentCapabilities,
    expected_component_ids: &[&str],
) {
    let actual_component_ids = components
        .descriptors()
        .iter()
        .map(|descriptor| descriptor.id().as_str())
        .collect::<Vec<_>>();
    assert_eq!(actual_component_ids, expected_component_ids);
}
