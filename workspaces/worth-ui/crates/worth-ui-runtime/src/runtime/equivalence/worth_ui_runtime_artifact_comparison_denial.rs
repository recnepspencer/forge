use crate::runtime::admission::WorthUiCandidateAdmissionDenial;
use crate::runtime::{
    WorthUiQuerySupportStatus, WorthUiReplacementCandidateBasis,
    WorthUiRuntimeArtifactComparisonCounters, WorthUiRuntimeEquivalenceBasis,
};

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum WorthUiRuntimeArtifactComparisonDenial {
    AdmissionReceiptChanged {
        denial: WorthUiCandidateAdmissionDenial,
        counters: WorthUiRuntimeArtifactComparisonCounters,
    },
    EquivalenceBasisMismatch {
        runtime_basis: WorthUiRuntimeEquivalenceBasis,
        candidate_basis: WorthUiReplacementCandidateBasis,
        candidate_query_support_status: WorthUiQuerySupportStatus,
        counters: WorthUiRuntimeArtifactComparisonCounters,
    },
}

impl WorthUiRuntimeArtifactComparisonDenial {
    pub fn counters(&self) -> WorthUiRuntimeArtifactComparisonCounters {
        match self {
            Self::AdmissionReceiptChanged { counters, .. }
            | Self::EquivalenceBasisMismatch { counters, .. } => *counters,
        }
    }
}
