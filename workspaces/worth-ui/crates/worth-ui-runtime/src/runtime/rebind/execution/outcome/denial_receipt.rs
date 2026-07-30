use super::{
    UiRebindDenialCause, UiRebindDenialReceipt, UiRebindOutcome, UiRebindStoppedPhase,
    UiRebindValidNextAction,
};
use crate::runtime::rebind::execution::state::UiRebindReservation;

impl<'session> UiRebindDenialReceipt<'session> {
    pub(crate) fn capacity(
        denial: super::super::UiRebindReservationDenial,
        retry: super::super::UiPreparedRebind<'session>,
    ) -> Self {
        Self {
            predecessor_remains_current: true,
            stopped_phase: UiRebindStoppedPhase::EffectAdmission,
            cause: UiRebindDenialCause::RuntimeCapacity(denial),
            valid_next_action: UiRebindValidNextAction::RetryPrepared,
            retry: Some(Box::new(retry)),
        }
    }

    pub(super) fn retry(
        plan: crate::runtime::rebind::UiRebindPlan,
        registration: UiRebindReservation,
        kind: super::super::preparation::UiPreparedRebindKind<'session>,
        stopped_phase: UiRebindStoppedPhase,
        cause: UiRebindDenialCause,
    ) -> Self {
        Self {
            predecessor_remains_current: true,
            stopped_phase,
            cause,
            valid_next_action: UiRebindValidNextAction::RetryPrepared,
            retry: Some(Box::new(super::super::UiPreparedRebind {
                plan,
                reservation: registration,
                kind,
            })),
        }
    }

    pub const fn predecessor_remains_current(&self) -> bool {
        self.predecessor_remains_current
    }

    pub const fn stopped_phase(&self) -> UiRebindStoppedPhase {
        self.stopped_phase
    }

    pub const fn cause(&self) -> UiRebindDenialCause {
        self.cause
    }

    pub const fn valid_next_action(&self) -> UiRebindValidNextAction {
        self.valid_next_action
    }

    pub fn retry_at(mut self, now_tick: u64) -> UiRebindOutcome<'session> {
        match self.retry.take() {
            Some(retry) => retry.execute(now_tick),
            None => UiRebindOutcome::RejectedBeforeEffects(self),
        }
    }
}
