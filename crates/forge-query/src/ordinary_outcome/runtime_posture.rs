use crate::evidence_identity::{
    forge_query_evidence_identity, ForgeQueryEvidenceIdentity, ForgeQueryEvidenceScope,
    ForgeQueryEvidenceTag,
};

#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub enum ForgeQueryOrdinaryRuntimePostureKind {
    Current,
    Remasked,
    Pending,
    Failed,
    Stale,
    Cancelled,
    Retried,
    Revalidating,
    Superseded,
    Denied,
    Unsupported,
}

impl ForgeQueryOrdinaryRuntimePostureKind {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Current => "current",
            Self::Remasked => "remasked",
            Self::Pending => "pending",
            Self::Failed => "failed",
            Self::Stale => "stale",
            Self::Cancelled => "cancelled",
            Self::Retried => "retried",
            Self::Revalidating => "revalidating",
            Self::Superseded => "superseded",
            Self::Denied => "denied",
            Self::Unsupported => "unsupported",
        }
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub enum ForgeQueryOrdinaryRuntimeCausePostureKind {
    Ordinary,
    TimeOnly,
    MixedCause,
}

impl ForgeQueryOrdinaryRuntimeCausePostureKind {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Ordinary => "ordinary",
            Self::TimeOnly => "time_only",
            Self::MixedCause => "mixed_cause",
        }
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub enum ForgeQueryOrdinaryRuntimeAsyncPostureKind {
    Pending,
    Current,
    Failed,
    Stale,
    Cancelled,
    Retried,
    Revalidating,
    Superseded,
    Denied,
}

impl ForgeQueryOrdinaryRuntimeAsyncPostureKind {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Pending => "pending",
            Self::Current => "current",
            Self::Failed => "failed",
            Self::Stale => "stale",
            Self::Cancelled => "cancelled",
            Self::Retried => "retried",
            Self::Revalidating => "revalidating",
            Self::Superseded => "superseded",
            Self::Denied => "denied",
        }
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub enum ForgeQueryOrdinaryRuntimeBasisPostureKind {
    Stable,
    BasisDrift,
    GenerationDrift,
}

impl ForgeQueryOrdinaryRuntimeBasisPostureKind {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Stable => "stable",
            Self::BasisDrift => "basis_drift",
            Self::GenerationDrift => "generation_drift",
        }
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub enum ForgeQueryOrdinaryRuntimeRemaskPostureKind {
    PolicyDrift,
    TenantDrift,
    RelationshipProofDrift,
    SchemaContextDrift,
}

impl ForgeQueryOrdinaryRuntimeRemaskPostureKind {
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
pub struct ForgeQueryOrdinaryRuntimePosture {
    kind: ForgeQueryOrdinaryRuntimePostureKind,
    cause_posture: ForgeQueryOrdinaryRuntimeCausePostureKind,
    async_posture: Option<ForgeQueryOrdinaryRuntimeAsyncPostureKind>,
    basis_posture: ForgeQueryOrdinaryRuntimeBasisPostureKind,
    remask_posture: Option<ForgeQueryOrdinaryRuntimeRemaskPostureKind>,
    support_evidence_identity: ForgeQueryEvidenceIdentity,
    posture_identity: ForgeQueryEvidenceIdentity,
}

impl ForgeQueryOrdinaryRuntimePosture {
    pub fn new(
        kind: ForgeQueryOrdinaryRuntimePostureKind,
        cause_posture: ForgeQueryOrdinaryRuntimeCausePostureKind,
        async_posture: Option<ForgeQueryOrdinaryRuntimeAsyncPostureKind>,
        basis_posture: ForgeQueryOrdinaryRuntimeBasisPostureKind,
        remask_posture: Option<ForgeQueryOrdinaryRuntimeRemaskPostureKind>,
        support_evidence_digest: impl Into<String>,
    ) -> Self {
        let support_evidence_digest = support_evidence_digest.into();
        let support_evidence_identity =
            forge_query_evidence_identity(ForgeQueryEvidenceScope::LowerRuntimeBoundaryEvidence)
                .field_shape(
                    ForgeQueryEvidenceTag::new("identity_family"),
                    "forge_query_ordinary_runtime_posture_support_digest_v1",
                )
                .field_shape(
                    ForgeQueryEvidenceTag::new("support_digest"),
                    &support_evidence_digest,
                )
                .seal();
        Self::new_with_support_identity(
            kind,
            cause_posture,
            async_posture,
            basis_posture,
            remask_posture,
            support_evidence_identity,
        )
    }

    pub fn new_with_support_identity(
        kind: ForgeQueryOrdinaryRuntimePostureKind,
        cause_posture: ForgeQueryOrdinaryRuntimeCausePostureKind,
        async_posture: Option<ForgeQueryOrdinaryRuntimeAsyncPostureKind>,
        basis_posture: ForgeQueryOrdinaryRuntimeBasisPostureKind,
        remask_posture: Option<ForgeQueryOrdinaryRuntimeRemaskPostureKind>,
        support_evidence_identity: ForgeQueryEvidenceIdentity,
    ) -> Self {
        let posture_identity =
            forge_query_evidence_identity(ForgeQueryEvidenceScope::LowerRuntimeBoundaryEvidence)
                .field_shape(
                    ForgeQueryEvidenceTag::new("identity_family"),
                    "forge_query_ordinary_runtime_posture_v1",
                )
                .field_shape(ForgeQueryEvidenceTag::new("kind"), kind.as_str())
                .field_shape(ForgeQueryEvidenceTag::new("cause"), cause_posture.as_str())
                .optional_shape(
                    ForgeQueryEvidenceTag::new("async"),
                    async_posture.map(ForgeQueryOrdinaryRuntimeAsyncPostureKind::as_str),
                )
                .field_shape(ForgeQueryEvidenceTag::new("basis"), basis_posture.as_str())
                .optional_shape(
                    ForgeQueryEvidenceTag::new("remask"),
                    remask_posture.map(ForgeQueryOrdinaryRuntimeRemaskPostureKind::as_str),
                )
                .field_evidence_identity(
                    ForgeQueryEvidenceTag::new("support"),
                    &support_evidence_identity,
                )
                .seal();
        Self {
            kind,
            cause_posture,
            async_posture,
            basis_posture,
            remask_posture,
            support_evidence_identity,
            posture_identity,
        }
    }

    pub fn kind(&self) -> ForgeQueryOrdinaryRuntimePostureKind {
        self.kind
    }

    pub fn cause_posture(&self) -> ForgeQueryOrdinaryRuntimeCausePostureKind {
        self.cause_posture
    }

    pub fn async_posture(&self) -> Option<ForgeQueryOrdinaryRuntimeAsyncPostureKind> {
        self.async_posture
    }

    pub fn basis_posture(&self) -> ForgeQueryOrdinaryRuntimeBasisPostureKind {
        self.basis_posture
    }

    pub fn remask_posture(&self) -> Option<ForgeQueryOrdinaryRuntimeRemaskPostureKind> {
        self.remask_posture
    }

    pub fn support_evidence_digest(&self) -> &str {
        self.support_evidence_identity.as_str()
    }

    pub fn support_evidence_identity(&self) -> &ForgeQueryEvidenceIdentity {
        &self.support_evidence_identity
    }

    pub fn posture_digest(&self) -> &str {
        self.posture_identity.as_str()
    }

    pub fn evidence_identity(&self) -> &ForgeQueryEvidenceIdentity {
        &self.posture_identity
    }
}
