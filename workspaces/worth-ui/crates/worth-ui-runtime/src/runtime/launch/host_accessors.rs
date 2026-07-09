use crate::runtime::active::{WorthUiActiveRuntimeObservation, WorthUiActiveRuntimeState};
use crate::runtime::{
    WorthUiLastValidObservation, WorthUiRuntimeFrameEpoch, WorthUiRuntimeLifecycle,
    WorthUiRuntimeShutdownReceipt,
};

use super::host::WorthUiRuntimeHost;
#[cfg(test)]
use super::launch_request::WorthUiRuntimeLaunchDenial;
use super::preservation::WorthUiLastValidRuntimeState;

impl WorthUiRuntimeHost {
    pub fn lifecycle(&self) -> WorthUiRuntimeLifecycle {
        self.active.lifecycle()
    }

    pub fn frame_epoch(&self) -> WorthUiRuntimeFrameEpoch {
        self.active.frame_epoch()
    }

    pub fn inspect_active(&self) -> WorthUiActiveRuntimeObservation {
        self.active.observation()
    }

    pub fn last_valid(&self) -> WorthUiLastValidObservation {
        self.last_valid.observation()
    }

    pub fn shutdown(self) -> WorthUiRuntimeShutdownReceipt {
        WorthUiRuntimeShutdownReceipt::new(self.active.frame_epoch())
    }

    pub(crate) fn active_state_for_swap_mut(&mut self) -> &mut WorthUiActiveRuntimeState {
        &mut self.active
    }

    pub(crate) fn record_last_valid_from_active_for_swap(&mut self) {
        self.last_valid = WorthUiLastValidRuntimeState::record_from_active(&self.active);
    }

    #[cfg(test)]
    pub(crate) fn reject_if_pending_activation_is_stale(
        &self,
        pending_activation: crate::runtime::WorthUiPendingActivation,
    ) -> Result<(), WorthUiRuntimeLaunchDenial> {
        let active_epoch = self.active.frame_epoch();
        let pending_epoch = pending_activation.frame_epoch();
        if pending_epoch == active_epoch {
            Ok(())
        } else {
            Err(WorthUiRuntimeLaunchDenial::StalePendingActivation {
                pending_epoch,
                active_epoch,
            })
        }
    }

    #[cfg(test)]
    pub(crate) fn advance_frame_epoch_for_test(&mut self) {
        self.active
            .advance_frame_epoch_for_test(self.active.frame_epoch().next());
    }
}
