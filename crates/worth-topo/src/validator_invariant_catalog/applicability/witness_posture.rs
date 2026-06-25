#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum WorthTopologyWitnessPosture {
    TouchedFacts,
    TouchedRelations,
    TouchedNeighborhood,
    CertificationOnlyComparison,
}

impl WorthTopologyWitnessPosture {
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::TouchedFacts => "touched-facts",
            Self::TouchedRelations => "touched-relations",
            Self::TouchedNeighborhood => "touched-neighborhood",
            Self::CertificationOnlyComparison => "certification-only-comparison",
        }
    }
}
