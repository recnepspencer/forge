use crate::graph::UiGraphFactLookupDenial;
use crate::runtime::rebind::UiRebindLimit;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum UiAffectedScopeGeneration {
    Predecessor,
    Candidate,
}

#[derive(Debug, Eq, PartialEq)]
pub enum UiAffectedScopeDenial {
    ForeignSession,
    StaleSourceBasis,
    StalePredecessorGeneration,
    Index {
        generation: UiAffectedScopeGeneration,
        fact_ordinal: usize,
        source: UiGraphFactLookupDenial,
    },
    UnknownAuthoredSelectorInBothGenerations {
        fact_ordinal: usize,
        authored_identity: Box<str>,
    },
    BudgetExceeded {
        limit: UiRebindLimit,
        configured: usize,
        observed: usize,
    },
}
