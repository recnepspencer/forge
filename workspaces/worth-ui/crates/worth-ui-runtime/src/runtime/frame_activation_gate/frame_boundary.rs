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
}

impl WorthUiFrameBoundary {
    pub(crate) fn safe_to_activate(frame_epoch: WorthUiRuntimeFrameEpoch) -> Self {
        Self {
            frame_epoch,
            posture: WorthUiFrameBoundaryPosture::SafeToActivate,
        }
    }

    #[cfg(test)]
    pub(crate) fn traversal_in_progress_for_test(frame_epoch: WorthUiRuntimeFrameEpoch) -> Self {
        Self {
            frame_epoch,
            posture: WorthUiFrameBoundaryPosture::TraversalInProgress,
        }
    }

    pub fn frame_epoch(self) -> WorthUiRuntimeFrameEpoch {
        self.frame_epoch
    }

    pub fn posture(self) -> WorthUiFrameBoundaryPosture {
        self.posture
    }

    pub fn is_safe_to_activate(self) -> bool {
        self.posture == WorthUiFrameBoundaryPosture::SafeToActivate
    }
}
