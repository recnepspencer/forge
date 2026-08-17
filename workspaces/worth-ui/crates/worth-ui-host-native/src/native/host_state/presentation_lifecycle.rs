use crate::native::physical_work_signal::UiNativePhysicalSignalSettlement;

use super::UiNativeHostState;

impl UiNativeHostState {
    pub(super) fn progress_pending_presentation(
        &mut self,
        identity: crate::native::physical_work_signal::UiNativePhysicalPresentationIdentity,
    ) -> bool {
        let Some(index) = self
            .pending_presentations
            .iter()
            .position(|pending| pending.physical_work() == identity)
        else {
            return false;
        };
        let mut pending = self.pending_presentations.remove(index);
        let device = self.graphics.as_ref().map(|graphics| &graphics.device);
        let Ok(token) = self.physical_signal.take_ready_presentation(identity) else {
            self.pending_presentations.insert(index, pending);
            return false;
        };
        if token != pending.physical_token() {
            self.pending_presentations.insert(index, pending);
            return false;
        }
        let observation = pending.poll_observation(device);
        match self.physical_signal.reconcile(observation) {
            UiNativePhysicalSignalSettlement::Completed => pending.release(&mut self.resources),
            UiNativePhysicalSignalSettlement::Indeterminate => {
                let Ok(recovery) = self
                    .physical_signal
                    .transition_presentation_to_recovery(identity)
                else {
                    self.pending_presentations.insert(index, pending);
                    return false;
                };
                if !pending.refresh_physical_token(recovery) {
                    return false;
                }
                self.pending_presentations.insert(index, pending);
            }
            UiNativePhysicalSignalSettlement::Pending
            | UiNativePhysicalSignalSettlement::Rejected
            | UiNativePhysicalSignalSettlement::Stale => {
                self.pending_presentations.insert(index, pending);
            }
        }
        true
    }
}
