use super::support as phase22_fixture;

use worth_store_recovery_physics::{
    layout_readmission, RecoveryLayoutReadmissionClass, RecoveryReadmissionLayoutReport,
};

#[test]
fn readmission_binds_record_backed_readmission_to_an_admitted_lane() {
    let record = phase22_fixture::authoritative_quarantine_record("phase22-readmission");
    let authority =
        phase22_fixture::current_authority("store.new.phase22.readmission", "phase22-readmission");
    let security = phase22_fixture::current_security_scope(
        "store.new.phase22.readmission",
        "phase22-readmission",
    );
    let witness = layout_readmission()
        .admit_quarantine(
            phase22_fixture::recovery_family_id(),
            &record,
            &authority,
            security.witnesses(),
        )
        .expect("record-backed witness");
    let report = RecoveryReadmissionLayoutReport::from_witness(&witness);
    assert_eq!(report.family_id(), phase22_fixture::recovery_family_id());
    assert_eq!(
        report.class(),
        RecoveryLayoutReadmissionClass::QuarantineRecovery
    );
}
