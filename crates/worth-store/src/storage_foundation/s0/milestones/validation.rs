use super::super::artifacts::S0ArtifactValidationCostSurface;
use super::super::evidence::S0StableDigest;
use super::physical_matrix::MilestonePhysicalStatusMatrix;
use super::physical_status::{MilestonePhysicalStatusRow, S0PhysicalStatus};
use super::sequence_status::{MilestoneStatusDeclaration, RoadmapGateReadinessWitness};
use serde::Serialize;
use sha2::{Digest, Sha256};
use std::collections::BTreeSet;

#[derive(Debug, Clone, PartialEq, Eq, serde::Serialize)]
pub struct S0ValidatedMilestonePhysicalStatusMatrixArtifact {
    matrix: MilestonePhysicalStatusMatrix,
    validation_cost: S0ArtifactValidationCostSurface,
}

impl S0ValidatedMilestonePhysicalStatusMatrixArtifact {
    pub fn matrix(&self) -> &MilestonePhysicalStatusMatrix {
        &self.matrix
    }

    pub fn validation_cost(&self) -> &S0ArtifactValidationCostSurface {
        &self.validation_cost
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, serde::Serialize)]
pub enum S0MilestoneAuditRejection {
    EmptyRequiredField,
    MissingMilestoneDeclaration,
    DuplicateMilestoneDeclaration,
    UnknownMilestoneReference,
    MissingGatePredecessorEvidence,
    GateReadinessBlockedBySequenceInconsistency,
    MissingEvidenceLane,
    MissingClaimFamily,
    PlatformGradeStatusRequiresGateReadiness,
    PhysicalDebtRequiresDeferredSequence,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, serde::Serialize)]
pub enum S0MilestoneMatrixBuildRejection {
    EmptyRequiredField,
    MissingMilestoneRow,
    DuplicateMilestoneRow,
    UnknownMilestoneReference,
    MissingRequiredMilestoneRow,
    InvalidDigest,
    InvalidEvidenceRef,
    InvalidDeferredSequence,
    InvalidForbiddenClaim,
    SequenceRejected(S0MilestoneAuditRejection),
    RowRejected(S0MilestoneAuditRejection),
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, serde::Serialize)]
pub enum S0MilestoneMatrixParseRejection {
    NonParseable,
    SerializationFailed,
    SchemaVersionMismatch,
    ArtifactKindMismatch,
    InvalidDigest,
    InvalidEvidenceRef,
    InvalidDeferredSequence,
    InvalidForbiddenClaim,
    InvalidGeneratedMetadata,
    SequenceRejected(S0MilestoneAuditRejection),
    RowRejected(S0MilestoneAuditRejection),
    MatrixRejected(S0MilestoneMatrixBuildRejection),
    DeterministicDigestMismatch,
}

impl From<S0MilestoneAuditRejection> for S0MilestoneMatrixParseRejection {
    fn from(value: S0MilestoneAuditRejection) -> Self {
        Self::SequenceRejected(value)
    }
}

impl From<S0MilestoneMatrixBuildRejection> for S0MilestoneMatrixParseRejection {
    fn from(value: S0MilestoneMatrixBuildRejection) -> Self {
        Self::MatrixRejected(value)
    }
}

pub(super) fn reject_duplicate_declarations(
    declarations: &[MilestoneStatusDeclaration],
) -> Result<(), S0MilestoneAuditRejection> {
    let mut seen = BTreeSet::new();
    if declarations
        .iter()
        .any(|declaration| !seen.insert(declaration.milestone_id()))
    {
        return Err(S0MilestoneAuditRejection::DuplicateMilestoneDeclaration);
    }
    Ok(())
}

pub(super) fn reject_duplicate_milestone_rows(
    rows: &[MilestonePhysicalStatusRow],
) -> Result<(), S0MilestoneMatrixBuildRejection> {
    let mut seen = BTreeSet::new();
    if rows
        .iter()
        .any(|row| !seen.insert(row.milestone_id().to_string()))
    {
        return Err(S0MilestoneMatrixBuildRejection::DuplicateMilestoneRow);
    }
    Ok(())
}

pub(super) fn reject_unknown_matrix_rows(
    sequence: &super::sequence_status::RoadmapSequenceStatusMatrix,
    rows: &[MilestonePhysicalStatusRow],
) -> Result<(), S0MilestoneMatrixBuildRejection> {
    let declared = sequence
        .declarations()
        .iter()
        .map(|declaration| declaration.milestone_id())
        .collect::<BTreeSet<_>>();
    if rows
        .iter()
        .any(|row| !declared.contains(row.milestone_id()))
    {
        return Err(S0MilestoneMatrixBuildRejection::UnknownMilestoneReference);
    }
    Ok(())
}

pub(super) fn reject_missing_required_milestone_rows(
    required_milestone_ids: &[String],
    rows: &[MilestonePhysicalStatusRow],
) -> Result<(), S0MilestoneMatrixBuildRejection> {
    let present = rows
        .iter()
        .map(|row| row.milestone_id())
        .collect::<BTreeSet<_>>();
    if required_milestone_ids
        .iter()
        .any(|milestone_id| !present.contains(milestone_id.as_str()))
    {
        return Err(S0MilestoneMatrixBuildRejection::MissingRequiredMilestoneRow);
    }
    Ok(())
}

pub(super) fn any_status_is_platform_grade(statuses: [S0PhysicalStatus; 5]) -> bool {
    statuses
        .into_iter()
        .any(|status| status == S0PhysicalStatus::PlatformGrade)
}

pub(super) fn any_status_is_physical_debt(statuses: [S0PhysicalStatus; 5]) -> bool {
    statuses
        .into_iter()
        .any(|status| status == S0PhysicalStatus::PhysicalDebt)
}

pub(super) fn validate_platform_grade_status(
    milestone_id: &str,
    statuses: [S0PhysicalStatus; 5],
    native_blob_chunk_status: Option<S0PhysicalStatus>,
    operator_security_status: Option<S0PhysicalStatus>,
    roadmap_gate: Option<&RoadmapGateReadinessWitness>,
) -> Result<(), S0MilestoneAuditRejection> {
    if any_status_is_platform_grade(statuses)
        || native_blob_chunk_status == Some(S0PhysicalStatus::PlatformGrade)
        || operator_security_status == Some(S0PhysicalStatus::PlatformGrade)
    {
        match roadmap_gate {
            Some(witness) if witness.milestone_id() == milestone_id => {}
            _ => return Err(S0MilestoneAuditRejection::PlatformGradeStatusRequiresGateReadiness),
        }
    }
    Ok(())
}

pub(super) fn validate_physical_debt_status(
    statuses: [S0PhysicalStatus; 5],
    native_blob_chunk_status: Option<S0PhysicalStatus>,
    operator_security_status: Option<S0PhysicalStatus>,
    deferred_s_sequences_empty: bool,
) -> Result<(), S0MilestoneAuditRejection> {
    if any_status_is_physical_debt(statuses)
        || native_blob_chunk_status == Some(S0PhysicalStatus::PhysicalDebt)
        || operator_security_status == Some(S0PhysicalStatus::PhysicalDebt)
    {
        if deferred_s_sequences_empty {
            return Err(S0MilestoneAuditRejection::PhysicalDebtRequiresDeferredSequence);
        }
    }
    Ok(())
}

pub(super) fn require_non_empty(
    value: impl Into<String>,
) -> Result<String, S0MilestoneAuditRejection> {
    let value = value.into();
    if value.trim().is_empty() {
        return Err(S0MilestoneAuditRejection::EmptyRequiredField);
    }
    Ok(value)
}

pub(super) fn stable_digest(value: &impl Serialize) -> Result<S0StableDigest, serde_json::Error> {
    let canonical = serde_json::to_vec(value)?;
    let mut hasher = Sha256::new();
    hasher.update(&canonical);
    let digest = format!("sha256:{:x}", hasher.finalize());
    S0StableDigest::new(digest).map_err(|_| {
        serde_json::Error::io(std::io::Error::new(
            std::io::ErrorKind::InvalidData,
            "invalid digest",
        ))
    })
}
