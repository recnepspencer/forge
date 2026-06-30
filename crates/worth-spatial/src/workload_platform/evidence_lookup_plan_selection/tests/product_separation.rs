use super::fixtures::PlanSelectionSubject;

#[test]
fn selected_plan_does_not_claim_execution_or_query_authority() {
    let plan = PlanSelectionSubject::projection_consumption().select_projection_plan();

    assert!(!plan.claims_lookup_execution());
    assert!(!plan.claims_index_construction());
    assert!(!plan.claims_query_descriptor_authority());
    assert_eq!(plan.counters().caller_owned_evidence_work_count(), 0);
}
