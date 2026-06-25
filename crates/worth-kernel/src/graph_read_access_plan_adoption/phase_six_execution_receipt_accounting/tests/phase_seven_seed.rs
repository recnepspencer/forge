use super::production_phase_six_closeout;

#[test]
fn phase_seven_seed_preserves_receipt_and_counter_accounting() {
    let closeout = production_phase_six_closeout();
    let seed = closeout.phase_seven_seed();

    assert_eq!(
        seed.receipt_accounting_report().report_digest(),
        closeout.receipt_accounting_report().report_digest()
    );
    assert_eq!(
        seed.counter_accounting_report().report_digest(),
        closeout.counter_accounting_report().report_digest()
    );
    assert_eq!(
        seed.batch_accounting_report().report_digest(),
        closeout.batch_accounting_report().report_digest()
    );
    assert!(!seed.claims_validator_selection());
}
