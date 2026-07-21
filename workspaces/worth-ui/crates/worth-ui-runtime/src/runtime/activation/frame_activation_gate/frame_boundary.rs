use crate::runtime::WorthUiRuntimeFrameEpoch;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum WorthUiFrameBoundaryPosture {
    SafeToActivate,
    TraversalInProgress,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct WorthUiFrameBoundary {
    frame_epoch: WorthUiRuntimeFrameEpoch,
    posture: WorthUiFrameBoundaryPosture,
    host_session: crate::facade::WorthUiHostSessionIdentity,
}

impl WorthUiFrameBoundary {
    pub(crate) fn safe_to_activate(
        frame_epoch: WorthUiRuntimeFrameEpoch,
        host_session: crate::facade::WorthUiHostSessionIdentity,
    ) -> Self {
        Self {
            frame_epoch,
            posture: WorthUiFrameBoundaryPosture::SafeToActivate,
            host_session,
        }
    }

    #[cfg(test)]
    pub(crate) fn traversal_in_progress_for_test(
        frame_epoch: WorthUiRuntimeFrameEpoch,
        host_session: crate::facade::WorthUiHostSessionIdentity,
    ) -> Self {
        Self {
            frame_epoch,
            posture: WorthUiFrameBoundaryPosture::TraversalInProgress,
            host_session,
        }
    }

    pub fn frame_epoch(self) -> WorthUiRuntimeFrameEpoch {
        self.frame_epoch
    }

    pub fn posture(self) -> WorthUiFrameBoundaryPosture {
        self.posture
    }

    pub fn host_session(self) -> crate::facade::WorthUiHostSessionIdentity {
        self.host_session
    }

    pub fn is_safe_to_activate(self) -> bool {
        self.posture == WorthUiFrameBoundaryPosture::SafeToActivate
    }
}
