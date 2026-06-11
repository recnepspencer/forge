use worth_ui::facade::{
    CapabilityDiagnosticCode, CapabilityRegistrationDiagnostic, FrozenTaskPresentationCapabilities,
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

pub(crate) fn assert_registered_task_presentation_ids(
    task_presentations: &FrozenTaskPresentationCapabilities,
    expected: &[&str],
) {
    let actual = task_presentations
        .entries()
        .iter()
        .map(|entry| entry.descriptor().id().as_str())
        .collect::<Vec<_>>();
    assert_eq!(actual, expected);
}
