use super::ForgeQueryRuntimeStateKind;
use crate::identity::hash_parts;

#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub enum ForgeQueryRuntimeRemaskDispositionKind {
    Remasked,
    Denied,
}

impl ForgeQueryRuntimeRemaskDispositionKind {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Remasked => "remasked",
            Self::Denied => "denied",
        }
    }

    pub(crate) fn state_kind(self) -> ForgeQueryRuntimeStateKind {
        match self {
            Self::Remasked => ForgeQueryRuntimeStateKind::Remasked,
            Self::Denied => ForgeQueryRuntimeStateKind::Denied,
        }
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub enum ForgeQueryRuntimeRemaskReasonKind {
    PolicyDrift,
    TenantDrift,
    RelationshipProofDrift,
    SchemaContextDrift,
}

impl ForgeQueryRuntimeRemaskReasonKind {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::PolicyDrift => "policy_drift",
            Self::TenantDrift => "tenant_drift",
            Self::RelationshipProofDrift => "relationship_proof_drift",
            Self::SchemaContextDrift => "schema_context_drift",
        }
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ForgeQueryRuntimeRemaskProjection {
    disposition_kind: ForgeQueryRuntimeRemaskDispositionKind,
    reason_kind: ForgeQueryRuntimeRemaskReasonKind,
    policy_digest: String,
    tenant_truth_digest: String,
    tenant_schema_digest: String,
    relationship_proof_digest: String,
    schema_context_digest: String,
}

impl ForgeQueryRuntimeRemaskProjection {
    pub fn remasked(
        reason_kind: ForgeQueryRuntimeRemaskReasonKind,
        policy_digest: impl Into<String>,
        tenant_truth_digest: impl Into<String>,
        tenant_schema_digest: impl Into<String>,
        relationship_proof_digest: impl Into<String>,
        schema_context_digest: impl Into<String>,
    ) -> Self {
        Self::new(
            ForgeQueryRuntimeRemaskDispositionKind::Remasked,
            reason_kind,
            policy_digest,
            tenant_truth_digest,
            tenant_schema_digest,
            relationship_proof_digest,
            schema_context_digest,
        )
    }

    pub fn denied(
        reason_kind: ForgeQueryRuntimeRemaskReasonKind,
        policy_digest: impl Into<String>,
        tenant_truth_digest: impl Into<String>,
        tenant_schema_digest: impl Into<String>,
        relationship_proof_digest: impl Into<String>,
        schema_context_digest: impl Into<String>,
    ) -> Self {
        Self::new(
            ForgeQueryRuntimeRemaskDispositionKind::Denied,
            reason_kind,
            policy_digest,
            tenant_truth_digest,
            tenant_schema_digest,
            relationship_proof_digest,
            schema_context_digest,
        )
    }

    fn new(
        disposition_kind: ForgeQueryRuntimeRemaskDispositionKind,
        reason_kind: ForgeQueryRuntimeRemaskReasonKind,
        policy_digest: impl Into<String>,
        tenant_truth_digest: impl Into<String>,
        tenant_schema_digest: impl Into<String>,
        relationship_proof_digest: impl Into<String>,
        schema_context_digest: impl Into<String>,
    ) -> Self {
        Self {
            disposition_kind,
            reason_kind,
            policy_digest: policy_digest.into(),
            tenant_truth_digest: tenant_truth_digest.into(),
            tenant_schema_digest: tenant_schema_digest.into(),
            relationship_proof_digest: relationship_proof_digest.into(),
            schema_context_digest: schema_context_digest.into(),
        }
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ForgeQueryRuntimeRemaskPosture {
    disposition_kind: ForgeQueryRuntimeRemaskDispositionKind,
    reason_kind: ForgeQueryRuntimeRemaskReasonKind,
    support_evidence_digest: String,
    basis_digest: String,
    policy_digest: String,
    tenant_truth_digest: String,
    tenant_schema_digest: String,
    relationship_proof_digest: String,
    schema_context_digest: String,
    remask_digest: String,
}

impl ForgeQueryRuntimeRemaskPosture {
    fn new(
        disposition_kind: ForgeQueryRuntimeRemaskDispositionKind,
        reason_kind: ForgeQueryRuntimeRemaskReasonKind,
        support_evidence_digest: impl Into<String>,
        basis_digest: impl Into<String>,
        policy_digest: impl Into<String>,
        tenant_truth_digest: impl Into<String>,
        tenant_schema_digest: impl Into<String>,
        relationship_proof_digest: impl Into<String>,
        schema_context_digest: impl Into<String>,
    ) -> Self {
        let support_evidence_digest = support_evidence_digest.into();
        let basis_digest = basis_digest.into();
        let policy_digest = policy_digest.into();
        let tenant_truth_digest = tenant_truth_digest.into();
        let tenant_schema_digest = tenant_schema_digest.into();
        let relationship_proof_digest = relationship_proof_digest.into();
        let schema_context_digest = schema_context_digest.into();
        let remask_digest = hash_parts(&[
            "forge_query_runtime_remask_posture_v1".to_string(),
            format!("disposition:{}", disposition_kind.as_str()),
            format!("reason:{}", reason_kind.as_str()),
            format!("support:{support_evidence_digest}"),
            format!("basis:{basis_digest}"),
            format!("policy:{policy_digest}"),
            format!("tenant-truth:{tenant_truth_digest}"),
            format!("tenant-schema:{tenant_schema_digest}"),
            format!("relationship-proof:{relationship_proof_digest}"),
            format!("schema-context:{schema_context_digest}"),
        ]);
        Self {
            disposition_kind,
            reason_kind,
            support_evidence_digest,
            basis_digest,
            policy_digest,
            tenant_truth_digest,
            tenant_schema_digest,
            relationship_proof_digest,
            schema_context_digest,
            remask_digest,
        }
    }

    pub(crate) fn from_activation_projection(
        projection: &ForgeQueryRuntimeRemaskProjection,
        support_evidence_digest: &str,
        basis_digest: &str,
    ) -> Self {
        Self::new(
            projection.disposition_kind,
            projection.reason_kind,
            support_evidence_digest,
            basis_digest,
            projection.policy_digest.clone(),
            projection.tenant_truth_digest.clone(),
            projection.tenant_schema_digest.clone(),
            projection.relationship_proof_digest.clone(),
            projection.schema_context_digest.clone(),
        )
    }

    pub fn disposition_kind(&self) -> ForgeQueryRuntimeRemaskDispositionKind {
        self.disposition_kind
    }

    pub fn reason_kind(&self) -> ForgeQueryRuntimeRemaskReasonKind {
        self.reason_kind
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
