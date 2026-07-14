use worth_store_layout_indexes::integrity::{import_readmission, layout_corruption};
use worth_store_layout_indexes::ObserveOwnerCase;
use worth_store_test_support::harness::layout::authoritative_layout_quarantine_record;

use super::fixture_authority::{authority, import_witness};
use super::LayoutOwnerObservationLedger;

pub(super) fn record(ledger: &mut LayoutOwnerObservationLedger) {
    let (fixture, family) = authority("integrity-import-matrix");
    let expected = import_witness(family, &fixture, "integrity-import-matrix");
    let requirement = || {
        layout_corruption()
            .require_import_readmission(family, expected.clone())
            .into_import_readmission_requirement()
            .unwrap()
    };
    let other = import_witness(family, &fixture, "integrity-import-other");
    let quarantine_record = authoritative_layout_quarantine_record("integrity-import-quarantine");
    let quarantine = worth_store_recovery_physics::layout_readmission()
        .admit_quarantine(
            family.family_id(),
            &quarantine_record,
            fixture.current_authority(),
            fixture.security_scope().witnesses(),
        )
        .expect("quarantine evidence must retain its lower-owner class");

    for witness in [expected.clone(), other, quarantine] {
        let outcome = import_readmission().admit(requirement(), witness);
        ledger.record_import_readmission(outcome.owner_case_observation());
    }
}
