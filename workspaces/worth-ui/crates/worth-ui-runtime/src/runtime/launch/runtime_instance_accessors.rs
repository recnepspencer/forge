use crate::runtime::active::WorthUiActiveRuntimeObservation;
use crate::runtime::{
    UiAllocationFrameDispatcherCounters, UiAllocationFrameDispatcherState,
    WorthUiLastValidObservation, WorthUiRuntimeFrameEpoch, WorthUiRuntimeLifecycle,
    WorthUiRuntimeShutdownReceipt,
};

#[cfg(test)]
use super::launch_request::WorthUiRuntimeLaunchDenial;
use super::runtime_instance::WorthUiRuntime;

impl WorthUiRuntime {
    pub(crate) fn bind_operational_host_session(
        &mut self,
        identity: crate::facade::WorthUiHostSessionIdentity,
        observation_generation: worth_ui_host_contract::WorthUiHostCapabilityObservationGeneration,
    ) {
        self.host_session_identity = Some(identity);
        self.host_observation_generation = Some(observation_generation);
    }

    #[cfg(test)]
    pub(crate) fn replace_host_observation_generation_for_test(
        &mut self,
        observation_generation: worth_ui_host_contract::WorthUiHostCapabilityObservationGeneration,
    ) {
        self.host_observation_generation = Some(observation_generation);
    }

    pub(crate) fn bind_active_application_generation(
        &mut self,
        generation_identity: crate::facade::prepared_application_authority::WorthUiPreparedApplicationGenerationIdentity,
    ) {
        self.active.bind_application_generation(generation_identity);
    }

    pub(crate) fn bind_retained_allocation_planning_evidence(
        &mut self,
        retained: std::rc::Rc<crate::runtime::WorthUiRetainedAllocationPlanningEvidenceRegistry>,
    ) {
        self.retained_allocation_planning_evidence = retained;
    }

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

    pub fn allocation_frame_dispatcher_state(&self) -> UiAllocationFrameDispatcherState {
        self.allocation_frame_scheduler.state()
    }

    pub fn allocation_frame_dispatcher_counters(&self) -> UiAllocationFrameDispatcherCounters {
        self.allocation_frame_scheduler.counters()
    }

    pub fn durable_semantic_state(
        &self,
    ) -> Option<crate::runtime::UiAllocationDurableSemanticState> {
        self.allocation_receipt_ledger.durable_semantic_state()
    }

    pub fn shutdown(mut self) -> WorthUiRuntimeShutdownReceipt {
        let queue_disposition = self.shutdown_allocation_frame_dispatcher();
        WorthUiRuntimeShutdownReceipt::new(self.active.frame_epoch(), queue_disposition)
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
        self.active.advance_frame_epoch_for_test();
    }
}
