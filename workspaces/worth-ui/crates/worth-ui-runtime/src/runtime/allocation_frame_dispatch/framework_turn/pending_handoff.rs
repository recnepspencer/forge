use super::super::UiAdmittedAllocationStreamFrame;

/// Runtime-owned Phase 4 output awaiting the mandatory Phase 5 consumer.
///
/// The frame cannot be inspected, acknowledged, or removed by adapters,
/// callbacks, or fixtures. Phase 5 will consume this wrapper inside the
/// allocation-frame subsystem and return its explicit acknowledgment.
#[derive(Debug)]
pub(crate) struct UiPendingAllocationFrameHandoff {
    sealed_frame: UiAdmittedAllocationStreamFrame,
}

impl UiPendingAllocationFrameHandoff {
    pub(crate) fn unchanged(sealed_frame: UiAdmittedAllocationStreamFrame) -> Self {
        Self { sealed_frame }
    }

    pub(crate) fn into_sealed_frame(self) -> UiAdmittedAllocationStreamFrame {
        self.sealed_frame
    }
}
