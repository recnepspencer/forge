#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd)]
pub enum PlanarBooleanSplitAffectedArtifact {
    QueryDeclaration,
    EndpointContactDecision,
    IntervalSubdivision,
    SplitVertex,
    SplitFragment,
    SplitFragmentCoverage,
    OverlapChainCoverage,
    PersistentName,
    PhaseStop,
}

impl PlanarBooleanSplitAffectedArtifact {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::QueryDeclaration => "query_declaration",
            Self::EndpointContactDecision => "endpoint_contact_decision",
            Self::IntervalSubdivision => "interval_subdivision",
            Self::SplitVertex => "split_vertex",
            Self::SplitFragment => "split_fragment",
            Self::SplitFragmentCoverage => "split_fragment_coverage",
            Self::OverlapChainCoverage => "overlap_chain_coverage",
            Self::PersistentName => "persistent_name",
            Self::PhaseStop => "phase_stop",
        }
    }
}
