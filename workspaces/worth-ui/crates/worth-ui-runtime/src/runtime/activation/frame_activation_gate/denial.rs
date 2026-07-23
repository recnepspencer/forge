use crate::runtime::{WorthUiActivationGateCounters, WorthUiRuntimeFrameEpoch};

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum WorthUiActivationGateDenialReason {
    UnsafeFrameBoundary,
    ForeignFrameBoundarySession,
    BoundaryFrameEpochMismatch,
    StaleFrameEpoch,
    FutureFrameEpochMismatch,
    PendingActivationNotReady,
    PendingAndPlanInputMismatch,
    HandleAllocationReceiptMismatch,
    ExecutionPlanHandleReceiptMismatch,
    QueryRebindDenied,
    MissingLaneParityReport,
    LaneParityDoesNotCertifyActivation,
    LaneParityDigestMismatch,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct WorthUiActivationGateDenial {
    active_artifact_digest: u64,
    candidate_artifact_digest: u64,
    ready_frame_epoch: WorthUiRuntimeFrameEpoch,
    boundary_frame_epoch: WorthUiRuntimeFrameEpoch,
    reason: WorthUiActivationGateDenialReason,
    counters: WorthUiActivationGateCounters,
}

impl WorthUiActivationGateDenial {
    pub(crate) fn new(
        active_artifact_digest: u64,
        candidate_artifact_digest: u64,
        ready_frame_epoch: WorthUiRuntimeFrameEpoch,
        boundary_frame_epoch: WorthUiRuntimeFrameEpoch,
        reason: WorthUiActivationGateDenialReason,
        counters: WorthUiActivationGateCounters,
    ) -> Self {
        Self {
            active_artifact_digest,
            candidate_artifact_digest,
            ready_frame_epoch,
            boundary_frame_epoch,
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

    pub fn ready_frame_epoch(&self) -> WorthUiRuntimeFrameEpoch {
        self.ready_frame_epoch
    }

    pub fn boundary_frame_epoch(&self) -> WorthUiRuntimeFrameEpoch {
        self.boundary_frame_epoch
    }

    pub fn reason(&self) -> WorthUiActivationGateDenialReason {
        self.reason
    }

    pub fn counters(&self) -> WorthUiActivationGateCounters {
        self.counters
    }
}
