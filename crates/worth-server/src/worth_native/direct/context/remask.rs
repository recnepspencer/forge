#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum WorthServerDirectRemaskDisposition {
    Visible,
    Remasked,
    Denied,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum WorthServerDirectRemaskPosture {
    Visible,
    Remasked(WorthServerDirectRemaskArtifact),
    Denied(WorthServerDirectRemaskArtifact),
    ProjectionRemasked(WorthServerDirectMaterializedRemaskArtifact),
}

impl WorthServerDirectRemaskPosture {
    pub(crate) fn visible() -> Self {
        Self::Visible
    }

    pub(crate) fn from_materialized_fact_posture(
        posture: Option<&worth_query::facade::runtime::ProjectionMaterializedFactPosture>,
    ) -> Self {
        match posture {
            Some(posture) if posture.kind().as_str() == "remasked" => {
                Self::ProjectionRemasked(WorthServerDirectMaterializedRemaskArtifact::new(
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
        snapshot: &worth_query::facade::runtime::WorthQueryRuntimeStateSnapshot,
    ) -> Self {
        match snapshot.remask_posture() {
            None => Self::Visible,
            Some(remask) if remask.disposition_kind().as_str() == "remasked" => Self::Remasked(
                WorthServerDirectRemaskArtifact::new(DirectRemaskArtifactParts {
                    reason_kind: remask.reason_kind().as_str(),
                    support_evidence_digest: remask.support_for_reporting(),
                    basis_digest: remask.basis_for_reporting(),
                    policy_digest: remask.policy_for_reporting(),
                    tenant_truth_digest: remask.tenant_truth_for_reporting(),
                    tenant_schema_digest: remask.tenant_schema_for_reporting(),
                    relationship_proof_digest: remask.relationship_proof_for_reporting(),
                    schema_context_digest: remask.schema_context_for_reporting(),
                    remask_digest: remask.remask_for_reporting(),
                }),
            ),
            Some(remask) => Self::Denied(WorthServerDirectRemaskArtifact::new(
                DirectRemaskArtifactParts {
                    reason_kind: remask.reason_kind().as_str(),
                    support_evidence_digest: remask.support_for_reporting(),
                    basis_digest: remask.basis_for_reporting(),
                    policy_digest: remask.policy_for_reporting(),
                    tenant_truth_digest: remask.tenant_truth_for_reporting(),
                    tenant_schema_digest: remask.tenant_schema_for_reporting(),
                    relationship_proof_digest: remask.relationship_proof_for_reporting(),
                    schema_context_digest: remask.schema_context_for_reporting(),
                    remask_digest: remask.remask_for_reporting(),
                },
            )),
        }
    }

    pub(crate) fn from_live_inspection(
        live: &worth_query::facade::runtime::WorthQueryLiveViewInspection,
    ) -> Self {
        match live.remask_posture() {
            None => Self::Visible,
            Some(remask) if remask.disposition_kind().as_str() == "remasked" => Self::Remasked(
                WorthServerDirectRemaskArtifact::new(DirectRemaskArtifactParts {
                    reason_kind: remask.reason_kind().as_str(),
                    support_evidence_digest: remask.support_for_reporting(),
                    basis_digest: remask.basis_for_reporting(),
                    policy_digest: remask.policy_for_reporting(),
                    tenant_truth_digest: remask.tenant_truth_for_reporting(),
                    tenant_schema_digest: remask.tenant_schema_for_reporting(),
                    relationship_proof_digest: remask.relationship_proof_for_reporting(),
                    schema_context_digest: remask.schema_context_for_reporting(),
                    remask_digest: remask.remask_for_reporting(),
                }),
            ),
            Some(remask) => Self::Denied(WorthServerDirectRemaskArtifact::new(
                DirectRemaskArtifactParts {
                    reason_kind: remask.reason_kind().as_str(),
                    support_evidence_digest: remask.support_for_reporting(),
                    basis_digest: remask.basis_for_reporting(),
                    policy_digest: remask.policy_for_reporting(),
                    tenant_truth_digest: remask.tenant_truth_for_reporting(),
                    tenant_schema_digest: remask.tenant_schema_for_reporting(),
                    relationship_proof_digest: remask.relationship_proof_for_reporting(),
                    schema_context_digest: remask.schema_context_for_reporting(),
                    remask_digest: remask.remask_for_reporting(),
                },
            )),
        }
    }

    pub fn disposition(&self) -> WorthServerDirectRemaskDisposition {
        match self {
            Self::Visible => WorthServerDirectRemaskDisposition::Visible,
            Self::Remasked(_) | Self::ProjectionRemasked(_) => {
                WorthServerDirectRemaskDisposition::Remasked
            }
            Self::Denied(_) => WorthServerDirectRemaskDisposition::Denied,
        }
    }

    pub fn artifact(&self) -> Option<&WorthServerDirectRemaskArtifact> {
        match self {
            Self::Visible => None,
            Self::Remasked(artifact) | Self::Denied(artifact) => Some(artifact),
            Self::ProjectionRemasked(_) => None,
        }
    }

    pub fn materialized_artifact(&self) -> Option<&WorthServerDirectMaterializedRemaskArtifact> {
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
pub struct WorthServerDirectRemaskArtifact {
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
pub struct WorthServerDirectMaterializedRemaskArtifact {
    basis_digest: String,
    support_evidence_digest: String,
    runtime_origin_digest: Option<String>,
    remask_digest: String,
}

impl WorthServerDirectMaterializedRemaskArtifact {
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

impl WorthServerDirectRemaskArtifact {
    fn new(parts: DirectRemaskArtifactParts<'_>) -> Self {
        let DirectRemaskArtifactParts {
            reason_kind,
            support_evidence_digest,
            basis_digest,
            policy_digest,
            tenant_truth_digest,
            tenant_schema_digest,
            relationship_proof_digest,
            schema_context_digest,
            remask_digest,
        } = parts;
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

struct DirectRemaskArtifactParts<'a> {
    reason_kind: &'a str,
    support_evidence_digest: &'a str,
    basis_digest: &'a str,
    policy_digest: &'a str,
    tenant_truth_digest: &'a str,
    tenant_schema_digest: &'a str,
    relationship_proof_digest: &'a str,
    schema_context_digest: &'a str,
    remask_digest: &'a str,
}
