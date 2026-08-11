use super::super::evidence::{S0ArtifactKind, S0StableDigest};
use super::deferred_guarantee_row::DeferredPhysicalGuaranteeRow;
use serde::Serialize;
use sha2::{Digest, Sha256};
use std::collections::BTreeSet;

#[derive(Debug, Clone, Copy, PartialEq, Eq, serde::Serialize)]
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

#[derive(Debug, Clone, Copy, PartialEq, Eq, serde::Serialize)]
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

pub(super) fn reject_duplicate_rows(
    rows: &[DeferredPhysicalGuaranteeRow],
) -> Result<(), S0DeferredGuaranteeBuildRejection> {
    let mut seen = BTreeSet::new();
    if rows.iter().any(|row| !seen.insert(row.row_id().clone())) {
        return Err(S0DeferredGuaranteeBuildRejection::DuplicateRowId);
    }
    Ok(())
}

pub(super) fn map_digest(
    source_revision: &str,
    roadmap_parent_digest: &S0StableDigest,
    generated_by: &str,
    rows: &[DeferredPhysicalGuaranteeRow],
) -> Result<S0StableDigest, S0DeferredGuaranteeBuildRejection> {
    stable_digest(&DeferredPhysicalGuaranteeMapDigestBasis {
        schema_version: super::super::artifacts::S0_ARTIFACT_SCHEMA_VERSION,
        artifact_kind: S0ArtifactKind::DeferredPhysicalGuaranteeMap,
        source_revision,
        roadmap_parent_digest,
        generated_by,
        rows,
    })
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

pub(super) fn stable_digest<T: Serialize + ?Sized>(
    value: &T,
) -> Result<S0StableDigest, S0DeferredGuaranteeBuildRejection> {
    let bytes = serde_json::to_vec(value)
        .map_err(|_| S0DeferredGuaranteeBuildRejection::DigestConstructionFailed)?;
    let mut hasher = Sha256::new();
    hasher.update(bytes);
    S0StableDigest::new(format!("{:x}", hasher.finalize()))
        .map_err(|_| S0DeferredGuaranteeBuildRejection::DigestConstructionFailed)
}

pub(super) fn require_non_empty(
    value: impl Into<String>,
) -> Result<String, S0DeferredGuaranteeBuildRejection> {
    let value = value.into();
    if value.trim().is_empty() {
        return Err(S0DeferredGuaranteeBuildRejection::EmptyRequiredField);
    }
    Ok(value)
}
