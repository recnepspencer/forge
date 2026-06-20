use super::{
    ForgeQueryGraphReadAccessAuthorityCounters, ForgeQueryGraphReadAccessAuthorityDenial,
    ForgeQueryGraphReadAccessAuthorityDenialKind, ForgeQueryGraphReadAccessAuthorityReceipt,
    ForgeQueryGraphReadAccessAuthorityRequest, ForgeQueryGraphReadPolicyTenantAuthorityRequest,
};
use crate::policy_basis::{
    admit_policy_tenant_context, AdmittedPolicyTenantContext, PolicyExecutionModeRequest,
};
use crate::relationship_proof::RelationshipProofAdmission;
use crate::runtime::{
    ForgeQueryBranchBasisAdmission, ForgeQueryGraphReadBasisBinding,
    ForgeQueryGraphReadBasisPosture, ForgeQueryGraphReadPolicyTenantPosture,
    ForgeQueryGraphReadPolicyTenantProofBinding,
    ForgeQueryGraphReadRelationshipProofBindingPosture, ForgeQueryPreviewBasisAdmission,
    ForgeQueryReadGraph,
};

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum ForgeQueryGraphReadAccessBasisScopeKind {
    RuntimeCurrent,
    Preview,
    Branch,
}

impl ForgeQueryGraphReadAccessBasisScopeKind {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::RuntimeCurrent => "runtime_current",
            Self::Preview => "preview",
            Self::Branch => "branch",
        }
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ForgeQueryGraphReadAccessBasisScope {
    kind: ForgeQueryGraphReadAccessBasisScopeKind,
    admission_digest: Option<String>,
}

impl ForgeQueryGraphReadAccessBasisScope {
    pub fn kind(&self) -> ForgeQueryGraphReadAccessBasisScopeKind {
        self.kind
    }

    pub fn admission_digest(&self) -> Option<&str> {
        self.admission_digest.as_deref()
    }

    pub(crate) fn runtime_current() -> Self {
        Self {
            kind: ForgeQueryGraphReadAccessBasisScopeKind::RuntimeCurrent,
            admission_digest: None,
        }
    }

    fn preview(basis: &ForgeQueryPreviewBasisAdmission) -> Self {
        Self {
            kind: ForgeQueryGraphReadAccessBasisScopeKind::Preview,
            admission_digest: Some(basis.admission_digest().as_str().to_string()),
        }
    }

    fn branch(basis: &ForgeQueryBranchBasisAdmission) -> Self {
        Self {
            kind: ForgeQueryGraphReadAccessBasisScopeKind::Branch,
            admission_digest: Some(basis.admission_digest().as_str().to_string()),
        }
    }

    pub(crate) fn digest_part(&self) -> String {
        format!(
            "basis_scope:{}:{}",
            self.kind.as_str(),
            self.admission_digest.as_deref().unwrap_or("none")
        )
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ForgeQueryGraphReadAccessAuthorityContext {
    basis_scope: ForgeQueryGraphReadAccessBasisScope,
    policy_tenant: Option<AdmittedPolicyTenantContext>,
    relationship_proof: Option<RelationshipProofAdmission>,
    receipt: ForgeQueryGraphReadAccessAuthorityReceipt,
}

impl ForgeQueryGraphReadAccessAuthorityContext {
    pub fn basis_scope(&self) -> &ForgeQueryGraphReadAccessBasisScope {
        &self.basis_scope
    }

    pub fn policy_tenant(&self) -> Option<&AdmittedPolicyTenantContext> {
        self.policy_tenant.as_ref()
    }

    pub fn relationship_proof(&self) -> Option<&RelationshipProofAdmission> {
        self.relationship_proof.as_ref()
    }

    pub fn receipt(&self) -> &ForgeQueryGraphReadAccessAuthorityReceipt {
        &self.receipt
    }

    pub(crate) fn runtime_current_compatibility() -> Self {
        let basis_scope = ForgeQueryGraphReadAccessBasisScope::runtime_current();
        let receipt = ForgeQueryGraphReadAccessAuthorityReceipt::runtime_current_compatibility();
        Self {
            basis_scope,
            policy_tenant: None,
            relationship_proof: None,
            receipt,
        }
    }

    pub(crate) fn bind_for_read_graph(
        &self,
        read_graph: &ForgeQueryReadGraph,
    ) -> (
        ForgeQueryGraphReadBasisBinding,
        ForgeQueryGraphReadPolicyTenantProofBinding,
    ) {
        let basis_binding = ForgeQueryGraphReadBasisBinding::new(
            read_graph.digest(),
            read_graph.schema_basis().as_str(),
            basis_posture_from_scope(&self.basis_scope),
        );
        let explicit_relationship_proof = self
            .relationship_proof
            .as_ref()
            .map(|admission| admission.identity().as_str().to_string());
        let relationship_proof_admission_digest =
            explicit_relationship_proof.clone().or_else(|| {
                read_graph
                    .relationship_proof_admission()
                    .map(|admission| admission.identity().as_str().to_string())
            });
        let relationship_posture = if explicit_relationship_proof.is_some() {
            relationship_posture_from_scope(&self.basis_scope)
        } else if relationship_proof_admission_digest.is_some() {
            ForgeQueryGraphReadRelationshipProofBindingPosture::DescriptorAdmittedSyntheticRuntime
        } else {
            ForgeQueryGraphReadRelationshipProofBindingPosture::NotRequired
        };
        let policy_posture = match self.basis_scope.kind {
            ForgeQueryGraphReadAccessBasisScopeKind::RuntimeCurrent => {
                if self.policy_tenant.is_some() {
                    ForgeQueryGraphReadPolicyTenantPosture::AdmittedCurrentRead
                } else {
                    ForgeQueryGraphReadPolicyTenantPosture::SyntheticRuntimeCurrentRead
                }
            }
            ForgeQueryGraphReadAccessBasisScopeKind::Preview => {
                ForgeQueryGraphReadPolicyTenantPosture::AdmittedPreviewRead
            }
            ForgeQueryGraphReadAccessBasisScopeKind::Branch => {
                ForgeQueryGraphReadPolicyTenantPosture::AdmittedBranchRead
            }
        };
        let policy_tenant_proof_binding = ForgeQueryGraphReadPolicyTenantProofBinding::new(
            read_graph.digest(),
            policy_posture,
            relationship_posture,
            relationship_proof_admission_digest,
            self.policy_tenant
                .as_ref()
                .map(|context| context.bundle().digest().as_str().to_string()),
            self.receipt.digest().to_string(),
        );
        (basis_binding, policy_tenant_proof_binding)
    }
}

pub fn admit_graph_read_access_authority(
    request: ForgeQueryGraphReadAccessAuthorityRequest,
) -> Result<ForgeQueryGraphReadAccessAuthorityContext, ForgeQueryGraphReadAccessAuthorityDenial> {
    let (basis_scope, policy_tenant, relationship_proof) = match request {
        ForgeQueryGraphReadAccessAuthorityRequest::CurrentHead {
            policy_tenant,
            relationship_proof,
        } => (
            ForgeQueryGraphReadAccessBasisScope::runtime_current(),
            policy_tenant,
            relationship_proof,
        ),
        ForgeQueryGraphReadAccessAuthorityRequest::Preview {
            basis,
            policy_tenant,
            relationship_proof,
        } => (
            ForgeQueryGraphReadAccessBasisScope::preview(&basis),
            policy_tenant,
            relationship_proof,
        ),
        ForgeQueryGraphReadAccessAuthorityRequest::Branch {
            basis,
            policy_tenant,
            relationship_proof,
        } => (
            ForgeQueryGraphReadAccessBasisScope::branch(&basis),
            policy_tenant,
            relationship_proof,
        ),
    };

    if relationship_proof.is_some() && policy_tenant.is_none() {
        return Err(ForgeQueryGraphReadAccessAuthorityDenial::new(
            ForgeQueryGraphReadAccessAuthorityDenialKind::RelationshipProofRequiresPolicyTenantContext,
            "relationship proof admission is only valid with its admitted policy/tenant context",
            ForgeQueryGraphReadAccessAuthorityCounters::denied(false, true),
        ));
    }

    if let Some(policy_tenant) = policy_tenant.as_ref() {
        let expected_mode = policy_execution_mode_for_basis_scope(&basis_scope);
        if policy_tenant.bundle().execution_mode() != expected_mode {
            return Err(ForgeQueryGraphReadAccessAuthorityDenial::new(
                ForgeQueryGraphReadAccessAuthorityDenialKind::PolicyTenantBasisScopeMismatch,
                format!(
                    "policy/tenant admission mode {} cannot authorize {} graph-read basis scope",
                    policy_tenant.bundle().execution_mode().as_str(),
                    basis_scope.kind().as_str()
                ),
                ForgeQueryGraphReadAccessAuthorityCounters::denied(
                    true,
                    relationship_proof.is_some(),
                ),
            ));
        }
    }

    if let (Some(policy_tenant), Some(relationship_proof)) =
        (policy_tenant.as_ref(), relationship_proof.as_ref())
    {
        if relationship_proof.policy_digest() != policy_tenant.bundle().policy_digest()
            || relationship_proof.tenant_schema_basis_digest()
                != policy_tenant.bundle().tenant_schema_basis_digest()
        {
            return Err(ForgeQueryGraphReadAccessAuthorityDenial::new(
                ForgeQueryGraphReadAccessAuthorityDenialKind::RelationshipProofPolicyTenantMismatch,
                "relationship proof admission must belong to the same admitted policy/tenant context",
                ForgeQueryGraphReadAccessAuthorityCounters::denied(true, true),
            ));
        }
    }

    let policy_tenant_digest = policy_tenant
        .as_ref()
        .map(|context| context.bundle().digest().as_str().to_string());
    let relationship_proof_digest = relationship_proof
        .as_ref()
        .map(|admission| admission.identity().as_str().to_string());
    let counters = ForgeQueryGraphReadAccessAuthorityCounters::admitted(
        policy_tenant_digest.is_some(),
        relationship_proof_digest.is_some(),
    );
    let receipt = ForgeQueryGraphReadAccessAuthorityReceipt::new(
        basis_scope.clone(),
        policy_tenant_digest,
        relationship_proof_digest,
        counters,
    );
    Ok(ForgeQueryGraphReadAccessAuthorityContext {
        basis_scope,
        policy_tenant,
        relationship_proof,
        receipt,
    })
}

pub fn admit_graph_read_access_authority_from_policy_tenant_request(
    request: ForgeQueryGraphReadPolicyTenantAuthorityRequest,
) -> Result<ForgeQueryGraphReadAccessAuthorityContext, ForgeQueryGraphReadAccessAuthorityDenial> {
    let policy_tenant = admit_policy_tenant_context(
        request.query(),
        request.policy().clone(),
        request.tenant().clone(),
        request.branch().clone(),
        request.schema().clone(),
        request.execution_mode(),
    )
    .map_err(|error| {
        ForgeQueryGraphReadAccessAuthorityDenial::new(
            ForgeQueryGraphReadAccessAuthorityDenialKind::PolicyTenantDenied,
            error.message(),
            ForgeQueryGraphReadAccessAuthorityCounters::denied(true, false),
        )
    })?;

    admit_graph_read_access_authority(
        ForgeQueryGraphReadAccessAuthorityRequest::current_head().with_policy_tenant(policy_tenant),
    )
}

fn policy_execution_mode_for_basis_scope(
    scope: &ForgeQueryGraphReadAccessBasisScope,
) -> PolicyExecutionModeRequest {
    match scope.kind {
        ForgeQueryGraphReadAccessBasisScopeKind::RuntimeCurrent
        | ForgeQueryGraphReadAccessBasisScopeKind::Preview => {
            PolicyExecutionModeRequest::CurrentRead
        }
        ForgeQueryGraphReadAccessBasisScopeKind::Branch => PolicyExecutionModeRequest::BranchRead,
    }
}

fn basis_posture_from_scope(
    scope: &ForgeQueryGraphReadAccessBasisScope,
) -> ForgeQueryGraphReadBasisPosture {
    match scope.kind {
        ForgeQueryGraphReadAccessBasisScopeKind::RuntimeCurrent => {
            ForgeQueryGraphReadBasisPosture::RuntimeCurrent
        }
        ForgeQueryGraphReadAccessBasisScopeKind::Preview => {
            ForgeQueryGraphReadBasisPosture::PreviewAdmitted
        }
        ForgeQueryGraphReadAccessBasisScopeKind::Branch => {
            ForgeQueryGraphReadBasisPosture::BranchAdmitted
        }
    }
}

fn relationship_posture_from_scope(
    scope: &ForgeQueryGraphReadAccessBasisScope,
) -> ForgeQueryGraphReadRelationshipProofBindingPosture {
    match scope.kind {
        ForgeQueryGraphReadAccessBasisScopeKind::RuntimeCurrent => {
            ForgeQueryGraphReadRelationshipProofBindingPosture::DescriptorAdmittedRuntimeCurrent
        }
        ForgeQueryGraphReadAccessBasisScopeKind::Preview => {
            ForgeQueryGraphReadRelationshipProofBindingPosture::DescriptorAdmittedPreview
        }
        ForgeQueryGraphReadAccessBasisScopeKind::Branch => {
            ForgeQueryGraphReadRelationshipProofBindingPosture::DescriptorAdmittedBranch
        }
    }
}
