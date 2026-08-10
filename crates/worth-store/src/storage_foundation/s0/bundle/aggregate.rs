use super::super::artifacts::S0ArtifactEnvelopeMetadata;
use super::super::counters::S0CounterSnapshot;
use super::super::evidence::{S0ArtifactValidationReport, S0StableDigest};
use super::super::milestones::RoadmapGateReadinessWitness;
use super::certification::S0CertificationMatrixRow;
use super::provenance::{
    S0ArtifactStalenessReport, S0EvidenceProvenance, S0RegenerationRequirement,
};
use serde::Serialize;

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct S0EvidenceBundle {
    #[serde(flatten)]
    pub(super) envelope: S0ArtifactEnvelopeMetadata,
    pub(super) certification_rows: Vec<S0CertificationMatrixRow>,
    pub(super) artifact_validation: S0ArtifactValidationReport,
    pub(super) evidence_provenance: S0EvidenceProvenance,
    pub(super) staleness_report: S0ArtifactStalenessReport,
    pub(super) regeneration_requirement: S0RegenerationRequirement,
    pub(super) accepted_handoff_digest: S0StableDigest,
    pub(super) release_claim_report_digest: S0StableDigest,
    pub(super) complexity_contract_summary_digest: S0StableDigest,
    pub(super) roadmap_gate_readiness: RoadmapGateReadinessWitness,
    pub(super) counter_snapshot: S0CounterSnapshot,
    pub(super) failure_digest: S0StableDigest,
}
