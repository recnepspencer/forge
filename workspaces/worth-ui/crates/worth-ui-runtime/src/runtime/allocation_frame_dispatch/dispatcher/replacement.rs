use super::{
    UiAllocationFrameDispatcher, UiAllocationFrameDispatcherState,
    UiAllocationFrameEpochAssignment, ALLOCATION_FRAME_MAILBOX_CAPACITY,
};
use crate::runtime::allocation_frame_dispatch::{
    UiAllocationFrameDispatchDenial, UiAllocationFrameMailbox, UiAllocationFramePauseReason,
    UiAllocationFrameQueueDisposition, UiAllocationFrameReplacementTransition,
    UiAllocationFrameRetryState,
};

impl UiAllocationFrameDispatcher {
    pub(super) fn launch_with_runtime_state(
        epoch: super::UiAllocationFrameEpoch,
        retry_state: UiAllocationFrameRetryState,
    ) -> Self {
        let mut dispatcher = Self::with_capacity(epoch, ALLOCATION_FRAME_MAILBOX_CAPACITY);
        dispatcher.retry_state = retry_state;
        dispatcher
    }

    pub(super) fn replacement_successor_epoch(&self) -> Option<super::UiAllocationFrameEpoch> {
        match self.state {
            UiAllocationFrameDispatcherState::Open(epoch)
            | UiAllocationFrameDispatcherState::Sealed(epoch)
            | UiAllocationFrameDispatcherState::Dispatched(epoch) => epoch.checked_next(),
            UiAllocationFrameDispatcherState::Closing { next_epoch, .. } => Some(next_epoch),
            UiAllocationFrameDispatcherState::Paused(_) => None,
        }
    }

    pub(in crate::runtime::allocation_frame_dispatch) fn prepare_replacement_assignment(
        &self,
    ) -> Option<UiAllocationFrameEpochAssignment> {
        self.replacement_successor_epoch()
            .map(UiAllocationFrameEpochAssignment::from_linearization)
    }

    pub(in crate::runtime::allocation_frame_dispatch) fn prepare_replacement_successor(
        &self,
    ) -> Result<
        (
            UiAllocationFrameEpochAssignment,
            UiAllocationFrameReplacementTransition,
            Self,
        ),
        UiAllocationFrameDispatchDenial,
    > {
        let assignment = self
            .prepare_replacement_assignment()
            .ok_or(UiAllocationFrameDispatchDenial::EpochExhausted)?;
        let lifecycle_is_quiescent = matches!(
            self.state,
            UiAllocationFrameDispatcherState::Open(_)
                | UiAllocationFrameDispatcherState::Dispatched(_)
        );
        if !lifecycle_is_quiescent
            || !self.mailbox.is_empty()
            || !self.successor_mailbox.is_empty()
            || self.sealed_frame.is_some()
        {
            return Err(UiAllocationFrameDispatchDenial::ReplacementNotQuiescent);
        }
        let mut counters = self.counters;
        counters.record_canonical_drain();
        let (ingress, _) = UiAllocationFrameMailbox::new(self.mailbox.capacity()).drain_canonical();
        let (successor_ingress, _) =
            UiAllocationFrameMailbox::new(self.successor_mailbox.capacity()).drain_canonical();
        let retry_state = self.retry_state.clone();
        let disposition = UiAllocationFrameQueueDisposition::disposed(
            &self.seal_authority,
            UiAllocationFramePauseReason::Replacement,
            ingress,
            successor_ingress,
            counters,
        );
        let transition = UiAllocationFrameReplacementTransition::paused(
            &self.seal_authority,
            disposition,
            assignment,
            retry_state.clone(),
        );
        let successor = Self::launch_with_runtime_state(assignment.epoch(), retry_state);
        Ok((assignment, transition, successor))
    }
}
