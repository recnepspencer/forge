use super::fixtures::{milestone_nine_closeout, operator_cutover_closeout};
use std::collections::BTreeSet;

#[test]
fn counters_match_the_phase_eight_cutover_evidence() {
    let cutover = operator_cutover_closeout();
    let closeout =
        crate::validator_invariant_catalog::WorthTopologyMilestoneNineCloseout::from_operator_cutover(
            cutover.phase_eight_seed(),
            &cutover,
        )
        .expect("Milestone 9 closeout should build");
    let counters = closeout.counters();
    assert_eq!(
        counters.selected_obligation_count(),
        cutover.selected_obligation_closeout_rows().len()
    );
    assert_eq!(
        counters.enforcement_receipt_count(),
        cutover
            .selected_obligation_closeout_rows()
            .iter()
            .filter(|row| !row.enforcement_receipt_digest().is_empty())
            .count()
    );
    assert_eq!(
        counters.worth_family_count(),
        cutover
            .selected_obligation_closeout_rows()
            .iter()
            .map(|row| row.worth_family_identity_digest())
            .collect::<BTreeSet<_>>()
            .len()
    );
    assert_eq!(
        counters.graph_read_receipt_count(),
        cutover
            .selected_obligation_closeout_rows()
            .iter()
            .filter(|row| !row.query_execution_row_digest().is_empty())
            .count()
    );
    assert_eq!(
        counters.budget_denial_count(),
        cutover
            .selected_obligation_closeout_rows()
            .iter()
            .filter(|row| row.query_execution_status() == "budget-exceeded")
            .count()
    );
    assert_eq!(
        counters.executor_row_count(),
        cutover
            .selected_obligation_closeout_rows()
            .iter()
            .filter(|row| row.query_execution_status() == "executed")
            .count()
    );
    assert_eq!(counters.stale_residue_row_count(), 0);
    assert_eq!(counters.uncapped_authority_count(), 0);
    assert_eq!(counters.source_firewall_violation_count(), 0);
}

#[test]
fn counters_preserve_exact_query_execution_and_consumer_kit_counts() {
    let closeout = milestone_nine_closeout();
    assert_eq!(
        closeout.counters().graph_read_receipt_count(),
        closeout.counters().selected_obligation_count()
    );
    assert_eq!(closeout.counters().support_pin_count(), 1);
    assert_eq!(
        closeout.counters().execution_backed_adoption_proof_count(),
        1
    );
    assert_eq!(
        closeout.counters().residue_manifest_count(),
        usize::from(
            !closeout
                .milestone_ten_seed()
                .residue_manifest_digest()
                .is_empty()
        )
    );
    assert_eq!(
        closeout.counters().deletion_ledger_row_count(),
        closeout.deletion_ledger().rows().len()
    );
    assert_eq!(
        closeout.counters().capped_residue_row_count(),
        closeout.residue_audit().capped_residue_count()
    );
    assert_eq!(
        closeout.counters().whole_view_certification_only_count(),
        closeout
            .deletion_ledger()
            .whole_view_certification_only_count()
    );
}
