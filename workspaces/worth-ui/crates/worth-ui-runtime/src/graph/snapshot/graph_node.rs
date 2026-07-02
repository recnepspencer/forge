use crate::declaration::{stable_text_digest, UiDeclarationIdentity};
use crate::graph::{
    UiGraphAttachmentPosture, UiGraphNodeIdentity, UiGraphParticipationPosture,
    UiRepeatedInstanceBasis,
};

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct UiGraphNode {
    graph_node_identity: UiGraphNodeIdentity,
    declaration_identity: UiDeclarationIdentity,
    authored_provenance_digest: u64,
    repeated_instance_basis: UiRepeatedInstanceBasis,
    attachment_posture: UiGraphAttachmentPosture,
    participation_posture: UiGraphParticipationPosture,
}

impl UiGraphNode {
    pub(crate) fn new(
        graph_node_identity: UiGraphNodeIdentity,
        declaration_identity: UiDeclarationIdentity,
        authored_provenance_digest: u64,
        repeated_instance_basis: UiRepeatedInstanceBasis,
        attachment_posture: UiGraphAttachmentPosture,
        participation_posture: UiGraphParticipationPosture,
    ) -> Self {
        Self {
            graph_node_identity,
            declaration_identity,
            authored_provenance_digest,
            repeated_instance_basis,
            attachment_posture,
            participation_posture,
        }
    }

    pub fn graph_node_identity(&self) -> UiGraphNodeIdentity {
        self.graph_node_identity
    }

    pub fn declaration_identity(&self) -> &UiDeclarationIdentity {
        &self.declaration_identity
    }

    pub fn repeated_instance_basis(&self) -> &UiRepeatedInstanceBasis {
        &self.repeated_instance_basis
    }

    pub fn authored_provenance_digest(&self) -> u64 {
        self.authored_provenance_digest
    }

    pub fn attachment_posture(&self) -> UiGraphAttachmentPosture {
        self.attachment_posture
    }

    pub fn participation_posture(&self) -> UiGraphParticipationPosture {
        self.participation_posture
    }

    pub(crate) fn authority_digest(&self) -> u64 {
        stable_text_digest("graph-node")
            ^ self.graph_node_identity.digest().rotate_left(7)
            ^ u64::from(self.attachment_posture.query_binding_attached()).rotate_left(13)
            ^ u64::from(self.attachment_posture.service_usage_attached()).rotate_left(17)
            ^ self.participation_posture.identity_digest().rotate_left(23)
    }
}
