#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd)]
pub enum TilingIterationBlocker {
    MissingCheckerEvidence,
    StaleDerivedFrontier,
    SuppressedDeadEndEquivalence,
    ResearchGraphInvariantLegality,
    AdvisoryOnlyAgentProposal,
    MissingQueryReadiness,
    UnsupportedTypedEvidence,
}

impl TilingIterationBlocker {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::MissingCheckerEvidence => "missing_checker_evidence",
            Self::StaleDerivedFrontier => "stale_derived_frontier",
            Self::SuppressedDeadEndEquivalence => "suppressed_dead_end_equivalence",
            Self::ResearchGraphInvariantLegality => "research_graph_invariant_legality",
            Self::AdvisoryOnlyAgentProposal => "advisory_only_agent_proposal",
            Self::MissingQueryReadiness => "missing_query_readiness",
            Self::UnsupportedTypedEvidence => "unsupported_typed_evidence",
        }
    }
}
