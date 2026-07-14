use super::super::dispatcher::{UiAllocationFrameEpochAssignment, UiAllocationFrameSealAuthority};
use super::{UiAllocationFrameDispatchDenial, UiAllocationFrameQueueDisposition};
use crate::runtime::allocation_frame_dispatch::UiAllocationFrameRetryState;
use crate::runtime::UiAllocationFrameEpoch;

/// Terminal old-generation accounting plus the only successor-epoch witness.
#[derive(Debug, PartialEq)]
pub struct UiAllocationFrameReplacementTransition {
    queue_disposition: UiAllocationFrameQueueDisposition,
    successor_assignment: Option<UiAllocationFrameEpochAssignment>,
    denial: Option<UiAllocationFrameDispatchDenial>,
    retry_state: UiAllocationFrameRetryState,
}

impl UiAllocationFrameReplacementTransition {
    pub(in crate::runtime::allocation_frame_dispatch) fn paused(
        _seal_authority: &UiAllocationFrameSealAuthority,
        queue_disposition: UiAllocationFrameQueueDisposition,
        successor_assignment: UiAllocationFrameEpochAssignment,
        retry_state: UiAllocationFrameRetryState,
    ) -> Self {
        Self {
            queue_disposition,
            successor_assignment: Some(successor_assignment),
            denial: None,
            retry_state,
        }
    }

    pub(in crate::runtime::allocation_frame_dispatch) fn denied(
        _seal_authority: &UiAllocationFrameSealAuthority,
        queue_disposition: UiAllocationFrameQueueDisposition,
        denial: UiAllocationFrameDispatchDenial,
        retry_state: UiAllocationFrameRetryState,
    ) -> Self {
        Self {
            queue_disposition,
            successor_assignment: None,
            denial: Some(denial),
            retry_state,
        }
    }

    pub(crate) fn queue_disposition(&self) -> &UiAllocationFrameQueueDisposition {
        &self.queue_disposition
    }

    pub fn successor_epoch(&self) -> Option<UiAllocationFrameEpoch> {
        self.successor_assignment
            .map(|assignment| assignment.epoch())
    }

    pub(crate) fn successor_assignment(&self) -> Option<UiAllocationFrameEpochAssignment> {
        self.successor_assignment
    }

    pub(crate) fn retry_state(&self) -> UiAllocationFrameRetryState {
        self.retry_state.clone()
    }
}
