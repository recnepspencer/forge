use super::{content_mapping, mapping, UiRebindCompletionInner, UiRebindCompletionState};

pub(crate) struct UiDetachedRebindCompletion {
    plan: crate::runtime::rebind::UiRebindPlan,
    registration: crate::runtime::rebind::UiRebindReservation,
    inner: UiDetachedRebindCompletionInner,
}

enum UiDetachedRebindCompletionInner {
    Changed(crate::facade::WorthUiDetachedMountedApplicationReplacementInFlight),
    Content {
        generation: crate::facade::prepared_application_authority::
            WorthUiPreparedApplicationGenerationIdentity,
        inner: crate::facade::entry::WorthUiDetachedMountedContentRebindInFlight,
    },
}

impl UiDetachedRebindCompletion {
    pub(super) fn from_state(state: UiRebindCompletionState<'_>) -> Self {
        let inner = match state.inner {
            UiRebindCompletionInner::Changed(inner) => {
                UiDetachedRebindCompletionInner::Changed(inner.detach())
            }
            UiRebindCompletionInner::Content { generation, inner } => {
                UiDetachedRebindCompletionInner::Content {
                    generation,
                    inner: inner.detach(),
                }
            }
        };
        Self {
            plan: state.plan,
            registration: state.registration,
            inner,
        }
    }

    pub(crate) fn session_identity(
        &self,
    ) -> crate::facade::WorthUiActiveApplicationSessionIdentity {
        match &self.inner {
            UiDetachedRebindCompletionInner::Changed(inner) => inner.session_identity(),
            UiDetachedRebindCompletionInner::Content { inner, .. } => inner.session_identity(),
        }
    }

    pub(crate) fn matches_native_progress(
        &self,
        progress: &crate::native_platform::UiNativeApplicationPhysicalProgress,
    ) -> bool {
        let class = match progress.class() {
            worth_ui_host_native::UiNativePhysicalProgressClass::Presentation => {
                worth_ui_host_contract::UiHostPresentationProgressClass::PhysicalSurface
            }
            worth_ui_host_native::UiNativePhysicalProgressClass::TextAtlas => {
                worth_ui_host_contract::UiHostPresentationProgressClass::TextAtlas
            }
            worth_ui_host_native::UiNativePhysicalProgressClass::PresentationRecovery => {
                return false;
            }
        };
        let (attempt, awaits, binding_matches) = match &self.inner {
            UiDetachedRebindCompletionInner::Changed(inner) => (
                inner.attempt(),
                inner.awaits_progress_class(class),
                progress.presentation().map_or(true, |presentation| {
                    inner
                        .pending_bindings()
                        .any(|binding| binding == presentation.binding())
                }),
            ),
            UiDetachedRebindCompletionInner::Content { inner, .. } => (
                inner.attempt(),
                inner.awaits_progress_class(class),
                progress.presentation().map_or(true, |presentation| {
                    inner
                        .pending_bindings()
                        .any(|binding| binding == presentation.binding())
                }),
            ),
        };
        awaits
            && binding_matches
            && progress
                .presentation()
                .map_or(true, |presentation| presentation.attempt() == attempt)
    }

    pub(crate) fn complete<'session>(
        self,
        session: &'session mut crate::facade::WorthUiActiveApplicationSession,
        now_tick: u64,
    ) -> crate::runtime::rebind::UiRebindOutcome<'session> {
        match self.inner {
            UiDetachedRebindCompletionInner::Changed(inner) => mapping::map_changed_completion(
                self.plan,
                self.registration,
                inner.complete(session, now_tick),
            ),
            UiDetachedRebindCompletionInner::Content { generation, inner } => {
                content_mapping::map_content_completion(
                    self.plan,
                    self.registration,
                    generation,
                    inner.complete(session, now_tick),
                )
            }
        }
    }

    pub(crate) fn cancel<'session>(
        self,
        session: &'session mut crate::facade::WorthUiActiveApplicationSession,
    ) -> crate::runtime::rebind::UiRebindOutcome<'session> {
        match self.inner {
            UiDetachedRebindCompletionInner::Changed(inner) => mapping::map_changed_cancellation(
                self.plan,
                self.registration,
                inner.cancel(session),
            ),
            UiDetachedRebindCompletionInner::Content { generation, inner } => {
                content_mapping::map_content_cancellation(
                    self.plan,
                    self.registration,
                    generation,
                    inner.cancel(session),
                )
            }
        }
    }
}
