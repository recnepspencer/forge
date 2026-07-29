use crate::declaration::{
    stable_text_digest, UiAspectContract, UiDeclarationIdentity, UiDeclarationPlanningOperatorKind,
    UiDeclarationRepetitionPosture, UiDeclarationStructuralDigest, UiDeclarationStructuralRole,
    UiDeclaredMeasurementConstraintModifier,
};
use crate::graph::{
    UiGraphAttachmentPosture, UiGraphNodeIdentity, UiGraphParticipationPosture,
    UiRepeatedInstanceBasis,
};

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct UiGraphNode {
    graph_node_identity: UiGraphNodeIdentity,
    declaration_identity: UiDeclarationIdentity,
    aspect_contract: UiAspectContract,
    structural_digest: UiDeclarationStructuralDigest,
    structural_role: UiDeclarationStructuralRole,
    operator_kind: UiDeclarationPlanningOperatorKind,
    repetition_posture: UiDeclarationRepetitionPosture,
    measurement_constraint_modifier: Option<UiDeclaredMeasurementConstraintModifier>,
    authored_provenance_digest: u64,
    repeated_instance_basis: UiRepeatedInstanceBasis,
    attachment_posture: UiGraphAttachmentPosture,
    participation_posture: UiGraphParticipationPosture,
}

pub(crate) struct UiGraphNodeInput {
    pub(crate) graph_node_identity: UiGraphNodeIdentity,
    pub(crate) declaration_identity: UiDeclarationIdentity,
    pub(crate) aspect_contract: UiAspectContract,
    pub(crate) structural_digest: UiDeclarationStructuralDigest,
    pub(crate) structural_role: UiDeclarationStructuralRole,
    pub(crate) operator_kind: UiDeclarationPlanningOperatorKind,
    pub(crate) repetition_posture: UiDeclarationRepetitionPosture,
    pub(crate) measurement_constraint_modifier: Option<UiDeclaredMeasurementConstraintModifier>,
    pub(crate) authored_provenance_digest: u64,
    pub(crate) repeated_instance_basis: UiRepeatedInstanceBasis,
    pub(crate) attachment_posture: UiGraphAttachmentPosture,
    pub(crate) participation_posture: UiGraphParticipationPosture,
}

impl UiGraphNode {
    pub(crate) fn new(input: UiGraphNodeInput) -> Self {
        let UiGraphNodeInput {
            graph_node_identity,
            declaration_identity,
            aspect_contract,
            structural_digest,
            structural_role,
            operator_kind,
            repetition_posture,
            measurement_constraint_modifier,
            authored_provenance_digest,
            repeated_instance_basis,
            attachment_posture,
            participation_posture,
        } = input;
        Self {
            graph_node_identity,
            declaration_identity,
            aspect_contract,
            structural_digest,
            structural_role,
            operator_kind,
            repetition_posture,
            measurement_constraint_modifier,
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

    pub fn aspect_contract(&self) -> &UiAspectContract {
        &self.aspect_contract
    }

    pub fn repeated_instance_basis(&self) -> &UiRepeatedInstanceBasis {
        &self.repeated_instance_basis
    }

    pub fn structural_role(&self) -> UiDeclarationStructuralRole {
        self.structural_role
    }

    pub fn operator_kind(&self) -> UiDeclarationPlanningOperatorKind {
        self.operator_kind
    }

    pub fn structural_digest(&self) -> UiDeclarationStructuralDigest {
        self.structural_digest
    }

    pub fn repetition_posture(&self) -> UiDeclarationRepetitionPosture {
        self.repetition_posture
    }

    pub fn measurement_constraint_modifier(
        &self,
    ) -> Option<UiDeclaredMeasurementConstraintModifier> {
        self.measurement_constraint_modifier
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
            ^ self.aspect_contract.digest_raw().rotate_left(8)
            ^ self.structural_digest.raw().rotate_left(9)
            ^ (self.structural_role as u64).rotate_left(11)
            ^ (self.operator_kind as u64).rotate_left(12)
            ^ (self.repetition_posture as u64).rotate_left(13)
            ^ measurement_constraint_modifier_digest(self.measurement_constraint_modifier)
                .rotate_left(15)
            ^ u64::from(self.attachment_posture.query_binding_attached()).rotate_left(17)
            ^ u64::from(self.attachment_posture.service_usage_attached()).rotate_left(19)
            ^ self.participation_posture.identity_digest().rotate_left(23)
    }
}

fn measurement_constraint_modifier_digest(
    modifier: Option<UiDeclaredMeasurementConstraintModifier>,
) -> u64 {
    match modifier {
        Some(UiDeclaredMeasurementConstraintModifier::Bounded) => {
            stable_text_digest("graph-node.constraint.bounded")
        }
        None => stable_text_digest("graph-node.constraint.none"),
    }
}
