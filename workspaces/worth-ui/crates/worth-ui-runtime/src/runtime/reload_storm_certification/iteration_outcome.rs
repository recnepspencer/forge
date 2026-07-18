use crate::runtime::{
    WorthUiCandidateAuthoringLane, WorthUiCandidateCompositionBasis,
    WorthUiCandidateOrderingReceipt, WorthUiReloadFailure, WorthUiReloadStormCandidateDenialReason,
    WorthUiSourceIngressCounters, WorthUiSourcePackageRevision, WorthUiWatchedCandidateSubmission,
};

#[derive(Debug, PartialEq)]
pub enum WorthUiReloadStormIterationOutcome {
    PreparedPendingCutover(Box<WorthUiReloadStormPreparedIteration>),
    DeniedPreserved(Box<WorthUiReloadStormDeniedIteration>),
}

/// A whole source submission retained without exposing artifact-only runtime
/// truth before application-authority cutover exists.
#[derive(Debug, Eq, PartialEq)]
pub struct WorthUiReloadStormPreparedIteration {
    label: String,
    submission: WorthUiWatchedCandidateSubmission,
    active_plan_digest_after: u64,
    last_valid_plan_digest_after: u64,
}

#[derive(Debug, Eq, PartialEq)]
pub struct WorthUiReloadStormDeniedIteration {
    label: String,
    candidate_denial_reason: WorthUiReloadStormCandidateDenialReason,
    failure: WorthUiReloadFailure,
    active_plan_digest_after: u64,
    last_valid_plan_digest_after: u64,
}

impl WorthUiReloadStormPreparedIteration {
    pub(crate) fn new(
        label: impl Into<String>,
        submission: WorthUiWatchedCandidateSubmission,
        active_plan_digest_after: u64,
        last_valid_plan_digest_after: u64,
    ) -> Self {
        Self {
            label: label.into(),
            submission,
            active_plan_digest_after,
            last_valid_plan_digest_after,
        }
    }

    pub fn label(&self) -> &str {
        &self.label
    }

    pub fn authoring_lane(&self) -> WorthUiCandidateAuthoringLane {
        self.submission.authoring_lane()
    }

    pub fn composition_basis(&self) -> &WorthUiCandidateCompositionBasis {
        self.submission.composition_basis()
    }

    pub fn source_revision(&self) -> &WorthUiSourcePackageRevision {
        self.submission.source_revision()
    }

    pub fn ordering_receipt(&self) -> &WorthUiCandidateOrderingReceipt {
        self.submission.ordering_receipt()
    }

    pub fn ingress_counters(&self) -> WorthUiSourceIngressCounters {
        self.submission.counters()
    }

    pub fn active_plan_digest_after(&self) -> u64 {
        self.active_plan_digest_after
    }

    pub fn last_valid_plan_digest_after(&self) -> u64 {
        self.last_valid_plan_digest_after
    }
}

impl WorthUiReloadStormDeniedIteration {
    pub(crate) fn new(
        label: impl Into<String>,
        candidate_denial_reason: WorthUiReloadStormCandidateDenialReason,
        failure: WorthUiReloadFailure,
        active_plan_digest_after: u64,
        last_valid_plan_digest_after: u64,
    ) -> Self {
        Self {
            label: label.into(),
            candidate_denial_reason,
            failure,
            active_plan_digest_after,
            last_valid_plan_digest_after,
        }
    }

    pub fn label(&self) -> &str {
        &self.label
    }

    pub fn failure(&self) -> &WorthUiReloadFailure {
        &self.failure
    }

    pub fn candidate_denial_reason(&self) -> &WorthUiReloadStormCandidateDenialReason {
        &self.candidate_denial_reason
    }

    pub fn active_plan_digest_after(&self) -> u64 {
        self.active_plan_digest_after
    }

    pub fn last_valid_plan_digest_after(&self) -> u64 {
        self.last_valid_plan_digest_after
    }
}
