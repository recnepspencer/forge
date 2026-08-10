use super::super::evidence::{S0ArtifactKind, S0StableDigest};
use super::artifact_envelope::{S0ArtifactValidationCostSurface, S0_ARTIFACT_SCHEMA_VERSION};
use super::capability_matrix::{BackendCapabilityMatrix, BackendCapabilityMatrixRow};
use super::first_audit_baseline::S0FirstAuditBaselineRowId;
use serde::Serialize;
use sha2::{Digest, Sha256};
use std::collections::BTreeSet;

#[derive(Debug, Clone, PartialEq, Eq, serde::Serialize)]
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

#[derive(Debug, Clone, Copy, PartialEq, Eq, serde::Serialize)]
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

#[derive(Debug, Clone, Copy, PartialEq, Eq, serde::Serialize)]
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

pub(super) fn backend_capability_matrix_digest(
    source_revision: &str,
    roadmap_parent_digest: &S0StableDigest,
    generated_by: &str,
    rows: &[BackendCapabilityMatrixRow],
) -> Result<S0StableDigest, S0ArtifactBuildRejection> {
    stable_digest(&BackendCapabilityMatrixDigestBasis {
        schema_version: S0_ARTIFACT_SCHEMA_VERSION,
        artifact_kind: S0ArtifactKind::BackendCapabilityMatrix,
        source_revision,
        roadmap_parent_digest,
        generated_by,
        rows,
    })
}

pub(super) fn reject_duplicate_rows(
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

pub(super) fn reject_missing_first_audit_rows(
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

pub(super) fn stable_digest<T: Serialize + ?Sized>(
    value: &T,
) -> Result<S0StableDigest, S0ArtifactBuildRejection> {
    let bytes = serde_json::to_vec(value)
        .map_err(|_| S0ArtifactBuildRejection::DigestConstructionFailed)?;
    let mut hasher = Sha256::new();
    hasher.update(bytes);
    S0StableDigest::new(format!("{:x}", hasher.finalize()))
        .map_err(|_| S0ArtifactBuildRejection::DigestConstructionFailed)
}

pub(super) fn require_non_empty(
    _field: &'static str,
    value: impl Into<String>,
) -> Result<String, S0ArtifactBuildRejection> {
    let value = value.into();
    if value.trim().is_empty() {
        return Err(S0ArtifactBuildRejection::EmptyRequiredField);
    }
    Ok(value)
}
