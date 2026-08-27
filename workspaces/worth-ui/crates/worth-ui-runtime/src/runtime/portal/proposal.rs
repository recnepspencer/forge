#[must_use = "a staged portal proposal must settle with existing publication"]
pub(crate) struct UiStagedPortalServiceProposal {
    transition: super::UiPreparedPortalServiceTransition,
    proposal: crate::runtime::session::service_proposal::UiServiceProposalIdentity,
    scope: crate::runtime::session::service_proposal::UiServiceProposalOccupancyScopeIdentity,
    mounted_work: crate::runtime::session::service_proposal::UiServiceMountedWorkReference,
}

impl UiStagedPortalServiceProposal {
    pub(crate) fn prepare(
        transition: super::UiPreparedPortalServiceTransition,
        proposal: crate::runtime::session::service_proposal::UiServiceProposalIdentity,
        scope: crate::runtime::session::service_proposal::UiServiceProposalOccupancyScopeIdentity,
    ) -> Self {
        Self {
            transition,
            proposal,
            scope,
            mounted_work: crate::runtime::session::service_proposal::UiServiceMountedWorkReference::for_portal_proposal(
                proposal,
                scope,
            ),
        }
    }

    pub(crate) fn family_proposal(
        transition: &super::UiPreparedPortalServiceTransition,
    ) -> crate::runtime::session::service_proposal::UiServiceFamilyProposal {
        let scope = crate::runtime::session::service_proposal::UiServiceProposalOccupancyScopeIdentity::for_mounted_owner(
            transition.request().portal().owner().mounted_instance_identity(),
        );
        crate::runtime::session::service_proposal::UiServiceFamilyProposal::portal(scope)
    }

    pub(crate) const fn scope(
        &self,
    ) -> crate::runtime::session::service_proposal::UiServiceProposalOccupancyScopeIdentity {
        self.scope
    }

    pub(crate) const fn mounted_work_reference(
        &self,
    ) -> crate::runtime::session::service_proposal::UiServiceMountedWorkReference {
        self.mounted_work
    }

    pub(crate) const fn transition(&self) -> &super::UiPreparedPortalServiceTransition {
        &self.transition
    }

    pub(crate) fn stage_receipt(
        &self,
    ) -> crate::runtime::session::service_proposal::UiServiceProposalStageReceipt {
        crate::runtime::session::service_proposal::UiServiceProposalStageReceipt::from_family_owner(
            self.proposal,
            crate::capability::UiRuntimeServiceFamily::Portal,
            self.scope,
            Vec::new(),
            vec![self.mounted_work],
        )
    }

    pub(crate) fn acknowledge(
        &self,
        publication: crate::runtime::session::service_proposal::UiServiceProposalPublicationReceipt,
    ) -> crate::runtime::session::service_proposal::UiServiceProposalOwnerAcknowledgement {
        crate::runtime::session::service_proposal::UiServiceProposalOwnerAcknowledgement::from_family_owner(
            publication,
            crate::capability::UiRuntimeServiceFamily::Portal,
            self.scope,
        )
    }

    pub(crate) fn into_transition(self) -> super::UiPreparedPortalServiceTransition {
        self.transition
    }
}
