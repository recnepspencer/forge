#[derive(Clone, Debug, Eq, PartialEq)]
pub enum WorthQueryGraphReadBasisPosture {
    RuntimeCurrent,
    PreviewAdmitted,
    BranchAdmitted,
}

impl WorthQueryGraphReadBasisPosture {
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::RuntimeCurrent => "runtime_current",
            Self::PreviewAdmitted => "preview_admitted",
            Self::BranchAdmitted => "branch_admitted",
        }
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct WorthQueryGraphReadBasisBinding {
    read_graph_digest: String,
    schema_basis_digest: String,
    posture: WorthQueryGraphReadBasisPosture,
}

impl WorthQueryGraphReadBasisBinding {
    pub fn read_graph_digest(&self) -> &str {
        &self.read_graph_digest
    }

    pub fn schema_basis_digest(&self) -> &str {
        &self.schema_basis_digest
    }

    pub fn posture(&self) -> &WorthQueryGraphReadBasisPosture {
        &self.posture
    }

    pub(crate) fn digest_part(&self) -> String {
        format!(
            "basis:{}:{}:{}",
            self.read_graph_digest,
            self.schema_basis_digest,
            self.posture.as_str()
        )
    }

    pub(crate) fn new(
        read_graph_digest: impl Into<String>,
        schema_basis_digest: impl Into<String>,
        posture: WorthQueryGraphReadBasisPosture,
    ) -> Self {
        Self {
            read_graph_digest: read_graph_digest.into(),
            schema_basis_digest: schema_basis_digest.into(),
            posture,
        }
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum WorthQueryGraphReadRelationshipProofBindingPosture {
    NotRequired,
    DescriptorAdmittedSyntheticRuntime,
    DescriptorAdmittedRuntimeCurrent,
    DescriptorAdmittedPreview,
    DescriptorAdmittedBranch,
}

impl WorthQueryGraphReadRelationshipProofBindingPosture {
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::NotRequired => "not_required",
            Self::DescriptorAdmittedSyntheticRuntime => "descriptor_admitted_synthetic_runtime",
            Self::DescriptorAdmittedRuntimeCurrent => "descriptor_admitted_runtime_current",
            Self::DescriptorAdmittedPreview => "descriptor_admitted_preview",
            Self::DescriptorAdmittedBranch => "descriptor_admitted_branch",
        }
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct WorthQueryGraphReadPolicyTenantProofBinding {
    read_graph_digest: String,
    policy_tenant_posture: WorthQueryGraphReadPolicyTenantPosture,
    relationship_proof_posture: WorthQueryGraphReadRelationshipProofBindingPosture,
    relationship_proof_admission_digest: Option<String>,
    policy_tenant_admission_digest: Option<String>,
    authority_receipt_digest: String,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum WorthQueryGraphReadPolicyTenantPosture {
    SyntheticRuntimeCurrentRead,
    AdmittedCurrentRead,
    AdmittedPreviewRead,
    AdmittedBranchRead,
}

impl WorthQueryGraphReadPolicyTenantPosture {
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::SyntheticRuntimeCurrentRead => "synthetic_runtime_current_read",
            Self::AdmittedCurrentRead => "admitted_current_read",
            Self::AdmittedPreviewRead => "admitted_preview_read",
            Self::AdmittedBranchRead => "admitted_branch_read",
        }
    }
}

impl WorthQueryGraphReadPolicyTenantProofBinding {
    pub fn read_graph_digest(&self) -> &str {
        &self.read_graph_digest
    }

    pub fn policy_tenant_posture(&self) -> &WorthQueryGraphReadPolicyTenantPosture {
        &self.policy_tenant_posture
    }

    pub fn relationship_proof_posture(
        &self,
    ) -> &WorthQueryGraphReadRelationshipProofBindingPosture {
        &self.relationship_proof_posture
    }

    pub fn relationship_proof_admission_digest(&self) -> Option<&str> {
        self.relationship_proof_admission_digest.as_deref()
    }

    pub fn policy_tenant_admission_digest(&self) -> Option<&str> {
        self.policy_tenant_admission_digest.as_deref()
    }

    pub fn authority_receipt_digest(&self) -> &str {
        &self.authority_receipt_digest
    }

    pub(crate) fn digest_part(&self) -> String {
        format!(
            "policy_tenant_proof:{}:{}:{}:{}:{}:{}",
            self.read_graph_digest,
            self.policy_tenant_posture.as_str(),
            self.relationship_proof_posture.as_str(),
            self.relationship_proof_admission_digest
                .as_deref()
                .unwrap_or("none"),
            self.policy_tenant_admission_digest
                .as_deref()
                .unwrap_or("none"),
            self.authority_receipt_digest
        )
    }

    pub(crate) fn new(
        read_graph_digest: impl Into<String>,
        policy_tenant_posture: WorthQueryGraphReadPolicyTenantPosture,
        relationship_proof_posture: WorthQueryGraphReadRelationshipProofBindingPosture,
        relationship_proof_admission_digest: Option<String>,
        policy_tenant_admission_digest: Option<String>,
        authority_receipt_digest: impl Into<String>,
    ) -> Self {
        Self {
            read_graph_digest: read_graph_digest.into(),
            policy_tenant_posture,
            relationship_proof_posture,
            relationship_proof_admission_digest,
            policy_tenant_admission_digest,
            authority_receipt_digest: authority_receipt_digest.into(),
        }
    }
}
