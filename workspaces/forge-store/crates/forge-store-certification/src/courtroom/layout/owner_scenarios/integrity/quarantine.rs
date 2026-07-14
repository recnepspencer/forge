use forge_store_layout_indexes::integrity::{layout_corruption, quarantine_readmission};
use forge_store_layout_indexes::ObserveOwnerCase;
use forge_store_test_support::harness::layout::authoritative_layout_quarantine_record;

use super::fixture_authority::{authority, import_witness};
use super::LayoutOwnerObservationLedger;

pub(super) fn record(ledger: &mut LayoutOwnerObservationLedger) {
    let (fixture, family) = authority("integrity-quarantine-matrix");
    let record = authoritative_layout_quarantine_record("integrity-quarantine-matrix");
    let requirement = || {
        layout_corruption()
            .require_record_backed_recovery_readmission(
                layout_corruption().assess_physical_quarantine(family, record.clone()),
                fixture.current_authority(),
                fixture.security_scope().witnesses(),
            )
            .unwrap()
            .into_quarantine_readmission_requirement()
            .unwrap()
    };
    let admitted = forge_store_recovery_physics::layout_readmission()
        .admit_quarantine(
            family.family_id(),
            &record,
            fixture.current_authority(),
            fixture.security_scope().witnesses(),
        )
        .expect("physical quarantine evidence must readmit through recovery");
    let imported = import_witness(family, &fixture, "integrity-quarantine-import");
    let other_record = authoritative_layout_quarantine_record("integrity-quarantine-other");
    let other = forge_store_recovery_physics::layout_readmission()
        .admit_quarantine(
            family.family_id(),
            &other_record,
            fixture.current_authority(),
            fixture.security_scope().witnesses(),
        )
        .expect("other physical quarantine remains valid lower evidence");

    for witness in [admitted, imported, other] {
        let outcome = quarantine_readmission().admit(requirement(), witness);
        ledger.record_quarantine_readmission(outcome.owner_case_observation());
    }
}
