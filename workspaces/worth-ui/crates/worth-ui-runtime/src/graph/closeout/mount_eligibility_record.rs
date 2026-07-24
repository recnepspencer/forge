use crate::graph::{
    UiGraphMountEligibilityIdentity, UiGraphMountEligibilityRelationship,
    UiGraphMountEligibilitySeed, UiGraphMountEligibilitySlot, UiGraphNodeIdentity,
};

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct UiGraphMountEligibilityRecord {
    mount_eligibility_identity: UiGraphMountEligibilityIdentity,
    graph_node_identity: UiGraphNodeIdentity,
    eligibility_seed: UiGraphMountEligibilitySeed,
    eligibility_relationship: UiGraphMountEligibilityRelationship,
}

impl UiGraphMountEligibilityRecord {
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

impl From<UiGraphMountEligibilitySlot> for UiGraphMountEligibilityRecord {
    fn from(slot: UiGraphMountEligibilitySlot) -> Self {
        Self {
            mount_eligibility_identity: slot.mount_eligibility_identity(),
            graph_node_identity: slot.graph_node_identity(),
            eligibility_seed: slot.eligibility_seed(),
            eligibility_relationship: slot.eligibility_relationship(),
        }
    }
}
