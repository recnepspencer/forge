use crate::runtime::ForgeQueryReadGraph;

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum ForgeQueryGraphReadBasisPosture {
    RuntimeCurrent,
}

impl ForgeQueryGraphReadBasisPosture {
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::RuntimeCurrent => "runtime_current",
        }
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ForgeQueryGraphReadBasisBinding {
    read_graph_digest: String,
    schema_basis_digest: String,
    posture: ForgeQueryGraphReadBasisPosture,
}

impl ForgeQueryGraphReadBasisBinding {
    pub fn read_graph_digest(&self) -> &str {
        &self.read_graph_digest
    }

    pub fn schema_basis_digest(&self) -> &str {
        &self.schema_basis_digest
    }

    pub fn posture(&self) -> &ForgeQueryGraphReadBasisPosture {
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
        posture: ForgeQueryGraphReadBasisPosture,
    ) -> Self {
        Self {
            read_graph_digest: read_graph_digest.into(),
            schema_basis_digest: schema_basis_digest.into(),
            posture,
        }
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum ForgeQueryGraphReadRelationshipProofBindingPosture {
    NotRequired,
    DescriptorAdmittedSyntheticRuntime,
}

impl ForgeQueryGraphReadRelationshipProofBindingPosture {
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::NotRequired => "not_required",
            Self::DescriptorAdmittedSyntheticRuntime => "descriptor_admitted_synthetic_runtime",
        }
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ForgeQueryGraphReadPolicyTenantProofBinding {
    read_graph_digest: String,
    policy_tenant_posture: ForgeQueryGraphReadPolicyTenantPosture,
    relationship_proof_posture: ForgeQueryGraphReadRelationshipProofBindingPosture,
    relationship_proof_admission_digest: Option<String>,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum ForgeQueryGraphReadPolicyTenantPosture {
    SyntheticRuntimeCurrentRead,
}

impl ForgeQueryGraphReadPolicyTenantPosture {
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::SyntheticRuntimeCurrentRead => "synthetic_runtime_current_read",
        }
    }
}

impl ForgeQueryGraphReadPolicyTenantProofBinding {
    pub fn read_graph_digest(&self) -> &str {
        &self.read_graph_digest
    }

    pub fn policy_tenant_posture(&self) -> &ForgeQueryGraphReadPolicyTenantPosture {
        &self.policy_tenant_posture
    }

    pub fn relationship_proof_posture(
        &self,
    ) -> &ForgeQueryGraphReadRelationshipProofBindingPosture {
        &self.relationship_proof_posture
    }

    pub fn relationship_proof_admission_digest(&self) -> Option<&str> {
        self.relationship_proof_admission_digest.as_deref()
    }

    pub(crate) fn digest_part(&self) -> String {
        format!(
            "policy_tenant_proof:{}:{}:{}:{}",
            self.read_graph_digest,
            self.policy_tenant_posture.as_str(),
            self.relationship_proof_posture.as_str(),
            self.relationship_proof_admission_digest
                .as_deref()
                .unwrap_or("none")
        )
    }

    pub(crate) fn new(
        read_graph_digest: impl Into<String>,
        policy_tenant_posture: ForgeQueryGraphReadPolicyTenantPosture,
        relationship_proof_posture: ForgeQueryGraphReadRelationshipProofBindingPosture,
        relationship_proof_admission_digest: Option<String>,
    ) -> Self {
        Self {
            read_graph_digest: read_graph_digest.into(),
            policy_tenant_posture,
            relationship_proof_posture,
            relationship_proof_admission_digest,
        }
    }
}

pub(crate) fn bind_graph_read_basis_for_read_graph(
    read_graph: &ForgeQueryReadGraph,
) -> (
    ForgeQueryGraphReadBasisBinding,
    ForgeQueryGraphReadPolicyTenantProofBinding,
) {
    let basis = ForgeQueryGraphReadBasisBinding::new(
        read_graph.digest(),
        read_graph.schema_basis().as_str(),
        ForgeQueryGraphReadBasisPosture::RuntimeCurrent,
    );
    let relationship_proof_admission_digest = read_graph
        .relationship_proof_admission()
        .map(|admission| admission.identity().as_str().to_string());
    let posture = if relationship_proof_admission_digest.is_some() {
        ForgeQueryGraphReadRelationshipProofBindingPosture::DescriptorAdmittedSyntheticRuntime
    } else {
        ForgeQueryGraphReadRelationshipProofBindingPosture::NotRequired
    };
    let policy_tenant_proof = ForgeQueryGraphReadPolicyTenantProofBinding::new(
        read_graph.digest(),
        ForgeQueryGraphReadPolicyTenantPosture::SyntheticRuntimeCurrentRead,
        posture,
        relationship_proof_admission_digest,
    );
    (basis, policy_tenant_proof)
}
