use crate::graph::UiGraphFactConsumerKey;

use super::{UiIdentityLifecycleDecision, UiIdentityLifecycleEntry};

pub struct UiResolvedIdentityLifecycle {
    scope: super::super::UiResolvedAffectedScope,
    selected: Box<[UiIdentityLifecycleEntry]>,
}

impl UiResolvedIdentityLifecycle {
    pub(crate) const fn new(
        scope: super::super::UiResolvedAffectedScope,
        selected: Box<[UiIdentityLifecycleEntry]>,
    ) -> Self {
        Self { scope, selected }
    }

    pub fn scope(&self) -> &super::super::UiResolvedAffectedScope {
        &self.scope
    }

    pub fn selected(&self) -> &[UiIdentityLifecycleEntry] {
        &self.selected
    }

    pub fn decision_for(
        &self,
        key: &UiGraphFactConsumerKey,
    ) -> Option<UiIdentityLifecycleDecision> {
        if let Some(entry) = self.selected.iter().find(|entry| entry.key() == key) {
            return Some(entry.decision());
        }
        self.scope
            .source_succession()
            .and_then(|succession| succession.identity_lifecycle_index())
            .filter(|index| index.knows(key))
            .map(|_| UiIdentityLifecycleDecision::Unaffected)
    }

    pub(crate) fn into_parts(
        self,
    ) -> (
        super::super::UiResolvedAffectedScope,
        Box<[UiIdentityLifecycleEntry]>,
    ) {
        (self.scope, self.selected)
    }

    #[cfg(any(test, feature = "certification-support"))]
    pub(crate) fn replace_planning_session_for_certification(
        &mut self,
        session: crate::facade::WorthUiActiveApplicationSessionIdentity,
    ) {
        self.scope
            .replace_planning_session_for_certification(session);
    }

    #[cfg(any(test, feature = "certification-support"))]
    pub(crate) fn replace_planning_predecessor_for_certification(
        &mut self,
        generation: crate::facade::prepared_application_authority::
            WorthUiPreparedApplicationGenerationIdentity,
    ) {
        self.scope
            .replace_planning_predecessor_for_certification(generation);
    }

    #[cfg(any(test, feature = "certification-support"))]
    pub(crate) fn replace_planning_candidate_for_certification(
        &mut self,
        generation: crate::facade::prepared_application_authority::
            WorthUiPreparedApplicationGenerationIdentity,
    ) {
        self.scope
            .replace_planning_candidate_for_certification(generation);
    }
}
