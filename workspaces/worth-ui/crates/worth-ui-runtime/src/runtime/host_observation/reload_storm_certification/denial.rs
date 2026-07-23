use crate::runtime::{
    WorthUiMeasurementCertificationDenial, WorthUiReloadCounterBoundaryDenial,
    WorthUiReloadLatencyCounters, WorthUiSourceIngressDenial,
    WorthUiWatchedCandidateSubmissionDenial,
};

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct WorthUiReloadStormCertificationDenial {
    reason: Box<WorthUiReloadStormCertificationDenialReason>,
    counters: Box<WorthUiReloadLatencyCounters>,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum WorthUiReloadStormCertificationDenialReason {
    EmptyStorm,
    MissingFileOrRustAuthoredCoverage,
    ProviderKindDoesNotMatchStepKind { label: String },
    SourceIngressDenied(WorthUiSourceIngressDenial),
    CandidateSubmissionDenied(WorthUiWatchedCandidateSubmissionDenial),
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
        Self {
            reason: Box::new(reason),
            counters: Box::new(counters),
        }
    }

    pub fn reason(&self) -> &WorthUiReloadStormCertificationDenialReason {
        self.reason.as_ref()
    }

    pub fn counters(&self) -> WorthUiReloadLatencyCounters {
        *self.counters
    }
}
