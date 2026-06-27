use worth_ui::facade::{
    CapabilityDiagnosticCode, CapabilityRegistrationDiagnostic, FrozenMosaicSizingCapabilities,
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

pub(crate) fn assert_registered_mosaic_sizing_ids(
    sizing_contracts: &FrozenMosaicSizingCapabilities,
    expected_contract_ids: &[&str],
) {
    let actual_contract_ids = sizing_contracts
        .descriptors()
        .iter()
        .map(|descriptor| descriptor.id().as_str())
        .collect::<Vec<_>>();
    assert_eq!(actual_contract_ids, expected_contract_ids);
}
