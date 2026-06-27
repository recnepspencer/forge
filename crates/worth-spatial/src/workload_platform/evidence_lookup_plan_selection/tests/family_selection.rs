use super::super::{
    EvidenceLookupPlanQueryPostureState, EvidenceLookupPlanRowOutcome,
    EvidenceLookupPlanTopologyPostureState,
};
use super::fixtures::PlanSelectionSubject;

#[test]
fn unrelated_lookup_families_remain_unselected() {
    let plan = PlanSelectionSubject::event_ledger().select_event_plan();

    assert_eq!(plan.rows().len(), plan.counters().candidate_family_count());
    assert_eq!(plan.counters().selected_family_count(), 1);
    assert_eq!(plan.counters().unaffected_family_count(), 2);
    assert_eq!(plan.counters().denied_family_count(), 0);
    assert_eq!(
        plan.rows()
            .iter()
            .filter(|row| row.outcome() == EvidenceLookupPlanRowOutcome::Unaffected)
            .count(),
        2
    );
    for row in plan
        .rows()
        .iter()
        .filter(|row| row.outcome() == EvidenceLookupPlanRowOutcome::Unaffected)
    {
        assert_eq!(
            row.query_posture().state(),
            &EvidenceLookupPlanQueryPostureState::NotEvaluatedForUnaffectedFamily
        );
        assert_eq!(
            row.topology_posture().state(),
            &EvidenceLookupPlanTopologyPostureState::NotEvaluatedForUnaffectedFamily
        );
    }
}
