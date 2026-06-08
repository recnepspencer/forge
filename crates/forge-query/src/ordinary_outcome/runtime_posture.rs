use crate::identity::hash_parts;

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
    support_evidence_digest: String,
    posture_digest: String,
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
        let posture_digest = hash_parts(&[
            "forge_query_ordinary_runtime_posture_v1".to_string(),
            format!("kind:{}", kind.as_str()),
            format!("cause:{}", cause_posture.as_str()),
            format!(
                "async:{}",
                async_posture
                    .map(ForgeQueryOrdinaryRuntimeAsyncPostureKind::as_str)
                    .unwrap_or("none")
            ),
            format!("basis:{}", basis_posture.as_str()),
            format!(
                "remask:{}",
                remask_posture
                    .map(ForgeQueryOrdinaryRuntimeRemaskPostureKind::as_str)
                    .unwrap_or("none")
            ),
            format!("support:{support_evidence_digest}"),
        ]);
        Self {
            kind,
            cause_posture,
            async_posture,
            basis_posture,
            remask_posture,
            support_evidence_digest,
            posture_digest,
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
        &self.support_evidence_digest
    }

    pub fn posture_digest(&self) -> &str {
        &self.posture_digest
    }
}
