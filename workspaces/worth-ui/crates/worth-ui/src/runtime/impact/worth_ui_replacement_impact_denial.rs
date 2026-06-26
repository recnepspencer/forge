use crate::runtime::{
    WorthUiCandidateAdmissionDenial, WorthUiReplacementImpactCounters,
    WorthUiUnsupportedReplacementImpact,
};

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum WorthUiReplacementImpactDenial {
    ComparisonActiveBasisMismatch {
        comparison_active_artifact_digest: u64,
        admitted_active_artifact_digest: u64,
        counters: WorthUiReplacementImpactCounters,
    },
    ComparisonCandidateMismatch {
        comparison_candidate_artifact_digest: u64,
        admitted_candidate_artifact_digest: u64,
        counters: WorthUiReplacementImpactCounters,
    },
    AdmissionReceiptChanged {
        denial: WorthUiCandidateAdmissionDenial,
        counters: WorthUiReplacementImpactCounters,
    },
    UnsupportedImpact {
        unsupported_impact: WorthUiUnsupportedReplacementImpact,
        counters: WorthUiReplacementImpactCounters,
    },
}

impl WorthUiReplacementImpactDenial {
    pub fn counters(&self) -> WorthUiReplacementImpactCounters {
        match self {
            Self::ComparisonActiveBasisMismatch { counters, .. }
            | Self::ComparisonCandidateMismatch { counters, .. }
            | Self::AdmissionReceiptChanged { counters, .. }
            | Self::UnsupportedImpact { counters, .. } => *counters,
        }
    }

    pub fn unsupported_impact(&self) -> Option<&WorthUiUnsupportedReplacementImpact> {
        match self {
            Self::ComparisonActiveBasisMismatch { .. } => None,
            Self::ComparisonCandidateMismatch { .. } => None,
            Self::AdmissionReceiptChanged { .. } => None,
            Self::UnsupportedImpact {
                unsupported_impact, ..
            } => Some(unsupported_impact),
        }
    }
}
