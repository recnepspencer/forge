use crate::runtime::allocation_frame_dispatch::{
    UiAllocationFrameMailboxDrain, UiAllocationFramePauseReason,
};

use super::super::dispatcher::UiAllocationFrameSealAuthority;
use super::{UiAdmittedAllocationStreamFrame, UiAllocationFrameDispatcherCounters};

/// Immutable terminal accounting for ingress retained when a dispatcher pauses.
#[derive(Debug, PartialEq)]
pub struct UiAllocationFrameQueueDisposition {
    representation: UiAllocationFrameQueueDispositionRepresentation,
}

#[derive(Debug, PartialEq)]
enum UiAllocationFrameQueueDispositionRepresentation {
    Disposed {
        reason: UiAllocationFramePauseReason,
        ingress: Box<UiAllocationFrameMailboxDrain>,
        successor_ingress: Box<UiAllocationFrameMailboxDrain>,
        counters: UiAllocationFrameDispatcherCounters,
    },
    Sealed {
        reason: UiAllocationFramePauseReason,
        frame: Box<UiAdmittedAllocationStreamFrame>,
        successor_ingress: Box<UiAllocationFrameMailboxDrain>,
        counters: UiAllocationFrameDispatcherCounters,
    },
}

impl UiAllocationFrameQueueDisposition {
    pub(in crate::runtime::allocation_frame_dispatch) fn disposed(
        _seal_authority: &UiAllocationFrameSealAuthority,
        reason: UiAllocationFramePauseReason,
        ingress: UiAllocationFrameMailboxDrain,
        successor_ingress: UiAllocationFrameMailboxDrain,
        counters: UiAllocationFrameDispatcherCounters,
    ) -> Self {
        Self {
            representation: UiAllocationFrameQueueDispositionRepresentation::Disposed {
                reason,
                ingress: Box::new(ingress),
                successor_ingress: Box::new(successor_ingress),
                counters,
            },
        }
    }

    pub(in crate::runtime::allocation_frame_dispatch) fn sealed(
        _seal_authority: &UiAllocationFrameSealAuthority,
        reason: UiAllocationFramePauseReason,
        frame: UiAdmittedAllocationStreamFrame,
        successor_ingress: UiAllocationFrameMailboxDrain,
        counters: UiAllocationFrameDispatcherCounters,
    ) -> Self {
        Self {
            representation: UiAllocationFrameQueueDispositionRepresentation::Sealed {
                reason,
                frame: Box::new(frame),
                successor_ingress: Box::new(successor_ingress),
                counters,
            },
        }
    }

    #[cfg(test)]
    pub fn reason(&self) -> UiAllocationFramePauseReason {
        match &self.representation {
            UiAllocationFrameQueueDispositionRepresentation::Disposed { reason, .. }
            | UiAllocationFrameQueueDispositionRepresentation::Sealed { reason, .. } => *reason,
        }
    }

    #[cfg(test)]
    pub fn ingress(&self) -> super::super::mailbox::UiAllocationFrameIngressView<'_> {
        match &self.representation {
            UiAllocationFrameQueueDispositionRepresentation::Disposed { ingress, .. } => {
                ingress.view()
            }
            UiAllocationFrameQueueDispositionRepresentation::Sealed { frame, .. } => {
                frame.ingress()
            }
        }
    }

    #[cfg(test)]
    pub fn counters(&self) -> UiAllocationFrameDispatcherCounters {
        match &self.representation {
            UiAllocationFrameQueueDispositionRepresentation::Disposed { counters, .. } => *counters,
            UiAllocationFrameQueueDispositionRepresentation::Sealed { counters, .. } => *counters,
        }
    }

    #[cfg(test)]
    pub fn successor_ingress(&self) -> super::super::mailbox::UiAllocationFrameIngressView<'_> {
        match &self.representation {
            UiAllocationFrameQueueDispositionRepresentation::Disposed {
                successor_ingress, ..
            }
            | UiAllocationFrameQueueDispositionRepresentation::Sealed {
                successor_ingress, ..
            } => successor_ingress.view(),
        }
    }

    #[cfg(test)]
    pub(crate) fn sealed_frame(&self) -> Option<&UiAdmittedAllocationStreamFrame> {
        match &self.representation {
            UiAllocationFrameQueueDispositionRepresentation::Disposed { .. } => None,
            UiAllocationFrameQueueDispositionRepresentation::Sealed { frame, .. } => Some(frame),
        }
    }
}
