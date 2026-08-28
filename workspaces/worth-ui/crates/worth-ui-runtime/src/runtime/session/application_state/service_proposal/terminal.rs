impl super::WorthUiApplicationSessionState {
    pub(super) fn cancel_portal_staging(
        &mut self,
        staging: crate::runtime::session::service_proposal::UiServiceProposalStaging,
        portal: &crate::runtime::portal::UiStagedPortalServiceProposal,
        focus: &crate::runtime::focus::UiStagedFocusServiceProposal,
        scroll: Option<&crate::runtime::scroll::UiStagedScrollServiceProposal>,
        selection: Option<&crate::runtime::selection::UiStagedDeclaredSelectionTransition>,
        motion_state: &mut crate::runtime::motion::UiMotionRuntimeState,
        motion: Option<crate::runtime::motion::UiStagedMotionServiceProposal>,
    ) {
        let motion_scope = motion.as_ref().map(|motion| motion.scope());
        if let Some(motion) = motion {
            motion_state.discard_staged(motion);
        }
        let teardown = self.runtime.service_proposals.cancel_staging(staging);
        self.finish_portal_teardown(
            teardown,
            portal,
            focus,
            scroll,
            selection,
            motion_scope,
            crate::runtime::session::service_proposal::UiServiceProposalTerminalReason::CancelledBeforePublication,
        );
    }

    pub(super) fn finish_portal_teardown(
        &mut self,
        mut teardown: crate::runtime::session::service_proposal::UiServiceProposalTeardown,
        portal: &crate::runtime::portal::UiStagedPortalServiceProposal,
        focus: &crate::runtime::focus::UiStagedFocusServiceProposal,
        scroll: Option<&crate::runtime::scroll::UiStagedScrollServiceProposal>,
        selection: Option<&crate::runtime::selection::UiStagedDeclaredSelectionTransition>,
        motion_scope: Option<
            crate::runtime::session::service_proposal::UiServiceProposalOccupancyScopeIdentity,
        >,
        reason: crate::runtime::session::service_proposal::UiServiceProposalTerminalReason,
    ) {
        let outcome = crate::runtime::session::service_proposal::UiServiceProposalTerminalOwnerOutcome::from_family_owner(
            teardown.proposal(),
            crate::capability::UiRuntimeServiceFamily::Portal,
            portal.scope(),
            reason,
        );
        self.runtime
            .service_proposals
            .acknowledge_terminal_owner(&mut teardown, outcome)
            .expect("exact portal staging teardown matches its owner");
        let focus_outcome = crate::runtime::session::service_proposal::UiServiceProposalTerminalOwnerOutcome::from_family_owner(
            teardown.proposal(),
            crate::capability::UiRuntimeServiceFamily::Focus,
            focus.scope(),
            reason,
        );
        self.runtime
            .service_proposals
            .acknowledge_terminal_owner(&mut teardown, focus_outcome)
            .expect("exact Focus staging teardown matches its owner");
        if let Some(scroll) = scroll {
            let scroll_outcome = crate::runtime::session::service_proposal::UiServiceProposalTerminalOwnerOutcome::from_family_owner(
                teardown.proposal(),
                crate::capability::UiRuntimeServiceFamily::Scroll,
                scroll.scope(),
                reason,
            );
            self.runtime
                .service_proposals
                .acknowledge_terminal_owner(&mut teardown, scroll_outcome)
                .expect("exact Scroll staging teardown matches its owner");
        }
        if let Some(selection) = selection {
            let selection_outcome = selection.terminal_outcome(reason);
            self.runtime
                .service_proposals
                .acknowledge_terminal_owner(&mut teardown, selection_outcome)
                .expect("exact Selection staging teardown matches its owner");
        }
        if let Some(motion_scope) = motion_scope {
            let motion_outcome = crate::runtime::session::service_proposal::UiServiceProposalTerminalOwnerOutcome::from_family_owner(
                teardown.proposal(),
                crate::capability::UiRuntimeServiceFamily::Motion,
                motion_scope,
                reason,
            );
            self.runtime
                .service_proposals
                .acknowledge_terminal_owner(&mut teardown, motion_outcome)
                .expect("exact Motion staging teardown matches its owner");
        }
        self.runtime
            .service_proposals
            .finish_teardown(teardown)
            .expect("exact portal staging teardown releases every compiler resource");
    }
}

impl super::UiStagedPortalProposalTransaction {
    pub(crate) fn accepted_publication(
        &self,
        mounted: &crate::mounting::UiMountedFramePublicationReceipt,
    ) -> Result<
        crate::runtime::session::service_proposal::UiServiceProposalPublicationReceipt,
        super::UiPortalProposalPreparationDenial,
    > {
        validate_prepared_frame(self.prepared_frame, mounted.frame())?;
        Ok(self.terminal_publication(
            crate::runtime::session::service_proposal::UiServiceProposalPublicationDisposition::Accepted,
        ))
    }

    pub(crate) const fn terminal_publication(
        &self,
        disposition: crate::runtime::session::service_proposal::UiServiceProposalPublicationDisposition,
    ) -> crate::runtime::session::service_proposal::UiServiceProposalPublicationReceipt {
        crate::runtime::session::service_proposal::UiServiceProposalPublicationReceipt::from_staged_batch(
            &self.batch,
            disposition,
        )
    }

    pub(crate) const fn rejected_publication(
        &self,
    ) -> crate::runtime::session::service_proposal::UiServiceProposalPublicationReceipt {
        self.terminal_publication(
            crate::runtime::session::service_proposal::UiServiceProposalPublicationDisposition::Rejected,
        )
    }
}

fn validate_prepared_frame(
    prepared: worth_ui_host_contract::UiMountedFrameIdentity,
    published: worth_ui_host_contract::UiMountedFrameIdentity,
) -> Result<(), super::UiPortalProposalPreparationDenial> {
    if prepared == published {
        Ok(())
    } else {
        Err(super::UiPortalProposalPreparationDenial::MountedFrameMismatch)
    }
}

impl super::UiPortalProposalSettlement {
    pub(super) fn into_parts(
        self,
    ) -> (
        crate::runtime::session::service_proposal::UiServiceProposalSettlement,
        crate::runtime::portal::UiPreparedPortalServiceTransition,
        crate::runtime::focus::UiStagedFocusServiceProposal,
        Option<crate::runtime::scroll::UiStagedScrollServiceProposal>,
        Option<super::UiStagedFocusReveal>,
        Option<crate::runtime::selection::UiStagedDeclaredSelectionTransition>,
        Option<crate::runtime::motion::UiDerivedMotionServiceProposal>,
        worth_ui_host_contract::UiMountedFrameIdentity,
        crate::runtime::session::service_proposal::UiServiceProposalPublicationReceipt,
        crate::runtime::session::service_proposal::UiServiceProposalOccupancyScopeIdentity,
    ) {
        (
            self.settlement,
            self.transition,
            self.focus,
            self.scroll,
            self.staged_reveal,
            self.selection,
            self.motion,
            self.prepared_frame,
            self.publication,
            self.scope,
        )
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn publication_must_name_the_exact_session_retained_prepared_frame() {
        let prepared = worth_ui_host_contract::UiMountedFrameIdentity::mint_unbound().unwrap();
        let foreign = worth_ui_host_contract::UiMountedFrameIdentity::mint_unbound().unwrap();

        assert!(super::validate_prepared_frame(prepared, prepared).is_ok());
        assert!(matches!(
            super::validate_prepared_frame(prepared, foreign),
            Err(super::super::UiPortalProposalPreparationDenial::MountedFrameMismatch)
        ));
    }
}
