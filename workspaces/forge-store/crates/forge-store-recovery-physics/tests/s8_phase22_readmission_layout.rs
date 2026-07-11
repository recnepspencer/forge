mod phase22_fixture;

use forge_store_layout_indexes::layout_strategy_admission::phase22_readmission_rule;
use forge_store_recovery_physics::{RecoveryLayoutAccess, RecoveryLayoutReadmissionClass};

#[test]
fn phase22_readmission_family_binds_record_backed_readmission_to_an_admitted_lane() {
    let record = phase22_fixture::authoritative_quarantine_record("phase22-readmission");
    let authority =
        phase22_fixture::current_authority("store.s8.phase22.readmission", "phase22-readmission");
    let family = RecoveryLayoutAccess::s8()
        .readmission_layout(&phase22_readmission_rule().unwrap())
        .expect("readmission family");
    let witness = family
        .admit_record_backed_witness(phase22_fixture::recovery_family_id(), &record, &authority)
        .expect("record-backed witness");
    let report = family.report_for(&witness);
    assert_eq!(report.family_id(), phase22_fixture::recovery_family_id());
    assert_eq!(
        report.class(),
        RecoveryLayoutReadmissionClass::QuarantineRecovery
    );
}
