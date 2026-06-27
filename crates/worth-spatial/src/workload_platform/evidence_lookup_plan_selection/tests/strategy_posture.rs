use super::super::{
    EvidenceLookupPlanQuerySurface, EvidenceLookupPlanRowOutcome,
    EvidenceLookupSelectedStrategyKind,
};
use super::fixtures::PlanSelectionSubject;

#[test]
fn selected_plan_carries_index_strategy_before_execution() {
    let plan = PlanSelectionSubject::projection_consumption().select_projection_plan();
    let selected = plan
        .rows()
        .iter()
        .find(|row| row.outcome() == EvidenceLookupPlanRowOutcome::Selected)
        .expect("projection family should select");

    assert_eq!(
        selected.strategy().map(|strategy| strategy.kind()),
        Some(EvidenceLookupSelectedStrategyKind::BoundedDenseIndexedLookupPlan)
    );
    assert_eq!(plan.counters().bounded_dense_lookup_plan_count(), 1);
    assert_eq!(plan.counters().required_query_posture_row_count(), 1);
    assert_eq!(plan.counters().query_support_rows_consumed_count(), 1);
    assert_eq!(plan.counters().caller_owned_evidence_work_count(), 0);
    assert_eq!(
        selected.query_posture().surface(),
        EvidenceLookupPlanQuerySurface::ProjectionConsumptionReceipt
    );
    assert!(!plan.claims_index_construction());
}
