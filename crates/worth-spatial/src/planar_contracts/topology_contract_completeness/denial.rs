use super::PlanarTopologyContractCompletenessCounters;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum PlanarTopologyContractCompletenessDenialKind {
    MissingTopologyReceipt,
    MissingLoopBasis,
    MissingShellBasis,
    MissingOrientationBasis,
    MissingNeighborhoodBasis,
    MissingValidationSurface,
    MissingDeclaredQuerySurface,
    ContradictoryTopologyFacts,
}

impl PlanarTopologyContractCompletenessDenialKind {
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::MissingTopologyReceipt => "missing-topology-receipt",
            Self::MissingLoopBasis => "missing-loop-basis",
            Self::MissingShellBasis => "missing-shell-basis",
            Self::MissingOrientationBasis => "missing-orientation-basis",
            Self::MissingNeighborhoodBasis => "missing-neighborhood-basis",
            Self::MissingValidationSurface => "missing-validation-surface",
            Self::MissingDeclaredQuerySurface => "missing-declared-query-surface",
            Self::ContradictoryTopologyFacts => "contradictory-topology-facts",
        }
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct PlanarTopologyContractCompletenessDenial {
    kind: PlanarTopologyContractCompletenessDenialKind,
    reason: String,
    counters: PlanarTopologyContractCompletenessCounters,
}

impl PlanarTopologyContractCompletenessDenial {
    pub(crate) fn new(
        kind: PlanarTopologyContractCompletenessDenialKind,
        reason: impl Into<String>,
    ) -> Self {
        Self {
            kind,
            reason: reason.into(),
            counters: PlanarTopologyContractCompletenessCounters::rejected_missing_fact(),
        }
    }

    pub fn kind(&self) -> PlanarTopologyContractCompletenessDenialKind {
        self.kind
    }

    pub fn reason(&self) -> &str {
        &self.reason
    }

    pub fn counters(&self) -> PlanarTopologyContractCompletenessCounters {
        self.counters
    }
}
