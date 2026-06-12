use crate::application::ForgeQueryCapabilityFamily;
use crate::evidence_identity::{
    forge_query_evidence_identity, ForgeQueryEvidenceIdentity, ForgeQueryEvidenceScope,
    ForgeQueryEvidenceTag,
};

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum ForgeQueryPreparedContinuationBasisKind {
    Current,
    Historical,
    PreviewDerived,
}

impl ForgeQueryPreparedContinuationBasisKind {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Current => "current",
            Self::Historical => "historical",
            Self::PreviewDerived => "preview_derived",
        }
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum ForgeQueryPreparedContinuationFreshnessPosture {
    Stable,
    Stale,
}

impl ForgeQueryPreparedContinuationFreshnessPosture {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Stable => "stable",
            Self::Stale => "stale",
        }
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum ForgeQueryPreparedContinuationDriftKind {
    AsyncRequest,
    Replay,
    Remask,
    PreviewCrossedResidue,
    StaleCompletion,
}

impl ForgeQueryPreparedContinuationDriftKind {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::AsyncRequest => "async_request",
            Self::Replay => "replay",
            Self::Remask => "remask",
            Self::PreviewCrossedResidue => "preview_crossed_residue",
            Self::StaleCompletion => "stale_completion",
        }
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum ForgeQueryPreparedContinuationAuthorityWitness {
    Runtime,
    RuntimeBridgeFacade,
    RelationalFacade,
    SignalFacade,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ForgeQueryPreparedContinuationBasisWitness {
    kind: ForgeQueryPreparedContinuationBasisKind,
    basis_identity: ForgeQueryEvidenceIdentity,
    expected_lower_runtime_binding_identity: Option<ForgeQueryEvidenceIdentity>,
    source_basis_identity: Option<ForgeQueryEvidenceIdentity>,
}

impl ForgeQueryPreparedContinuationBasisWitness {
    pub(crate) fn new(
        kind: ForgeQueryPreparedContinuationBasisKind,
        basis_identity: ForgeQueryEvidenceIdentity,
        expected_lower_runtime_binding_identity: Option<ForgeQueryEvidenceIdentity>,
        source_basis_identity: Option<ForgeQueryEvidenceIdentity>,
    ) -> Self {
        Self {
            kind,
            basis_identity,
            expected_lower_runtime_binding_identity,
            source_basis_identity,
        }
    }

    pub fn kind(&self) -> ForgeQueryPreparedContinuationBasisKind {
        self.kind
    }

    pub fn basis_identity_digest(&self) -> &str {
        self.basis_identity.as_str()
    }

    pub fn basis_identity(&self) -> &ForgeQueryEvidenceIdentity {
        &self.basis_identity
    }

    pub fn expected_lower_runtime_binding_digest(&self) -> Option<&str> {
        self.expected_lower_runtime_binding_identity
            .as_ref()
            .map(ForgeQueryEvidenceIdentity::as_str)
    }

    pub fn expected_lower_runtime_binding_identity(&self) -> Option<&ForgeQueryEvidenceIdentity> {
        self.expected_lower_runtime_binding_identity.as_ref()
    }

    pub fn source_basis_identity_digest(&self) -> Option<&str> {
        self.source_basis_identity
            .as_ref()
            .map(ForgeQueryEvidenceIdentity::as_str)
    }

    pub fn source_basis_identity(&self) -> Option<&ForgeQueryEvidenceIdentity> {
        self.source_basis_identity.as_ref()
    }
}

pub(crate) fn continuation_readmission_basis_identity(
    basis_kind: ForgeQueryPreparedContinuationBasisKind,
    identity: impl AsRef<str>,
) -> ForgeQueryEvidenceIdentity {
    forge_query_evidence_identity(ForgeQueryEvidenceScope::ContinuationReadmissionBasis)
        .field_shape(
            ForgeQueryEvidenceTag::new("basis_kind"),
            basis_kind.as_str(),
        )
        .field_identity(ForgeQueryEvidenceTag::new("basis_identity"), identity)
        .seal()
}

pub(crate) fn continuation_readmission_lower_runtime_binding_identity(
    identity: impl AsRef<str>,
) -> ForgeQueryEvidenceIdentity {
    forge_query_evidence_identity(
        ForgeQueryEvidenceScope::ContinuationReadmissionLowerRuntimeBinding,
    )
    .field_identity(
        ForgeQueryEvidenceTag::new("lower_runtime_binding"),
        identity,
    )
    .seal()
}

pub(crate) fn continuation_readmission_source_basis_identity(
    identity: impl AsRef<str>,
) -> ForgeQueryEvidenceIdentity {
    forge_query_evidence_identity(ForgeQueryEvidenceScope::ContinuationReadmissionSourceBasis)
        .field_identity(ForgeQueryEvidenceTag::new("source_basis"), identity)
        .seal()
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ForgeQueryPreparedContinuationExecutionReadmission {
    basis_witness: ForgeQueryPreparedContinuationBasisWitness,
    authority_witness: ForgeQueryPreparedContinuationAuthorityWitness,
    freshness_posture: ForgeQueryPreparedContinuationFreshnessPosture,
    drift_kind: Option<ForgeQueryPreparedContinuationDriftKind>,
    required_capability_families: Vec<ForgeQueryCapabilityFamily>,
}

impl ForgeQueryPreparedContinuationExecutionReadmission {
    pub(crate) fn new(
        basis_witness: ForgeQueryPreparedContinuationBasisWitness,
        authority_witness: ForgeQueryPreparedContinuationAuthorityWitness,
        freshness_posture: ForgeQueryPreparedContinuationFreshnessPosture,
        drift_kind: Option<ForgeQueryPreparedContinuationDriftKind>,
        required_capability_families: Vec<ForgeQueryCapabilityFamily>,
    ) -> Self {
        Self {
            basis_witness,
            authority_witness,
            freshness_posture,
            drift_kind,
            required_capability_families,
        }
    }

    pub fn basis_witness(&self) -> &ForgeQueryPreparedContinuationBasisWitness {
        &self.basis_witness
    }

    pub fn authority_witness(&self) -> ForgeQueryPreparedContinuationAuthorityWitness {
        self.authority_witness
    }

    pub fn freshness_posture(&self) -> ForgeQueryPreparedContinuationFreshnessPosture {
        self.freshness_posture
    }

    pub fn drift_kind(&self) -> Option<ForgeQueryPreparedContinuationDriftKind> {
        self.drift_kind
    }

    pub fn required_capability_families(&self) -> &[ForgeQueryCapabilityFamily] {
        &self.required_capability_families
    }
}
