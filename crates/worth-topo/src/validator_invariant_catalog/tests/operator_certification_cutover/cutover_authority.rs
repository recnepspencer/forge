use super::fixtures::rewire_operator_enforcement_closeout;
use crate::validator_invariant_catalog::WorthTopologyOperatorCertificationCutoverCloseout;

#[test]
fn operator_cutover_consumes_selected_obligation_receipts_as_closeout_authority() {
    let enforcement = rewire_operator_enforcement_closeout();
    let cutover =
        WorthTopologyOperatorCertificationCutoverCloseout::from_selected_graph_obligation_enforcement(
            &enforcement,
        )
        .expect("Phase 7 operator cutover should close from selected obligation receipts");

    assert_eq!(
        cutover.selected_obligation_closeout_rows().len(),
        enforcement.enforcement_receipts().len()
    );
    assert_eq!(
        cutover.counters().selected_obligation_closeout_row_count(),
        enforcement.enforcement_receipts().len()
    );
    assert_eq!(
        cutover.counters().executed_obligation_count(),
        enforcement.enforcement_receipts().len()
    );
    assert_eq!(cutover.counters().source_firewall_violation_count(), 0);
    assert_eq!(
        cutover.counters().scanned_source_file_count(),
        cutover.source_firewall().scanned_source_paths().len()
    );
    assert_eq!(
        cutover
            .counters()
            .uncapped_old_expectation_authority_count(),
        0
    );
    assert_eq!(
        cutover
            .counters()
            .capped_old_expectation_residue_row_count(),
        cutover.old_expectation_residue().rows().len()
    );
    assert!(cutover.old_expectation_residue().is_capped());
    assert_eq!(
        cutover
            .phase_eight_seed()
            .selected_obligation_row_digests()
            .len(),
        cutover.selected_obligation_closeout_rows().len()
    );
    assert_eq!(
        cutover
            .phase_eight_seed()
            .phase_seven_enforcement_seed_digest(),
        enforcement.phase_seven_seed().seed_digest()
    );
    assert!(cutover
        .selected_obligation_closeout_rows()
        .iter()
        .all(|row| row.query_support_lane() == "worth-topo-operator-catalog"));
}
