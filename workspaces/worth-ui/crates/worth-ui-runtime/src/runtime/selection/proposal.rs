#[must_use = "the Selection owner must acknowledge or discard its staged declared action"]
pub(in crate::runtime) struct UiStagedSelectionServiceProposal {
    proposal: crate::runtime::session::service_proposal::UiServiceProposalIdentity,
    scope: crate::runtime::session::service_proposal::UiServiceProposalOccupancyScopeIdentity,
    fact: crate::runtime::session::service_proposal::UiServiceProducedFactReference,
    action: crate::runtime::session::service_proposal::UiDeclaredFocusSelectionAction,
}

impl UiStagedSelectionServiceProposal {
    pub(in crate::runtime) fn family_proposal(
        action: crate::runtime::session::service_proposal::UiDeclaredFocusSelectionAction,
    ) -> crate::runtime::session::service_proposal::UiServiceFamilyProposal {
        crate::runtime::session::service_proposal::UiServiceFamilyProposal::selection(
            crate::runtime::session::service_proposal::UiServiceProposalOccupancyScopeIdentity::for_mounted_owner(
                action.mounted_target(),
            ),
        )
    }

    pub(in crate::runtime) fn prepare(
        proposal: crate::runtime::session::service_proposal::UiServiceProposalIdentity,
        action: crate::runtime::session::service_proposal::UiDeclaredFocusSelectionAction,
    ) -> Self {
        let scope = Self::family_proposal(action).scope();
        Self {
            proposal,
            scope,
            fact: crate::runtime::session::service_proposal::UiServiceProducedFactReference::for_selection_proposal(
                proposal,
                scope,
            ),
            action,
        }
    }

    pub(in crate::runtime) const fn action(
        &self,
    ) -> crate::runtime::session::service_proposal::UiDeclaredFocusSelectionAction {
        self.action
    }

    pub(in crate::runtime) const fn scope(
        &self,
    ) -> crate::runtime::session::service_proposal::UiServiceProposalOccupancyScopeIdentity {
        self.scope
    }

    pub(in crate::runtime) const fn proposal(
        &self,
    ) -> crate::runtime::session::service_proposal::UiServiceProposalIdentity {
        self.proposal
    }

    pub(in crate::runtime) fn family_stage_receipt(
        &self,
    ) -> crate::runtime::session::service_proposal::UiServiceProposalStageReceipt {
        crate::runtime::session::service_proposal::UiServiceProposalStageReceipt::from_family_owner(
            self.proposal,
            crate::capability::UiRuntimeServiceFamily::Selection,
            self.scope,
            vec![self.fact],
            Vec::new(),
        )
    }
}
