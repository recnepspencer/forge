#[must_use = "the Focus owner must acknowledge or discard its staged portal proposal"]
pub(in crate::runtime) struct UiStagedFocusServiceProposal {
    proposal: crate::runtime::session::service_proposal::UiServiceProposalIdentity,
    scope: crate::runtime::session::service_proposal::UiServiceProposalOccupancyScopeIdentity,
    fact: crate::runtime::session::service_proposal::UiServiceProducedFactReference,
    requirement: UiPortalFocusRequirement,
}

#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub(in crate::runtime) struct UiPortalFocusBoundaryIdentity(
    crate::runtime::session::service_proposal::UiServiceProposalOccupancyScopeIdentity,
);

pub(in crate::runtime) struct UiPortalFocusRequirement {
    boundary: UiPortalFocusBoundaryIdentity,
    owner: worth_ui_host_contract::UiMountedInstanceIdentity,
    opening: bool,
    closed_descendants: Box<[UiPortalFocusBoundaryIdentity]>,
}

impl UiPortalFocusRequirement {
    pub(in crate::runtime) fn new(
        scope: crate::runtime::session::service_proposal::UiServiceProposalOccupancyScopeIdentity,
        owner: worth_ui_host_contract::UiMountedInstanceIdentity,
        opening: bool,
        closed_descendants: Vec<UiPortalFocusBoundaryIdentity>,
    ) -> Self {
        Self {
            boundary: UiPortalFocusBoundaryIdentity(scope),
            owner,
            opening,
            closed_descendants: closed_descendants.into_boxed_slice(),
        }
    }

    pub(in crate::runtime) const fn boundary(&self) -> UiPortalFocusBoundaryIdentity {
        self.boundary
    }

    pub(in crate::runtime) const fn owner(
        &self,
    ) -> worth_ui_host_contract::UiMountedInstanceIdentity {
        self.owner
    }

    pub(in crate::runtime) const fn opening(&self) -> bool {
        self.opening
    }

    pub(in crate::runtime) fn closed_descendants(&self) -> &[UiPortalFocusBoundaryIdentity] {
        &self.closed_descendants
    }
}

impl UiPortalFocusBoundaryIdentity {
    pub(in crate::runtime) const fn from_scope(
        scope: crate::runtime::session::service_proposal::UiServiceProposalOccupancyScopeIdentity,
    ) -> Self {
        Self(scope)
    }
}

impl UiStagedFocusServiceProposal {
    pub(in crate::runtime) fn family_proposal(
        requirement: &UiPortalFocusRequirement,
    ) -> crate::runtime::session::service_proposal::UiServiceFamilyProposal {
        crate::runtime::session::service_proposal::UiServiceFamilyProposal::focus(
            requirement.boundary.0,
        )
    }

    pub(in crate::runtime) fn prepare(
        proposal: crate::runtime::session::service_proposal::UiServiceProposalIdentity,
        requirement: UiPortalFocusRequirement,
    ) -> Self {
        let scope = requirement.boundary.0;
        Self {
            proposal,
            scope,
            fact: crate::runtime::session::service_proposal::UiServiceProducedFactReference::for_focus_proposal(
                proposal,
                scope,
            ),
            requirement,
        }
    }

    pub(in crate::runtime) const fn requirement(&self) -> &UiPortalFocusRequirement {
        &self.requirement
    }

    pub(in crate::runtime) const fn proposal(
        &self,
    ) -> crate::runtime::session::service_proposal::UiServiceProposalIdentity {
        self.proposal
    }

    pub(in crate::runtime) const fn scope(
        &self,
    ) -> crate::runtime::session::service_proposal::UiServiceProposalOccupancyScopeIdentity {
        self.scope
    }

    pub(in crate::runtime) fn family_stage_receipt(
        &self,
    ) -> crate::runtime::session::service_proposal::UiServiceProposalStageReceipt {
        crate::runtime::session::service_proposal::UiServiceProposalStageReceipt::from_family_owner(
            self.proposal,
            crate::capability::UiRuntimeServiceFamily::Focus,
            self.scope,
            vec![self.fact],
            Vec::new(),
        )
    }

    pub(in crate::runtime) fn resolution_receipt(
        &self,
    ) -> crate::runtime::session::service_proposal::UiServiceProposalStageReceipt {
        crate::runtime::session::service_proposal::UiServiceProposalStageReceipt::focus_resolution(
            self.proposal,
            false,
        )
    }
}
