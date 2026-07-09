use crate::strategy::S8LayoutStrategyFamily;
use worth_store_budgets::S8PreExecutionBudgetDenial;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum S8SelectionCandidateRejection {
    StrategyUnsupported,
    CapabilityUnsupported,
    LaneUnsupported,
    MutationShapeUnsupported,
    MaterializationRequired,
    MaterializationInexact,
    MissingPlannedCounterEnvelope,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum S8PlanSelectionDenied {
    NoEligibleAlternative,
    AmbiguousAlternativeOrdering {
        first_family: S8LayoutStrategyFamily,
        second_family: S8LayoutStrategyFamily,
    },
    BudgetDenied(S8PreExecutionBudgetDenial),
}
