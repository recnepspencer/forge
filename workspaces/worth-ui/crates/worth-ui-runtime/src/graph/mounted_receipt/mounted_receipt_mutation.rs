use crate::graph::{
    UiGraphAxisParticipation, UiGraphMountedPostureRelationship, UiGraphMountedReceiptAuthoritySeed,
    UiGraphMountedReceiptTransition, UiGraphNodeIdentity, UiMountedReceiptIdentity,
};

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum UiGraphMountedReceiptMutationKind {
    CreateSlot,
    RemoveSlot,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct UiGraphMountedReceiptMutation {
    graph_node_identity: UiGraphNodeIdentity,
    mounted_receipt_identity: UiMountedReceiptIdentity,
    kind: UiGraphMountedReceiptMutationKind,
    prior_mounted_axis_participation: UiGraphAxisParticipation,
    next_mounted_axis_participation: UiGraphAxisParticipation,
    authority_seed: UiGraphMountedReceiptAuthoritySeed,
    mounted_posture_relationship: UiGraphMountedPostureRelationship,
}

impl UiGraphMountedReceiptMutation {
    pub fn from_transition(transition: UiGraphMountedReceiptTransition) -> Self {
        let slot = transition.authority_record();
        Self {
            graph_node_identity: slot.graph_node_identity(),
            mounted_receipt_identity: slot.mounted_receipt_identity(),
            kind: transition.kind(),
            prior_mounted_axis_participation: transition.prior_mounted_axis_participation(),
            next_mounted_axis_participation: transition.next_mounted_axis_participation(),
            authority_seed: slot.authority_seed(),
            mounted_posture_relationship: slot.mounted_posture_relationship(),
        }
    }

    pub fn graph_node_identity(self) -> UiGraphNodeIdentity {
        self.graph_node_identity
    }

    pub fn mounted_receipt_identity(self) -> UiMountedReceiptIdentity {
        self.mounted_receipt_identity
    }

    pub fn kind(self) -> UiGraphMountedReceiptMutationKind {
        self.kind
    }

    pub fn prior_mounted_axis_participation(self) -> UiGraphAxisParticipation {
        self.prior_mounted_axis_participation
    }

    pub fn next_mounted_axis_participation(self) -> UiGraphAxisParticipation {
        self.next_mounted_axis_participation
    }

    pub fn authority_seed(self) -> UiGraphMountedReceiptAuthoritySeed {
        self.authority_seed
    }

    pub fn mounted_posture_relationship(self) -> UiGraphMountedPostureRelationship {
        self.mounted_posture_relationship
    }
}
