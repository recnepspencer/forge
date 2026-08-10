use super::super::artifacts::{S0NondeterministicMetadata, S0_ARTIFACT_SCHEMA_VERSION};
use super::super::counters::S0CounterSnapshot;
use super::super::evidence::{S0ArtifactKind, S0ArtifactValidationReport, S0StableDigest};
use super::super::milestones::RoadmapGateReadinessWitness;
use super::certification::S0CertificationMatrixRow;
use super::provenance::{
    S0ArtifactStalenessReport, S0EvidenceProvenance, S0RegenerationRequirement,
};
use super::validation::S0EvidenceBundleParseRejection;
use serde::Deserialize;

pub(super) fn parse_bundle(
    bytes: &[u8],
) -> Result<RawS0EvidenceBundle, S0EvidenceBundleParseRejection> {
    serde_json::from_slice::<RawS0EvidenceBundle>(bytes)
        .map_err(|_| S0EvidenceBundleParseRejection::NonParseable)
}

pub(super) fn ensure_supported_schema_and_kind(
    raw: &RawS0EvidenceBundle,
) -> Result<(), S0EvidenceBundleParseRejection> {
    if raw.envelope.schema_version != S0_ARTIFACT_SCHEMA_VERSION {
        return Err(S0EvidenceBundleParseRejection::SchemaVersionMismatch);
    }
    if raw.envelope.artifact_kind != S0ArtifactKind::S0EvidenceBundle {
        return Err(S0EvidenceBundleParseRejection::ArtifactKindMismatch);
    }
    Ok(())
}

#[derive(Deserialize)]
pub(super) struct RawS0EvidenceBundle {
    #[serde(flatten)]
    pub(super) envelope: RawS0ArtifactEnvelope,
    pub(super) certification_rows: Vec<S0CertificationMatrixRow>,
    pub(super) artifact_validation: S0ArtifactValidationReport,
    pub(super) evidence_provenance: S0EvidenceProvenance,
    pub(super) staleness_report: S0ArtifactStalenessReport,
    pub(super) regeneration_requirement: S0RegenerationRequirement,
    pub(super) accepted_handoff_digest: S0StableDigest,
    pub(super) release_claim_report_digest: S0StableDigest,
    pub(super) complexity_contract_summary_digest: S0StableDigest,
    pub(super) roadmap_gate_readiness: RawRoadmapGateReadinessWitness,
    pub(super) counter_snapshot: S0CounterSnapshot,
    pub(super) failure_digest: S0StableDigest,
}

impl RawS0EvidenceBundle {
    pub(super) fn into_validated_parts(
        self,
    ) -> Result<RawS0EvidenceBundleParts, S0EvidenceBundleParseRejection> {
        let RawS0EvidenceBundle {
            envelope,
            certification_rows,
            artifact_validation,
            evidence_provenance,
            staleness_report,
            regeneration_requirement,
            accepted_handoff_digest,
            release_claim_report_digest,
            complexity_contract_summary_digest,
            roadmap_gate_readiness,
            counter_snapshot,
            failure_digest,
        } = self;
        let RawS0ArtifactEnvelope {
            source_revision,
            roadmap_parent_digest,
            generated_by,
            deterministic_digest,
            nondeterministic_metadata,
            ..
        } = envelope;
        let roadmap_parent_digest = S0StableDigest::new(roadmap_parent_digest)
            .map_err(|_| S0EvidenceBundleParseRejection::InvalidDigest)?;
        let expected_digest = S0StableDigest::new(deterministic_digest)
            .map_err(|_| S0EvidenceBundleParseRejection::InvalidDigest)?;
        let row_count = certification_rows.len() as u64;
        let nondeterministic_metadata = nondeterministic_metadata.into_validated()?;
        let roadmap_gate_readiness = roadmap_gate_readiness.into_validated()?;
        Ok(RawS0EvidenceBundleParts {
            expected_digest,
            row_count,
            source_revision,
            roadmap_parent_digest,
            generated_by,
            nondeterministic_metadata,
            certification_rows,
            artifact_validation,
            evidence_provenance,
            staleness_report,
            regeneration_requirement,
            accepted_handoff_digest,
            release_claim_report_digest,
            complexity_contract_summary_digest,
            roadmap_gate_readiness,
            counter_snapshot,
            failure_digest,
        })
    }
}

pub(super) struct RawS0EvidenceBundleParts {
    pub(super) expected_digest: S0StableDigest,
    pub(super) row_count: u64,
    pub(super) source_revision: String,
    pub(super) roadmap_parent_digest: S0StableDigest,
    pub(super) generated_by: String,
    pub(super) nondeterministic_metadata: S0NondeterministicMetadata,
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

#[derive(Deserialize)]
pub(super) struct RawS0ArtifactEnvelope {
    pub(super) schema_version: String,
    pub(super) artifact_kind: S0ArtifactKind,
    pub(super) source_revision: String,
    pub(super) roadmap_parent_digest: String,
    pub(super) generated_by: String,
    pub(super) deterministic_digest: String,
    pub(super) nondeterministic_metadata: RawS0NondeterministicMetadata,
}

#[derive(Deserialize)]
pub(super) struct RawS0NondeterministicMetadata {
    pub(super) generated_at_policy: String,
    pub(super) local_path_hint: Option<String>,
    pub(super) host_hint: Option<String>,
}

impl RawS0NondeterministicMetadata {
    fn into_validated(self) -> Result<S0NondeterministicMetadata, S0EvidenceBundleParseRejection> {
        S0NondeterministicMetadata::excluded(
            self.generated_at_policy,
            self.local_path_hint,
            self.host_hint,
        )
        .map_err(|_| S0EvidenceBundleParseRejection::InvalidGeneratedMetadata)
    }
}

#[derive(Deserialize)]
pub(super) struct RawRoadmapGateReadinessWitness {
    pub(super) milestone_id: String,
    pub(super) predecessor_evidence_count: u64,
}

impl RawRoadmapGateReadinessWitness {
    fn into_validated(self) -> Result<RoadmapGateReadinessWitness, S0EvidenceBundleParseRejection> {
        if self.milestone_id.trim().is_empty() {
            return Err(S0EvidenceBundleParseRejection::InvalidRoadmapGateReadiness);
        }
        Ok(RoadmapGateReadinessWitness::new(
            self.milestone_id,
            self.predecessor_evidence_count,
        ))
    }
}
