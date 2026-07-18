use super::super::{
    UiAdmittedAllocationStreamFrame, UiAllocationFrameDispatchDenial,
    UiAllocationFrameDispatcherCounters,
};

#[derive(Debug, PartialEq)]
pub(crate) enum UiAllocationFrameTurnOutcome {
    NoAdmittedIngress {
        counters: UiAllocationFrameDispatcherCounters,
    },
    SealedFrameReady {
        sealed_frame: Box<UiAdmittedAllocationStreamFrame>,
        frame_epoch_assignment: super::super::UiAllocationFrameEpochAssignment,
    },
    Denied {
        denial: UiAllocationFrameDispatchDenial,
        counters: UiAllocationFrameDispatcherCounters,
    },
}
