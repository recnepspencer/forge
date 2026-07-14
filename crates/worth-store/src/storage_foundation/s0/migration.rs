use super::artifacts::{
    S0ArtifactEnvelopeMetadata, S0ArtifactRowId, S0ArtifactRowStatus, S0ArtifactSubjectKind,
    S0ArtifactValidationCostSurface, S0NondeterministicMetadata, S0_ARTIFACT_SCHEMA_VERSION,
};
use super::claims::SemanticPhysicalClaimStatus;
use super::evidence::{S0ArtifactKind, S0EvidenceRef, S0StableDigest};
use super::milestones::MilestonePhysicalStatusRow;
use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};
use std::collections::BTreeSet;

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct TestMigrationNoteRow {
    row_id: S0ArtifactRowId,
    subject_kind: S0ArtifactSubjectKind,
    subject_path_or_symbol: String,
    classification: String,
    evidence_refs: Vec<S0EvidenceRef>,
    forbidden_claims: Vec<super::capability::BackendForbiddenClaim>,
    deferred_s_sequences: Vec<super::capability::Roadmap2SequenceId>,
    status: S0ArtifactRowStatus,
    notes: String,
    named_suite: String,
    evidence_scope: SemanticPhysicalClaimStatus,
    required_followup_guarantees: Vec<String>,
}

impl TestMigrationNoteRow {
    #[allow(clippy::too_many_arguments)]
    pub fn new(
        row_id: S0ArtifactRowId,
        subject_path_or_symbol: impl Into<String>,
        evidence_refs: Vec<S0EvidenceRef>,
        forbidden_claims: Vec<super::capability::BackendForbiddenClaim>,
        deferred_s_sequences: Vec<super::capability::Roadmap2SequenceId>,
        status: S0ArtifactRowStatus,
        notes: impl Into<String>,
        named_suite: impl Into<String>,
        evidence_scope: SemanticPhysicalClaimStatus,
        required_followup_guarantees: Vec<String>,
    ) -> Result<Self, S0TestMigrationBuildRejection> {
        let subject_path_or_symbol = require_non_empty(subject_path_or_symbol)?;
        let notes = require_non_empty(notes)?;
        let named_suite = require_non_empty(named_suite)?;
        if evidence_refs.is_empty() {
            return Err(S0TestMigrationBuildRejection::MissingEvidenceRef);
        }
        if required_followup_guarantees.is_empty()
            && evidence_scope != SemanticPhysicalClaimStatus::PlatformGrade
        {
            return Err(S0TestMigrationBuildRejection::MissingRequiredFollowupGuarantee);
        }
        Ok(Self {
            row_id,
            subject_kind: S0ArtifactSubjectKind::TestSuite,
            subject_path_or_symbol,
            classification: "test-migration-note".to_string(),
            evidence_refs,
            forbidden_claims,
            deferred_s_sequences,
            status,
            notes,
            named_suite,
            evidence_scope,
            required_followup_guarantees,
        })
    }

    pub fn row_id(&self) -> &S0ArtifactRowId {
        &self.row_id
    }

    pub fn evidence_scope(&self) -> SemanticPhysicalClaimStatus {
        self.evidence_scope
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct TestMigrationNotes {
    #[serde(flatten)]
    envelope: S0ArtifactEnvelopeMetadata,
    rows: Vec<TestMigrationNoteRow>,
}

impl TestMigrationNotes {
    pub fn new(
        source_revision: impl Into<String>,
        roadmap_parent_digest: S0StableDigest,
        generated_by: impl Into<String>,
        nondeterministic_metadata: S0NondeterministicMetadata,
        mut rows: Vec<TestMigrationNoteRow>,
    ) -> Result<Self, S0TestMigrationBuildRejection> {
        let source_revision = require_non_empty(source_revision)?;
        let generated_by = require_non_empty(generated_by)?;
        rows.sort_by(|left, right| left.row_id.cmp(&right.row_id));
        reject_duplicate_rows(&rows)?;
        let deterministic_digest = stable_digest(&TestMigrationNotesDigestBasis {
            schema_version: S0_ARTIFACT_SCHEMA_VERSION,
            artifact_kind: S0ArtifactKind::TestMigrationNotes,
            source_revision: &source_revision,
            roadmap_parent_digest: &roadmap_parent_digest,
            generated_by: &generated_by,
            rows: &rows,
        })?;
        Ok(Self {
            envelope: S0ArtifactEnvelopeMetadata::new(
                S0ArtifactKind::TestMigrationNotes,
                source_revision,
                roadmap_parent_digest,
                generated_by,
                deterministic_digest,
                nondeterministic_metadata,
            ),
            rows,
        })
    }

    pub fn from_milestone_rows(
        source_revision: impl Into<String>,
        roadmap_parent_digest: S0StableDigest,
        generated_by: impl Into<String>,
        nondeterministic_metadata: S0NondeterministicMetadata,
        milestone_rows: &[MilestonePhysicalStatusRow],
    ) -> Result<Self, S0TestMigrationBuildRejection> {
        let rows = milestone_rows
            .iter()
            .map(row_for_milestone)
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

    pub fn rows(&self) -> &[TestMigrationNoteRow] {
        &self.rows
    }

    pub fn to_canonical_json_bytes(&self) -> Result<Vec<u8>, S0TestMigrationParseRejection> {
        serde_json::to_vec_pretty(self)
            .map_err(|_| S0TestMigrationParseRejection::SerializationFailed)
    }

    pub fn validate_canonical_json_bytes(
        bytes: &[u8],
    ) -> Result<S0ValidatedTestMigrationNotesArtifact, S0TestMigrationParseRejection> {
        let raw = serde_json::from_slice::<RawTestMigrationNotes>(bytes)
            .map_err(|_| S0TestMigrationParseRejection::NonParseable)?;
        if raw.envelope.schema_version != S0_ARTIFACT_SCHEMA_VERSION {
            return Err(S0TestMigrationParseRejection::SchemaVersionMismatch);
        }
        if raw.envelope.artifact_kind != S0ArtifactKind::TestMigrationNotes {
            return Err(S0TestMigrationParseRejection::ArtifactKindMismatch);
        }
        let roadmap_parent_digest = S0StableDigest::new(raw.envelope.roadmap_parent_digest)
            .map_err(|_| S0TestMigrationParseRejection::InvalidDigest)?;
        let expected_digest = S0StableDigest::new(raw.envelope.deterministic_digest)
            .map_err(|_| S0TestMigrationParseRejection::InvalidDigest)?;
        let rows = raw
            .rows
            .into_iter()
            .map(RawTestMigrationNoteRow::into_validated)
            .collect::<Result<Vec<_>, _>>()?;
        let report = Self::new(
            raw.envelope.source_revision,
            roadmap_parent_digest,
            raw.envelope.generated_by,
            raw.envelope.nondeterministic_metadata.into_validated()?,
            rows,
        )?;
        let row_count = report.rows().len() as u64;
        if report.envelope().deterministic_digest() != &expected_digest {
            return Err(S0TestMigrationParseRejection::DeterministicDigestMismatch);
        }
        let canonicalized_row_byte_count = serde_json::to_vec(report.rows())
            .map_err(|_| S0TestMigrationParseRejection::SerializationFailed)?
            .len() as u64;
        Ok(S0ValidatedTestMigrationNotesArtifact {
            report,
            validation_cost: S0ArtifactValidationCostSurface::new(
                bytes.len() as u64,
                row_count,
                canonicalized_row_byte_count,
                row_count,
            ),
        })
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct S0ValidatedTestMigrationNotesArtifact {
    report: TestMigrationNotes,
    validation_cost: S0ArtifactValidationCostSurface,
}

impl S0ValidatedTestMigrationNotesArtifact {
    pub fn report(&self) -> &TestMigrationNotes {
        &self.report
    }

    pub fn validation_cost(&self) -> &S0ArtifactValidationCostSurface {
        &self.validation_cost
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize)]
pub enum S0TestMigrationBuildRejection {
    EmptyRequiredField,
    MissingEvidenceRef,
    MissingRequiredFollowupGuarantee,
    DuplicateRowId,
    InvalidDigest,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize)]
pub enum S0TestMigrationParseRejection {
    NonParseable,
    SerializationFailed,
    SchemaVersionMismatch,
    ArtifactKindMismatch,
    InvalidDigest,
    InvalidEvidenceRef,
    InvalidDeferredSequence,
    RowBuildRejected(S0TestMigrationBuildRejection),
    ReportBuildRejected(S0TestMigrationBuildRejection),
    DeterministicDigestMismatch,
}

impl From<S0TestMigrationBuildRejection> for S0TestMigrationParseRejection {
    fn from(value: S0TestMigrationBuildRejection) -> Self {
        Self::ReportBuildRejected(value)
    }
}

#[derive(Serialize)]
struct TestMigrationNotesDigestBasis<'a> {
    schema_version: &'static str,
    artifact_kind: S0ArtifactKind,
    source_revision: &'a str,
    roadmap_parent_digest: &'a S0StableDigest,
    generated_by: &'a str,
    rows: &'a [TestMigrationNoteRow],
}

#[derive(Deserialize)]
struct RawTestMigrationNotes {
    #[serde(flatten)]
    envelope: RawS0ArtifactEnvelope,
    rows: Vec<RawTestMigrationNoteRow>,
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
    fn into_validated(self) -> Result<S0NondeterministicMetadata, S0TestMigrationParseRejection> {
        S0NondeterministicMetadata::excluded(
            self.generated_at_policy,
            self.local_path_hint,
            self.host_hint,
        )
        .map_err(|_| {
            S0TestMigrationParseRejection::RowBuildRejected(
                S0TestMigrationBuildRejection::EmptyRequiredField,
            )
        })
    }
}

#[derive(Deserialize)]
struct RawTestMigrationNoteRow {
    row_id: String,
    subject_path_or_symbol: String,
    evidence_refs: Vec<RawS0EvidenceRef>,
    forbidden_claims: Vec<RawBackendForbiddenClaim>,
    deferred_s_sequences: Vec<String>,
    status: S0ArtifactRowStatus,
    notes: String,
    named_suite: String,
    evidence_scope: SemanticPhysicalClaimStatus,
    required_followup_guarantees: Vec<String>,
}

impl RawTestMigrationNoteRow {
    fn into_validated(self) -> Result<TestMigrationNoteRow, S0TestMigrationParseRejection> {
        let row_id = S0ArtifactRowId::new(self.row_id).map_err(|_| {
            S0TestMigrationParseRejection::RowBuildRejected(
                S0TestMigrationBuildRejection::EmptyRequiredField,
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
                super::capability::Roadmap2SequenceId::new(sequence)
                    .map_err(|_| S0TestMigrationParseRejection::InvalidDeferredSequence)
            })
            .collect::<Result<Vec<_>, _>>()?;
        TestMigrationNoteRow::new(
            row_id,
            self.subject_path_or_symbol,
            evidence_refs,
            forbidden_claims,
            deferred_s_sequences,
            self.status,
            self.notes,
            self.named_suite,
            self.evidence_scope,
            self.required_followup_guarantees,
        )
        .map_err(S0TestMigrationParseRejection::RowBuildRejected)
    }
}

#[derive(Deserialize)]
struct RawS0EvidenceRef {
    artifact_kind: S0ArtifactKind,
    digest: String,
}

impl RawS0EvidenceRef {
    fn into_validated(self) -> Result<S0EvidenceRef, S0TestMigrationParseRejection> {
        let digest = S0StableDigest::new(self.digest)
            .map_err(|_| S0TestMigrationParseRejection::InvalidDigest)?;
        Ok(S0EvidenceRef::new(self.artifact_kind, digest))
    }
}

#[derive(Deserialize)]
struct RawBackendForbiddenClaim {
    claim_kind: super::capability::BackendForbiddenClaimKind,
    deferred_sequence: String,
}

impl RawBackendForbiddenClaim {
    fn into_validated(
        self,
    ) -> Result<super::capability::BackendForbiddenClaim, S0TestMigrationParseRejection> {
        super::capability::BackendForbiddenClaim::new(self.claim_kind, self.deferred_sequence)
            .map_err(|_| S0TestMigrationParseRejection::InvalidDeferredSequence)
    }
}

fn row_for_milestone(
    row: &MilestonePhysicalStatusRow,
) -> Result<TestMigrationNoteRow, S0TestMigrationBuildRejection> {
    let evidence_scope = milestone_scope(row);
    let required_followup_guarantees = row
        .forbidden_claims()
        .iter()
        .map(|claim| format!("{:?}", claim.claim_kind()))
        .collect::<BTreeSet<_>>()
        .into_iter()
        .collect::<Vec<_>>();
    let status = match evidence_scope {
        SemanticPhysicalClaimStatus::FoundationBacked
        | SemanticPhysicalClaimStatus::PlatformGrade => S0ArtifactRowStatus::Admitted,
        _ => S0ArtifactRowStatus::Deferred,
    };
    TestMigrationNoteRow::new(
        migration_row_id(row.milestone_id(), row.named_suite())?,
        row.milestone_id(),
        vec![migration_evidence_ref(
            row.closeout_or_planned_source(),
            row.named_suite(),
        )?],
        row.forbidden_claims().to_vec(),
        row.deferred_s_sequences().to_vec(),
        status,
        format!(
            "{} remains {:?} evidence until deferred Roadmap 2 guarantees close.",
            row.named_suite(),
            evidence_scope
        ),
        row.named_suite(),
        evidence_scope,
        if required_followup_guarantees.is_empty() {
            vec!["no additional followup guarantee required".to_string()]
        } else {
            required_followup_guarantees
        },
    )
}

fn milestone_scope(row: &MilestonePhysicalStatusRow) -> SemanticPhysicalClaimStatus {
    let strongest = row
        .claim_families()
        .iter()
        .map(|family| row.physical_status_for_claim_family(*family))
        .max()
        .unwrap_or(super::milestones::S0PhysicalStatus::SemanticOnly);
    match strongest {
        super::milestones::S0PhysicalStatus::NotApplicable
        | super::milestones::S0PhysicalStatus::NotStarted
        | super::milestones::S0PhysicalStatus::SemanticOnly => {
            SemanticPhysicalClaimStatus::SemanticOnly
        }
        super::milestones::S0PhysicalStatus::BootstrapPhysical => {
            SemanticPhysicalClaimStatus::BootstrapPhysical
        }
        super::milestones::S0PhysicalStatus::PhysicalDebt => {
            SemanticPhysicalClaimStatus::PhysicalDebt
        }
        super::milestones::S0PhysicalStatus::PartiallyFoundationBacked => {
            SemanticPhysicalClaimStatus::PartiallyFoundationBacked
        }
        super::milestones::S0PhysicalStatus::FoundationBacked => {
            SemanticPhysicalClaimStatus::FoundationBacked
        }
        super::milestones::S0PhysicalStatus::PlatformGrade => {
            SemanticPhysicalClaimStatus::PlatformGrade
        }
    }
}

fn migration_row_id(
    milestone_id: &str,
    named_suite: &str,
) -> Result<S0ArtifactRowId, S0TestMigrationBuildRejection> {
    let mut hasher = Sha256::new();
    hasher.update(milestone_id.as_bytes());
    hasher.update(named_suite.as_bytes());
    let digest = format!("{:x}", hasher.finalize());
    S0ArtifactRowId::new(format!("TestMigration{}", &digest[..16]))
        .map_err(|_| S0TestMigrationBuildRejection::EmptyRequiredField)
}

fn migration_evidence_ref(
    closeout_or_planned_source: &str,
    named_suite: &str,
) -> Result<S0EvidenceRef, S0TestMigrationBuildRejection> {
    Ok(S0EvidenceRef::new(
        S0ArtifactKind::MilestonePhysicalStatusMatrix,
        stable_digest(&(closeout_or_planned_source, named_suite))?,
    ))
}

fn reject_duplicate_rows(
    rows: &[TestMigrationNoteRow],
) -> Result<(), S0TestMigrationBuildRejection> {
    let mut seen = BTreeSet::new();
    if rows.iter().any(|row| !seen.insert(row.row_id().clone())) {
        return Err(S0TestMigrationBuildRejection::DuplicateRowId);
    }
    Ok(())
}

fn stable_digest<T: Serialize + ?Sized>(
    value: &T,
) -> Result<S0StableDigest, S0TestMigrationBuildRejection> {
    let bytes =
        serde_json::to_vec(value).map_err(|_| S0TestMigrationBuildRejection::InvalidDigest)?;
    let mut hasher = Sha256::new();
    hasher.update(bytes);
    S0StableDigest::new(format!("{:x}", hasher.finalize()))
        .map_err(|_| S0TestMigrationBuildRejection::InvalidDigest)
}

fn require_non_empty(value: impl Into<String>) -> Result<String, S0TestMigrationBuildRejection> {
    let value = value.into();
    if value.trim().is_empty() {
        return Err(S0TestMigrationBuildRejection::EmptyRequiredField);
    }
    Ok(value)
}
