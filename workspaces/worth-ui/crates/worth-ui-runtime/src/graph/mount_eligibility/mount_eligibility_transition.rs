use crate::graph::{
    UiGraphAxisParticipation, UiGraphMountEligibilityMutation, UiGraphMountEligibilityMutationKind,
    UiGraphMountEligibilityRecord, UiGraphMountEligibilitySlot,
};

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct UiGraphMountEligibilityTransition {
    graph_authority_identity: crate::graph::UiGraphAuthorityIdentity,
    slot: UiGraphMountEligibilitySlot,
    prior_eligibility: UiGraphAxisParticipation,
    next_eligibility: UiGraphAxisParticipation,
    mutation_kind: UiGraphMountEligibilityMutationKind,
}

impl UiGraphMountEligibilityTransition {
    pub(crate) fn from_slot_axis_transition(
        graph_authority_identity: crate::graph::UiGraphAuthorityIdentity,
        slot: UiGraphMountEligibilitySlot,
        prior_eligibility: UiGraphAxisParticipation,
        next_eligibility: UiGraphAxisParticipation,
    ) -> Option<Self> {
        let mutation_kind = match (
            prior_eligibility.status().admitted(),
            next_eligibility.status().admitted(),
        ) {
            (false, true) => UiGraphMountEligibilityMutationKind::BecomeEligible,
            (true, false) => UiGraphMountEligibilityMutationKind::BecomeIneligible,
            _ => return None,
        };

        Some(Self {
            graph_authority_identity,
            slot,
            prior_eligibility,
            next_eligibility,
            mutation_kind,
        })
    }

    pub fn eligibility_record(self) -> UiGraphMountEligibilityRecord {
        self.slot.into()
    }

    pub fn kind(self) -> UiGraphMountEligibilityMutationKind {
        self.mutation_kind
    }

    pub fn prior_eligibility(self) -> UiGraphAxisParticipation {
        self.prior_eligibility
    }

    pub fn next_eligibility(self) -> UiGraphAxisParticipation {
        self.next_eligibility
    }

    pub fn mutation(self) -> UiGraphMountEligibilityMutation {
        UiGraphMountEligibilityMutation::from_transition(self)
    }

    pub(crate) fn graph_authority_identity(self) -> crate::graph::UiGraphAuthorityIdentity {
        self.graph_authority_identity
    }
}
