use worth_ui::facade::diagnostics::{CapabilityDiagnosticCode, CapabilityRegistrationDiagnostic};
use worth_ui_runtime::facade::registry::snapshot::FrozenMosaicPlacementCapabilities;

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

pub(crate) fn assert_registered_mosaic_placement_ids(
    mosaic_placements: &FrozenMosaicPlacementCapabilities,
    expected_policy_ids: &[&str],
) {
    let actual_policy_ids = mosaic_placements
        .descriptors()
        .iter()
        .map(|descriptor| descriptor.id().as_str())
        .collect::<Vec<_>>();
    assert_eq!(actual_policy_ids, expected_policy_ids);
}
