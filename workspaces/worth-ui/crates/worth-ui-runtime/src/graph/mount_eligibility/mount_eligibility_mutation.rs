use crate::graph::{
    UiGraphAxisParticipation, UiGraphMountEligibilityIdentity, UiGraphMountEligibilityRelationship,
    UiGraphMountEligibilitySeed, UiGraphMountEligibilityTransition, UiGraphNodeIdentity,
};

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum UiGraphMountEligibilityMutationKind {
    BecomeEligible,
    BecomeIneligible,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct UiGraphMountEligibilityMutation {
    graph_node_identity: UiGraphNodeIdentity,
    mount_eligibility_identity: UiGraphMountEligibilityIdentity,
    kind: UiGraphMountEligibilityMutationKind,
    prior_eligibility: UiGraphAxisParticipation,
    next_eligibility: UiGraphAxisParticipation,
    eligibility_seed: UiGraphMountEligibilitySeed,
    eligibility_relationship: UiGraphMountEligibilityRelationship,
}

impl UiGraphMountEligibilityMutation {
    pub(crate) fn from_transition(transition: UiGraphMountEligibilityTransition) -> Self {
        let slot = transition.eligibility_record();
        Self {
            graph_node_identity: slot.graph_node_identity(),
            mount_eligibility_identity: slot.mount_eligibility_identity(),
            kind: transition.kind(),
            prior_eligibility: transition.prior_eligibility(),
            next_eligibility: transition.next_eligibility(),
            eligibility_seed: slot.eligibility_seed(),
            eligibility_relationship: slot.eligibility_relationship(),
        }
    }

    pub fn graph_node_identity(self) -> UiGraphNodeIdentity {
        self.graph_node_identity
    }

    pub fn mount_eligibility_identity(self) -> UiGraphMountEligibilityIdentity {
        self.mount_eligibility_identity
    }

    pub fn kind(self) -> UiGraphMountEligibilityMutationKind {
        self.kind
    }

    pub fn prior_eligibility(self) -> UiGraphAxisParticipation {
        self.prior_eligibility
    }

    pub fn next_eligibility(self) -> UiGraphAxisParticipation {
        self.next_eligibility
    }

    pub fn eligibility_seed(self) -> UiGraphMountEligibilitySeed {
        self.eligibility_seed
    }

    pub fn eligibility_relationship(self) -> UiGraphMountEligibilityRelationship {
        self.eligibility_relationship
    }
}
