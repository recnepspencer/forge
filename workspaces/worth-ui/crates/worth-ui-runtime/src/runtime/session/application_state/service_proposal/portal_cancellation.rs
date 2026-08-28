impl super::WorthUiApplicationSessionState {
    pub(crate) fn cancel_portal_service_proposal(
        &mut self,
        transaction: super::UiStagedPortalProposalTransaction,
        focus: &mut crate::runtime::focus::UiFocusRuntimeState,
        motion_state: &mut crate::runtime::motion::UiMotionRuntimeState,
    ) {
        focus
            .discard_portal_proposal(transaction.focus.proposal())
            .expect("cancelled transaction retains its exact Focus proposal");
        let motion_scope = transaction.motion.as_ref().map(|motion| motion.scope());
        if let Some(motion) = transaction.motion {
            motion_state.discard_derived(motion);
        }
        let teardown = self
            .runtime
            .service_proposals
            .cancel_staged(transaction.batch);
        self.finish_portal_teardown(
            teardown,
            &transaction.portal,
            &transaction.focus,
            transaction.scroll.as_ref(),
            transaction.selection.as_ref(),
            motion_scope,
            crate::runtime::session::service_proposal::UiServiceProposalTerminalReason::CancelledBeforePublication,
        );
    }

    pub(crate) fn cancel_portal_service_proposal_preparation(
        &mut self,
        preparation: super::UiPortalProposalPreparation,
        motion_state: &mut crate::runtime::motion::UiMotionRuntimeState,
    ) {
        self.cancel_portal_staging(
            preparation.staging,
            &preparation.portal,
            &preparation.focus,
            preparation.scroll.as_ref(),
            preparation.selection.as_ref(),
            motion_state,
            preparation.motion,
        );
    }
}
