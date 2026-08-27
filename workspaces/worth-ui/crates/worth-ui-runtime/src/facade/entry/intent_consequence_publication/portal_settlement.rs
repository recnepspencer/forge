pub(super) fn settle_published_portal_proposal(
    admitted: &mut super::UiIntentConsequenceAdmitted<'_>,
    mounted: &crate::mounting::UiMountedFramePublicationReceipt,
) -> Option<crate::facade::entry::focus_placement::UiSemanticFocusPublicationReceipt> {
    let Some(transaction) = admitted.transfer.portal_proposal.take() else {
        return None;
    };
    let (focus, motion, exit_retention) = admitted
        .session
        .application
        .settle_published_portal_service_proposal(
            transaction,
            mounted,
            &mut admitted.session.portal,
            &mut admitted.session.focus,
            &mut admitted.session.motion,
        )
        .expect("exact staged portal proposal accepts its publication receipt");
    admitted
        .session
        .rebind_portal_after_current_published_frame();
    let focus = admitted
        .session
        .place_committed_semantic_focus(focus, mounted)
        .expect("published Focus successor retains exact mounted presentation basis");
    admitted
        .session
        .install_portal_exit_retention(exit_retention);
    admitted.session.install_committed_motion(motion);
    Some(focus)
}

pub(super) fn settle_indeterminate_portal_proposal(
    admitted: &mut super::UiIntentConsequenceAdmitted<'_>,
) -> Option<crate::runtime::session::UiIndeterminatePortalProposalTransaction> {
    let Some(transaction) = admitted.transfer.portal_proposal.take() else {
        return None;
    };
    Some(
        admitted
            .session
            .application
            .settle_indeterminate_portal_service_proposal(
                transaction,
                &mut admitted.session.portal,
                &mut admitted.session.focus,
                &mut admitted.session.motion,
            )
            .expect("indeterminate physical settlement retains exact semantic proposal"),
    )
}

pub(super) fn settle_rejected_portal_proposal(
    admitted: &mut super::UiIntentConsequenceAdmitted<'_>,
) {
    let Some(transaction) = admitted.transfer.portal_proposal.take() else {
        return;
    };
    admitted
        .session
        .application
        .settle_rejected_portal_service_proposal(
            transaction,
            &mut admitted.session.focus,
            &mut admitted.session.motion,
        )
        .expect("before-effect rejection retains exact semantic proposal");
}
