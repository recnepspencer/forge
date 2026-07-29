use super::{UiRebindPlanningContext, UiRebindPlanningDenial};

pub(super) fn require_scope_currentness(
    context: &UiRebindPlanningContext<'_>,
    scope: &super::super::UiResolvedAffectedScope,
) -> Result<(), UiRebindPlanningDenial> {
    let basis = scope.basis();
    require_classification_currentness(context, basis.classification())?;
    if basis.predecessor_graph() != context.current_graph() {
        return Err(UiRebindPlanningDenial::StalePredecessorGraph);
    }
    let (candidate_generation, candidate_graph) = scope
        .source_succession()
        .map(|succession| {
            let authority = succession.successor_authority();
            (
                authority.generation_identity(),
                crate::graph::UiGraphFactIndexBasis::from_generation(
                    authority.graph_snapshot(),
                    authority.capabilities(),
                ),
            )
        })
        .unwrap_or((context.current_generation(), context.current_graph()));
    if basis.candidate_generation() != candidate_generation {
        return Err(UiRebindPlanningDenial::StaleCandidateGeneration);
    }
    if basis.candidate_graph() != candidate_graph {
        return Err(UiRebindPlanningDenial::StaleCandidateGraph);
    }
    Ok(())
}

pub(super) fn require_classification_currentness(
    context: &UiRebindPlanningContext<'_>,
    basis: &crate::runtime::observation::UiChangeClassificationBasis,
) -> Result<(), UiRebindPlanningDenial> {
    if basis.session() != context.session() {
        return Err(UiRebindPlanningDenial::ForeignSession);
    }
    if basis.source_basis() != context.current_source_basis() {
        return Err(UiRebindPlanningDenial::StaleSourceBasis);
    }
    if basis.predecessor_generation() != context.current_generation() {
        return Err(UiRebindPlanningDenial::StalePredecessorGeneration);
    }
    Ok(())
}
