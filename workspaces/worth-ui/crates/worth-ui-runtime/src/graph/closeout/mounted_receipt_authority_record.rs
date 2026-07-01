use crate::graph::{
    UiGraphMountedPostureRelationship, UiGraphMountedReceiptAuthoritySeed,
    UiGraphMountedReceiptSlot, UiGraphNodeIdentity, UiMountedReceiptIdentity,
};

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct UiGraphMountedReceiptAuthorityRecord {
    mounted_receipt_identity: UiMountedReceiptIdentity,
    graph_node_identity: UiGraphNodeIdentity,
    authority_seed: UiGraphMountedReceiptAuthoritySeed,
    mounted_posture_relationship: UiGraphMountedPostureRelationship,
}

impl UiGraphMountedReceiptAuthorityRecord {
    pub fn mounted_receipt_identity(self) -> UiMountedReceiptIdentity {
        self.mounted_receipt_identity
    }

    pub fn graph_node_identity(self) -> UiGraphNodeIdentity {
        self.graph_node_identity
    }

    pub fn authority_seed(self) -> UiGraphMountedReceiptAuthoritySeed {
        self.authority_seed
    }

    pub fn mounted_posture_relationship(self) -> UiGraphMountedPostureRelationship {
        self.mounted_posture_relationship
    }
}

impl From<UiGraphMountedReceiptSlot> for UiGraphMountedReceiptAuthorityRecord {
    fn from(slot: UiGraphMountedReceiptSlot) -> Self {
        Self {
            mounted_receipt_identity: slot.mounted_receipt_identity(),
            graph_node_identity: slot.graph_node_identity(),
            authority_seed: slot.authority_seed(),
            mounted_posture_relationship: slot.mounted_posture_relationship(),
        }
    }
}
