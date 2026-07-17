use super::super::{
    UiAdmittedAllocationStreamFrame, UiAllocationFrameDispatchDenial,
    UiAllocationFrameDispatcherCounters,
};

#[derive(Debug, PartialEq)]
pub(crate) enum UiAllocationFrameTurnOutcome {
    NoAdmittedIngress {
        counters: UiAllocationFrameDispatcherCounters,
    },
    DownstreamBackpressured {
        sealed_frame: Box<UiAdmittedAllocationStreamFrame>,
    },
    Denied {
        denial: UiAllocationFrameDispatchDenial,
        counters: UiAllocationFrameDispatcherCounters,
    },
}
