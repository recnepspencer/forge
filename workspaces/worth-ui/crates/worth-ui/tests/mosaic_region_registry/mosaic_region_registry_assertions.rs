use worth_ui::facade::diagnostics::{CapabilityDiagnosticCode, CapabilityRegistrationDiagnostic};
use worth_ui_runtime::facade::registry::snapshot::FrozenMosaicRegionCapabilities;

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
                    .expect("diagnostic should identify mosaic region registration"),
            )
        })
        .collect::<Vec<_>>();
    assert_eq!(actual_codes_and_identities, expected_codes_and_identities);
}

pub(crate) fn assert_registered_mosaic_region_ids(
    mosaic_regions: &FrozenMosaicRegionCapabilities,
    expected_region_ids: &[&str],
) {
    let actual_region_ids = mosaic_regions
        .descriptors()
        .iter()
        .map(|descriptor| descriptor.id().as_str())
        .collect::<Vec<_>>();
    assert_eq!(actual_region_ids, expected_region_ids);
}

pub(crate) fn assert_exact_diagnostic_topology(
    diagnostics: &[CapabilityRegistrationDiagnostic],
    expected_topology: &[DiagnosticTopology],
) {
    let actual_topology = diagnostics
        .iter()
        .map(diagnostic_topology)
        .collect::<Vec<_>>();

    assert_eq!(actual_topology, expected_topology);
}

#[derive(Debug, Eq, PartialEq)]
pub(crate) struct DiagnosticTopology<'a> {
    code: CapabilityDiagnosticCode,
    identity_text: &'a str,
}

impl<'a> DiagnosticTopology<'a> {
    pub(crate) fn new(code: CapabilityDiagnosticCode, identity_text: &'a str) -> Self {
        Self {
            code,
            identity_text,
        }
    }
}

fn diagnostic_topology(diagnostic: &CapabilityRegistrationDiagnostic) -> DiagnosticTopology<'_> {
    DiagnosticTopology {
        code: diagnostic.code(),
        identity_text: diagnostic
            .identity_text()
            .expect("diagnostic should identify the invalid mosaic region"),
    }
}
