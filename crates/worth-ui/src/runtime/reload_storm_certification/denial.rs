use crate::runtime::{
    WorthUiFileRustReplacementParityDenial, WorthUiMeasurementCertificationDenial,
    WorthUiReloadCounterBoundaryDenial, WorthUiReloadLatencyCounters, WorthUiSourceIngressDenial,
    WorthUiWatchedCandidateSubmissionDenial,
};

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct WorthUiReloadStormCertificationDenial {
    reason: WorthUiReloadStormCertificationDenialReason,
    counters: WorthUiReloadLatencyCounters,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum WorthUiReloadStormCertificationDenialReason {
    EmptyStorm,
    MissingFileOrRustAuthoredCoverage,
    ProviderKindDoesNotMatchStepKind { label: String },
    SourceIngressDenied(WorthUiSourceIngressDenial),
    CandidateSubmissionDenied(WorthUiWatchedCandidateSubmissionDenial),
    CandidateAdmissionDenied,
    ArtifactComparisonDenied,
    ActivationDenied(WorthUiFileRustReplacementParityDenial),
    ForgedReceiptReuseAcrossCandidates,
    FoundationalMeasurementDenied(WorthUiMeasurementCertificationDenial),
    FoundationalLoweringDenied(WorthUiMeasurementCertificationDenial),
    ReloadCounterBoundaryDenied(WorthUiReloadCounterBoundaryDenial),
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum WorthUiReloadStormCandidateDenialReason {
    SourceIngressDenied(WorthUiSourceIngressDenial),
    CandidateSubmissionDenied(WorthUiWatchedCandidateSubmissionDenial),
}

impl WorthUiReloadStormCertificationDenial {
    pub(crate) fn new(
        reason: WorthUiReloadStormCertificationDenialReason,
        counters: WorthUiReloadLatencyCounters,
    ) -> Self {
        Self { reason, counters }
    }

    pub fn reason(&self) -> &WorthUiReloadStormCertificationDenialReason {
        &self.reason
    }

    pub fn counters(&self) -> WorthUiReloadLatencyCounters {
        self.counters
    }
}
