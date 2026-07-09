use super::artifacts::{
    S0ArtifactEnvelopeMetadata, S0ArtifactRowId, S0ArtifactRowStatus, S0ArtifactSubjectKind,
    S0ArtifactValidationCostSurface, S0NondeterministicMetadata, S0_ARTIFACT_SCHEMA_VERSION,
};
use super::capability::{BackendForbiddenClaimKind, Roadmap2SequenceId};
use super::evidence::{S0ArtifactKind, S0EvidenceRef, S0StableDigest};
use super::milestones::{
    MilestonePhysicalStatusRow, S0PhysicalStatus, SemanticPhysicalClaimFamily,
};
use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};
use std::collections::BTreeSet;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
pub enum DeferredPhysicalGuaranteeCategory {
    PageSegmentExtentSubstrate,
    MemoryAllocationBoundedness,
    PageFrameChunkIntegrityAndCorruptionLocalization,
    WalCheckpointLsnRecoveryPhysics,
    PhysicalReadStabilityDuringMaintenance,
    HardwareAwareIoAndForegroundQos,
    NativeBlobObjectChunkStore,
    IndexLayoutAccessPathDiscipline,
    FormalCrashConcurrencyModels,
    BackupPitrRepairAndForensics,
    SecurityTenantBoundariesKeysAndAuditability,
    PhysicalDatabaseCertificationAndPerformance,
}

impl DeferredPhysicalGuaranteeCategory {
    fn minimum_required_sequences(self) -> &'static [&'static str] {
        match self {
            Self::PageSegmentExtentSubstrate => &["S1"],
            Self::MemoryAllocationBoundedness => &["S2"],
            Self::PageFrameChunkIntegrityAndCorruptionLocalization => &["S3"],
            Self::WalCheckpointLsnRecoveryPhysics => &["S4"],
            Self::PhysicalReadStabilityDuringMaintenance => &["S5"],
            Self::HardwareAwareIoAndForegroundQos => &["S6"],
            Self::NativeBlobObjectChunkStore => &["S7"],
            Self::IndexLayoutAccessPathDiscipline => &["S8"],
            Self::FormalCrashConcurrencyModels => &["S9"],
            Self::BackupPitrRepairAndForensics => &["S10"],
            Self::SecurityTenantBoundariesKeysAndAuditability => &["S11"],
            Self::PhysicalDatabaseCertificationAndPerformance => &["S12"],
        }
    }

    fn missing_proof_summary(self) -> &'static str {
        match self {
            Self::PageSegmentExtentSubstrate => {
                "page/segment/extent substrate proof remains unearned"
            }
            Self::MemoryAllocationBoundedness => {
                "memory and allocation boundedness proof remains unearned"
            }
            Self::PageFrameChunkIntegrityAndCorruptionLocalization => {
                "page/frame/chunk integrity and corruption localization proof remains unearned"
            }
            Self::WalCheckpointLsnRecoveryPhysics => {
                "WAL/checkpoint/LSN recovery physics proof remains unearned"
            }
            Self::PhysicalReadStabilityDuringMaintenance => {
                "physical read stability during maintenance remains unearned"
            }
            Self::HardwareAwareIoAndForegroundQos => {
                "hardware-aware I/O and foreground QoS proof remains unearned"
            }
            Self::NativeBlobObjectChunkStore => {
                "native blob/object chunk store proof remains unearned"
            }
            Self::IndexLayoutAccessPathDiscipline => {
                "index/layout/access-path discipline proof remains unearned"
            }
            Self::FormalCrashConcurrencyModels => {
                "formal crash/concurrency model proof remains unearned"
            }
            Self::BackupPitrRepairAndForensics => {
                "backup, PITR, repair, and forensics proof remains unearned"
            }
            Self::SecurityTenantBoundariesKeysAndAuditability => {
                "security, tenant boundary, key, and auditability proof remains unearned"
            }
            Self::PhysicalDatabaseCertificationAndPerformance => {
                "physical database certification and performance proof remains unearned"
            }
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct DeferredPhysicalGuaranteeRow {
    row_id: S0ArtifactRowId,
    subject_kind: S0ArtifactSubjectKind,
    subject_path_or_symbol: String,
    classification: String,
    evidence_refs: Vec<S0EvidenceRef>,
    forbidden_claims: Vec<super::capability::BackendForbiddenClaim>,
    deferred_s_sequences: Vec<Roadmap2SequenceId>,
    status: S0ArtifactRowStatus,
    notes: String,
    guarantee_category: DeferredPhysicalGuaranteeCategory,
    current_evidence_status: S0PhysicalStatus,
    missing_proof: String,
    dependent_named_suite: String,
    dependent_evidence_lanes: Vec<String>,
}

impl DeferredPhysicalGuaranteeRow {
    #[allow(clippy::too_many_arguments)]
    pub fn new(
        row_id: S0ArtifactRowId,
        subject_kind: S0ArtifactSubjectKind,
        subject_path_or_symbol: impl Into<String>,
        classification: impl Into<String>,
        evidence_refs: Vec<S0EvidenceRef>,
        forbidden_claims: Vec<super::capability::BackendForbiddenClaim>,
        deferred_s_sequences: Vec<Roadmap2SequenceId>,
        status: S0ArtifactRowStatus,
        notes: impl Into<String>,
        guarantee_category: DeferredPhysicalGuaranteeCategory,
        current_evidence_status: S0PhysicalStatus,
        missing_proof: impl Into<String>,
        dependent_named_suite: impl Into<String>,
        dependent_evidence_lanes: Vec<String>,
    ) -> Result<Self, S0DeferredGuaranteeBuildRejection> {
        let subject_path_or_symbol = require_non_empty(subject_path_or_symbol)?;
        let classification = require_non_empty(classification)?;
        let notes = require_non_empty(notes)?;
        let missing_proof = require_non_empty(missing_proof)?;
        let dependent_named_suite = require_non_empty(dependent_named_suite)?;
        if evidence_refs.is_empty() {
            return Err(S0DeferredGuaranteeBuildRejection::MissingEvidenceRef);
        }
        if dependent_evidence_lanes.is_empty() {
            return Err(S0DeferredGuaranteeBuildRejection::MissingEvidenceLane);
        }
        if deferred_s_sequences.is_empty() {
            return Err(S0DeferredGuaranteeBuildRejection::DeferredSequenceMissing);
        }
        if matches!(
            current_evidence_status,
            S0PhysicalStatus::FoundationBacked
                | S0PhysicalStatus::PlatformGrade
                | S0PhysicalStatus::NotApplicable
        ) {
            return Err(S0DeferredGuaranteeBuildRejection::GuaranteeAlreadySatisfied);
        }
        if !guarantee_category
            .minimum_required_sequences()
            .iter()
            .any(|required| {
                deferred_s_sequences
                    .iter()
                    .any(|sequence| sequence.as_str() == *required)
            })
        {
            return Err(S0DeferredGuaranteeBuildRejection::GuaranteeCategorySequenceMismatch);
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
            guarantee_category,
            current_evidence_status,
            missing_proof,
            dependent_named_suite,
            dependent_evidence_lanes,
        })
    }

    pub fn row_id(&self) -> &S0ArtifactRowId {
        &self.row_id
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct S0ValidatedDeferredPhysicalGuaranteeMapArtifact {
    map: DeferredPhysicalGuaranteeMap,
    validation_cost: S0ArtifactValidationCostSurface,
}

impl S0ValidatedDeferredPhysicalGuaranteeMapArtifact {
    pub fn map(&self) -> &DeferredPhysicalGuaranteeMap {
        &self.map
    }

    pub fn validation_cost(&self) -> &S0ArtifactValidationCostSurface {
        &self.validation_cost
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct DeferredPhysicalGuaranteeMap {
    #[serde(flatten)]
    envelope: S0ArtifactEnvelopeMetadata,
    rows: Vec<DeferredPhysicalGuaranteeRow>,
}

impl DeferredPhysicalGuaranteeMap {
    pub fn new(
        source_revision: impl Into<String>,
        roadmap_parent_digest: S0StableDigest,
        generated_by: impl Into<String>,
        nondeterministic_metadata: S0NondeterministicMetadata,
        mut rows: Vec<DeferredPhysicalGuaranteeRow>,
    ) -> Result<Self, S0DeferredGuaranteeBuildRejection> {
        let source_revision = require_non_empty(source_revision)?;
        let generated_by = require_non_empty(generated_by)?;
        rows.sort_by(|left, right| left.row_id.cmp(&right.row_id));
        reject_duplicate_rows(&rows)?;
        let deterministic_digest = stable_digest(&DeferredPhysicalGuaranteeMapDigestBasis {
            schema_version: S0_ARTIFACT_SCHEMA_VERSION,
            artifact_kind: S0ArtifactKind::DeferredPhysicalGuaranteeMap,
            source_revision: &source_revision,
            roadmap_parent_digest: &roadmap_parent_digest,
            generated_by: &generated_by,
            rows: &rows,
        })?;
        let envelope = S0ArtifactEnvelopeMetadata::new(
            S0ArtifactKind::DeferredPhysicalGuaranteeMap,
            source_revision,
            roadmap_parent_digest,
            generated_by,
            deterministic_digest,
            nondeterministic_metadata,
        );
        Ok(Self { envelope, rows })
    }

    pub fn from_milestone_rows(
        source_revision: impl Into<String>,
        roadmap_parent_digest: S0StableDigest,
        generated_by: impl Into<String>,
        nondeterministic_metadata: S0NondeterministicMetadata,
        milestone_rows: &[MilestonePhysicalStatusRow],
    ) -> Result<Self, S0DeferredGuaranteeBuildRejection> {
        let rows = milestone_rows
            .iter()
            .flat_map(rows_for_milestone)
            .collect::<Result<Vec<_>, _>>()?;
        Self::new(
            source_revision,
            roadmap_parent_digest,
            generated_by,
            nondeterministic_metadata,
            rows,
        )
    }

    pub fn envelope(&self) -> &S0ArtifactEnvelopeMetadata {
        &self.envelope
    }

    pub fn rows(&self) -> &[DeferredPhysicalGuaranteeRow] {
        &self.rows
    }

    pub fn to_canonical_json_bytes(&self) -> Result<Vec<u8>, S0DeferredGuaranteeParseRejection> {
        serde_json::to_vec_pretty(self)
            .map_err(|_| S0DeferredGuaranteeParseRejection::SerializationFailed)
    }

    pub fn validate_canonical_json_bytes(
        bytes: &[u8],
    ) -> Result<S0ValidatedDeferredPhysicalGuaranteeMapArtifact, S0DeferredGuaranteeParseRejection>
    {
        let raw = serde_json::from_slice::<RawDeferredPhysicalGuaranteeMap>(bytes)
            .map_err(|_| S0DeferredGuaranteeParseRejection::NonParseable)?;
        if raw.envelope.schema_version != S0_ARTIFACT_SCHEMA_VERSION {
            return Err(S0DeferredGuaranteeParseRejection::SchemaVersionMismatch);
        }
        if raw.envelope.artifact_kind != S0ArtifactKind::DeferredPhysicalGuaranteeMap {
            return Err(S0DeferredGuaranteeParseRejection::ArtifactKindMismatch);
        }
        let roadmap_parent_digest = S0StableDigest::new(raw.envelope.roadmap_parent_digest)
            .map_err(|_| S0DeferredGuaranteeParseRejection::InvalidDigest)?;
        let expected_digest = S0StableDigest::new(raw.envelope.deterministic_digest)
            .map_err(|_| S0DeferredGuaranteeParseRejection::InvalidDigest)?;
        let rows = raw
            .rows
            .into_iter()
            .map(RawDeferredPhysicalGuaranteeRow::into_validated)
            .collect::<Result<Vec<_>, _>>()?;
        let map = Self::new(
            raw.envelope.source_revision,
            roadmap_parent_digest,
            raw.envelope.generated_by,
            raw.envelope.nondeterministic_metadata.into_validated()?,
            rows,
        )?;
        let row_count = map.rows().len() as u64;
        if map.envelope().deterministic_digest() != &expected_digest {
            return Err(S0DeferredGuaranteeParseRejection::DeterministicDigestMismatch);
        }
        let canonicalized_row_byte_count = serde_json::to_vec(map.rows())
            .map_err(|_| S0DeferredGuaranteeParseRejection::SerializationFailed)?
            .len() as u64;
        Ok(S0ValidatedDeferredPhysicalGuaranteeMapArtifact {
            map,
            validation_cost: S0ArtifactValidationCostSurface::new(
                bytes.len() as u64,
                row_count,
                canonicalized_row_byte_count,
                row_count,
            ),
        })
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize)]
pub enum S0DeferredGuaranteeBuildRejection {
    EmptyRequiredField,
    MissingEvidenceRef,
    MissingEvidenceLane,
    DeferredSequenceMissing,
    DuplicateRowId,
    GuaranteeCategorySequenceMismatch,
    GuaranteeAlreadySatisfied,
    DigestConstructionFailed,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize)]
pub enum S0DeferredGuaranteeParseRejection {
    NonParseable,
    SerializationFailed,
    SchemaVersionMismatch,
    ArtifactKindMismatch,
    InvalidDigest,
    InvalidEvidenceRef,
    InvalidDeferredSequence,
    RowBuildRejected(S0DeferredGuaranteeBuildRejection),
    MapBuildRejected(S0DeferredGuaranteeBuildRejection),
    DeterministicDigestMismatch,
}

impl From<S0DeferredGuaranteeBuildRejection> for S0DeferredGuaranteeParseRejection {
    fn from(value: S0DeferredGuaranteeBuildRejection) -> Self {
        Self::MapBuildRejected(value)
    }
}

#[derive(Serialize)]
struct DeferredPhysicalGuaranteeMapDigestBasis<'a> {
    schema_version: &'static str,
    artifact_kind: S0ArtifactKind,
    source_revision: &'a str,
    roadmap_parent_digest: &'a S0StableDigest,
    generated_by: &'a str,
    rows: &'a [DeferredPhysicalGuaranteeRow],
}

#[derive(Deserialize)]
struct RawDeferredPhysicalGuaranteeMap {
    #[serde(flatten)]
    envelope: RawS0ArtifactEnvelope,
    rows: Vec<RawDeferredPhysicalGuaranteeRow>,
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
    fn into_validated(
        self,
    ) -> Result<S0NondeterministicMetadata, S0DeferredGuaranteeParseRejection> {
        S0NondeterministicMetadata::excluded(
            self.generated_at_policy,
            self.local_path_hint,
            self.host_hint,
        )
        .map_err(|_| {
            S0DeferredGuaranteeParseRejection::RowBuildRejected(
                S0DeferredGuaranteeBuildRejection::EmptyRequiredField,
            )
        })
    }
}

#[derive(Deserialize)]
struct RawDeferredPhysicalGuaranteeRow {
    row_id: String,
    subject_kind: S0ArtifactSubjectKind,
    subject_path_or_symbol: String,
    classification: String,
    evidence_refs: Vec<RawS0EvidenceRef>,
    forbidden_claims: Vec<RawBackendForbiddenClaim>,
    deferred_s_sequences: Vec<String>,
    status: S0ArtifactRowStatus,
    notes: String,
    guarantee_category: DeferredPhysicalGuaranteeCategory,
    current_evidence_status: S0PhysicalStatus,
    missing_proof: String,
    dependent_named_suite: String,
    dependent_evidence_lanes: Vec<String>,
}

impl RawDeferredPhysicalGuaranteeRow {
    fn into_validated(
        self,
    ) -> Result<DeferredPhysicalGuaranteeRow, S0DeferredGuaranteeParseRejection> {
        let row_id = S0ArtifactRowId::new(self.row_id).map_err(|_| {
            S0DeferredGuaranteeParseRejection::RowBuildRejected(
                S0DeferredGuaranteeBuildRejection::EmptyRequiredField,
            )
        })?;
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
                    .map_err(|_| S0DeferredGuaranteeParseRejection::InvalidDeferredSequence)
            })
            .collect::<Result<Vec<_>, _>>()?;
        DeferredPhysicalGuaranteeRow::new(
            row_id,
            self.subject_kind,
            self.subject_path_or_symbol,
            self.classification,
            evidence_refs,
            forbidden_claims,
            deferred_s_sequences,
            self.status,
            self.notes,
            self.guarantee_category,
            self.current_evidence_status,
            self.missing_proof,
            self.dependent_named_suite,
            self.dependent_evidence_lanes,
        )
        .map_err(S0DeferredGuaranteeParseRejection::RowBuildRejected)
    }
}

#[derive(Deserialize)]
struct RawS0EvidenceRef {
    artifact_kind: S0ArtifactKind,
    digest: String,
}

impl RawS0EvidenceRef {
    fn into_validated(self) -> Result<S0EvidenceRef, S0DeferredGuaranteeParseRejection> {
        let digest = S0StableDigest::new(self.digest)
            .map_err(|_| S0DeferredGuaranteeParseRejection::InvalidDigest)?;
        Ok(S0EvidenceRef::new(self.artifact_kind, digest))
    }
}

#[derive(Deserialize)]
struct RawBackendForbiddenClaim {
    claim_kind: BackendForbiddenClaimKind,
    deferred_sequence: String,
}

impl RawBackendForbiddenClaim {
    fn into_validated(
        self,
    ) -> Result<super::capability::BackendForbiddenClaim, S0DeferredGuaranteeParseRejection> {
        super::capability::BackendForbiddenClaim::new(self.claim_kind, self.deferred_sequence)
            .map_err(|_| S0DeferredGuaranteeParseRejection::InvalidDeferredSequence)
    }
}

fn rows_for_milestone(
    row: &MilestonePhysicalStatusRow,
) -> impl Iterator<Item = Result<DeferredPhysicalGuaranteeRow, S0DeferredGuaranteeBuildRejection>> + '_
{
    let mut categories = row
        .claim_families()
        .iter()
        .filter_map(|family| deferred_category_from_claim_family(*family, row))
        .collect::<Vec<_>>();
    categories.extend(
        row.forbidden_claims().iter().filter_map(|claim| {
            supplementary_category_from_forbidden_claim_kind(claim.claim_kind())
        }),
    );
    categories.sort();
    categories.dedup();

    categories.into_iter().map(|category| {
        let current_status = current_status_for_category(row, category);
        DeferredPhysicalGuaranteeRow::new(
            guarantee_row_id(row.milestone_id(), category)?,
            S0ArtifactSubjectKind::Milestone,
            row.milestone_id(),
            "deferred-physical-guarantee",
            vec![milestone_evidence_ref(row.milestone_id(), category)],
            row.forbidden_claims().to_vec(),
            row.deferred_s_sequences().to_vec(),
            S0ArtifactRowStatus::Deferred,
            guarantee_notes(row),
            category,
            current_status,
            category.missing_proof_summary(),
            row.named_suite(),
            row.evidence_lanes().to_vec(),
        )
    })
}

fn deferred_category_from_claim_family(
    family: SemanticPhysicalClaimFamily,
    row: &MilestonePhysicalStatusRow,
) -> Option<DeferredPhysicalGuaranteeCategory> {
    let status = row.physical_status_for_claim_family(family);
    if matches!(
        status,
        S0PhysicalStatus::FoundationBacked
            | S0PhysicalStatus::PlatformGrade
            | S0PhysicalStatus::NotApplicable
    ) {
        return None;
    }
    match family {
        SemanticPhysicalClaimFamily::PhysicalSubstrate => {
            Some(DeferredPhysicalGuaranteeCategory::PageSegmentExtentSubstrate)
        }
        SemanticPhysicalClaimFamily::PhysicalBoundedness => {
            Some(DeferredPhysicalGuaranteeCategory::MemoryAllocationBoundedness)
        }
        SemanticPhysicalClaimFamily::PhysicalIntegrity => Some(
            DeferredPhysicalGuaranteeCategory::PageFrameChunkIntegrityAndCorruptionLocalization,
        ),
        SemanticPhysicalClaimFamily::PhysicalRecoveryPhysics => {
            Some(DeferredPhysicalGuaranteeCategory::WalCheckpointLsnRecoveryPhysics)
        }
        SemanticPhysicalClaimFamily::PhysicalIsolation => {
            Some(DeferredPhysicalGuaranteeCategory::PhysicalReadStabilityDuringMaintenance)
        }
        SemanticPhysicalClaimFamily::PhysicalIo => {
            Some(DeferredPhysicalGuaranteeCategory::HardwareAwareIoAndForegroundQos)
        }
        SemanticPhysicalClaimFamily::PhysicalOperationalSafety => {
            Some(DeferredPhysicalGuaranteeCategory::BackupPitrRepairAndForensics)
        }
        SemanticPhysicalClaimFamily::PhysicalSecurity => {
            Some(DeferredPhysicalGuaranteeCategory::SecurityTenantBoundariesKeysAndAuditability)
        }
        _ => None,
    }
}

fn supplementary_category_from_forbidden_claim_kind(
    kind: BackendForbiddenClaimKind,
) -> Option<DeferredPhysicalGuaranteeCategory> {
    match kind {
        BackendForbiddenClaimKind::PlatformGradeDurability
        | BackendForbiddenClaimKind::PhysicalQueryPerformance => {
            Some(DeferredPhysicalGuaranteeCategory::PhysicalDatabaseCertificationAndPerformance)
        }
        BackendForbiddenClaimKind::PlatformGradeRecovery => {
            Some(DeferredPhysicalGuaranteeCategory::WalCheckpointLsnRecoveryPhysics)
        }
        BackendForbiddenClaimKind::PlatformGradeConcurrency => {
            Some(DeferredPhysicalGuaranteeCategory::PhysicalReadStabilityDuringMaintenance)
        }
        BackendForbiddenClaimKind::PlatformGradeMultiTenantIsolation => {
            Some(DeferredPhysicalGuaranteeCategory::SecurityTenantBoundariesKeysAndAuditability)
        }
        BackendForbiddenClaimKind::PhysicalPersistence => {
            Some(DeferredPhysicalGuaranteeCategory::PageSegmentExtentSubstrate)
        }
    }
}

fn current_status_for_category(
    row: &MilestonePhysicalStatusRow,
    category: DeferredPhysicalGuaranteeCategory,
) -> S0PhysicalStatus {
    match category {
        DeferredPhysicalGuaranteeCategory::PageSegmentExtentSubstrate => {
            row.physical_status_for_claim_family(SemanticPhysicalClaimFamily::PhysicalSubstrate)
        }
        DeferredPhysicalGuaranteeCategory::MemoryAllocationBoundedness => {
            row.physical_status_for_claim_family(SemanticPhysicalClaimFamily::PhysicalBoundedness)
        }
        DeferredPhysicalGuaranteeCategory::PageFrameChunkIntegrityAndCorruptionLocalization => {
            row.physical_status_for_claim_family(SemanticPhysicalClaimFamily::PhysicalIntegrity)
        }
        DeferredPhysicalGuaranteeCategory::WalCheckpointLsnRecoveryPhysics => row
            .physical_status_for_claim_family(SemanticPhysicalClaimFamily::PhysicalRecoveryPhysics),
        DeferredPhysicalGuaranteeCategory::PhysicalReadStabilityDuringMaintenance => {
            row.physical_status_for_claim_family(SemanticPhysicalClaimFamily::PhysicalIsolation)
        }
        DeferredPhysicalGuaranteeCategory::HardwareAwareIoAndForegroundQos => {
            row.physical_status_for_claim_family(SemanticPhysicalClaimFamily::PhysicalIo)
        }
        DeferredPhysicalGuaranteeCategory::NativeBlobObjectChunkStore => row
            .native_blob_chunk_status()
            .unwrap_or(S0PhysicalStatus::PhysicalDebt),
        DeferredPhysicalGuaranteeCategory::IndexLayoutAccessPathDiscipline => {
            S0PhysicalStatus::PhysicalDebt
        }
        DeferredPhysicalGuaranteeCategory::FormalCrashConcurrencyModels => {
            S0PhysicalStatus::PhysicalDebt
        }
        DeferredPhysicalGuaranteeCategory::BackupPitrRepairAndForensics => row
            .operator_security_status()
            .unwrap_or(S0PhysicalStatus::PhysicalDebt),
        DeferredPhysicalGuaranteeCategory::SecurityTenantBoundariesKeysAndAuditability => row
            .operator_security_status()
            .unwrap_or(S0PhysicalStatus::PhysicalDebt),
        DeferredPhysicalGuaranteeCategory::PhysicalDatabaseCertificationAndPerformance => {
            S0PhysicalStatus::PhysicalDebt
        }
    }
}

fn guarantee_row_id(
    milestone_id: &str,
    category: DeferredPhysicalGuaranteeCategory,
) -> Result<S0ArtifactRowId, S0DeferredGuaranteeBuildRejection> {
    let milestone = milestone_id.replace('.', "_");
    let category = match category {
        DeferredPhysicalGuaranteeCategory::PageSegmentExtentSubstrate => {
            "PageSegmentExtentSubstrate"
        }
        DeferredPhysicalGuaranteeCategory::MemoryAllocationBoundedness => {
            "MemoryAllocationBoundedness"
        }
        DeferredPhysicalGuaranteeCategory::PageFrameChunkIntegrityAndCorruptionLocalization => {
            "PageFrameChunkIntegrity"
        }
        DeferredPhysicalGuaranteeCategory::WalCheckpointLsnRecoveryPhysics => {
            "WalCheckpointRecoveryPhysics"
        }
        DeferredPhysicalGuaranteeCategory::PhysicalReadStabilityDuringMaintenance => {
            "PhysicalReadStability"
        }
        DeferredPhysicalGuaranteeCategory::HardwareAwareIoAndForegroundQos => "HardwareAwareIoQos",
        DeferredPhysicalGuaranteeCategory::NativeBlobObjectChunkStore => "NativeBlobChunkStore",
        DeferredPhysicalGuaranteeCategory::IndexLayoutAccessPathDiscipline => {
            "IndexLayoutAccessPathDiscipline"
        }
        DeferredPhysicalGuaranteeCategory::FormalCrashConcurrencyModels => {
            "FormalCrashConcurrencyModels"
        }
        DeferredPhysicalGuaranteeCategory::BackupPitrRepairAndForensics => {
            "BackupPitrRepairForensics"
        }
        DeferredPhysicalGuaranteeCategory::SecurityTenantBoundariesKeysAndAuditability => {
            "SecurityTenantKeysAuditability"
        }
        DeferredPhysicalGuaranteeCategory::PhysicalDatabaseCertificationAndPerformance => {
            "PhysicalDatabaseCertification"
        }
    };
    S0ArtifactRowId::new(format!("Milestone{milestone}{category}"))
        .map_err(|_| S0DeferredGuaranteeBuildRejection::EmptyRequiredField)
}

fn guarantee_notes(row: &MilestonePhysicalStatusRow) -> String {
    if row.required_wording_cleanup().is_empty() {
        "S.0 deferred physical guarantee row.".to_string()
    } else {
        format!(
            "S.0 deferred physical guarantee row. Required wording cleanup: {}",
            row.required_wording_cleanup().join("; ")
        )
    }
}

fn milestone_evidence_ref(
    milestone_id: &str,
    category: DeferredPhysicalGuaranteeCategory,
) -> S0EvidenceRef {
    S0EvidenceRef::new(
        S0ArtifactKind::MilestonePhysicalStatusMatrix,
        S0StableDigest::new(format!("deferred:{milestone_id}:{category:?}"))
            .expect("synthetic deferred evidence digest is non-empty"),
    )
}

fn reject_duplicate_rows(
    rows: &[DeferredPhysicalGuaranteeRow],
) -> Result<(), S0DeferredGuaranteeBuildRejection> {
    let mut seen = BTreeSet::new();
    if rows.iter().any(|row| !seen.insert(row.row_id().clone())) {
        return Err(S0DeferredGuaranteeBuildRejection::DuplicateRowId);
    }
    Ok(())
}

fn stable_digest<T: Serialize + ?Sized>(
    value: &T,
) -> Result<S0StableDigest, S0DeferredGuaranteeBuildRejection> {
    let bytes = serde_json::to_vec(value)
        .map_err(|_| S0DeferredGuaranteeBuildRejection::DigestConstructionFailed)?;
    let mut hasher = Sha256::new();
    hasher.update(bytes);
    S0StableDigest::new(format!("{:x}", hasher.finalize()))
        .map_err(|_| S0DeferredGuaranteeBuildRejection::DigestConstructionFailed)
}

fn require_non_empty(
    value: impl Into<String>,
) -> Result<String, S0DeferredGuaranteeBuildRejection> {
    let value = value.into();
    if value.trim().is_empty() {
        return Err(S0DeferredGuaranteeBuildRejection::EmptyRequiredField);
    }
    Ok(value)
}
