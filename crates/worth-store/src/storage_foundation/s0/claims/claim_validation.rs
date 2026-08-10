use super::super::evidence::{S0ArtifactKind, S0StableDigest};
use super::claim_report_row::SemanticPhysicalClaimReportRow;
use serde::Serialize;
use sha2::{Digest, Sha256};
use std::collections::BTreeSet;

#[derive(Debug, Clone, Copy, PartialEq, Eq, serde::Serialize)]
pub enum S0ClaimReportBuildRejection {
    EmptyRequiredField,
    MissingEvidenceRef,
    MissingEvidenceLane,
    DeferredSequenceMissing,
    DuplicateRowId,
    DigestConstructionFailed,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, serde::Serialize)]
pub enum S0ClaimReportParseRejection {
    NonParseable,
    SerializationFailed,
    SchemaVersionMismatch,
    ArtifactKindMismatch,
    InvalidDigest,
    InvalidEvidenceRef,
    InvalidDeferredSequence,
    RowBuildRejected(S0ClaimReportBuildRejection),
    ReportBuildRejected(S0ClaimReportBuildRejection),
    DeterministicDigestMismatch,
}

impl From<S0ClaimReportBuildRejection> for S0ClaimReportParseRejection {
    fn from(value: S0ClaimReportBuildRejection) -> Self {
        Self::ReportBuildRejected(value)
    }
}

pub(super) fn reject_duplicate_rows(
    rows: &[SemanticPhysicalClaimReportRow],
) -> Result<(), S0ClaimReportBuildRejection> {
    let mut seen = BTreeSet::new();
    if rows.iter().any(|row| !seen.insert(row.row_id().clone())) {
        return Err(S0ClaimReportBuildRejection::DuplicateRowId);
    }
    Ok(())
}

pub(super) fn report_digest(
    source_revision: &str,
    roadmap_parent_digest: &S0StableDigest,
    generated_by: &str,
    rows: &[SemanticPhysicalClaimReportRow],
) -> Result<S0StableDigest, S0ClaimReportBuildRejection> {
    stable_digest(&SemanticPhysicalClaimReportDigestBasis {
        schema_version: super::super::artifacts::S0_ARTIFACT_SCHEMA_VERSION,
        artifact_kind: S0ArtifactKind::SemanticPhysicalClaimReport,
        source_revision,
        roadmap_parent_digest,
        generated_by,
        rows,
    })
}

#[derive(Serialize)]
struct SemanticPhysicalClaimReportDigestBasis<'a> {
    schema_version: &'static str,
    artifact_kind: S0ArtifactKind,
    source_revision: &'a str,
    roadmap_parent_digest: &'a S0StableDigest,
    generated_by: &'a str,
    rows: &'a [SemanticPhysicalClaimReportRow],
}

pub(super) fn stable_digest<T: Serialize + ?Sized>(
    value: &T,
) -> Result<S0StableDigest, S0ClaimReportBuildRejection> {
    let bytes = serde_json::to_vec(value)
        .map_err(|_| S0ClaimReportBuildRejection::DigestConstructionFailed)?;
    let mut hasher = Sha256::new();
    hasher.update(bytes);
    S0StableDigest::new(format!("{:x}", hasher.finalize()))
        .map_err(|_| S0ClaimReportBuildRejection::DigestConstructionFailed)
}

pub(super) fn require_non_empty(
    value: impl Into<String>,
) -> Result<String, S0ClaimReportBuildRejection> {
    let value = value.into();
    if value.trim().is_empty() {
        return Err(S0ClaimReportBuildRejection::EmptyRequiredField);
    }
    Ok(value)
}
