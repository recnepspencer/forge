#[must_use = "the Scroll owner must acknowledge or discard its staged reveal proposal"]
pub(in crate::runtime) struct UiStagedScrollServiceProposal {
    proposal: crate::runtime::session::service_proposal::UiServiceProposalIdentity,
    scope: crate::runtime::session::service_proposal::UiServiceProposalOccupancyScopeIdentity,
    fact: crate::runtime::session::service_proposal::UiServiceProducedFactReference,
    requirement: crate::runtime::session::service_proposal::UiFocusRevealRequirement,
}

impl UiStagedScrollServiceProposal {
    pub(in crate::runtime) fn family_proposal(
        scope: crate::runtime::session::service_proposal::UiServiceProposalOccupancyScopeIdentity,
    ) -> crate::runtime::session::service_proposal::UiServiceFamilyProposal {
        crate::runtime::session::service_proposal::UiServiceFamilyProposal::scroll(scope)
    }

    pub(in crate::runtime) fn prepare(
        proposal: crate::runtime::session::service_proposal::UiServiceProposalIdentity,
        scope: crate::runtime::session::service_proposal::UiServiceProposalOccupancyScopeIdentity,
        requirement: crate::runtime::session::service_proposal::UiFocusRevealRequirement,
    ) -> Self {
        Self {
            proposal,
            scope,
            fact: crate::runtime::session::service_proposal::UiServiceProducedFactReference::for_scroll_proposal(
                proposal,
                scope,
            ),
            requirement,
        }
    }

    pub(in crate::runtime) const fn with_requirement(
        mut self,
        requirement: crate::runtime::session::service_proposal::UiFocusRevealRequirement,
    ) -> Self {
        self.requirement = requirement;
        self
    }

    pub(in crate::runtime) const fn scope(
        &self,
    ) -> crate::runtime::session::service_proposal::UiServiceProposalOccupancyScopeIdentity {
        self.scope
    }

    pub(in crate::runtime) const fn requirement(
        &self,
    ) -> crate::runtime::session::service_proposal::UiFocusRevealRequirement {
        self.requirement
    }

    pub(in crate::runtime) fn family_stage_receipt(
        &self,
    ) -> crate::runtime::session::service_proposal::UiServiceProposalStageReceipt {
        crate::runtime::session::service_proposal::UiServiceProposalStageReceipt::from_family_owner(
            self.proposal,
            crate::capability::UiRuntimeServiceFamily::Scroll,
            self.scope,
            vec![self.fact],
            Vec::new(),
        )
    }
}
