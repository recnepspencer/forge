use super::super::artifacts::S0ArtifactRowId;
use super::super::evidence::{S0ArtifactKind, S0EvidenceRef, S0StableDigest};
use super::test_migration_note_row::TestMigrationNoteRow;
use serde::Serialize;
use sha2::{Digest, Sha256};
use std::collections::BTreeSet;

#[derive(Debug, Clone, Copy, PartialEq, Eq, serde::Serialize)]
pub enum S0TestMigrationBuildRejection {
    EmptyRequiredField,
    MissingEvidenceRef,
    MissingRequiredFollowupGuarantee,
    DuplicateRowId,
    InvalidDigest,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, serde::Serialize)]
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

pub(super) fn reject_duplicate_rows(
    rows: &[TestMigrationNoteRow],
) -> Result<(), S0TestMigrationBuildRejection> {
    let mut seen = BTreeSet::new();
    if rows.iter().any(|row| !seen.insert(row.row_id().clone())) {
        return Err(S0TestMigrationBuildRejection::DuplicateRowId);
    }
    Ok(())
}

pub(super) fn test_migration_notes_digest(
    source_revision: &str,
    roadmap_parent_digest: &S0StableDigest,
    generated_by: &str,
    rows: &[TestMigrationNoteRow],
) -> Result<S0StableDigest, S0TestMigrationBuildRejection> {
    stable_digest(&TestMigrationNotesDigestBasis {
        schema_version: super::super::artifacts::S0_ARTIFACT_SCHEMA_VERSION,
        artifact_kind: S0ArtifactKind::TestMigrationNotes,
        source_revision,
        roadmap_parent_digest,
        generated_by,
        rows,
    })
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

pub(super) fn stable_digest<T: Serialize + ?Sized>(
    value: &T,
) -> Result<S0StableDigest, S0TestMigrationBuildRejection> {
    let bytes =
        serde_json::to_vec(value).map_err(|_| S0TestMigrationBuildRejection::InvalidDigest)?;
    let mut hasher = Sha256::new();
    hasher.update(bytes);
    S0StableDigest::new(format!("{:x}", hasher.finalize()))
        .map_err(|_| S0TestMigrationBuildRejection::InvalidDigest)
}

pub(super) fn require_non_empty(
    value: impl Into<String>,
) -> Result<String, S0TestMigrationBuildRejection> {
    let value = value.into();
    if value.trim().is_empty() {
        return Err(S0TestMigrationBuildRejection::EmptyRequiredField);
    }
    Ok(value)
}

pub(super) fn migration_row_id(
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

pub(super) fn migration_evidence_ref(
    closeout_or_planned_source: &str,
    named_suite: &str,
) -> Result<S0EvidenceRef, S0TestMigrationBuildRejection> {
    Ok(S0EvidenceRef::new(
        S0ArtifactKind::MilestonePhysicalStatusMatrix,
        stable_digest(&(closeout_or_planned_source, named_suite))?,
    ))
}
