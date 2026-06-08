use crate::application::ForgeQueryCapabilityFamily;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum ForgeQueryPreparedContinuationBasisKind {
    Current,
    Historical,
    PreviewDerived,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum ForgeQueryPreparedContinuationFreshnessPosture {
    Stable,
    Stale,
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
    basis_identity_digest: String,
    expected_lower_runtime_binding_digest: Option<String>,
    source_basis_identity_digest: Option<String>,
}

impl ForgeQueryPreparedContinuationBasisWitness {
    pub(crate) fn new(
        kind: ForgeQueryPreparedContinuationBasisKind,
        basis_identity_digest: String,
        expected_lower_runtime_binding_digest: Option<String>,
        source_basis_identity_digest: Option<String>,
    ) -> Self {
        Self {
            kind,
            basis_identity_digest,
            expected_lower_runtime_binding_digest,
            source_basis_identity_digest,
        }
    }

    pub fn kind(&self) -> ForgeQueryPreparedContinuationBasisKind {
        self.kind
    }

    pub fn basis_identity_digest(&self) -> &str {
        &self.basis_identity_digest
    }

    pub fn expected_lower_runtime_binding_digest(&self) -> Option<&str> {
        self.expected_lower_runtime_binding_digest.as_deref()
    }

    pub fn source_basis_identity_digest(&self) -> Option<&str> {
        self.source_basis_identity_digest.as_deref()
    }
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
