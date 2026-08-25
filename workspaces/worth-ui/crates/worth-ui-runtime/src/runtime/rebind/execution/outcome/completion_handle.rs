use super::{
    content_mapping, mapping, UiRebindCompletionHandle, UiRebindCompletionInner,
    UiRebindCompletionState, UiRebindOutcome,
};
use crate::runtime::rebind::execution::state::UiRebindReservation;

impl<'session> UiRebindCompletionHandle<'session> {
    pub(super) fn new(
        plan: crate::runtime::rebind::UiRebindPlan,
        registration: UiRebindReservation,
        inner: Box<crate::facade::WorthUiMountedApplicationReplacementInFlight<'session>>,
    ) -> Self {
        Self {
            state: Some(Box::new(UiRebindCompletionState {
                plan,
                registration,
                inner: UiRebindCompletionInner::Changed(inner),
            })),
        }
    }

    pub(super) fn content(
        plan: crate::runtime::rebind::UiRebindPlan,
        registration: UiRebindReservation,
        generation: crate::facade::prepared_application_authority::
            WorthUiPreparedApplicationGenerationIdentity,
        inner: Box<crate::facade::entry::WorthUiMountedContentRebindInFlight<'session>>,
    ) -> Self {
        Self {
            state: Some(Box::new(UiRebindCompletionState {
                plan,
                registration,
                inner: UiRebindCompletionInner::Content { generation, inner },
            })),
        }
    }

    pub fn attempt(&self) -> worth_ui_host_contract::UiMountedPresentationAttemptIdentity {
        match &self.state().inner {
            UiRebindCompletionInner::Changed(inner) => inner.attempt(),
            UiRebindCompletionInner::Content { inner, .. } => inner.attempt(),
        }
    }

    pub fn deadline(&self) -> worth_ui_host_contract::UiPresentationDeadline {
        match &self.state().inner {
            UiRebindCompletionInner::Changed(inner) => inner.deadline(),
            UiRebindCompletionInner::Content { inner, .. } => inner.deadline(),
        }
    }

    pub fn complete(self, now_tick: u64) -> UiRebindOutcome<'session> {
        let state = self.into_state();
        match state.inner {
            UiRebindCompletionInner::Changed(inner) => {
                let outcome = inner.complete(now_tick);
                mapping::map_changed_completion(state.plan, state.registration, outcome)
            }
            UiRebindCompletionInner::Content { generation, inner } => {
                let outcome = inner.complete(now_tick);
                content_mapping::map_content_completion(
                    state.plan,
                    state.registration,
                    generation,
                    outcome,
                )
            }
        }
    }

    pub fn dispose(self) -> UiRebindOutcome<'session> {
        let state = self.into_state();
        match state.inner {
            UiRebindCompletionInner::Changed(inner) => {
                let outcome = inner.cancel();
                mapping::map_changed_cancellation(state.plan, state.registration, outcome)
            }
            UiRebindCompletionInner::Content { generation, inner } => {
                let outcome = inner.cancel();
                content_mapping::map_content_cancellation(
                    state.plan,
                    state.registration,
                    generation,
                    outcome,
                )
            }
        }
    }

    pub(crate) fn detach_for_native(self) -> super::UiDetachedRebindCompletion {
        super::UiDetachedRebindCompletion::from_state(*self.into_state())
    }

    fn state(&self) -> &UiRebindCompletionState<'session> {
        self.state
            .as_deref()
            .expect("live completion handle owns its state")
    }

    fn into_state(mut self) -> Box<UiRebindCompletionState<'session>> {
        self.state
            .take()
            .expect("live completion handle owns its state")
    }
}

impl Drop for UiRebindCompletionHandle<'_> {
    fn drop(&mut self) {
        let Some(state) = self.state.take() else {
            return;
        };
        let outcome = match state.inner {
            UiRebindCompletionInner::Changed(inner) => {
                mapping::map_changed_cancellation(state.plan, state.registration, inner.cancel())
            }
            UiRebindCompletionInner::Content { generation, inner } => {
                content_mapping::map_content_cancellation(
                    state.plan,
                    state.registration,
                    generation,
                    inner.cancel(),
                )
            }
        };
        drop(outcome);
    }
}
