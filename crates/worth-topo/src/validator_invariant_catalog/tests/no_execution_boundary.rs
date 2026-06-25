use super::production_phase_two_closeout;

#[test]
fn phase_two_closeout_does_not_claim_selection_or_enforcement() {
    let closeout = production_phase_two_closeout();
    let seed = closeout.phase_three_seed();
    let no_execution = closeout.no_execution_proof();

    assert!(!closeout.claims_selected_obligations());
    assert!(!closeout.claims_enforcement_receipts());
    assert!(!no_execution.claims_selected_obligations());
    assert!(!no_execution.claims_enforcement_receipts());
    assert!(no_execution.selected_obligation_digests().is_empty());
    assert!(no_execution.enforcement_receipt_digests().is_empty());
    assert_eq!(
        seed.no_execution_proof_digest(),
        no_execution.proof_digest()
    );
    assert!(!seed.claims_validator_selection());
    assert_eq!(seed.selected_obligation_count(), 0);
    assert_eq!(seed.enforcement_receipt_count(), 0);
    assert_eq!(seed.validator_family_count(), 5);
    assert_eq!(seed.invariant_family_count(), 14);
}
