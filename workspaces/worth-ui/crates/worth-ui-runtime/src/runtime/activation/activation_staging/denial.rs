use crate::runtime::{WorthUiActivationStagingCounters, WorthUiRuntimeFrameEpoch};

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum WorthUiActivationStagingDenialReason {
    CandidateApplicationAuthorityMismatch,
    MissingDurableStateReconciliation,
    MissingQueryLiveRebindPlan,
    ActiveArtifactDigestMismatch,
    CandidateArtifactDigestMismatch,
    AdmittedQuerySupportContractChanged,
    ActiveRuntimeMutatedDuringStaging,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct WorthUiActivationStagingDenial {
    active_artifact_digest: u64,
    candidate_artifact_digest: u64,
    frame_epoch: WorthUiRuntimeFrameEpoch,
    reason: WorthUiActivationStagingDenialReason,
    counters: WorthUiActivationStagingCounters,
}

impl WorthUiActivationStagingDenial {
    pub(crate) fn new(
        active_artifact_digest: u64,
        candidate_artifact_digest: u64,
        frame_epoch: WorthUiRuntimeFrameEpoch,
        reason: WorthUiActivationStagingDenialReason,
        counters: WorthUiActivationStagingCounters,
    ) -> Self {
        Self {
            active_artifact_digest,
            candidate_artifact_digest,
            frame_epoch,
            reason,
            counters,
        }
    }

    pub fn active_artifact_digest(&self) -> u64 {
        self.active_artifact_digest
    }

    pub fn candidate_artifact_digest(&self) -> u64 {
        self.candidate_artifact_digest
    }

    pub fn frame_epoch(&self) -> WorthUiRuntimeFrameEpoch {
        self.frame_epoch
    }

    pub fn reason(&self) -> WorthUiActivationStagingDenialReason {
        self.reason
    }

    pub fn counters(&self) -> WorthUiActivationStagingCounters {
        self.counters
    }
}
