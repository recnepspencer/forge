use crate::graph::{
    UiGraphMountedReceiptAuthoritySeed, UiGraphNodeIdentity, UiMountedReceiptIdentity,
};

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum UiGraphMountedPostureRelationship {
    ReservedMountedAuthoritySlot,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct UiGraphMountedReceiptSlot {
    mounted_receipt_identity: UiMountedReceiptIdentity,
    graph_node_identity: UiGraphNodeIdentity,
    authority_seed: UiGraphMountedReceiptAuthoritySeed,
    mounted_posture_relationship: UiGraphMountedPostureRelationship,
}

impl UiGraphMountedReceiptSlot {
    pub(in crate::graph::mounted_receipt) const fn new(
        mounted_receipt_identity: UiMountedReceiptIdentity,
        graph_node_identity: UiGraphNodeIdentity,
        authority_seed: UiGraphMountedReceiptAuthoritySeed,
        mounted_posture_relationship: UiGraphMountedPostureRelationship,
    ) -> Self {
        Self {
            mounted_receipt_identity,
            graph_node_identity,
            authority_seed,
            mounted_posture_relationship,
        }
    }

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
