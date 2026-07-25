use crate::graph::{
    UiGraphMountEligibilityIdentity, UiGraphMountEligibilitySeed, UiGraphNodeIdentity,
};

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum UiGraphMountEligibilityRelationship {
    ReservedMountEligibility,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct UiGraphMountEligibilitySlot {
    mount_eligibility_identity: UiGraphMountEligibilityIdentity,
    graph_node_identity: UiGraphNodeIdentity,
    eligibility_seed: UiGraphMountEligibilitySeed,
    eligibility_relationship: UiGraphMountEligibilityRelationship,
}

impl UiGraphMountEligibilitySlot {
    pub(in crate::graph::mount_eligibility) const fn new(
        mount_eligibility_identity: UiGraphMountEligibilityIdentity,
        graph_node_identity: UiGraphNodeIdentity,
        eligibility_seed: UiGraphMountEligibilitySeed,
        eligibility_relationship: UiGraphMountEligibilityRelationship,
    ) -> Self {
        Self {
            mount_eligibility_identity,
            graph_node_identity,
            eligibility_seed,
            eligibility_relationship,
        }
    }

    pub fn mount_eligibility_identity(self) -> UiGraphMountEligibilityIdentity {
        self.mount_eligibility_identity
    }

    pub fn graph_node_identity(self) -> UiGraphNodeIdentity {
        self.graph_node_identity
    }

    pub fn eligibility_seed(self) -> UiGraphMountEligibilitySeed {
        self.eligibility_seed
    }

    pub fn eligibility_relationship(self) -> UiGraphMountEligibilityRelationship {
        self.eligibility_relationship
    }
}
