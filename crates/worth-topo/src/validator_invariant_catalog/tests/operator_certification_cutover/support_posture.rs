use super::fixtures::rewire_operator_enforcement_closeout;
use crate::validator_invariant_catalog::WorthTopologyOperatorCertificationCutoverCloseout;

#[test]
fn support_posture_rows_keep_query_lane_and_status_visible() {
    let enforcement = rewire_operator_enforcement_closeout();
    let cutover =
        WorthTopologyOperatorCertificationCutoverCloseout::from_selected_graph_obligation_enforcement(
            &enforcement,
        )
        .expect("Phase 7 operator cutover should close");

    assert_eq!(cutover.support_posture_rows().len(), 1);
    let row = &cutover.support_posture_rows()[0];
    assert_eq!(row.query_support_lane(), "worth-topo-operator-catalog");
    assert_eq!(row.query_support_status(), "supported");
    assert_eq!(
        row.receipt_count(),
        enforcement.enforcement_receipts().len()
    );
    assert!(!row.query_execution_budget_digests().is_empty());
    assert_eq!(
        cutover.counters().support_posture_row_count(),
        cutover.support_posture_rows().len()
    );
    assert_eq!(
        cutover.counters().visible_unsupported_or_diagnostic_count(),
        0,
        "non-supported Query postures are selection-denial authority until Query executes them"
    );
}
