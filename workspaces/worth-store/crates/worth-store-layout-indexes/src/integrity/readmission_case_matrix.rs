use super::readmission_test_support::{
    authoritative_quarantine_record, current_authority, current_security_scope,
    record_backed_witness_for_store,
};
use super::tests::{admitted_family, family};
use super::{
    layout_corruption, quarantine_readmission, CorruptionDenial, LayoutReadmissionSource,
    QuarantineReadmissionView,
};

#[test]
fn quarantine_readmission_rejects_equal_record_evidence_from_another_store() {
    let record = authoritative_quarantine_record("cross-store-quarantine");
    let required = layout_corruption()
        .require_record_backed_recovery_readmission(
            layout_corruption().assess_physical_quarantine(admitted_family(), record.clone()),
            &current_authority("store.new.strategy", "cross-store-quarantine"),
            current_security_scope("store.new.strategy", "cross-store-quarantine").witnesses(),
        )
        .unwrap()
        .into_quarantine_readmission_requirement()
        .unwrap();
    let foreign = record_backed_witness_for_store(
        family(),
        &record,
        "store.new.corruption",
        "cross-store-quarantine",
    );

    assert!(matches!(
        quarantine_readmission().admit(required, foreign).view(),
        QuarantineReadmissionView::Denied(denied)
            if matches!(denied.denial(), CorruptionDenial::FamilyBoundReadmissionWitnessRequired {
                family: actual_family,
                source: LayoutReadmissionSource::QuarantineRecovery,
            } if *actual_family == family())
    ));
}
