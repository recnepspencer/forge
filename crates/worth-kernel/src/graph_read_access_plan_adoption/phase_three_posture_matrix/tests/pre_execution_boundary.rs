use super::production_phase_three_closeout;

#[test]
fn posture_selection_precedes_expensive_work() {
    let phase_three = production_phase_three_closeout();

    assert!(phase_three.claims_access_plan_admission_attempts());
    assert!(!phase_three.claims_access_plan_consumption());
    assert!(!phase_three.claims_graph_read_execution());
    assert!(!phase_three.claims_graph_read_receipts());
    assert!(!phase_three.claims_validator_selection());
    assert!(!phase_three.claims_milestone_nine_seed_export());
    assert_eq!(0, phase_three.counters().graph_traversal_attempt_count());
    assert_eq!(
        0,
        phase_three
            .counters()
            .dense_frontier_allocation_attempt_count()
    );
    assert_eq!(
        0,
        phase_three
            .counters()
            .streaming_page_creation_attempt_count()
    );
    assert_eq!(0, phase_three.counters().index_construction_attempt_count());
    assert_eq!(
        0,
        phase_three
            .counters()
            .access_plan_consumption_attempt_count()
    );
    assert_eq!(0, phase_three.counters().graph_read_execution_count());
    assert_eq!(0, phase_three.counters().graph_read_receipt_count());
}
