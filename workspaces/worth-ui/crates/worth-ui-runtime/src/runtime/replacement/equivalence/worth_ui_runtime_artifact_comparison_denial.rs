use crate::runtime::{
    WorthUiReplacementCandidateBasis, WorthUiRuntimeArtifactComparisonCounters,
    WorthUiRuntimeEquivalenceBasis,
};

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum WorthUiRuntimeArtifactComparisonDenial {
    EquivalenceBasisMismatch {
        runtime_basis: WorthUiRuntimeEquivalenceBasis,
        candidate_basis: WorthUiReplacementCandidateBasis,
        counters: WorthUiRuntimeArtifactComparisonCounters,
    },
    StructuralCapacityExceeded {
        limit: usize,
        observed: usize,
        counters: WorthUiRuntimeArtifactComparisonCounters,
    },
}

impl WorthUiRuntimeArtifactComparisonDenial {
    pub fn counters(&self) -> WorthUiRuntimeArtifactComparisonCounters {
        match self {
            Self::EquivalenceBasisMismatch { counters, .. }
            | Self::StructuralCapacityExceeded { counters, .. } => *counters,
        }
    }
}
