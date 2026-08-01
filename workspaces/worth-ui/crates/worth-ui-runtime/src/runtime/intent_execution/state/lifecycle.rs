use super::UiIntentExecutionState;
use crate::runtime::intent::UiIntentAdmissionCancellationReason;
use crate::runtime::intent_execution::UiIntentExecutionCancellationReason;

impl UiIntentExecutionState {
    pub(crate) fn cancel_instance(
        &mut self,
        instance: worth_ui_host_contract::UiMountedInstanceIdentity,
    ) -> usize {
        self.cancel_matching(
            crate::runtime::intent::UiIntentAdmissionCancellationReason::MountedInstanceRemoved,
            |target| target.mounted_instance() == instance,
        )
    }

    pub(crate) fn cancel_binding(
        &mut self,
        binding: worth_ui_host_contract::UiSurfaceBindingGeneration,
    ) -> usize {
        self.cancel_matching(
            crate::runtime::intent::UiIntentAdmissionCancellationReason::SurfaceRebound,
            |target| target.binding() == binding,
        )
    }

    pub(crate) fn cancel_all(&mut self, reason: UiIntentAdmissionCancellationReason) -> usize {
        self.cancel_matching(reason, |_| true)
    }

    fn cancel_matching(
        &mut self,
        reason: crate::runtime::intent::UiIntentAdmissionCancellationReason,
        matches: impl Fn(crate::runtime::interaction::UiPresentedInteractionTargetView) -> bool,
    ) -> usize {
        let mut selected = 0;
        for slot in &mut self.slots {
            let is_selected = slot
                .phase
                .as_ref()
                .and_then(|phase| phase.reservation())
                .is_some_and(|reservation| matches(reservation.target));
            if !is_selected {
                continue;
            }
            let phase = slot
                .phase
                .take()
                .expect("selected execution slot is active");
            slot.phase = match phase {
                super::UiIntentExecutionSlotPhase::Admitted(reserved) => {
                    let _ = self.occupancy.release(reserved.reservation.core.occupancy);
                    reserved.reservation.core.lease.mark_cancelled(reason);
                    selected += 1;
                    None
                }
                super::UiIntentExecutionSlotPhase::AttemptPrepared(prepared) => {
                    let _ = self.occupancy.release(prepared.reservation.core.occupancy);
                    prepared.reservation.core.lease.mark_cancelled(reason);
                    selected += 1;
                    None
                }
                super::UiIntentExecutionSlotPhase::Running(mut running) => {
                    if running.cancellation.is_none() {
                        running.cancellation = Some(execution_cancellation(reason));
                        selected += 1;
                    }
                    Some(super::UiIntentExecutionSlotPhase::Running(running))
                }
                retained @ super::UiIntentExecutionSlotPhase::Recovery(_) => Some(retained),
                retained @ super::UiIntentExecutionSlotPhase::ConsequencePending(_) => {
                    Some(retained)
                }
                retained @ super::UiIntentExecutionSlotPhase::ConsequenceReady(_) => Some(retained),
                retained @ super::UiIntentExecutionSlotPhase::ConsequenceHandoff(_) => {
                    Some(retained)
                }
            };
        }
        selected
    }
}

const fn execution_cancellation(
    reason: UiIntentAdmissionCancellationReason,
) -> UiIntentExecutionCancellationReason {
    match reason {
        UiIntentAdmissionCancellationReason::MountedInstanceRemoved => {
            UiIntentExecutionCancellationReason::MountedInstanceRemoved
        }
        UiIntentAdmissionCancellationReason::SurfaceRebound => {
            UiIntentExecutionCancellationReason::SurfaceRebound
        }
        UiIntentAdmissionCancellationReason::ApplicationRebound => {
            UiIntentExecutionCancellationReason::ApplicationRebound
        }
        UiIntentAdmissionCancellationReason::Shutdown => {
            UiIntentExecutionCancellationReason::Shutdown
        }
    }
}
