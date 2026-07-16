use worth_store_formal_models::{
    map_quarantine_record, QuarantineReadmissionDenial, QuarantineReadmissionModel,
    QuarantineReadmissionState,
};
use worth_store_test_support::harness::recovery::source_precedence::wal_tail_quarantine_record;

use super::scenario::execute_ordinary_quarantine_entry;

#[test]
fn ordinary_owner_execution_covers_every_quarantine_readmission_state() {
    let mut observed = execute_ordinary_quarantine_entry();
    observed.sort_by_key(|state| *state as u8);
    observed.dedup();

    assert_eq!(observed, QuarantineReadmissionState::all());
}

#[test]
fn real_quarantine_record_maps_without_gaining_repair_authority() {
    let record = wal_tail_quarantine_record();
    let observation = map_quarantine_record(&record);

    assert!(!observation.receipt_digest().is_empty());
    assert!(!observation.proves_repair());
    assert!(!record.proves_recovery());
}

#[test]
fn readmission_requires_exact_scope_verification_and_current_authority() {
    let mut model = QuarantineReadmissionModel::sealed("segment:7/page:2/generation:4");
    model.begin_verification();
    assert_eq!(
        model.readmit("segment:7/page:2/generation:4", false, true),
        Err(QuarantineReadmissionDenial::VerificationFrontierIncomplete)
    );
    assert_eq!(
        model.readmit("segment:7/page:9/generation:4", true, true),
        Err(QuarantineReadmissionDenial::ScopeMismatch)
    );
    assert_eq!(
        model.readmit("segment:7/page:2/generation:4", true, false),
        Err(QuarantineReadmissionDenial::CurrentAuthorityRequired)
    );
    model
        .readmit("segment:7/page:2/generation:4", true, true)
        .unwrap();
    assert_eq!(model.state(), QuarantineReadmissionState::Readmitted);
}

#[test]
fn observation_and_operator_intent_are_false_authority_mutants() {
    assert_eq!(
        QuarantineReadmissionModel::reject_offline_observation(),
        QuarantineReadmissionDenial::ObservationIsNotRepairAuthority
    );
    assert_eq!(
        QuarantineReadmissionModel::reject_operator_repair(),
        QuarantineReadmissionDenial::OperatorIntentIsNotRepairAuthority
    );
}
