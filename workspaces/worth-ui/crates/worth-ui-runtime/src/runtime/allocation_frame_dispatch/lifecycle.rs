use super::UiAllocationFrameEpoch;

/// Observable lifecycle of the one runtime-owned allocation frame dispatcher.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum UiAllocationFrameDispatcherState {
    Paused(UiAllocationFramePauseReason),
    Open(UiAllocationFrameEpoch),
    Closing {
        epoch: UiAllocationFrameEpoch,
        next_epoch: UiAllocationFrameEpoch,
    },
    Sealed(UiAllocationFrameEpoch),
    Dispatched(UiAllocationFrameEpoch),
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum UiAllocationFramePauseReason {
    Replacement,
    Shutdown,
    EpochExhausted,
}
