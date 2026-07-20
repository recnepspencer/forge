#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct WorthUiHostOutputGeneration {
    host_session_identity: u64,
    active_artifact_digest: u64,
    active_plan_digest: u64,
    frame_epoch: u64,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct WorthUiHostOutputGenerationDenial {
    reason: WorthUiHostOutputGenerationDenialReason,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum WorthUiHostOutputGenerationDenialReason {
    HostSessionMismatch,
    ActiveArtifactMismatch,
    ActivePlanMismatch,
    FrameEpochMismatch,
}

impl WorthUiHostOutputGeneration {
    pub fn new(
        host_session_identity: u64,
        active_artifact_digest: u64,
        active_plan_digest: u64,
        frame_epoch: u64,
    ) -> Self {
        Self {
            host_session_identity,
            active_artifact_digest,
            active_plan_digest,
            frame_epoch,
        }
    }

    pub fn host_session_identity(self) -> u64 {
        self.host_session_identity
    }

    pub fn active_artifact_digest(self) -> u64 {
        self.active_artifact_digest
    }

    pub fn active_plan_digest(self) -> u64 {
        self.active_plan_digest
    }

    pub fn frame_epoch(self) -> u64 {
        self.frame_epoch
    }

    pub(super) fn validate(self, expected: Self) -> Result<(), WorthUiHostOutputGenerationDenial> {
        let reason = if self.host_session_identity != expected.host_session_identity {
            WorthUiHostOutputGenerationDenialReason::HostSessionMismatch
        } else if self.active_artifact_digest != expected.active_artifact_digest {
            WorthUiHostOutputGenerationDenialReason::ActiveArtifactMismatch
        } else if self.active_plan_digest != expected.active_plan_digest {
            WorthUiHostOutputGenerationDenialReason::ActivePlanMismatch
        } else if self.frame_epoch != expected.frame_epoch {
            WorthUiHostOutputGenerationDenialReason::FrameEpochMismatch
        } else {
            return Ok(());
        };
        Err(WorthUiHostOutputGenerationDenial { reason })
    }
}

impl WorthUiHostOutputGenerationDenial {
    pub fn reason(self) -> WorthUiHostOutputGenerationDenialReason {
        self.reason
    }
}
