#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(crate) struct UiSemanticKeyboardFocus {
    participant: super::UiFocusParticipant,
}

impl UiSemanticKeyboardFocus {
    pub(super) const fn new(participant: super::UiFocusParticipant) -> Self {
        Self { participant }
    }

    pub(in crate::runtime) const fn participant(self) -> super::UiFocusParticipantIdentity {
        self.participant.identity()
    }

    pub(crate) const fn mounted_instance(
        self,
    ) -> worth_ui_host_contract::UiMountedInstanceIdentity {
        self.participant.identity().mounted_instance()
    }

    pub(crate) const fn scope(self) -> super::UiFocusScopeIdentity {
        self.participant.scope()
    }

    pub(crate) const fn incarnation(self) -> worth_ui_host_contract::UiMountIncarnation {
        self.participant.incarnation()
    }

    pub(crate) const fn graph_node(self) -> crate::graph::UiGraphNodeIdentity {
        self.participant.graph_node()
    }

    pub(crate) const fn mounted_target(self) -> worth_ui_host_contract::UiHostFocusPlacementTarget {
        worth_ui_host_contract::UiHostFocusPlacementTarget::new(
            self.participant.identity().mounted_instance(),
            self.participant.node_receipt(),
        )
    }

    pub(in crate::runtime) const fn reveal_requirement(
        self,
    ) -> crate::runtime::session::service_proposal::UiFocusRevealRequirement {
        crate::runtime::session::service_proposal::UiFocusRevealRequirement::new(
            self.participant().mounted_instance(),
        )
    }

    pub(super) const fn exact_participant(self) -> super::UiFocusParticipant {
        self.participant
    }
}
