#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd)]
pub enum PlanarBooleanSplitDecisionPhase {
    QueryDeclaration,
    EndpointBoundaryNormalization,
    IntervalSubdivisionNormalization,
    SplitVertexIdentity,
    SplitEdgeFragmentConstruction,
    SplitChainValidation,
    SplitPersistentNaming,
    PhaseStop,
}

impl PlanarBooleanSplitDecisionPhase {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::QueryDeclaration => "query_declaration",
            Self::EndpointBoundaryNormalization => "endpoint_boundary_normalization",
            Self::IntervalSubdivisionNormalization => "interval_subdivision_normalization",
            Self::SplitVertexIdentity => "split_vertex_identity",
            Self::SplitEdgeFragmentConstruction => "split_edge_fragment_construction",
            Self::SplitChainValidation => "split_chain_validation",
            Self::SplitPersistentNaming => "split_persistent_naming",
            Self::PhaseStop => "phase_stop",
        }
    }
}
