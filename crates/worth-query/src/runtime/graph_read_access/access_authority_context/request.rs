use crate::canonicalization::CanonicalQueryArtifact;
use crate::policy_basis::{
    AdmittedPolicyTenantContext, BranchAccessGrant, PolicyExecutionModeRequest, PolicyRuleSnapshot,
};
use crate::relationship_proof::RelationshipProofAdmission;
use crate::runtime::{WorthQueryBranchBasisAdmission, WorthQueryPreviewBasisAdmission};
use crate::tenant_basis::{SchemaVariantSnapshot, TenantBindingSnapshot};

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum WorthQueryGraphReadAccessAuthorityRequest {
    CurrentHead {
        policy_tenant: Option<AdmittedPolicyTenantContext>,
        relationship_proof: Option<RelationshipProofAdmission>,
    },
    Preview {
        basis: WorthQueryPreviewBasisAdmission,
        policy_tenant: Option<AdmittedPolicyTenantContext>,
        relationship_proof: Option<RelationshipProofAdmission>,
    },
    Branch {
        basis: WorthQueryBranchBasisAdmission,
        policy_tenant: Option<AdmittedPolicyTenantContext>,
        relationship_proof: Option<RelationshipProofAdmission>,
    },
}

#[derive(Clone, Debug, PartialEq)]
pub struct WorthQueryGraphReadPolicyTenantAuthorityRequest {
    query: CanonicalQueryArtifact,
    policy: PolicyRuleSnapshot,
    tenant: TenantBindingSnapshot,
    branch: BranchAccessGrant,
    schema: SchemaVariantSnapshot,
    execution_mode: PolicyExecutionModeRequest,
}

impl WorthQueryGraphReadPolicyTenantAuthorityRequest {
    pub fn current_read(
        query: &CanonicalQueryArtifact,
        policy: PolicyRuleSnapshot,
        tenant: TenantBindingSnapshot,
        branch: BranchAccessGrant,
        schema: SchemaVariantSnapshot,
    ) -> Self {
        Self {
            query: query.clone(),
            policy,
            tenant,
            branch,
            schema,
            execution_mode: PolicyExecutionModeRequest::CurrentRead,
        }
    }

    pub(crate) fn query(&self) -> &CanonicalQueryArtifact {
        &self.query
    }

    pub(crate) fn policy(&self) -> &PolicyRuleSnapshot {
        &self.policy
    }

    pub(crate) fn tenant(&self) -> &TenantBindingSnapshot {
        &self.tenant
    }

    pub(crate) fn branch(&self) -> &BranchAccessGrant {
        &self.branch
    }

    pub(crate) fn schema(&self) -> &SchemaVariantSnapshot {
        &self.schema
    }

    pub(crate) fn execution_mode(&self) -> PolicyExecutionModeRequest {
        self.execution_mode
    }
}

impl WorthQueryGraphReadAccessAuthorityRequest {
    pub fn current_head() -> Self {
        Self::CurrentHead {
            policy_tenant: None,
            relationship_proof: None,
        }
    }

    pub fn preview(basis: &WorthQueryPreviewBasisAdmission) -> Self {
        Self::Preview {
            basis: basis.clone(),
            policy_tenant: None,
            relationship_proof: None,
        }
    }

    pub fn branch(basis: &WorthQueryBranchBasisAdmission) -> Self {
        Self::Branch {
            basis: basis.clone(),
            policy_tenant: None,
            relationship_proof: None,
        }
    }

    pub fn with_policy_tenant(self, policy_tenant: AdmittedPolicyTenantContext) -> Self {
        match self {
            Self::CurrentHead {
                relationship_proof, ..
            } => Self::CurrentHead {
                policy_tenant: Some(policy_tenant),
                relationship_proof,
            },
            Self::Preview {
                basis,
                relationship_proof,
                ..
            } => Self::Preview {
                basis,
                policy_tenant: Some(policy_tenant),
                relationship_proof,
            },
            Self::Branch {
                basis,
                relationship_proof,
                ..
            } => Self::Branch {
                basis,
                policy_tenant: Some(policy_tenant),
                relationship_proof,
            },
        }
    }

    pub fn with_relationship_proofs(self, relationship_proof: RelationshipProofAdmission) -> Self {
        match self {
            Self::CurrentHead { policy_tenant, .. } => Self::CurrentHead {
                policy_tenant,
                relationship_proof: Some(relationship_proof),
            },
            Self::Preview {
                basis,
                policy_tenant,
                ..
            } => Self::Preview {
                basis,
                policy_tenant,
                relationship_proof: Some(relationship_proof),
            },
            Self::Branch {
                basis,
                policy_tenant,
                ..
            } => Self::Branch {
                basis,
                policy_tenant,
                relationship_proof: Some(relationship_proof),
            },
        }
    }
}
