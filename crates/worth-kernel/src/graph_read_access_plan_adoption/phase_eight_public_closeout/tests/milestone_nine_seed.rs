use super::phase_chain_fixture::production_phase_eight_closeout;

#[test]
fn milestone_nine_seed_preserves_phase_eight_closeout_identity() {
    let closeout = production_phase_eight_closeout();
    let seed = closeout.milestone_nine_seed();

    assert_eq!(
        seed.milestone_eight_closeout_digest(),
        closeout.closeout_digest()
    );
    assert_eq!(seed.receipts(), closeout.receipts());
    assert_eq!(seed.postures(), closeout.postures());
    assert_eq!(seed.counters(), closeout.counters());
    assert_eq!(
        seed.counter_accounting_report(),
        closeout.counter_accounting_report()
    );
    assert_eq!(
        seed.batch_accounting_report(),
        closeout.batch_accounting_report()
    );
    assert_eq!(seed.deletion(), closeout.deletion());
    assert_eq!(seed.residue(), closeout.residue());
    assert_eq!(seed.source_firewall(), closeout.source_firewall());
    assert_eq!(
        seed.bounded_execution_contract(),
        closeout.bounded_execution_contract()
    );
    assert_eq!(
        seed.phase_four_cutover_proof(),
        closeout.phase_four_cutover_proof()
    );
    assert!(!seed.seed_digest().is_empty());
}

#[test]
fn closeout_and_milestone_nine_seed_do_not_claim_validator_selection() {
    let closeout = production_phase_eight_closeout();

    assert!(!closeout.claims_validator_selection());
    assert!(!closeout.milestone_nine_seed().claims_validator_selection());
}
