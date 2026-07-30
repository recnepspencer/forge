use super::model::{next, UiActiveLocalRecipient, UiDraftRuntimeState, UiDraftStateSnapshot};
use super::{UiDraftSessionIdentity, UiLocalInputStop, UiLocalInputStopReason};

impl UiDraftRuntimeState {
    pub(crate) fn snapshot(&self) -> UiDraftStateSnapshot {
        UiDraftStateSnapshot {
            active_recipients: usize::from(self.active.is_some()),
            active_sessions: self.sessions.len(),
            retained_utf8_bytes: self
                .sessions
                .values()
                .map(super::model::UiDraftSession::retained_utf8_bytes)
                .sum(),
            counters: self.counters,
        }
    }

    pub(crate) fn cancel_binding(
        &mut self,
        binding: worth_ui_host_contract::UiSurfaceBindingGeneration,
        reason: UiLocalInputStopReason,
    ) -> Vec<UiLocalInputStop> {
        self.cancel_where(|target| target.binding() == binding, reason)
    }

    pub(crate) fn cancel_instance(
        &mut self,
        instance: worth_ui_host_contract::UiMountedInstanceIdentity,
        reason: UiLocalInputStopReason,
    ) -> Vec<UiLocalInputStop> {
        self.cancel_where(|target| target.mounted_instance() == instance, reason)
    }

    pub(crate) fn cancel_all(&mut self, reason: UiLocalInputStopReason) -> Vec<UiLocalInputStop> {
        self.cancel_where(|_| true, reason)
    }

    pub(super) fn cancel_active(
        &mut self,
        reason: UiLocalInputStopReason,
    ) -> Option<UiLocalInputStop> {
        match self.active.take()? {
            UiActiveLocalRecipient::Draft(session) => self.cancel_session(session, reason),
            UiActiveLocalRecipient::Activation(context)
            | UiActiveLocalRecipient::Submit(context) => {
                self.counters.stop_outcomes = next(self.counters.stop_outcomes);
                Some(UiLocalInputStop::for_settled_recipient(
                    context.target.surface(),
                    context.target.presentation(),
                    reason,
                ))
            }
        }
    }

    pub(super) fn suspend_active(
        &mut self,
        reason: UiLocalInputStopReason,
    ) -> Option<UiLocalInputStop> {
        let active = self.active.take()?;
        self.counters.stop_outcomes = next(self.counters.stop_outcomes);
        Some(match active {
            UiActiveLocalRecipient::Draft(session) => {
                let draft = self
                    .sessions
                    .get(&session)
                    .expect("an active draft session remains retained");
                UiLocalInputStop::for_suspended_session(
                    session,
                    draft.target.surface(),
                    draft.target.presentation(),
                    reason,
                )
            }
            UiActiveLocalRecipient::Activation(context)
            | UiActiveLocalRecipient::Submit(context) => UiLocalInputStop::for_settled_recipient(
                context.target.surface(),
                context.target.presentation(),
                reason,
            ),
        })
    }

    pub(super) fn cancel_session(
        &mut self,
        session: UiDraftSessionIdentity,
        reason: UiLocalInputStopReason,
    ) -> Option<UiLocalInputStop> {
        let draft = self.sessions.remove(&session)?;
        if matches!(self.active, Some(UiActiveLocalRecipient::Draft(active)) if active == session) {
            self.active = None;
        }
        self.counters.sessions_settled = next(self.counters.sessions_settled);
        self.counters.stop_outcomes = next(self.counters.stop_outcomes);
        Some(UiLocalInputStop::for_settled_session(
            session,
            draft.target.surface(),
            draft.target.presentation(),
            reason,
        ))
    }

    fn cancel_where(
        &mut self,
        predicate: impl Fn(crate::runtime::interaction::UiPresentedInteractionTargetView) -> bool,
        reason: UiLocalInputStopReason,
    ) -> Vec<UiLocalInputStop> {
        let selected = self
            .sessions
            .iter()
            .filter_map(|(identity, session)| predicate(session.target).then_some(*identity))
            .collect::<Vec<_>>();
        let active_ephemeral = self.active_context().filter(|context| {
            !matches!(self.active, Some(UiActiveLocalRecipient::Draft(_)))
                && predicate(context.target)
        });
        let mut stops = selected
            .into_iter()
            .filter_map(|identity| self.cancel_session(identity, reason))
            .collect::<Vec<_>>();
        if let Some(context) = active_ephemeral {
            self.active = None;
            self.counters.stop_outcomes = next(self.counters.stop_outcomes);
            stops.push(UiLocalInputStop::for_settled_recipient(
                context.target.surface(),
                context.target.presentation(),
                reason,
            ));
        }
        stops
    }

    pub(super) fn unsettled_stop(
        &mut self,
        core: worth_ui_host_contract::UiHostObservationCanonicalCore,
        reason: UiLocalInputStopReason,
    ) -> UiLocalInputStop {
        self.counters.stop_outcomes = next(self.counters.stop_outcomes);
        UiLocalInputStop::for_unsettled_report(core.presentation(), reason)
    }

    pub(super) fn unsettled_session_stop(
        &mut self,
        session: UiDraftSessionIdentity,
        reason: UiLocalInputStopReason,
    ) -> UiLocalInputStop {
        self.counters.stop_outcomes = next(self.counters.stop_outcomes);
        let draft = self
            .sessions
            .get(&session)
            .expect("validated active draft remains present");
        UiLocalInputStop::for_unsettled_session(
            session,
            draft.target.surface(),
            draft.target.presentation(),
            reason,
        )
    }
}
