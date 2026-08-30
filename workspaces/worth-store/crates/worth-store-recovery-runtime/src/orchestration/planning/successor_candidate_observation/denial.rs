use worth_store_physical_format::RecordArtifactFile;

use super::artifact_generation;
use crate::entry::PhysicalRecoverySuccessorCandidateDenial;
use crate::orchestration::planning::manifest_entry_budget::ManifestEntryBudget;

pub(super) const fn invalid(
    artifact: RecordArtifactFile,
) -> PhysicalRecoverySuccessorCandidateDenial {
    PhysicalRecoverySuccessorCandidateDenial::InvalidArtifact {
        artifact,
        generation: artifact_generation(artifact),
    }
}

const fn limit(
    artifact: RecordArtifactFile,
    observed: u64,
    admitted: u64,
) -> PhysicalRecoverySuccessorCandidateDenial {
    PhysicalRecoverySuccessorCandidateDenial::ManifestEntryLimit {
        artifact,
        generation: artifact_generation(artifact),
        observed,
        admitted,
    }
}

pub(super) fn admit_successor_read(
    budget: &ManifestEntryBudget,
    artifact: RecordArtifactFile,
) -> Result<(), PhysicalRecoverySuccessorCandidateDenial> {
    budget
        .successor_read_evidence()
        .map_err(|(observed, admitted)| limit(artifact, observed, admitted))
}

pub(super) fn consume_successor(
    budget: &mut ManifestEntryBudget,
    entries: usize,
    artifact: RecordArtifactFile,
) -> Result<(), PhysicalRecoverySuccessorCandidateDenial> {
    budget
        .consume_with_evidence(entries)
        .map_err(|(observed, admitted)| limit(artifact, observed, admitted))
}
