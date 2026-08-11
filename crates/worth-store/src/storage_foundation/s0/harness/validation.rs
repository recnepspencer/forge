use super::super::artifacts::S0ArtifactRowId;
use super::maturity::HarnessSubsystemMaturity;
use super::row::HarnessMaturityRow;
use serde::Serialize;
use sha2::{Digest, Sha256};
use std::collections::BTreeSet;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize)]
pub enum S0HarnessMaturityBuildRejection {
    EmptyRequiredField,
    MissingEvidenceRef,
    MissingRequiredSequence,
    DuplicateRowId,
    MissingRequiredHarnessSubsystem,
    InvalidDigest,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize)]
pub enum S0HarnessMaturityParseRejection {
    NonParseable,
    SerializationFailed,
    SchemaVersionMismatch,
    ArtifactKindMismatch,
    InvalidDigest,
    InvalidEvidenceRef,
    InvalidDeferredSequence,
    RowBuildRejected(S0HarnessMaturityBuildRejection),
    ReportBuildRejected(S0HarnessMaturityBuildRejection),
    DeterministicDigestMismatch,
}

impl From<S0HarnessMaturityBuildRejection> for S0HarnessMaturityParseRejection {
    fn from(value: S0HarnessMaturityBuildRejection) -> Self {
        Self::ReportBuildRejected(value)
    }
}

pub(super) fn require_non_empty(value: impl Into<String>) -> Result<String, String> {
    let value = value.into();
    if value.trim().is_empty() {
        return Err(value);
    }
    Ok(value)
}

pub(super) fn ensure_required_harness_subsystems(
    rows: &[HarnessMaturityRow],
) -> Result<(), S0HarnessMaturityBuildRejection> {
    let present = rows
        .iter()
        .map(|row| row.subsystem())
        .collect::<BTreeSet<_>>();
    for required in [
        HarnessSubsystemMaturity::TerminologyClaimGate,
        HarnessSubsystemMaturity::BackendTierFenceEnforcement,
        HarnessSubsystemMaturity::DeferredGuaranteeValidation,
        HarnessSubsystemMaturity::MilestoneStatusCompleteness,
        HarnessSubsystemMaturity::CompileTimeBoundaryFixtures,
        HarnessSubsystemMaturity::StaleHandoffRejection,
    ] {
        if !present.contains(&required) {
            return Err(S0HarnessMaturityBuildRejection::MissingRequiredHarnessSubsystem);
        }
    }
    Ok(())
}

pub(super) fn reject_duplicate_rows(
    rows: &[HarnessMaturityRow],
) -> Result<(), S0HarnessMaturityBuildRejection> {
    let mut seen = BTreeSet::new();
    if rows.iter().any(|row| !seen.insert(row.row_id().clone())) {
        return Err(S0HarnessMaturityBuildRejection::DuplicateRowId);
    }
    Ok(())
}

pub(super) fn harness_row_id(
    label: &str,
) -> Result<S0ArtifactRowId, S0HarnessMaturityBuildRejection> {
    let mut hasher = Sha256::new();
    hasher.update(label.as_bytes());
    let digest = format!("{:x}", hasher.finalize());
    S0ArtifactRowId::new(format!("Harness{}", &digest[..16]))
        .map_err(|_| S0HarnessMaturityBuildRejection::EmptyRequiredField)
}
