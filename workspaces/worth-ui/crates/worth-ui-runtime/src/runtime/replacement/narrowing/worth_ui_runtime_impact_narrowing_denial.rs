use crate::runtime::{WorthUiCandidateAdmissionDenial, WorthUiImpactLookupCounters};

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum WorthUiRuntimeImpactNarrowingDenial {
    ClassificationActiveBasisMismatch {
        classification_active_artifact_digest: u64,
        admitted_active_artifact_digest: u64,
        counters: WorthUiImpactLookupCounters,
    },
    ClassificationCandidateMismatch {
        classification_candidate_artifact_digest: u64,
        admitted_candidate_artifact_digest: u64,
        counters: WorthUiImpactLookupCounters,
    },
    AdmissionReceiptChanged {
        denial: WorthUiCandidateAdmissionDenial,
        counters: WorthUiImpactLookupCounters,
    },
    QueryDependencyMetadataReceiptMismatch {
        receipt_runtime_hook_count: usize,
        metadata_runtime_hook_count: usize,
        counters: WorthUiImpactLookupCounters,
    },
    QueryDependencyPostureMissing {
        expected_runtime_hook_count: usize,
        observed_runtime_hook_count: usize,
        counters: WorthUiImpactLookupCounters,
    },
}

impl WorthUiRuntimeImpactNarrowingDenial {
    pub fn counters(&self) -> WorthUiImpactLookupCounters {
        match self {
            Self::ClassificationActiveBasisMismatch { counters, .. }
            | Self::ClassificationCandidateMismatch { counters, .. }
            | Self::AdmissionReceiptChanged { counters, .. }
            | Self::QueryDependencyMetadataReceiptMismatch { counters, .. }
            | Self::QueryDependencyPostureMissing { counters, .. } => *counters,
        }
    }
}
