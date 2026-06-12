#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum ForgeServerDirectRemaskDisposition {
    Visible,
    Remasked,
    Denied,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum ForgeServerDirectRemaskPosture {
    Visible,
    Remasked(ForgeServerDirectRemaskArtifact),
    Denied(ForgeServerDirectRemaskArtifact),
    ProjectionRemasked(ForgeServerDirectMaterializedRemaskArtifact),
}

impl ForgeServerDirectRemaskPosture {
    pub(crate) fn visible() -> Self {
        Self::Visible
    }

    pub(crate) fn from_materialized_fact_posture(
        posture: Option<&forge_query::facade::ProjectionMaterializedFactPosture>,
    ) -> Self {
        match posture {
            Some(posture) if posture.kind().as_str() == "remasked" => {
                Self::ProjectionRemasked(ForgeServerDirectMaterializedRemaskArtifact::new(
                    posture.basis_digest(),
                    posture.support_evidence_digest(),
                    posture.runtime_origin_digest(),
                    posture.posture_digest(),
                ))
            }
            _ => Self::Visible,
        }
    }

    pub(crate) fn from_state_snapshot(
        snapshot: &forge_query::facade::ForgeQueryRuntimeStateSnapshot,
    ) -> Self {
        match snapshot.remask_posture() {
            None => Self::Visible,
            Some(remask) if remask.disposition_kind().as_str() == "remasked" => {
                Self::Remasked(ForgeServerDirectRemaskArtifact::new(
                    remask.reason_kind().as_str(),
                    remask.support_evidence_digest(),
                    remask.basis_digest(),
                    remask.policy_digest(),
                    remask.tenant_truth_digest(),
                    remask.tenant_schema_digest(),
                    remask.relationship_proof_digest(),
                    remask.schema_context_digest(),
                    remask.remask_digest(),
                ))
            }
            Some(remask) => Self::Denied(ForgeServerDirectRemaskArtifact::new(
                remask.reason_kind().as_str(),
                remask.support_evidence_digest(),
                remask.basis_digest(),
                remask.policy_digest(),
                remask.tenant_truth_digest(),
                remask.tenant_schema_digest(),
                remask.relationship_proof_digest(),
                remask.schema_context_digest(),
                remask.remask_digest(),
            )),
        }
    }

    pub(crate) fn from_live_inspection(
        live: &forge_query::facade::ForgeQueryLiveViewInspection,
    ) -> Self {
        match live.remask_posture() {
            None => Self::Visible,
            Some(remask) if remask.disposition_kind().as_str() == "remasked" => {
                Self::Remasked(ForgeServerDirectRemaskArtifact::new(
                    remask.reason_kind().as_str(),
                    remask.support_evidence_digest(),
                    remask.basis_digest(),
                    remask.policy_digest(),
                    remask.tenant_truth_digest(),
                    remask.tenant_schema_digest(),
                    remask.relationship_proof_digest(),
                    remask.schema_context_digest(),
                    remask.remask_digest(),
                ))
            }
            Some(remask) => Self::Denied(ForgeServerDirectRemaskArtifact::new(
                remask.reason_kind().as_str(),
                remask.support_evidence_digest(),
                remask.basis_digest(),
                remask.policy_digest(),
                remask.tenant_truth_digest(),
                remask.tenant_schema_digest(),
                remask.relationship_proof_digest(),
                remask.schema_context_digest(),
                remask.remask_digest(),
            )),
        }
    }

    pub fn disposition(&self) -> ForgeServerDirectRemaskDisposition {
        match self {
            Self::Visible => ForgeServerDirectRemaskDisposition::Visible,
            Self::Remasked(_) | Self::ProjectionRemasked(_) => {
                ForgeServerDirectRemaskDisposition::Remasked
            }
            Self::Denied(_) => ForgeServerDirectRemaskDisposition::Denied,
        }
    }

    pub fn artifact(&self) -> Option<&ForgeServerDirectRemaskArtifact> {
        match self {
            Self::Visible => None,
            Self::Remasked(artifact) | Self::Denied(artifact) => Some(artifact),
            Self::ProjectionRemasked(_) => None,
        }
    }

    pub fn materialized_artifact(&self) -> Option<&ForgeServerDirectMaterializedRemaskArtifact> {
        match self {
            Self::ProjectionRemasked(artifact) => Some(artifact),
            Self::Visible | Self::Remasked(_) | Self::Denied(_) => None,
        }
    }

    pub fn remask_digest(&self) -> Option<&str> {
        match self {
            Self::Visible => None,
            Self::Remasked(artifact) | Self::Denied(artifact) => Some(artifact.remask_digest()),
            Self::ProjectionRemasked(artifact) => Some(artifact.remask_digest()),
        }
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ForgeServerDirectRemaskArtifact {
    reason_kind: String,
    support_evidence_digest: String,
    basis_digest: String,
    policy_digest: String,
    tenant_truth_digest: String,
    tenant_schema_digest: String,
    relationship_proof_digest: String,
    schema_context_digest: String,
    remask_digest: String,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ForgeServerDirectMaterializedRemaskArtifact {
    basis_digest: String,
    support_evidence_digest: String,
    runtime_origin_digest: Option<String>,
    remask_digest: String,
}

impl ForgeServerDirectMaterializedRemaskArtifact {
    fn new(
        basis_digest: &str,
        support_evidence_digest: &str,
        runtime_origin_digest: Option<&str>,
        remask_digest: &str,
    ) -> Self {
        Self {
            basis_digest: basis_digest.to_string(),
            support_evidence_digest: support_evidence_digest.to_string(),
            runtime_origin_digest: runtime_origin_digest.map(str::to_string),
            remask_digest: remask_digest.to_string(),
        }
    }

    pub fn basis_digest(&self) -> &str {
        &self.basis_digest
    }

    pub fn support_evidence_digest(&self) -> &str {
        &self.support_evidence_digest
    }

    pub fn runtime_origin_digest(&self) -> Option<&str> {
        self.runtime_origin_digest.as_deref()
    }

    pub fn remask_digest(&self) -> &str {
        &self.remask_digest
    }
}

impl ForgeServerDirectRemaskArtifact {
    fn new(
        reason_kind: &str,
        support_evidence_digest: &str,
        basis_digest: &str,
        policy_digest: &str,
        tenant_truth_digest: &str,
        tenant_schema_digest: &str,
        relationship_proof_digest: &str,
        schema_context_digest: &str,
        remask_digest: &str,
    ) -> Self {
        Self {
            reason_kind: reason_kind.to_string(),
            support_evidence_digest: support_evidence_digest.to_string(),
            basis_digest: basis_digest.to_string(),
            policy_digest: policy_digest.to_string(),
            tenant_truth_digest: tenant_truth_digest.to_string(),
            tenant_schema_digest: tenant_schema_digest.to_string(),
            relationship_proof_digest: relationship_proof_digest.to_string(),
            schema_context_digest: schema_context_digest.to_string(),
            remask_digest: remask_digest.to_string(),
        }
    }

    pub fn reason_kind(&self) -> &str {
        &self.reason_kind
    }

    pub fn support_evidence_digest(&self) -> &str {
        &self.support_evidence_digest
    }

    pub fn basis_digest(&self) -> &str {
        &self.basis_digest
    }

    pub fn policy_digest(&self) -> &str {
        &self.policy_digest
    }

    pub fn tenant_truth_digest(&self) -> &str {
        &self.tenant_truth_digest
    }

    pub fn tenant_schema_digest(&self) -> &str {
        &self.tenant_schema_digest
    }

    pub fn relationship_proof_digest(&self) -> &str {
        &self.relationship_proof_digest
    }

    pub fn schema_context_digest(&self) -> &str {
        &self.schema_context_digest
    }

    pub fn remask_digest(&self) -> &str {
        &self.remask_digest
    }
}
