use super::ForgeQueryRuntimeStateKind;
use crate::evidence_identity::ForgeQueryEvidenceIdentity;

use super::evidence_identities::runtime_remask_posture_identity;
use crate::evidence_identity::{
    forge_query_evidence_identity, ForgeQueryEvidenceScope, ForgeQueryEvidenceTag,
};

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
    support_identity: ForgeQueryEvidenceIdentity,
    basis_identity: ForgeQueryEvidenceIdentity,
    policy_identity: ForgeQueryEvidenceIdentity,
    tenant_truth_identity: ForgeQueryEvidenceIdentity,
    tenant_schema_identity: ForgeQueryEvidenceIdentity,
    relationship_proof_identity: ForgeQueryEvidenceIdentity,
    schema_context_identity: ForgeQueryEvidenceIdentity,
    remask_identity: ForgeQueryEvidenceIdentity,
}

impl ForgeQueryRuntimeRemaskPosture {
    fn new(
        disposition_kind: ForgeQueryRuntimeRemaskDispositionKind,
        reason_kind: ForgeQueryRuntimeRemaskReasonKind,
        support_identity: ForgeQueryEvidenceIdentity,
        basis_identity: ForgeQueryEvidenceIdentity,
        policy_digest: impl Into<String>,
        tenant_truth_digest: impl Into<String>,
        tenant_schema_digest: impl Into<String>,
        relationship_proof_digest: impl Into<String>,
        schema_context_digest: impl Into<String>,
    ) -> Self {
        let policy_digest = policy_digest.into();
        let tenant_truth_digest = tenant_truth_digest.into();
        let tenant_schema_digest = tenant_schema_digest.into();
        let relationship_proof_digest = relationship_proof_digest.into();
        let schema_context_digest = schema_context_digest.into();
        let policy_identity = remask_drift_label_identity("remask_policy_label_v1", &policy_digest);
        let tenant_truth_identity =
            remask_drift_label_identity("remask_tenant_truth_label_v1", &tenant_truth_digest);
        let tenant_schema_identity =
            remask_drift_label_identity("remask_tenant_schema_label_v1", &tenant_schema_digest);
        let relationship_proof_identity = remask_drift_label_identity(
            "remask_relationship_proof_label_v1",
            &relationship_proof_digest,
        );
        let schema_context_identity =
            remask_drift_label_identity("remask_schema_context_label_v1", &schema_context_digest);
        let remask_identity = runtime_remask_posture_identity(
            disposition_kind,
            reason_kind,
            &support_identity,
            &basis_identity,
            &policy_identity,
            &tenant_truth_identity,
            &tenant_schema_identity,
            &relationship_proof_identity,
            &schema_context_identity,
        );
        Self {
            disposition_kind,
            reason_kind,
            support_identity,
            basis_identity,
            policy_identity,
            tenant_truth_identity,
            tenant_schema_identity,
            relationship_proof_identity,
            schema_context_identity,
            remask_identity,
        }
    }

    pub(crate) fn from_activation_projection(
        projection: &ForgeQueryRuntimeRemaskProjection,
        support_identity: &ForgeQueryEvidenceIdentity,
        basis_identity: &ForgeQueryEvidenceIdentity,
    ) -> Self {
        Self::new(
            projection.disposition_kind,
            projection.reason_kind,
            support_identity.clone(),
            basis_identity.clone(),
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

    pub fn support_identity(&self) -> &ForgeQueryEvidenceIdentity {
        &self.support_identity
    }

    pub fn support_for_reporting(&self) -> &str {
        self.support_identity.as_str()
    }

    pub fn basis_identity(&self) -> &ForgeQueryEvidenceIdentity {
        &self.basis_identity
    }

    pub fn basis_for_reporting(&self) -> &str {
        self.basis_identity.as_str()
    }

    pub fn policy_identity(&self) -> &ForgeQueryEvidenceIdentity {
        &self.policy_identity
    }

    pub fn policy_for_reporting(&self) -> &str {
        self.policy_identity.as_str()
    }

    pub fn tenant_truth_identity(&self) -> &ForgeQueryEvidenceIdentity {
        &self.tenant_truth_identity
    }

    pub fn tenant_truth_for_reporting(&self) -> &str {
        self.tenant_truth_identity.as_str()
    }

    pub fn tenant_schema_identity(&self) -> &ForgeQueryEvidenceIdentity {
        &self.tenant_schema_identity
    }

    pub fn tenant_schema_for_reporting(&self) -> &str {
        self.tenant_schema_identity.as_str()
    }

    pub fn relationship_proof_identity(&self) -> &ForgeQueryEvidenceIdentity {
        &self.relationship_proof_identity
    }

    pub fn relationship_proof_for_reporting(&self) -> &str {
        self.relationship_proof_identity.as_str()
    }

    pub fn schema_context_identity(&self) -> &ForgeQueryEvidenceIdentity {
        &self.schema_context_identity
    }

    pub fn schema_context_for_reporting(&self) -> &str {
        self.schema_context_identity.as_str()
    }

    pub fn remask_identity(&self) -> &ForgeQueryEvidenceIdentity {
        &self.remask_identity
    }

    pub fn remask_for_reporting(&self) -> &str {
        self.remask_identity.as_str()
    }
}

fn remask_drift_label_identity(identity_family: &str, digest: &str) -> ForgeQueryEvidenceIdentity {
    forge_query_evidence_identity(ForgeQueryEvidenceScope::LowerRuntimeBoundaryEvidence)
        .field_shape(
            ForgeQueryEvidenceTag::new("identity_family"),
            identity_family,
        )
        .field_shape(ForgeQueryEvidenceTag::new("digest"), digest)
        .seal()
}
