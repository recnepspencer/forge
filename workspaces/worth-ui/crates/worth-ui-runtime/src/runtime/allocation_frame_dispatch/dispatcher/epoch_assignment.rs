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
}
