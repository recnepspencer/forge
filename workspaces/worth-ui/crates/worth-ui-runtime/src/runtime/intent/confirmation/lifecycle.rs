use super::state::{
    UiIntentConfirmationSlotState, UiIntentConfirmationState, UiIntentConfirmationTerminal,
    UiIntentConfirmationTerminalKind,
};
use super::{
    UiIntentConfirmationCancellationReason, UiIntentConfirmationChallenge,
    UiIntentConfirmationSettlementReceipt, UiIntentConfirmationShutdownReport,
};

impl UiIntentConfirmationState {
    pub(crate) fn cancel_instance(
        &mut self,
        instance: worth_ui_host_contract::UiMountedInstanceIdentity,
        reason: UiIntentConfirmationCancellationReason,
    ) -> UiIntentConfirmationSettlementReceipt {
        self.cancel_matching(reason, |challenge| {
            challenge
                .candidate
                .input_basis()
                .target()
                .mounted_instance()
                == instance
        })
    }

    pub(crate) fn cancel_binding(
        &mut self,
        binding: worth_ui_host_contract::UiSurfaceBindingGeneration,
        reason: UiIntentConfirmationCancellationReason,
    ) -> UiIntentConfirmationSettlementReceipt {
        self.cancel_matching(reason, |challenge| {
            challenge.candidate.input_basis().target().binding() == binding
        })
    }

    pub(crate) fn cancel_all(
        &mut self,
        reason: UiIntentConfirmationCancellationReason,
    ) -> UiIntentConfirmationSettlementReceipt {
        self.cancel_matching(reason, |_| true)
    }

    fn cancel_matching(
        &mut self,
        reason: UiIntentConfirmationCancellationReason,
        matches: impl Fn(&UiIntentConfirmationChallenge) -> bool,
    ) -> UiIntentConfirmationSettlementReceipt {
        let mut settled = 0;
        for slot in &mut self.slots {
            let should_cancel = match &slot.state {
                UiIntentConfirmationSlotState::Pending(challenge) => matches(challenge),
                UiIntentConfirmationSlotState::Vacant
                | UiIntentConfirmationSlotState::Terminal(_) => false,
            };
            if !should_cancel {
                continue;
            }
            let UiIntentConfirmationSlotState::Pending(challenge) =
                core::mem::replace(&mut slot.state, UiIntentConfirmationSlotState::Vacant)
            else {
                unreachable!("the matching slot was pending")
            };
            slot.state = UiIntentConfirmationSlotState::Terminal(
                UiIntentConfirmationTerminal::from_challenge(
                    &challenge,
                    UiIntentConfirmationTerminalKind::Cancelled(reason),
                ),
            );
            drop(challenge);
            settled += 1;
        }
        self.record_cancelled(settled);
        UiIntentConfirmationSettlementReceipt::new(reason, settled, self.pending_count())
    }

    pub(crate) fn shutdown(&mut self) -> UiIntentConfirmationShutdownReport {
        let settlement = self.cancel_all(UiIntentConfirmationCancellationReason::Shutdown);
        UiIntentConfirmationShutdownReport::new(
            settlement.settled_challenges(),
            settlement.pending_after(),
        )
    }
}
