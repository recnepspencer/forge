use super::super::PlatformPulseApplicationRuntime;

const MAX_LOCAL_PRODUCT_CYCLE_ROUNDS: u8 = 8;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(in crate::native_application) enum PlatformPulseIntentProductCycleOutcome {
    Quiescent { rounds: u8 },
    Interrupted { rounds: u8 },
    Saturated { rounds: u8 },
}

impl PlatformPulseApplicationRuntime {
    pub(in crate::native_application) fn drain_intent_product_cycle(
        &mut self,
    ) -> PlatformPulseIntentProductCycleOutcome {
        for round in 1..=MAX_LOCAL_PRODUCT_CYCLE_ROUNDS {
            let execution_progress = self.advance_intent_execution();
            let product_progress = self.poll_intent_action_port();
            if self.terminal_error.is_some()
                || self.pending_managed_rebind.is_some()
                || self.pending_frame_presentation.is_some()
            {
                return PlatformPulseIntentProductCycleOutcome::Interrupted { rounds: round };
            }
            if execution_progress == 0 && product_progress == 0 {
                return PlatformPulseIntentProductCycleOutcome::Quiescent { rounds: round };
            }
        }
        self.fail_intent_settlement(format!(
            "local intent product cycle exceeded {MAX_LOCAL_PRODUCT_CYCLE_ROUNDS} rounds"
        ));
        PlatformPulseIntentProductCycleOutcome::Saturated {
            rounds: MAX_LOCAL_PRODUCT_CYCLE_ROUNDS,
        }
    }
}
