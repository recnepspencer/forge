#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum UiRebindCandidatePreparationDenial {
    MountEligibility,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum UiCollectionProjectionContentDenial {
    DuplicateChangedRow,
    MissingChangedRow,
    UnusedChangedRow,
    SelectedFieldCapacityExceeded,
    ResetReachedContentPlanning,
}

#[derive(Debug)]
pub enum UiRebindPlanningDenial {
    MissingSourceSuccession,
    WrongSourceSuccessionPosture,
    ForeignSession,
    StaleSourceBasis,
    StalePredecessorGeneration,
    StalePredecessorGraph,
    StaleCandidateGeneration,
    StaleCandidateGraph,
    ForeignExecutionPolicySession,
    AmbiguousProjectionContent {
        graph_node: crate::graph::UiGraphNodeIdentity,
    },
    AmbiguousProjectionInput {
        projection: worth_ui_query_binding::WorthUiQueryViewIdentity,
    },
    ProjectionSchemaTransitionUncorrelated {
        component_identity: Box<str>,
    },
    InvalidCollectionProjectionContent(UiCollectionProjectionContentDenial),
    BudgetExceeded {
        limit: crate::runtime::rebind::UiRebindLimit,
        configured: usize,
        observed: usize,
    },
    CandidatePreparation(UiRebindCandidatePreparationDenial),
    Replacement(Box<crate::runtime::WorthUiReplacementLoweringDenial>),
}
