use super::super::counters::{S0ComplexityContractReport, S0CounterSnapshot};
use super::super::evidence::{S0ArtifactKind, S0ArtifactValidationReport, S0StableDigest};
use super::super::milestones::RoadmapGateReadinessWitness;
use super::certification::{S0CertificationMatrixRow, S0CertificationStatus};
use super::provenance::{
    S0ArtifactStalenessReport, S0EvidenceProvenance, S0RegenerationRequirement,
};
use serde::Serialize;
use sha2::{Digest, Sha256};

#[derive(Serialize)]
pub(super) struct S0EvidenceBundleDigestBasis<'a> {
    pub(super) schema_version: &'static str,
    pub(super) artifact_kind: S0ArtifactKind,
    pub(super) source_revision: &'a str,
    pub(super) roadmap_parent_digest: &'a S0StableDigest,
    pub(super) generated_by: &'a str,
    pub(super) certification_rows: &'a [S0CertificationMatrixRow],
    pub(super) artifact_validation: &'a S0ArtifactValidationReport,
    pub(super) evidence_provenance: &'a S0EvidenceProvenance,
    pub(super) staleness_report: &'a S0ArtifactStalenessReport,
    pub(super) regeneration_requirement: &'a S0RegenerationRequirement,
    pub(super) accepted_handoff_digest: &'a S0StableDigest,
    pub(super) release_claim_report_digest: &'a S0StableDigest,
    pub(super) complexity_contract_summary_digest: &'a S0StableDigest,
    pub(super) roadmap_gate_readiness: &'a RoadmapGateReadinessWitness,
    pub(super) counter_snapshot: &'a S0CounterSnapshot,
    pub(super) failure_digest: &'a S0StableDigest,
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

pub(super) fn failure_digest(
    rows: &[S0CertificationMatrixRow],
) -> Result<S0StableDigest, serde_json::Error> {
    let failures = rows
        .iter()
        .filter(|row| row.certification_status() == S0CertificationStatus::Blocking)
        .map(|row| row.row_id.as_str())
        .collect::<Vec<_>>();
    stable_digest(&failures)
}

pub(super) fn complexity_summary_digest(
    report: &S0ComplexityContractReport,
) -> Result<S0StableDigest, serde_json::Error> {
    let value = serde_json::to_vec(&(
        report.required_contract_count(),
        report.observed_contract_count(),
        report.missing_complexity_contract_count(),
        report.duplicate_complexity_contract_count(),
        report.complexity_debt_count(),
        report.max_global_scans(),
        report.max_unindexed_repo_passes(),
    ))?;
    let mut hasher = Sha256::new();
    hasher.update(value);
    S0StableDigest::new(format!("{:x}", hasher.finalize())).map_err(|_| {
        serde_json::Error::io(std::io::Error::new(
            std::io::ErrorKind::InvalidData,
            "invalid digest",
        ))
    })
}
