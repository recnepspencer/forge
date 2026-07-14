use crate::runtime::{UiAllocationFrameDispatcherCounters, UiAllocationFrameIngressDescriptor};

/// Immutable explanation of one source fact crossing a production gateway.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct UiAllocationSourceGatewayEvidence {
    ingress: UiAllocationFrameIngressDescriptor,
    counters: UiAllocationFrameDispatcherCounters,
}

impl UiAllocationSourceGatewayEvidence {
    pub(crate) fn new(
        ingress: UiAllocationFrameIngressDescriptor,
        counters: UiAllocationFrameDispatcherCounters,
    ) -> Self {
        Self { ingress, counters }
    }

    pub fn ingress(&self) -> UiAllocationFrameIngressDescriptor {
        self.ingress.clone()
    }

    pub fn counters(self) -> UiAllocationFrameDispatcherCounters {
        self.counters
    }
}
