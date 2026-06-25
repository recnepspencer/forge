use super::{production_phase_seven_closeout, production_phase_seven_seed};

#[test]
fn phase_eight_seed_carries_deletion_and_firewall_proof_without_validator_selection() {
    let source_seed = production_phase_seven_seed();
    let closeout = production_phase_seven_closeout();
    let seed = closeout.phase_eight_seed();

    assert_eq!(
        seed.phase_seven_closeout_digest(),
        closeout.closeout_digest()
    );
    assert_eq!(
        seed.deletion_proof_report(),
        closeout.deletion_proof_report()
    );
    assert_eq!(
        seed.capped_residue_report(),
        closeout.capped_residue_report()
    );
    assert_eq!(
        seed.source_firewall_report(),
        closeout.source_firewall_report()
    );
    assert_eq!(
        seed.receipt_accounting_report().report_digest(),
        source_seed.receipt_accounting_report().report_digest()
    );
    assert_eq!(
        seed.counter_accounting_report().report_digest(),
        source_seed.counter_accounting_report().report_digest()
    );
    assert_eq!(
        seed.batch_accounting_report().report_digest(),
        source_seed.batch_accounting_report().report_digest()
    );
    assert_eq!(
        seed.prior_source_firewall_report().report_digest(),
        source_seed.source_firewall_report().report_digest()
    );
    assert_eq!(
        seed.bounded_execution_contract().contract_digest(),
        source_seed.bounded_execution_contract().contract_digest()
    );
    assert_eq!(
        seed.phase_four_cutover_proof().cutover_digest(),
        source_seed.phase_four_cutover_proof().cutover_digest()
    );
    assert!(!seed.claims_validator_selection());
    assert!(!closeout.claims_validator_selection());
}
