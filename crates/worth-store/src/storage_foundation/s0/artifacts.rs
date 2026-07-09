use super::capability::{
    BackendForbiddenClaim, BackendForbiddenClaimKind, Roadmap2SequenceId,
    S0ClaimPromotionRejection, StoreBackendCapabilityTier,
};
use super::evidence::{S0ArtifactKind, S0EvidenceRef, S0StableDigest};
use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};
use std::collections::BTreeSet;

pub const S0_ARTIFACT_SCHEMA_VERSION: &str = "storage-foundation-s0/v1";

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
pub struct S0ArtifactRowId(String);

impl S0ArtifactRowId {
    pub fn new(value: impl Into<String>) -> Result<Self, S0ArtifactBuildRejection> {
        let value = value.into();
        let unstable_marker = value.contains(':')
            || value.contains('/')
            || value.contains('\\')
            || value.contains('#')
            || value.to_ascii_lowercase().contains("line");
        if value.trim().is_empty() {
            return Err(S0ArtifactBuildRejection::EmptyRowId);
        }
        if unstable_marker {
            return Err(S0ArtifactBuildRejection::UnstableRowId);
        }
        Ok(Self(value))
    }

    pub fn as_str(&self) -> &str {
        &self.0
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum S0ArtifactSubjectKind {
    Backend,
    EvidenceLane,
    Milestone,
    ClaimSurface,
    TestSuite,
    Harness,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum S0ArtifactRowStatus {
    Present,
    AbsentWithInventoryEvidence,
    Deferred,
    NotApplicable,
    Admitted,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize)]
pub enum S0FirstAuditBaselineRowId {
    AbsentMode,
    InMemoryHarness,
    EmbeddedMode,
    DurableMode,
    LocalFileBackend,
    SqliteBackend,
    SemanticCertificationHarness,
    SubscriptionSupportTrustEvidence,
    Roadmap2PhysicalBackendCandidate,
    FuturePlatformGradeBackend,
}

impl S0FirstAuditBaselineRowId {
    pub fn required() -> [Self; 10] {
        [
            Self::AbsentMode,
            Self::InMemoryHarness,
            Self::EmbeddedMode,
            Self::DurableMode,
            Self::LocalFileBackend,
            Self::SqliteBackend,
            Self::SemanticCertificationHarness,
            Self::SubscriptionSupportTrustEvidence,
            Self::Roadmap2PhysicalBackendCandidate,
            Self::FuturePlatformGradeBackend,
        ]
    }

    pub fn row_id(self) -> S0ArtifactRowId {
        S0ArtifactRowId::new(match self {
            Self::AbsentMode => "AbsentMode",
            Self::InMemoryHarness => "InMemoryHarness",
            Self::EmbeddedMode => "EmbeddedMode",
            Self::DurableMode => "DurableMode",
            Self::LocalFileBackend => "LocalFileBackend",
            Self::SqliteBackend => "SqliteBackend",
            Self::SemanticCertificationHarness => "SemanticCertificationHarness",
            Self::SubscriptionSupportTrustEvidence => "SubscriptionSupportTrustEvidence",
            Self::Roadmap2PhysicalBackendCandidate => "Roadmap2PhysicalBackendCandidate",
            Self::FuturePlatformGradeBackend => "FuturePlatformGradeBackend",
        })
        .expect("required S.0 first-audit row ids are stable constants")
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct S0NondeterministicMetadata {
    generated_at_policy: String,
    local_path_hint: Option<String>,
    host_hint: Option<String>,
}

impl S0NondeterministicMetadata {
    pub fn excluded(
        generated_at_policy: impl Into<String>,
        local_path_hint: Option<impl Into<String>>,
        host_hint: Option<impl Into<String>>,
    ) -> Result<Self, S0ArtifactBuildRejection> {
        Ok(Self {
            generated_at_policy: require_non_empty("generated_at_policy", generated_at_policy)?,
            local_path_hint: local_path_hint.map(Into::into),
            host_hint: host_hint.map(Into::into),
        })
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct S0ArtifactValidationCostSurface {
    artifact_byte_count: u64,
    row_count: u64,
    canonicalized_row_byte_count: u64,
    sort_row_count: u64,
}

impl S0ArtifactValidationCostSurface {
    pub(crate) fn new(
        artifact_byte_count: u64,
        row_count: u64,
        canonicalized_row_byte_count: u64,
        sort_row_count: u64,
    ) -> Self {
        Self {
            artifact_byte_count,
            row_count,
            canonicalized_row_byte_count,
            sort_row_count,
        }
    }

    pub fn artifact_byte_count(&self) -> u64 {
        self.artifact_byte_count
    }

    pub fn row_count(&self) -> u64 {
        self.row_count
    }

    pub fn canonicalized_row_byte_count(&self) -> u64 {
        self.canonicalized_row_byte_count
    }

    pub fn sort_row_count(&self) -> u64 {
        self.sort_row_count
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct S0ValidatedBackendCapabilityMatrixArtifact {
    matrix: BackendCapabilityMatrix,
    validation_cost: S0ArtifactValidationCostSurface,
}

impl S0ValidatedBackendCapabilityMatrixArtifact {
    pub fn matrix(&self) -> &BackendCapabilityMatrix {
        &self.matrix
    }

    pub fn validation_cost(&self) -> &S0ArtifactValidationCostSurface {
        &self.validation_cost
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct BackendCapabilityMatrixRow {
    row_id: S0ArtifactRowId,
    subject_kind: S0ArtifactSubjectKind,
    subject_path_or_symbol: String,
    classification: String,
    evidence_refs: Vec<S0EvidenceRef>,
    forbidden_claims: Vec<BackendForbiddenClaim>,
    deferred_s_sequences: Vec<Roadmap2SequenceId>,
    status: S0ArtifactRowStatus,
    notes: String,
    capability_tier: StoreBackendCapabilityTier,
    valid_use: String,
    required_evidence_before_promotion: Vec<String>,
    known_semantic_guarantees: Vec<String>,
    known_physical_gaps: Vec<String>,
}

impl BackendCapabilityMatrixRow {
    #[allow(clippy::too_many_arguments)]
    pub fn new(
        row_id: S0ArtifactRowId,
        subject_kind: S0ArtifactSubjectKind,
        subject_path_or_symbol: impl Into<String>,
        classification: impl Into<String>,
        evidence_refs: Vec<S0EvidenceRef>,
        forbidden_claims: Vec<BackendForbiddenClaim>,
        deferred_s_sequences: Vec<Roadmap2SequenceId>,
        status: S0ArtifactRowStatus,
        notes: impl Into<String>,
        capability_tier: StoreBackendCapabilityTier,
        valid_use: impl Into<String>,
        required_evidence_before_promotion: Vec<String>,
        known_semantic_guarantees: Vec<String>,
        known_physical_gaps: Vec<String>,
    ) -> Result<Self, S0ArtifactBuildRejection> {
        let subject_path_or_symbol =
            require_non_empty("subject_path_or_symbol", subject_path_or_symbol)?;
        let classification = require_non_empty("classification", classification)?;
        let notes = require_non_empty("notes", notes)?;
        let valid_use = require_non_empty("valid_use", valid_use)?;
        if evidence_refs.is_empty() {
            return Err(S0ArtifactBuildRejection::MissingEvidenceRef);
        }
        if status != S0ArtifactRowStatus::Admitted && forbidden_claims.is_empty() {
            return Err(S0ArtifactBuildRejection::ForbiddenClaimsMissing);
        }
        if !known_physical_gaps.is_empty() && deferred_s_sequences.is_empty() {
            return Err(S0ArtifactBuildRejection::DeferredSequenceMissing);
        }
        Ok(Self {
            row_id,
            subject_kind,
            subject_path_or_symbol,
            classification,
            evidence_refs,
            forbidden_claims,
            deferred_s_sequences,
            status,
            notes,
            capability_tier,
            valid_use,
            required_evidence_before_promotion,
            known_semantic_guarantees,
            known_physical_gaps,
        })
    }

    pub fn row_id(&self) -> &S0ArtifactRowId {
        &self.row_id
    }

    pub fn subject_path_or_symbol(&self) -> &str {
        &self.subject_path_or_symbol
    }

    pub fn evidence_refs(&self) -> &[S0EvidenceRef] {
        &self.evidence_refs
    }

    pub fn forbidden_claims(&self) -> &[BackendForbiddenClaim] {
        &self.forbidden_claims
    }

    pub fn deferred_s_sequences(&self) -> &[Roadmap2SequenceId] {
        &self.deferred_s_sequences
    }

    pub fn capability_tier(&self) -> StoreBackendCapabilityTier {
        self.capability_tier
    }

    pub fn status(&self) -> S0ArtifactRowStatus {
        self.status
    }

    pub fn known_physical_gaps(&self) -> &[String] {
        &self.known_physical_gaps
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct S0ArtifactEnvelopeMetadata {
    schema_version: String,
    artifact_kind: S0ArtifactKind,
    source_revision: String,
    roadmap_parent_digest: S0StableDigest,
    generated_by: String,
    deterministic_digest: S0StableDigest,
    nondeterministic_metadata: S0NondeterministicMetadata,
}

impl S0ArtifactEnvelopeMetadata {
    pub(crate) fn new(
        artifact_kind: S0ArtifactKind,
        source_revision: String,
        roadmap_parent_digest: S0StableDigest,
        generated_by: String,
        deterministic_digest: S0StableDigest,
        nondeterministic_metadata: S0NondeterministicMetadata,
    ) -> Self {
        Self {
            schema_version: S0_ARTIFACT_SCHEMA_VERSION.to_string(),
            artifact_kind,
            source_revision,
            roadmap_parent_digest,
            generated_by,
            deterministic_digest,
            nondeterministic_metadata,
        }
    }

    pub fn deterministic_digest(&self) -> &S0StableDigest {
        &self.deterministic_digest
    }

    pub fn artifact_kind(&self) -> S0ArtifactKind {
        self.artifact_kind
    }

    pub fn source_revision(&self) -> &str {
        &self.source_revision
    }

    pub fn roadmap_parent_digest(&self) -> &S0StableDigest {
        &self.roadmap_parent_digest
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct BackendCapabilityMatrix {
    #[serde(flatten)]
    envelope: S0ArtifactEnvelopeMetadata,
    rows: Vec<BackendCapabilityMatrixRow>,
}

impl BackendCapabilityMatrix {
    pub fn new(
        source_revision: impl Into<String>,
        roadmap_parent_digest: S0StableDigest,
        generated_by: impl Into<String>,
        nondeterministic_metadata: S0NondeterministicMetadata,
        mut rows: Vec<BackendCapabilityMatrixRow>,
    ) -> Result<Self, S0ArtifactBuildRejection> {
        let source_revision = require_non_empty("source_revision", source_revision)?;
        let generated_by = require_non_empty("generated_by", generated_by)?;
        rows.sort_by(|left, right| left.row_id.cmp(&right.row_id));
        reject_duplicate_rows(&rows)?;
        reject_missing_first_audit_rows(&rows)?;
        let deterministic_digest = stable_digest(&BackendCapabilityMatrixDigestBasis {
            schema_version: S0_ARTIFACT_SCHEMA_VERSION,
            artifact_kind: S0ArtifactKind::BackendCapabilityMatrix,
            source_revision: &source_revision,
            roadmap_parent_digest: &roadmap_parent_digest,
            generated_by: &generated_by,
            rows: &rows,
        })?;
        let envelope = S0ArtifactEnvelopeMetadata {
            schema_version: S0_ARTIFACT_SCHEMA_VERSION.to_string(),
            artifact_kind: S0ArtifactKind::BackendCapabilityMatrix,
            source_revision,
            roadmap_parent_digest,
            generated_by,
            deterministic_digest,
            nondeterministic_metadata,
        };
        Ok(Self { envelope, rows })
    }

    pub fn first_audit_baseline(
        source_revision: impl Into<String>,
        roadmap_parent_digest: S0StableDigest,
        generated_by: impl Into<String>,
        nondeterministic_metadata: S0NondeterministicMetadata,
    ) -> Result<Self, S0ArtifactBuildRejection> {
        Self::new(
            source_revision,
            roadmap_parent_digest,
            generated_by,
            nondeterministic_metadata,
            first_audit_baseline_rows(),
        )
    }

    pub fn envelope(&self) -> &S0ArtifactEnvelopeMetadata {
        &self.envelope
    }

    pub fn rows(&self) -> &[BackendCapabilityMatrixRow] {
        &self.rows
    }

    pub fn to_canonical_json_bytes(&self) -> Result<Vec<u8>, S0ArtifactParseRejection> {
        serde_json::to_vec_pretty(self).map_err(|_| S0ArtifactParseRejection::SerializationFailed)
    }

    pub fn validate_canonical_json_bytes(
        bytes: &[u8],
    ) -> Result<S0ValidatedBackendCapabilityMatrixArtifact, S0ArtifactParseRejection> {
        let raw = serde_json::from_slice::<RawBackendCapabilityMatrix>(bytes)
            .map_err(|_| S0ArtifactParseRejection::NonParseable)?;
        if raw.envelope.schema_version != S0_ARTIFACT_SCHEMA_VERSION {
            return Err(S0ArtifactParseRejection::SchemaVersionMismatch);
        }
        if raw.envelope.artifact_kind != S0ArtifactKind::BackendCapabilityMatrix {
            return Err(S0ArtifactParseRejection::ArtifactKindMismatch);
        }

        let roadmap_parent_digest = S0StableDigest::new(raw.envelope.roadmap_parent_digest)
            .map_err(|_| S0ArtifactParseRejection::InvalidDigest)?;
        let expected_digest = S0StableDigest::new(raw.envelope.deterministic_digest)
            .map_err(|_| S0ArtifactParseRejection::InvalidDigest)?;
        let rows = raw
            .rows
            .into_iter()
            .map(RawBackendCapabilityMatrixRow::into_validated)
            .collect::<Result<Vec<_>, _>>()?;
        let matrix = Self::new(
            raw.envelope.source_revision,
            roadmap_parent_digest,
            raw.envelope.generated_by,
            raw.envelope.nondeterministic_metadata.into_validated()?,
            rows,
        )?;
        if matrix.envelope().deterministic_digest() != &expected_digest {
            return Err(S0ArtifactParseRejection::DeterministicDigestMismatch);
        }
        let canonicalized_row_byte_count = serde_json::to_vec(matrix.rows())
            .map_err(|_| S0ArtifactParseRejection::SerializationFailed)?
            .len() as u64;
        let validation_cost = S0ArtifactValidationCostSurface {
            artifact_byte_count: bytes.len() as u64,
            row_count: matrix.rows().len() as u64,
            canonicalized_row_byte_count,
            sort_row_count: matrix.rows().len() as u64,
        };
        Ok(S0ValidatedBackendCapabilityMatrixArtifact {
            matrix,
            validation_cost,
        })
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize)]
pub enum S0ArtifactBuildRejection {
    EmptyRowId,
    UnstableRowId,
    EmptyRequiredField,
    MissingEvidenceRef,
    ForbiddenClaimsMissing,
    DeferredSequenceMissing,
    DuplicateRowId,
    MissingFirstAuditBaselineRow,
    DigestConstructionFailed,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize)]
pub enum S0ArtifactParseRejection {
    NonParseable,
    SerializationFailed,
    SchemaVersionMismatch,
    ArtifactKindMismatch,
    InvalidDigest,
    InvalidForbiddenClaim,
    InvalidDeferredSequence,
    RowBuildRejected(S0ArtifactBuildRejection),
    MatrixBuildRejected(S0ArtifactBuildRejection),
    DeterministicDigestMismatch,
}

impl From<S0ArtifactBuildRejection> for S0ArtifactParseRejection {
    fn from(value: S0ArtifactBuildRejection) -> Self {
        Self::MatrixBuildRejected(value)
    }
}

#[derive(Serialize)]
struct BackendCapabilityMatrixDigestBasis<'a> {
    schema_version: &'static str,
    artifact_kind: S0ArtifactKind,
    source_revision: &'a str,
    roadmap_parent_digest: &'a S0StableDigest,
    generated_by: &'a str,
    rows: &'a [BackendCapabilityMatrixRow],
}

#[derive(Deserialize)]
struct RawBackendCapabilityMatrix {
    #[serde(flatten)]
    envelope: RawS0ArtifactEnvelope,
    rows: Vec<RawBackendCapabilityMatrixRow>,
}

#[derive(Deserialize)]
struct RawS0ArtifactEnvelope {
    schema_version: String,
    artifact_kind: S0ArtifactKind,
    source_revision: String,
    roadmap_parent_digest: String,
    generated_by: String,
    deterministic_digest: String,
    nondeterministic_metadata: RawS0NondeterministicMetadata,
}

#[derive(Deserialize)]
struct RawS0NondeterministicMetadata {
    generated_at_policy: String,
    local_path_hint: Option<String>,
    host_hint: Option<String>,
}

impl RawS0NondeterministicMetadata {
    fn into_validated(self) -> Result<S0NondeterministicMetadata, S0ArtifactParseRejection> {
        S0NondeterministicMetadata::excluded(
            self.generated_at_policy,
            self.local_path_hint,
            self.host_hint,
        )
        .map_err(S0ArtifactParseRejection::RowBuildRejected)
    }
}

#[derive(Deserialize)]
struct RawBackendCapabilityMatrixRow {
    row_id: String,
    subject_kind: S0ArtifactSubjectKind,
    subject_path_or_symbol: String,
    classification: String,
    evidence_refs: Vec<RawS0EvidenceRef>,
    forbidden_claims: Vec<RawBackendForbiddenClaim>,
    deferred_s_sequences: Vec<String>,
    status: S0ArtifactRowStatus,
    notes: String,
    capability_tier: StoreBackendCapabilityTier,
    valid_use: String,
    required_evidence_before_promotion: Vec<String>,
    known_semantic_guarantees: Vec<String>,
    known_physical_gaps: Vec<String>,
}

impl RawBackendCapabilityMatrixRow {
    fn into_validated(self) -> Result<BackendCapabilityMatrixRow, S0ArtifactParseRejection> {
        let row_id = S0ArtifactRowId::new(self.row_id)
            .map_err(S0ArtifactParseRejection::RowBuildRejected)?;
        let evidence_refs = self
            .evidence_refs
            .into_iter()
            .map(RawS0EvidenceRef::into_validated)
            .collect::<Result<Vec<_>, _>>()?;
        let forbidden_claims = self
            .forbidden_claims
            .into_iter()
            .map(RawBackendForbiddenClaim::into_validated)
            .collect::<Result<Vec<_>, _>>()?;
        let deferred_s_sequences = self
            .deferred_s_sequences
            .into_iter()
            .map(|sequence| {
                Roadmap2SequenceId::new(sequence)
                    .map_err(|_| S0ArtifactParseRejection::InvalidDeferredSequence)
            })
            .collect::<Result<Vec<_>, _>>()?;
        BackendCapabilityMatrixRow::new(
            row_id,
            self.subject_kind,
            self.subject_path_or_symbol,
            self.classification,
            evidence_refs,
            forbidden_claims,
            deferred_s_sequences,
            self.status,
            self.notes,
            self.capability_tier,
            self.valid_use,
            self.required_evidence_before_promotion,
            self.known_semantic_guarantees,
            self.known_physical_gaps,
        )
        .map_err(S0ArtifactParseRejection::RowBuildRejected)
    }
}

#[derive(Deserialize)]
struct RawS0EvidenceRef {
    artifact_kind: S0ArtifactKind,
    digest: String,
}

impl RawS0EvidenceRef {
    fn into_validated(self) -> Result<S0EvidenceRef, S0ArtifactParseRejection> {
        let digest = S0StableDigest::new(self.digest)
            .map_err(|_| S0ArtifactParseRejection::InvalidDigest)?;
        Ok(S0EvidenceRef::new(self.artifact_kind, digest))
    }
}

#[derive(Deserialize)]
struct RawBackendForbiddenClaim {
    claim_kind: BackendForbiddenClaimKind,
    deferred_sequence: String,
}

impl RawBackendForbiddenClaim {
    fn into_validated(self) -> Result<BackendForbiddenClaim, S0ArtifactParseRejection> {
        BackendForbiddenClaim::new(self.claim_kind, self.deferred_sequence).map_err(|error| {
            if error == S0ClaimPromotionRejection::MissingSequenceMapping {
                S0ArtifactParseRejection::InvalidDeferredSequence
            } else {
                S0ArtifactParseRejection::InvalidForbiddenClaim
            }
        })
    }
}

fn first_audit_baseline_rows() -> Vec<BackendCapabilityMatrixRow> {
    S0FirstAuditBaselineRowId::required()
        .into_iter()
        .map(first_audit_baseline_row)
        .collect()
}

fn first_audit_baseline_row(id: S0FirstAuditBaselineRowId) -> BackendCapabilityMatrixRow {
    let (subject, subject_kind, tier, classification, valid_use, semantic, gaps, sequences, status) =
        match id {
            S0FirstAuditBaselineRowId::AbsentMode => (
                "worth_store::modes::AbsentMode",
                S0ArtifactSubjectKind::Backend,
                StoreBackendCapabilityTier::Bootstrap,
                "optional-store-boundary",
                "Proves optional Store boundaries without persistence claims.",
                vec!["optional Store semantics".to_string()],
                vec!["physical persistence".to_string()],
                vec!["S1", "S4"],
                S0ArtifactRowStatus::Present,
            ),
            S0FirstAuditBaselineRowId::InMemoryHarness => (
                "worth_store::tests::harness",
                S0ArtifactSubjectKind::Harness,
                StoreBackendCapabilityTier::SemanticCertification,
                "semantic-harness",
                "Exercises semantic behavior without durable survival evidence.",
                vec!["semantic replay".to_string()],
                vec!["durable media survival".to_string()],
                vec!["S1", "S2"],
                S0ArtifactRowStatus::Present,
            ),
            S0FirstAuditBaselineRowId::EmbeddedMode => (
                "worth_store::modes::EmbeddedMode",
                S0ArtifactSubjectKind::Backend,
                StoreBackendCapabilityTier::SemanticCertification,
                "embedded-semantic-mode",
                "Proves lifecycle and artifact reception semantics.",
                vec!["embedded lifecycle".to_string()],
                vec!["platform-grade physical database posture".to_string()],
                vec!["S1", "S5"],
                S0ArtifactRowStatus::Present,
            ),
            S0FirstAuditBaselineRowId::DurableMode => (
                "worth_store::modes::DurableMode",
                S0ArtifactSubjectKind::Backend,
                StoreBackendCapabilityTier::SemanticCertification,
                "durable-mode-orchestration",
                "Proves durable-mode orchestration semantics.",
                vec!["semantic durable-mode orchestration".to_string()],
                vec!["S.4 recovery physics".to_string()],
                vec!["S4"],
                S0ArtifactRowStatus::Present,
            ),
            S0FirstAuditBaselineRowId::LocalFileBackend => (
                "worth_store::backend::local_file",
                S0ArtifactSubjectKind::Backend,
                StoreBackendCapabilityTier::Compatibility,
                "local-file-compatibility",
                "Compatibility path until Roadmap 2 physical gates are proven.",
                vec!["bootstrap file persistence".to_string()],
                vec!["bounded page substrate".to_string()],
                vec!["S1", "S3", "S6"],
                S0ArtifactRowStatus::Deferred,
            ),
            S0FirstAuditBaselineRowId::SqliteBackend => (
                "worth_store::backend::sqlite",
                S0ArtifactSubjectKind::Backend,
                StoreBackendCapabilityTier::Compatibility,
                "sqlite-compatibility",
                "Compatibility path until Store-owned physical gates are proven.",
                vec!["bootstrap SQLite interoperability".to_string()],
                vec!["Store-native physical authority".to_string()],
                vec!["S1", "S6"],
                S0ArtifactRowStatus::Deferred,
            ),
            S0FirstAuditBaselineRowId::SemanticCertificationHarness => (
                "worth_store::evidence",
                S0ArtifactSubjectKind::EvidenceLane,
                StoreBackendCapabilityTier::SemanticCertification,
                "semantic-certification-evidence",
                "Certifies semantic Store behavior without physical substrate claims.",
                vec!["semantic certification".to_string()],
                vec!["physical boundedness".to_string()],
                vec!["S2", "S12"],
                S0ArtifactRowStatus::Present,
            ),
            S0FirstAuditBaselineRowId::SubscriptionSupportTrustEvidence => (
                "worth_store::subscription_support::trust",
                S0ArtifactSubjectKind::EvidenceLane,
                StoreBackendCapabilityTier::SemanticCertification,
                "closed-semantic-trust-evidence",
                "Milestone 13.3 trust evidence; not physical database readiness.",
                vec!["role-scoped subscription-support trust".to_string()],
                vec!["physical database readiness".to_string()],
                vec!["S12"],
                S0ArtifactRowStatus::Present,
            ),
            S0FirstAuditBaselineRowId::Roadmap2PhysicalBackendCandidate => (
                "worth_store::storage_foundation::s1",
                S0ArtifactSubjectKind::Backend,
                StoreBackendCapabilityTier::PhysicalFoundation,
                "physical-backend-candidate",
                "Candidate row for S.1 physical substrate evidence.",
                vec!["none admitted yet".to_string()],
                vec!["closed physical foundation gates".to_string()],
                vec!["S1"],
                S0ArtifactRowStatus::Deferred,
            ),
            S0FirstAuditBaselineRowId::FuturePlatformGradeBackend => (
                "worth_store::storage_foundation::platform_grade",
                S0ArtifactSubjectKind::Backend,
                StoreBackendCapabilityTier::PlatformGrade,
                "future-platform-grade-target",
                "Target posture only; requires closed Roadmap 2 platform evidence.",
                vec!["none admitted yet".to_string()],
                vec!["all required Roadmap 2 platform gates".to_string()],
                vec![
                    "S1", "S2", "S3", "S4", "S5", "S6", "S7", "S8", "S9", "S10", "S11", "S12",
                ],
                S0ArtifactRowStatus::Deferred,
            ),
        };
    let deferred_sequences = sequences
        .into_iter()
        .map(|sequence| Roadmap2SequenceId::new(sequence).unwrap())
        .collect::<Vec<_>>();
    BackendCapabilityMatrixRow::new(
        id.row_id(),
        subject_kind,
        subject,
        classification,
        vec![baseline_evidence_ref()],
        baseline_forbidden_claims(&deferred_sequences),
        deferred_sequences,
        status,
        "S.0 first-audit baseline row.",
        tier,
        valid_use,
        vec!["Closed Roadmap 2 evidence witness".to_string()],
        semantic,
        gaps,
    )
    .expect("first-audit baseline constants must satisfy row invariants")
}

fn baseline_forbidden_claims(sequences: &[Roadmap2SequenceId]) -> Vec<BackendForbiddenClaim> {
    let sequence = sequences
        .first()
        .cloned()
        .unwrap_or_else(|| Roadmap2SequenceId::new("S1").unwrap());
    [
        BackendForbiddenClaimKind::PlatformGradeDurability,
        BackendForbiddenClaimKind::PhysicalPersistence,
    ]
    .into_iter()
    .map(|kind| {
        BackendForbiddenClaim::new(kind, sequence.as_str())
            .expect("baseline deferred sequence is known non-empty")
    })
    .collect()
}

fn baseline_evidence_ref() -> S0EvidenceRef {
    S0EvidenceRef::new(
        S0ArtifactKind::S0EvidenceBundle,
        S0StableDigest::new("s0:first-audit-baseline").unwrap(),
    )
}

fn reject_duplicate_rows(
    rows: &[BackendCapabilityMatrixRow],
) -> Result<(), S0ArtifactBuildRejection> {
    if rows
        .windows(2)
        .any(|pair| pair[0].row_id() == pair[1].row_id())
    {
        return Err(S0ArtifactBuildRejection::DuplicateRowId);
    }
    Ok(())
}

fn reject_missing_first_audit_rows(
    rows: &[BackendCapabilityMatrixRow],
) -> Result<(), S0ArtifactBuildRejection> {
    let present = rows
        .iter()
        .map(|row| row.row_id().clone())
        .collect::<BTreeSet<_>>();
    if S0FirstAuditBaselineRowId::required()
        .into_iter()
        .any(|required| !present.contains(&required.row_id()))
    {
        return Err(S0ArtifactBuildRejection::MissingFirstAuditBaselineRow);
    }
    Ok(())
}

fn stable_digest<T: Serialize + ?Sized>(
    value: &T,
) -> Result<S0StableDigest, S0ArtifactBuildRejection> {
    let bytes = serde_json::to_vec(value)
        .map_err(|_| S0ArtifactBuildRejection::DigestConstructionFailed)?;
    let mut hasher = Sha256::new();
    hasher.update(bytes);
    S0StableDigest::new(format!("{:x}", hasher.finalize()))
        .map_err(|_| S0ArtifactBuildRejection::DigestConstructionFailed)
}

fn require_non_empty(
    _field: &'static str,
    value: impl Into<String>,
) -> Result<String, S0ArtifactBuildRejection> {
    let value = value.into();
    if value.trim().is_empty() {
        return Err(S0ArtifactBuildRejection::EmptyRequiredField);
    }
    Ok(value)
}
