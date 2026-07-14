use worth_query::facade::inspection::{
    declare, inspection_basis, ScopedInspectionBasis, WorthQueryInspectionOutcome,
    WorthQueryOutcomeNavigation, WorthQueryOutcomePosture, WorthQueryWorkspace,
};
use worth_query::facade::read::WorthQueryReadCompletion;
use worth_query::facade::read::{project_facts, WorthQueryProjectionOutcome};

fn ordinary_inspection_journey(
    completion: &WorthQueryReadCompletion,
    basis: ScopedInspectionBasis,
    workspace: &WorthQueryWorkspace,
) -> WorthQueryInspectionOutcome {
    declare(completion)
        .with_rich_inspection()
        .using(inspection_basis(basis))
        .run(workspace)
}

fn common_outcome_navigation(outcome: &WorthQueryInspectionOutcome) {
    let posture = outcome.posture();
    assert!(matches!(
        posture,
        WorthQueryOutcomePosture::Completed
            | WorthQueryOutcomePosture::Advisory
            | WorthQueryOutcomePosture::Violation
            | WorthQueryOutcomePosture::Deferred
            | WorthQueryOutcomePosture::Unavailable
    ));
}

fn ordinary_projection_journey(
    completion: &WorthQueryReadCompletion,
) -> WorthQueryProjectionOutcome {
    completion.consume_projection(project_facts().entity_identities())
}

#[test]
fn inspection_journey_uses_only_ordinary_facade_vocabulary() {
    let _ = ordinary_inspection_journey
        as fn(
            &WorthQueryReadCompletion,
            ScopedInspectionBasis,
            &WorthQueryWorkspace,
        ) -> WorthQueryInspectionOutcome;
    let _ = common_outcome_navigation as fn(&WorthQueryInspectionOutcome);
    let _ =
        ordinary_projection_journey as fn(&WorthQueryReadCompletion) -> WorthQueryProjectionOutcome;

    assert_navigation::<worth_query::facade::read::WorthQueryReadOutcome>();
    assert_navigation::<WorthQueryProjectionOutcome>();
    assert_navigation::<worth_query::facade::aggregate::WorthQueryCountOutcome>();
    assert_navigation::<worth_query::facade::live::WorthQueryLiveOpenOutcome>();
    assert_navigation::<worth_query::facade::history::WorthQueryHistoricalOutcome>();
    assert_navigation::<worth_query::facade::comparison::WorthQueryComparisonOutcome>();
    assert_navigation::<worth_query::facade::mutation::WorthQueryMutationOutcome>();
    assert_navigation::<worth_query::facade::preview::WorthQueryPreviewJourneyOutcome>();
    assert_navigation::<worth_query::facade::workflow::WorthQueryWorkflowOutcome>();
    assert_navigation::<worth_query::facade::workflow::WorthQueryWritebackOutcome>();
    assert_navigation::<worth_query::facade::workflow::WorthQueryBranchMergeOutcome>();
    assert_navigation::<worth_query::facade::domain::WorthQueryDomainWorkflowOutcome>();
    assert_navigation::<WorthQueryInspectionOutcome>();
}

fn assert_navigation<T: WorthQueryOutcomeNavigation>() {}
