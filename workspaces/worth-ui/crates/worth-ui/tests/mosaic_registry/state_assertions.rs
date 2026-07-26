use worth_ui::facade::diagnostics::{CapabilityDiagnosticCode, CapabilityRegistrationDiagnostic};
use worth_ui_runtime::facade::registry::snapshot::FrozenMosaicStateCapabilities;

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

pub(crate) fn assert_registered_mosaic_state_slot_ids(
    state_slots: &FrozenMosaicStateCapabilities,
    expected_slot_ids: &[&str],
) {
    let actual_slot_ids = state_slots
        .entries()
        .iter()
        .map(|entry| entry.descriptor().id().as_str())
        .collect::<Vec<_>>();
    assert_eq!(actual_slot_ids, expected_slot_ids);
}

pub(crate) fn assert_reconciliation_keys(
    state_slots: &FrozenMosaicStateCapabilities,
    expected_keys: &[&str],
) {
    let actual_keys = state_slots
        .entries()
        .iter()
        .map(|entry| entry.reconciliation_key().as_str())
        .collect::<Vec<_>>();
    assert_eq!(actual_keys, expected_keys);
}
