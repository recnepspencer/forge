use super::production_phase_two_closeout;

#[test]
fn phase_two_claims_admission_attempts_but_no_execution_or_receipts() {
    let closeout = production_phase_two_closeout();

    assert!(closeout.claims_access_plan_admission_attempts());
    assert!(!closeout.claims_access_plan_consumption());
    assert!(!closeout.claims_graph_read_execution());
    assert!(!closeout.claims_graph_read_receipts());
    assert!(!closeout.claims_validator_selection());
    assert!(!closeout.claims_milestone_nine_seed_export());
}

#[test]
fn phase_two_attempts_do_not_smuggle_receipt_or_plan_consumption() {
    let closeout = production_phase_two_closeout();

    assert!(closeout
        .adoption_ledger()
        .attempts()
        .iter()
        .all(|attempt| attempt.admitted_plan_digest().is_none()));
    assert!(closeout
        .posture_report()
        .posture_rows()
        .iter()
        .all(|row| !row.claims_access_plan_consumption() && !row.claims_graph_read_execution()));
}
