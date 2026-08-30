#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(in crate::runtime) struct UiFocusRestorationToken {
    participant: super::UiFocusParticipantIdentity,
    scope: super::UiFocusScopeIdentity,
    incarnation: worth_ui_host_contract::UiMountIncarnation,
}

impl UiFocusRestorationToken {
    pub(super) const fn from_focus(focus: super::UiSemanticKeyboardFocus) -> Self {
        Self {
            participant: focus.participant(),
            scope: focus.scope(),
            incarnation: focus.incarnation(),
        }
    }

    pub(super) const fn participant(self) -> super::UiFocusParticipantIdentity {
        self.participant
    }
    pub(super) const fn scope(self) -> super::UiFocusScopeIdentity {
        self.scope
    }
    pub(super) const fn incarnation(self) -> worth_ui_host_contract::UiMountIncarnation {
        self.incarnation
    }
}

impl super::UiFocusRuntimeState {
    #[cfg(test)]
    pub(super) fn plan_restoration(
        &self,
        token: UiFocusRestorationToken,
    ) -> Result<super::UiFocusPlan, super::UiFocusRoutingDenial> {
        let exact = self
            .exact_participant(token.scope(), token.participant(), token.incarnation())
            .ok();
        let next = exact.or_else(|| self.first_in_scope(token.scope()));
        Ok(self.plan_for(next, super::UiFocusCause::PortalRestoration, 1))
    }

    pub(in crate::runtime) const fn restoration_token(&self) -> Option<UiFocusRestorationToken> {
        match self.current {
            Some(focus) => Some(UiFocusRestorationToken::from_focus(focus)),
            None => None,
        }
    }
}
