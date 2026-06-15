use crate::runtime::{WorthUiReloadCheckedStopPosture, WorthUiReloadFailureStage};

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct WorthUiReloadDenial {
    stage: WorthUiReloadFailureStage,
    upstream_evidence_digest: Option<u64>,
    checked_stop_posture: WorthUiReloadCheckedStopPosture,
}

impl WorthUiReloadDenial {
    pub(crate) fn new(
        stage: WorthUiReloadFailureStage,
        upstream_evidence_digest: Option<u64>,
        checked_stop_posture: WorthUiReloadCheckedStopPosture,
    ) -> Self {
        Self {
            stage,
            upstream_evidence_digest,
            checked_stop_posture,
        }
    }

    pub(crate) fn ordinary(
        stage: WorthUiReloadFailureStage,
        upstream_evidence_digest: Option<u64>,
    ) -> Self {
        Self::new(
            stage,
            upstream_evidence_digest,
            WorthUiReloadCheckedStopPosture::ordinary(),
        )
    }

    pub(crate) fn query_checked_stop(
        stage: WorthUiReloadFailureStage,
        upstream_evidence_digest: Option<u64>,
    ) -> Self {
        Self::new(
            stage,
            upstream_evidence_digest,
            WorthUiReloadCheckedStopPosture::query_support_denied(),
        )
    }

    pub(crate) fn query_recovery_preserved(
        stage: WorthUiReloadFailureStage,
        upstream_evidence_digest: Option<u64>,
    ) -> Self {
        Self::new(
            stage,
            upstream_evidence_digest,
            WorthUiReloadCheckedStopPosture::query_recovery_preserved(),
        )
    }

    pub fn stage(self) -> WorthUiReloadFailureStage {
        self.stage
    }

    pub fn upstream_evidence_digest(self) -> Option<u64> {
        self.upstream_evidence_digest
    }

    pub fn checked_stop_posture(self) -> WorthUiReloadCheckedStopPosture {
        self.checked_stop_posture
    }
}
