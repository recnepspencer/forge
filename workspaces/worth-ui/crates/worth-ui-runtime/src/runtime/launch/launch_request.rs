use crate::runtime::WorthUiReplacementCandidate;
use crate::runtime::WorthUiRuntimeDiagnosticPolicy;
use crate::runtime::WorthUiRuntimeFrameEpoch;
use crate::source::WorthUiArtifact;

/// Launch request for creating an active runtime instance from canonical candidate truth.
#[derive(Debug)]
pub struct WorthUiRuntimeLaunch {
    pub(crate) artifact: WorthUiArtifact,
    pub(crate) frame_epoch: WorthUiRuntimeFrameEpoch,
    pub(crate) diagnostic_policy: WorthUiRuntimeDiagnosticPolicy,
    pub(crate) candidate_snapshot_digest: Option<u64>,
    pub(crate) candidate_artifact_digest: Option<crate::source::WorthUiArtifactDigest>,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum WorthUiRuntimeLaunchDenial {
    StalePendingActivation {
        pending_epoch: WorthUiRuntimeFrameEpoch,
        active_epoch: WorthUiRuntimeFrameEpoch,
    },
    CandidateSnapshotMismatch {
        candidate_snapshot_digest: u64,
        app_snapshot_digest: u64,
    },
}

impl WorthUiRuntimeLaunch {
    #[cfg(any(test, feature = "certification-support"))]
    pub(crate) fn from_canonical_artifact(artifact: WorthUiArtifact) -> Self {
        Self {
            artifact,
            frame_epoch: WorthUiRuntimeFrameEpoch::initial(),
            diagnostic_policy: WorthUiRuntimeDiagnosticPolicy::minimal(),
            candidate_snapshot_digest: None,
            candidate_artifact_digest: None,
        }
    }

    /// Construct an ordinary launch from a production-lowered candidate artifact.
    pub fn from_candidate(candidate: WorthUiReplacementCandidate) -> Self {
        let candidate_snapshot_digest = candidate.lowering_basis().snapshot_digest();
        let candidate_artifact_digest = candidate.basis().artifact_digest();
        Self {
            artifact: candidate.into_artifact(),
            frame_epoch: WorthUiRuntimeFrameEpoch::initial(),
            diagnostic_policy: WorthUiRuntimeDiagnosticPolicy::minimal(),
            candidate_snapshot_digest: Some(candidate_snapshot_digest),
            candidate_artifact_digest: Some(candidate_artifact_digest),
        }
    }

    pub fn with_diagnostics(mut self, diagnostic_policy: WorthUiRuntimeDiagnosticPolicy) -> Self {
        self.diagnostic_policy = diagnostic_policy;
        self
    }
}
