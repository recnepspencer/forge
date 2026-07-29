#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum UiRebindCandidatePreparationDenial {
    MountEligibility,
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
    BudgetExceeded {
        limit: crate::runtime::rebind::UiRebindLimit,
        configured: usize,
        observed: usize,
    },
    CandidatePreparation(UiRebindCandidatePreparationDenial),
    Replacement(Box<crate::runtime::WorthUiReplacementLoweringDenial>),
}
