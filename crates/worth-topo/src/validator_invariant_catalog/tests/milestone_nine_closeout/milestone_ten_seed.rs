use super::fixtures::operator_cutover_closeout;

#[test]
fn milestone_ten_seed_carries_the_closeout_without_claiming_invalidation() {
    let cutover = operator_cutover_closeout();
    let closeout =
        crate::validator_invariant_catalog::WorthTopologyMilestoneNineCloseout::from_operator_cutover(
            cutover.phase_eight_seed(),
            &cutover,
        )
        .expect("Milestone 9 closeout should build");
    let seed = closeout.milestone_ten_seed();
    assert_eq!(
        seed.phase_nine_closeout_digest(),
        closeout.closeout_digest()
    );
    assert_eq!(
        seed.phase_eight_cutover_seed_digest(),
        closeout.phase_eight_cutover_seed_digest()
    );
    assert_eq!(
        seed.phase_six_seed_digest(),
        cutover.phase_six_seed_digest()
    );
    assert_eq!(
        seed.phase_seven_enforcement_seed_digest(),
        cutover.phase_seven_enforcement_seed_digest()
    );
    assert_eq!(
        seed.routing_closure_digest(),
        cutover.routing_closure_digest()
    );
    assert_eq!(
        seed.query_execution_envelope_digest(),
        cutover.query_execution_envelope_digest()
    );
    assert_eq!(
        seed.selected_obligation_row_digests(),
        cutover
            .selected_obligation_closeout_rows()
            .iter()
            .map(|row| row.row_digest().to_string())
            .collect::<Vec<_>>()
    );
    assert_eq!(
        seed.query_selected_obligation_digests(),
        cutover
            .selected_obligation_closeout_rows()
            .iter()
            .map(|row| row.query_rule_identity_digest().to_string())
            .collect::<Vec<_>>()
    );
    assert_eq!(
        seed.enforcement_receipt_digests(),
        cutover
            .selected_obligation_closeout_rows()
            .iter()
            .map(|row| row.enforcement_receipt_digest().to_string())
            .collect::<Vec<_>>()
    );
    assert_eq!(
        seed.query_execution_row_digests(),
        cutover
            .selected_obligation_closeout_rows()
            .iter()
            .map(|row| row.query_execution_row_digest().to_string())
            .collect::<Vec<_>>()
    );
    assert_eq!(
        seed.support_posture_digests(),
        cutover
            .selected_obligation_closeout_rows()
            .iter()
            .map(|row| row.query_support_posture_digest().to_string())
            .collect::<Vec<_>>()
    );
    assert_eq!(
        seed.execution_backed_adoption_manifest_digest(),
        cutover.execution_backed_adoption_manifest_digest()
    );
    assert_eq!(seed.support_pin_digest(), cutover.support_pin_digest());
    assert_eq!(
        seed.support_matrix_digest(),
        cutover.support_matrix_digest()
    );
    assert_eq!(
        seed.residue_manifest_digest(),
        cutover.residue_manifest_digest()
    );
    assert_eq!(
        seed.local_ceremony_audit_digest(),
        cutover.local_ceremony_audit_digest()
    );
    assert_eq!(
        seed.in_memory_proof_digest(),
        cutover.in_memory_proof_digest()
    );
    assert_eq!(
        seed.execution_proof_digest(),
        cutover.execution_proof_digest()
    );
    assert!(!seed.claims_invalidation_planning());
}

#[test]
fn closeout_and_seed_identities_are_canonical_hashes_not_joined_proof_parts() {
    let cutover = operator_cutover_closeout();
    let closeout =
        crate::validator_invariant_catalog::WorthTopologyMilestoneNineCloseout::from_operator_cutover(
            cutover.phase_eight_seed(),
            &cutover,
        )
        .expect("Milestone 9 closeout should build");
    assert_canonical_hash(closeout.closeout_digest());
    assert_canonical_hash(closeout.milestone_ten_seed().seed_digest());
}

fn assert_canonical_hash(digest: &str) {
    assert_eq!(digest.len(), 64);
    assert!(
        digest
            .chars()
            .all(|character| character.is_ascii_hexdigit()),
        "{digest} must be hexadecimal"
    );
    assert!(
        !digest.contains('|'),
        "{digest} must not expose joined proof parts"
    );
}
