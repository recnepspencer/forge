use super::fixtures::PlanSelectionSubject;

#[test]
fn selected_lookup_plan_breadth_matches_touch_and_family_expansion() {
    let plan = PlanSelectionSubject::event_ledger().select_event_plan();

    assert_eq!(plan.counters().candidate_family_count(), 3);
    assert_eq!(plan.counters().selected_spatial_region_count(), 1);
    assert_eq!(plan.counters().selected_stage_receipt_count(), 1);
    assert_eq!(plan.counters().selected_family_membership_probe_count(), 3);
    assert_eq!(plan.counters().topology_support_rows_consumed_count(), 1);
    assert_eq!(plan.counters().query_support_rows_consumed_count(), 1);
    assert_eq!(plan.counters().sparse_lookup_plan_count(), 1);
    assert_eq!(plan.counters().bounded_dense_lookup_plan_count(), 0);
    assert_eq!(plan.counters().caller_owned_evidence_work_count(), 0);
    assert_eq!(plan.counters().raw_evidence_row_scan_count(), 0);
    assert_eq!(plan.counters().broad_receipt_scan_count(), 0);
}
