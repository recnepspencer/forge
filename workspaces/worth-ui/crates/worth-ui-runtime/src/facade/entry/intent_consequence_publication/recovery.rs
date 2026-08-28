use super::*;

impl<'session> UiIntentConsequencePublicationRecovery<'session> {
    pub fn frame(&self) -> &crate::mounting::UiMountedIndeterminateFrame {
        &self
            .state
            .as_deref()
            .expect("live consequence recovery owns its state")
            .frame
    }

    #[cfg(any(test, feature = "certification-support"))]
    #[doc(hidden)]
    pub fn inspect_retained_service_family_state_for_certification(
        &self,
    ) -> (
        crate::certification_support::UiPortalRuntimeCertificationSnapshot,
        crate::certification_support::UiFocusRuntimeCertificationSnapshot,
        crate::certification_support::UiServiceProposalCertificationSnapshot,
    ) {
        let state = self
            .state
            .as_deref()
            .expect("live consequence recovery owns its state");
        let session = &*state.admitted.session;
        (
            session.inspect_portal_runtime_for_certification(),
            session.inspect_focus_runtime_for_certification(),
            session.inspect_service_proposals_for_certification(),
        )
    }

    pub fn into_session_for_shutdown(mut self) -> &'session mut WorthUiActiveApplicationSession {
        let mut state = self
            .state
            .take()
            .expect("live consequence recovery owns its state");
        if let Some(portal) = state.portal.take() {
            state
                .admitted
                .session
                .application
                .abandon_indeterminate_portal_service_proposal_for_shutdown(
                    portal,
                    state
                        .admitted
                        .session
                        .focus
                        .as_mut()
                        .expect("indeterminate proposal retains Focus installation"),
                    state
                        .admitted
                        .session
                        .motion
                        .as_mut()
                        .expect("indeterminate proposal retains Motion installation"),
                );
        }
        withdraw_query(&mut state.admitted);
        state
            .admitted
            .session
            .intent_execution
            .dispose_consequence_handoff(state.admitted.transfer.consequence);
        drop((state.admitted.plan, state.admitted.reservation, state.frame));
        state.admitted.session
    }
}

impl Drop for UiIntentConsequencePublicationRecovery<'_> {
    fn drop(&mut self) {
        let Some(mut state) = self.state.take() else {
            return;
        };
        if let Some(portal) = state.portal.take() {
            state
                .admitted
                .session
                .application
                .dispose_indeterminate_portal_service_proposal(
                    portal,
                    state
                        .admitted
                        .session
                        .focus
                        .as_mut()
                        .expect("indeterminate proposal retains Focus installation"),
                    state
                        .admitted
                        .session
                        .motion
                        .as_mut()
                        .expect("indeterminate proposal retains Motion installation"),
                );
        }
        withdraw_query(&mut state.admitted);
        state
            .admitted
            .session
            .intent_execution
            .dispose_consequence_handoff(state.admitted.transfer.consequence);
        drop((state.admitted.plan, state.admitted.reservation, state.frame));
    }
}
