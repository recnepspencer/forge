use super::support as phase22_fixture;

use forge_store_recovery_physics::{
    admit_record_backed_layout_readmission, RecoveryLayoutReadmissionClass,
    RecoveryReadmissionLayoutReport,
};

#[test]
fn phase22_readmission_binds_record_backed_readmission_to_an_admitted_lane() {
    let record = phase22_fixture::authoritative_quarantine_record("phase22-readmission");
    let authority =
        phase22_fixture::current_authority("store.s8.phase22.readmission", "phase22-readmission");
    let witness = admit_record_backed_layout_readmission(
        phase22_fixture::recovery_family_id(),
        &record,
        &authority,
    )
    .expect("record-backed witness");
    let report = RecoveryReadmissionLayoutReport::from_witness(&witness);
    assert_eq!(report.family_id(), phase22_fixture::recovery_family_id());
    assert_eq!(
        report.class(),
        RecoveryLayoutReadmissionClass::QuarantineRecovery
    );
}
