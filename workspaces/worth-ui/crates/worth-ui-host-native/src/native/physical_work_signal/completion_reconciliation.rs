use super::routing::{
    UiNativePhysicalSignalExternalObservation, UiNativePhysicalSignalExternalStatus,
    UiNativePhysicalSignalRequestToken,
};
use super::UiNativePhysicalSignalOwner;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(crate) enum UiNativePhysicalSignalSettlement {
    Pending,
    Completed,
    Rejected,
    Indeterminate,
    Stale,
}

pub(crate) fn reconcile(
    token: UiNativePhysicalSignalRequestToken,
    observation: UiNativePhysicalSignalExternalObservation,
) -> UiNativePhysicalSignalSettlement {
    if token.work() != observation.work() || token.handle() != observation.handle() {
        return UiNativePhysicalSignalSettlement::Stale;
    }
    match observation.status() {
        UiNativePhysicalSignalExternalStatus::Pending => UiNativePhysicalSignalSettlement::Pending,
        UiNativePhysicalSignalExternalStatus::Completed => {
            UiNativePhysicalSignalSettlement::Completed
        }
        UiNativePhysicalSignalExternalStatus::RejectedBeforeEffects
        | UiNativePhysicalSignalExternalStatus::RejectedAfterRasterization => {
            UiNativePhysicalSignalSettlement::Rejected
        }
        UiNativePhysicalSignalExternalStatus::EffectsIndeterminate => {
            UiNativePhysicalSignalSettlement::Indeterminate
        }
    }
}

impl UiNativePhysicalSignalOwner {
    pub(super) fn note_stale(&mut self) {
        self.counters.stale_observations = self.counters.stale_observations.saturating_add(1);
    }

    pub(crate) fn reconcile(
        &mut self,
        observation: UiNativePhysicalSignalExternalObservation,
    ) -> UiNativePhysicalSignalSettlement {
        let Some(token) = self.current_token(observation) else {
            return UiNativePhysicalSignalSettlement::Stale;
        };
        let status = observation.status();
        let settlement = reconcile(token, observation);
        match settlement {
            UiNativePhysicalSignalSettlement::Pending => self.reconcile_pending(token),
            UiNativePhysicalSignalSettlement::Completed
            | UiNativePhysicalSignalSettlement::Rejected
            | UiNativePhysicalSignalSettlement::Indeterminate => {
                self.reconcile_terminal(token, status, settlement)
            }
            UiNativePhysicalSignalSettlement::Stale => {
                self.note_stale();
                UiNativePhysicalSignalSettlement::Stale
            }
        }
    }

    fn current_token(
        &mut self,
        observation: UiNativePhysicalSignalExternalObservation,
    ) -> Option<UiNativePhysicalSignalRequestToken> {
        if observation.runtime() == self.runtime_identity {
            if let Ok(token) = self.begin_work(observation.work()) {
                return Some(token);
            }
        }
        self.note_stale();
        None
    }

    fn reconcile_pending(
        &mut self,
        token: UiNativePhysicalSignalRequestToken,
    ) -> UiNativePhysicalSignalSettlement {
        self.counters.pending_observations = self.counters.pending_observations.saturating_add(1);
        match self
            .worker_mut()
            .and_then(|worker| worker.schedule_pending_poll(token.handle()))
        {
            Ok(true) => UiNativePhysicalSignalSettlement::Pending,
            Ok(false) => {
                self.counters.indeterminate_observations =
                    self.counters.indeterminate_observations.saturating_add(1);
                UiNativePhysicalSignalSettlement::Indeterminate
            }
            Err(()) => {
                self.note_stale();
                UiNativePhysicalSignalSettlement::Stale
            }
        }
    }

    fn reconcile_terminal(
        &mut self,
        token: UiNativePhysicalSignalRequestToken,
        status: UiNativePhysicalSignalExternalStatus,
        settlement: UiNativePhysicalSignalSettlement,
    ) -> UiNativePhysicalSignalSettlement {
        let resolving_recovery = self.worker().is_ok_and(|worker| {
            worker.request_uses_operation(
                token.handle(),
                token.work(),
                super::declarations::UiNativePhysicalSignalOperation::Recovery,
            )
        });
        if self
            .worker_mut()
            .and_then(|worker| worker.reconcile(token.handle(), token.work(), status))
            .is_err()
        {
            self.note_stale();
            return UiNativePhysicalSignalSettlement::Stale;
        }
        self.record_terminal(token, settlement, resolving_recovery);
        settlement
    }

    fn record_terminal(
        &mut self,
        token: UiNativePhysicalSignalRequestToken,
        settlement: UiNativePhysicalSignalSettlement,
        resolving_recovery: bool,
    ) {
        match settlement {
            UiNativePhysicalSignalSettlement::Completed => {
                self.counters.completed_observations =
                    self.counters.completed_observations.saturating_add(1);
                if resolving_recovery {
                    self.counters.recovery_resolutions =
                        self.counters.recovery_resolutions.saturating_add(1);
                }
                self.route.remove(token);
                self.wake.remove(token.work());
            }
            UiNativePhysicalSignalSettlement::Rejected => {
                self.counters.rejected_observations =
                    self.counters.rejected_observations.saturating_add(1);
                self.route.remove(token);
                self.wake.remove(token.work());
            }
            UiNativePhysicalSignalSettlement::Indeterminate => {
                self.counters.indeterminate_observations =
                    self.counters.indeterminate_observations.saturating_add(1);
            }
            _ => unreachable!("only terminal postures reach terminal accounting"),
        }
    }
}
