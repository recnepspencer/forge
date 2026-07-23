use worth_ui::facade::{
    diagnostics::{CapabilityDiagnosticCode, CapabilityRegistrationDiagnostic},
    registry::FrozenRuntimeOutcomeProjectionCapabilities,
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

pub(crate) fn assert_registered_runtime_outcome_projection_ids(
    projections: &FrozenRuntimeOutcomeProjectionCapabilities,
    expected_projection_ids: &[&str],
) {
    let actual_projection_ids = projections
        .entries()
        .iter()
        .map(|entry| entry.descriptor().id().as_str())
        .collect::<Vec<_>>();
    assert_eq!(actual_projection_ids, expected_projection_ids);
}
