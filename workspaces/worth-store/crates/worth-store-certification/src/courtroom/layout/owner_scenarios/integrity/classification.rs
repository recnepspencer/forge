use worth_store_layout_indexes::integrity::layout_corruption;
use worth_store_layout_indexes::ObserveOwnerCase;
use worth_store_test_support::harness::layout::authoritative_layout_quarantine_record;

use super::fixture_authority::{authority, import_witness};
use super::LayoutOwnerObservationLedger;

pub(super) fn record(ledger: &mut LayoutOwnerObservationLedger) {
    let (fixture, family) = authority("integrity-classification");
    let record = authoritative_layout_quarantine_record("integrity-classification");
    let quarantine = layout_corruption().assess_physical_quarantine(family, record.clone());
    ledger.record_corruption_classification(quarantine.owner_case_observation());
    let required = layout_corruption()
        .require_record_backed_recovery_readmission(
            quarantine,
            fixture.current_authority(),
            fixture.security_scope().witnesses(),
        )
        .expect("record-backed quarantine must require explicit readmission");
    ledger.record_corruption_classification(required.owner_case_observation());

    let imported = import_witness(family, &fixture, "integrity-classification-import");
    let import = layout_corruption().require_import_readmission(family, imported);
    ledger.record_corruption_classification(import.owner_case_observation());
}
