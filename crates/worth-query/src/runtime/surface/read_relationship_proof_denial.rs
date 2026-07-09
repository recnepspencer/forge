use crate::policy_basis::PolicyTenantAdmissionFailureClass;
use crate::relationship_proof::RelationshipProofFailureClass;

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum WorthQueryReadRelationshipProofDenialStage {
    SyntheticRuntimeContext,
    DescriptorAdmission,
}

impl WorthQueryReadRelationshipProofDenialStage {
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::SyntheticRuntimeContext => "synthetic_runtime_context",
            Self::DescriptorAdmission => "descriptor_admission",
        }
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct WorthQueryReadRelationshipProofDenial {
    stage: WorthQueryReadRelationshipProofDenialStage,
    policy_failure_class: Option<PolicyTenantAdmissionFailureClass>,
    relationship_proof_failure_class: Option<RelationshipProofFailureClass>,
}

impl WorthQueryReadRelationshipProofDenial {
    pub fn stage(&self) -> &WorthQueryReadRelationshipProofDenialStage {
        &self.stage
    }

    pub fn policy_failure_class(&self) -> Option<PolicyTenantAdmissionFailureClass> {
        self.policy_failure_class
    }

    pub fn relationship_proof_failure_class(&self) -> Option<RelationshipProofFailureClass> {
        self.relationship_proof_failure_class
    }

    pub(in crate::runtime) fn for_policy_failure(
        failure_class: PolicyTenantAdmissionFailureClass,
    ) -> Self {
        Self {
            stage: WorthQueryReadRelationshipProofDenialStage::SyntheticRuntimeContext,
            policy_failure_class: Some(failure_class),
            relationship_proof_failure_class: None,
        }
    }

    pub(in crate::runtime) fn for_relationship_proof_failure(
        failure_class: RelationshipProofFailureClass,
    ) -> Self {
        Self {
            stage: WorthQueryReadRelationshipProofDenialStage::DescriptorAdmission,
            policy_failure_class: None,
            relationship_proof_failure_class: Some(failure_class),
        }
    }
}
