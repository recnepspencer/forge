use super::super::UiAllocationFrameEpoch;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(crate) struct UiAllocationFrameEpochAssignment {
    epoch: UiAllocationFrameEpoch,
}

impl UiAllocationFrameEpochAssignment {
    pub(super) fn from_linearization(epoch: UiAllocationFrameEpoch) -> Self {
        Self { epoch }
    }

    pub(crate) fn epoch(self) -> UiAllocationFrameEpoch {
        self.epoch
    }

    pub(in crate::runtime::allocation_frame_dispatch) fn from_sealed_frame(
        frame: &super::super::UiAdmittedAllocationStreamFrame,
    ) -> Self {
        Self {
            epoch: frame.epoch(),
        }
    }
}
