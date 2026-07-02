use crate::declaration::UiDeclarationIdentity;
use crate::graph::{
    UiGraphAttachmentPosture, UiGraphNode, UiGraphNodeIdentity, UiGraphParticipationPosture,
    UiRepeatedInstanceBasis,
};

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct UiGraphNodeRecord {
    graph_node_identity: UiGraphNodeIdentity,
    declaration_identity: UiDeclarationIdentity,
    repeated_instance_basis: UiRepeatedInstanceBasis,
    attachment_posture: UiGraphAttachmentPosture,
    participation_posture: UiGraphParticipationPosture,
}

impl UiGraphNodeRecord {
    pub fn graph_node_identity(&self) -> UiGraphNodeIdentity {
        self.graph_node_identity
    }

    pub fn declaration_identity(&self) -> &UiDeclarationIdentity {
        &self.declaration_identity
    }

    pub fn repeated_instance_basis(&self) -> &UiRepeatedInstanceBasis {
        &self.repeated_instance_basis
    }

    pub fn attachment_posture(&self) -> UiGraphAttachmentPosture {
        self.attachment_posture
    }

    pub fn participation_posture(&self) -> UiGraphParticipationPosture {
        self.participation_posture
    }
}

impl From<&UiGraphNode> for UiGraphNodeRecord {
    fn from(node: &UiGraphNode) -> Self {
        Self {
            graph_node_identity: node.graph_node_identity(),
            declaration_identity: node.declaration_identity().clone(),
            repeated_instance_basis: node.repeated_instance_basis().clone(),
            attachment_posture: node.attachment_posture(),
            participation_posture: node.participation_posture(),
        }
    }
}
