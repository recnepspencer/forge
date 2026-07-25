#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct WorthUiFrameExecutionBasis {
    host_session: u64,
    active_artifact: u64,
    active_plan: u64,
    frame_epoch: u64,
}

impl WorthUiFrameExecutionBasis {
    pub(crate) const fn new(
        host_session: u64,
        active_artifact: u64,
        active_plan: u64,
        frame_epoch: u64,
    ) -> Self {
        Self {
            host_session,
            active_artifact,
            active_plan,
            frame_epoch,
        }
    }

    pub const fn host_session(self) -> u64 {
        self.host_session
    }

    pub const fn active_artifact(self) -> u64 {
        self.active_artifact
    }

    pub const fn active_plan(self) -> u64 {
        self.active_plan
    }

    pub const fn frame_epoch(self) -> u64 {
        self.frame_epoch
    }
}
