use crate::runtime::WorthUiRuntimeDiagnosticPolicy;
use crate::runtime::WorthUiRuntimeFrameEpoch;
use crate::source::WorthUiArtifact;
use std::rc::Rc;

/// Launch request for creating an active runtime instance from canonical candidate truth.
#[derive(Debug)]
pub struct WorthUiRuntimeLaunch {
    pub(crate) artifact: Rc<WorthUiArtifact>,
    pub(crate) frame_epoch: WorthUiRuntimeFrameEpoch,
    pub(crate) diagnostic_policy: WorthUiRuntimeDiagnosticPolicy,
    pub(crate) candidate_snapshot_digest: Option<u64>,
    pub(crate) candidate_artifact_digest: Option<crate::source::WorthUiArtifactDigest>,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum WorthUiRuntimeLaunchDenial {
    HostSessionIdentityExhausted,
    PreparedApplicationHasNoRuntimeArtifact,
    InitialAllocationGraphAuthorityMismatch,
    InitialAllocationObligationsUnsettled {
        node_count: usize,
    },
    CandidateGraphAuthorityMismatch,
    CandidateArtifactAuthorityMismatch,
    ForeignAllocationProjection,
    MissingQueryPosture,
    UnexpectedQueryPosture,
    QueryDefinitionNotInstalled,
    ForeignQueryInstalledAuthority,
    RegionalDeltaDuplicateCandidateRegion,
    PlanInput(crate::runtime::WorthUiPlanLoweringDenial),
    HandleAllocation(crate::runtime::WorthUiRuntimeHandleAllocationDenial),
    TopologyAssembly(crate::runtime::WorthUiPlanTopologyDenial),
    ExecutionPlanAuthorityMismatch,
    OrdinaryPlan(crate::runtime::WorthUiOrdinaryLanePlanDenial),
    VirtualizedPlan(crate::runtime::WorthUiVirtualizedDataPlanDenial),
    CanvasSpatialPlan(crate::runtime::WorthUiCanvasSpatialPlanDenial),
    RealtimeOverlayPlan(crate::runtime::WorthUiHudPlanDenial),
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
            artifact: Rc::new(artifact),
            frame_epoch: WorthUiRuntimeFrameEpoch::initial(),
            diagnostic_policy: WorthUiRuntimeDiagnosticPolicy::minimal(),
            candidate_snapshot_digest: None,
            candidate_artifact_digest: None,
        }
    }

    #[cfg(test)]
    pub(crate) fn from_candidate(candidate: crate::runtime::WorthUiReplacementCandidate) -> Self {
        let candidate_snapshot_digest = candidate.lowering_basis().snapshot_digest();
        let candidate_artifact_digest = candidate.basis().artifact_digest();
        Self {
            artifact: candidate.artifact_bundle().artifact_authority(),
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
