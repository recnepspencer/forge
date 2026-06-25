use super::fixtures::operator_cutover_closeout;

#[test]
fn public_proof_is_query_backed_and_not_selection_only() {
    let cutover = operator_cutover_closeout();
    let closeout =
        crate::validator_invariant_catalog::WorthTopologyMilestoneNineCloseout::from_operator_cutover(
            cutover.phase_eight_seed(),
            &cutover,
        )
        .expect("Milestone 9 closeout should build");
    let proof = closeout.public_proof();
    assert!(proof.old_authority_closed());
    assert!(proof.ordinary_operator_paths_query_backed());
    assert_eq!(
        proof.selected_obligation_row_digests(),
        cutover
            .selected_obligation_closeout_rows()
            .iter()
            .map(|row| row.row_digest().to_string())
            .collect::<Vec<_>>()
    );
    assert_eq!(
        proof.query_selected_obligation_digests(),
        cutover
            .selected_obligation_closeout_rows()
            .iter()
            .map(|row| row.query_rule_identity_digest().to_string())
            .collect::<Vec<_>>()
    );
    assert_eq!(
        proof.enforcement_receipt_digests(),
        cutover
            .selected_obligation_closeout_rows()
            .iter()
            .map(|row| row.enforcement_receipt_digest().to_string())
            .collect::<Vec<_>>()
    );
    assert_eq!(
        proof.execution_proof_digest(),
        cutover.execution_proof_digest()
    );
    assert_eq!(proof.support_pin_digest(), cutover.support_pin_digest());
    assert_eq!(
        proof.adoption_manifest_digest(),
        cutover.execution_backed_adoption_manifest_digest()
    );
    assert_eq!(
        proof.residue_manifest_digest(),
        cutover.residue_manifest_digest()
    );
    assert_eq!(proof.closeout_digest(), closeout.closeout_digest());
}
