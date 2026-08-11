use super::super::evidence::{S0ArtifactKind, S0EvidenceRef, S0StableDigest};
use super::phrase_finding::TerminologyPhraseFinding;
use serde::Serialize;
use sha2::{Digest, Sha256};
use std::collections::BTreeSet;

#[derive(Debug, Clone, Copy, PartialEq, Eq, serde::Serialize)]
pub enum TerminologyCleanupRejection {
    EmptyRequiredField,
    AbsolutePath,
    ParentTraversal,
    MissingScanScope,
    DuplicateScanScope,
    DuplicateAllowlistEntry,
    RejectedWorkspaceGlobalScope,
    MissingEvidenceRef,
    InvalidLineNumber,
    QualifiedPhysicalDebtMissingSequence,
    QualifierAppliedToNonRiskPhrase,
    DuplicateScanInput,
    InputOutsideDeclaredScanScope,
    InputOutsideManifest,
    UnclassifiedPhraseFinding,
    SerializationFailed,
    NonParseable,
    SchemaVersionMismatch,
    ArtifactKindMismatch,
    InvalidDigest,
    InvalidEvidenceRef,
    InvalidDeferredSequence,
    DuplicateRowId,
    MissingReleaseSurface,
    DuplicateReleaseSurface,
    UnscannedReleaseSurface,
    DeterministicDigestMismatch,
}

pub(super) fn reject_duplicate_rows(
    rows: &[TerminologyPhraseFinding],
) -> Result<(), TerminologyCleanupRejection> {
    let mut seen = BTreeSet::new();
    if rows.iter().any(|row| !seen.insert(row.row_id().clone())) {
        return Err(TerminologyCleanupRejection::DuplicateRowId);
    }
    Ok(())
}

pub(super) fn terminology_report_digest(
    basis: &TerminologyRiskReportDigestBasis<'_>,
) -> Result<S0StableDigest, TerminologyCleanupRejection> {
    stable_digest(basis)
}

#[derive(Serialize)]
pub(super) struct TerminologyRiskReportDigestBasis<'a> {
    pub(super) schema_version: &'static str,
    pub(super) artifact_kind: S0ArtifactKind,
    pub(super) source_revision: &'a str,
    pub(super) roadmap_parent_digest: &'a S0StableDigest,
    pub(super) generated_by: &'a str,
    pub(super) scan_digest: &'a S0StableDigest,
    pub(super) rows: &'a [TerminologyPhraseFinding],
}

pub(super) fn stable_digest<T: Serialize + ?Sized>(
    value: &T,
) -> Result<S0StableDigest, TerminologyCleanupRejection> {
    let bytes =
        serde_json::to_vec(value).map_err(|_| TerminologyCleanupRejection::SerializationFailed)?;
    let mut hasher = Sha256::new();
    hasher.update(bytes);
    S0StableDigest::new(format!("{:x}", hasher.finalize()))
        .map_err(|_| TerminologyCleanupRejection::InvalidDigest)
}

pub(super) fn require_non_empty(
    value: impl Into<String>,
) -> Result<String, TerminologyCleanupRejection> {
    let value = value.into();
    if value.trim().is_empty() {
        return Err(TerminologyCleanupRejection::EmptyRequiredField);
    }
    Ok(value)
}

pub(super) fn normalize_relative_path(
    value: impl Into<String>,
) -> Result<String, TerminologyCleanupRejection> {
    let normalized = require_non_empty(value)?.replace('\\', "/");
    if normalized.starts_with('/') || normalized.contains(":/") {
        return Err(TerminologyCleanupRejection::AbsolutePath);
    }
    if normalized.split('/').any(|part| part == "..") {
        return Err(TerminologyCleanupRejection::ParentTraversal);
    }
    Ok(normalized.trim_matches('/').to_string())
}

pub(super) fn path_is_under_scope(path: &str, scope: &str) -> bool {
    path == scope
        || path
            .strip_prefix(scope)
            .is_some_and(|suffix| suffix.starts_with('/'))
}

pub(super) fn finding_row_id(
    path: &str,
    line_number: u64,
    phrase: &str,
) -> Result<super::super::artifacts::S0ArtifactRowId, TerminologyCleanupRejection> {
    let mut hasher = Sha256::new();
    hasher.update(path.as_bytes());
    hasher.update(line_number.to_le_bytes());
    hasher.update(phrase.as_bytes());
    let digest = format!("{:x}", hasher.finalize());
    super::super::artifacts::S0ArtifactRowId::new(format!("TerminologyFinding{}", &digest[..16]))
        .map_err(|_| TerminologyCleanupRejection::EmptyRequiredField)
}

pub(super) fn terminology_evidence_ref(
    path: &str,
    line_number: u64,
    phrase: &str,
) -> S0EvidenceRef {
    let digest = stable_digest(&(path, line_number, phrase))
        .expect("terminology evidence digest basis must serialize");
    S0EvidenceRef::new(S0ArtifactKind::TerminologyRiskReport, digest)
}
