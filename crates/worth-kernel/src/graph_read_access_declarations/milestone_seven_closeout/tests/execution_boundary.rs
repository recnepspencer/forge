use super::common::production_closeout;

#[test]
fn closeout_does_not_claim_access_plan_consumption() {
    let closeout = production_closeout();
    let seed = closeout.milestone_eight_seed();

    assert!(!closeout.claims_graph_read_execution());
    assert!(!closeout.claims_access_plan_consumption());
    assert!(!closeout.claims_graph_read_receipts_complete());
    assert!(!closeout.claims_milestone_eight_access_plan_adoption());
    assert!(!seed.claims_graph_read_execution());
    assert!(!seed.claims_access_plan_consumption());
    assert_eq!(closeout.closeout_counters().execution_receipt_count(), 0);
    assert_eq!(
        closeout.closeout_counters().access_plan_consumption_count(),
        0
    );
}
