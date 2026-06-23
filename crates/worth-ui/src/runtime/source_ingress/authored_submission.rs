use crate::runtime::{
    WorthUiAuthoredDeltaSummary, WorthUiCandidateOrderingReceipt,
    WorthUiCandidateRuntimeAuthoringSnapshot, WorthUiReplacementCandidate,
    WorthUiSourceIngressCounters, WorthUiSourcePackageRevision, WorthUiWatchedCandidateSubmission,
};

#[derive(Debug, Eq, PartialEq)]
pub struct WorthUiSourceAuthoredCandidateSubmission {
    inner: WorthUiWatchedCandidateSubmission,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum WorthUiSourceAuthoredCandidateSubmissionDenial {
    MissingAuthoredDeltaProof,
}

impl WorthUiSourceAuthoredCandidateSubmission {
    pub(super) fn new(inner: WorthUiWatchedCandidateSubmission) -> Self {
        Self { inner }
    }

    pub fn ordering_receipt(&self) -> &WorthUiCandidateOrderingReceipt {
        self.inner.ordering_receipt()
    }

    pub fn source_revision(&self) -> &WorthUiSourcePackageRevision {
        self.inner.source_revision()
    }

    pub fn counters(&self) -> WorthUiSourceIngressCounters {
        self.inner.counters()
    }

    pub fn authored_delta_summary(&self) -> &WorthUiAuthoredDeltaSummary {
        self.inner
            .authored_delta_summary()
            .expect("source-authored submission preserves authored delta proof")
    }

    pub(crate) fn into_parts(
        self,
    ) -> (
        WorthUiReplacementCandidate,
        WorthUiCandidateRuntimeAuthoringSnapshot,
        WorthUiAuthoredDeltaSummary,
        WorthUiSourcePackageRevision,
        WorthUiCandidateOrderingReceipt,
        WorthUiSourceIngressCounters,
    ) {
        let (
            candidate,
            candidate_authoring_snapshot,
            authored_delta_summary,
            revision,
            ordering_receipt,
            counters,
        ) = self.inner.into_parts();
        (
            candidate,
            candidate_authoring_snapshot
                .expect("source-authored submission preserves authoring snapshot proof"),
            authored_delta_summary.expect("source-authored submission preserves authored delta"),
            revision,
            ordering_receipt,
            counters,
        )
    }
}
