use worth_ui::facade::{
    CapabilityDiagnosticCode, CapabilityRegistrationDiagnostic, SnapshotReferenceValidationReport,
    SnapshotReferenceViolationKind,
};

pub(crate) fn diagnostic_codes(
    diagnostics: &[CapabilityRegistrationDiagnostic],
) -> Vec<CapabilityDiagnosticCode> {
    diagnostics
        .iter()
        .map(CapabilityRegistrationDiagnostic::code)
        .collect()
}

pub(crate) type CapabilityDiagnosticTopology = (
    CapabilityDiagnosticCode,
    Option<String>,
    Option<String>,
    Option<String>,
    Option<String>,
);

pub(crate) fn diagnostic_topology(
    diagnostics: &[CapabilityRegistrationDiagnostic],
) -> Vec<CapabilityDiagnosticTopology> {
    diagnostics
        .iter()
        .map(|diagnostic| {
            (
                diagnostic.code(),
                diagnostic.family_name().map(str::to_owned),
                diagnostic.identity_text().map(str::to_owned),
                diagnostic.related_family_name().map(str::to_owned),
                diagnostic.related_identity_text().map(str::to_owned),
            )
        })
        .collect()
}

pub(crate) fn violation_kinds(
    report: &SnapshotReferenceValidationReport,
) -> Vec<SnapshotReferenceViolationKind> {
    report
        .violations()
        .iter()
        .map(|violation| violation.kind())
        .collect()
}
