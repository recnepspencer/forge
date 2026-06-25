use super::common::production_admission_posture_closeout;

#[test]
fn admission_posture_is_not_receipt_or_plan_consumption() {
    let closeout = production_admission_posture_closeout();

    assert!(!closeout.claims_graph_read_execution());
    assert!(!closeout.claims_access_plan_consumption());
    assert!(!closeout.claims_graph_read_receipts_complete());
    assert!(closeout
        .posture_records()
        .iter()
        .all(|record| !record.claims_graph_read_execution()
            && !record.claims_access_plan_consumption()
            && !record.posture_outcome().claims_graph_read_execution()
            && !record.posture_outcome().claims_access_plan_consumption()));
}

#[test]
fn phase_six_seed_carries_no_execution_authority() {
    let closeout = production_admission_posture_closeout();
    let seed = closeout.phase_six_seed();

    assert!(!seed.claims_graph_read_execution());
    assert!(!seed.claims_access_plan_consumption());
}
