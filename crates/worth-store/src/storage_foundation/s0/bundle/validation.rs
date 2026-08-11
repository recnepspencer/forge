use super::super::artifacts::BackendCapabilityMatrix;
use super::super::artifacts::S0ArtifactEnvelopeMetadata;
use super::super::counters::S0ComplexityContractReport;
use super::super::deferred::DeferredPhysicalGuaranteeMap;
use super::super::evidence::{
    S0ArtifactKind, S0ArtifactSchemaCompatibility, S0CanonicalArtifactSpec, S0StableDigest,
};
use super::super::handoff::StorageFoundationS1Handoff;
use super::super::manifest::S0AuditInputManifest;
use super::super::terminology::TerminologyRiskReport;
use super::digests::complexity_summary_digest;
use serde::Serialize;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize)]
pub enum S0StaleEvidenceRejection {
    SourceRevisionMismatch,
    RoadmapParentDigestMismatch,
    AuditInputManifestDigestMismatch,
    BackendCapabilityMatrixDigestMismatch,
    DeferredGuaranteeMapDigestMismatch,
    TerminologyReportDigestMismatch,
    ComplexitySummaryDigestMismatch,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize)]
pub enum S0EvidenceBundleBuildRejection {
    EmptyRequiredField,
    InvalidRowId,
    MissingEvidenceRef,
    InvalidDigest,
    StaleEvidence(S0StaleEvidenceRejection),
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize)]
pub enum S0EvidenceBundleParseRejection {
    NonParseable,
    SerializationFailed,
    SchemaVersionMismatch,
    ArtifactKindMismatch,
    InvalidDigest,
    InvalidGeneratedMetadata,
    InvalidRoadmapGateReadiness,
    ArtifactValidationMismatch,
    ProvenanceMismatch,
    FailureDigestMismatch,
    DeterministicDigestMismatch,
}

pub(super) fn artifact_spec(
    kind: S0ArtifactKind,
    digest: S0StableDigest,
) -> S0CanonicalArtifactSpec {
    S0CanonicalArtifactSpec::new(kind, digest, S0ArtifactSchemaCompatibility::Compatible)
}

pub(super) fn require_non_empty(
    value: impl Into<String>,
) -> Result<String, S0EvidenceBundleBuildRejection> {
    let value = value.into();
    if value.trim().is_empty() {
        return Err(S0EvidenceBundleBuildRejection::EmptyRequiredField);
    }
    Ok(value)
}

pub(super) struct SharedProvenanceInputs<'a> {
    pub(super) source_revision: &'a str,
    pub(super) roadmap_parent_digest: &'a S0StableDigest,
    pub(super) backend_matrix: &'a S0ArtifactEnvelopeMetadata,
    pub(super) milestone_matrix: &'a S0ArtifactEnvelopeMetadata,
    pub(super) claim_report: &'a S0ArtifactEnvelopeMetadata,
    pub(super) deferred_map: &'a S0ArtifactEnvelopeMetadata,
    pub(super) terminology_report: &'a S0ArtifactEnvelopeMetadata,
    pub(super) migration_notes: &'a S0ArtifactEnvelopeMetadata,
    pub(super) harness_report: &'a S0ArtifactEnvelopeMetadata,
    pub(super) s1_handoff: &'a S0ArtifactEnvelopeMetadata,
    pub(super) manifest: &'a S0AuditInputManifest,
}

pub(super) fn enforce_shared_provenance(
    inputs: SharedProvenanceInputs<'_>,
) -> Result<(), S0EvidenceBundleBuildRejection> {
    let all_match = [
        inputs.backend_matrix,
        inputs.milestone_matrix,
        inputs.claim_report,
        inputs.deferred_map,
        inputs.terminology_report,
        inputs.migration_notes,
        inputs.harness_report,
        inputs.s1_handoff,
    ]
    .into_iter()
    .all(|envelope| {
        envelope.source_revision() == inputs.source_revision
            && envelope.roadmap_parent_digest() == inputs.roadmap_parent_digest
    });
    if !all_match || inputs.manifest.source_revision() != inputs.source_revision {
        return Err(S0EvidenceBundleBuildRejection::StaleEvidence(
            S0StaleEvidenceRejection::SourceRevisionMismatch,
        ));
    }
    Ok(())
}

pub(super) struct StaleHandoffInputs<'a> {
    pub(super) handoff: &'a StorageFoundationS1Handoff,
    pub(super) backend_matrix: &'a BackendCapabilityMatrix,
    pub(super) deferred_map: &'a DeferredPhysicalGuaranteeMap,
    pub(super) terminology_report: &'a TerminologyRiskReport,
    pub(super) manifest: &'a S0AuditInputManifest,
    pub(super) complexity_report: &'a S0ComplexityContractReport,
}

pub(super) fn reject_stale_handoff_inputs(
    inputs: StaleHandoffInputs<'_>,
) -> Result<(), S0EvidenceBundleBuildRejection> {
    if inputs.handoff.backend_tier_matrix_digest()
        != inputs.backend_matrix.envelope().deterministic_digest()
    {
        return Err(S0EvidenceBundleBuildRejection::StaleEvidence(
            S0StaleEvidenceRejection::BackendCapabilityMatrixDigestMismatch,
        ));
    }
    if inputs.handoff.deferred_guarantee_map_digest()
        != inputs.deferred_map.envelope().deterministic_digest()
    {
        return Err(S0EvidenceBundleBuildRejection::StaleEvidence(
            S0StaleEvidenceRejection::DeferredGuaranteeMapDigestMismatch,
        ));
    }
    if inputs.handoff.terminology_scan_digest() != inputs.terminology_report.scan_digest() {
        return Err(S0EvidenceBundleBuildRejection::StaleEvidence(
            S0StaleEvidenceRejection::TerminologyReportDigestMismatch,
        ));
    }
    if inputs.handoff.audit_input_manifest_digest() != inputs.manifest.manifest_digest() {
        return Err(S0EvidenceBundleBuildRejection::StaleEvidence(
            S0StaleEvidenceRejection::AuditInputManifestDigestMismatch,
        ));
    }
    let expected_complexity_digest = complexity_summary_digest(inputs.complexity_report)
        .map_err(|_| S0EvidenceBundleBuildRejection::InvalidDigest)?;
    if inputs.handoff.complexity_contract_summary_digest() != &expected_complexity_digest {
        return Err(S0EvidenceBundleBuildRejection::StaleEvidence(
            S0StaleEvidenceRejection::ComplexitySummaryDigestMismatch,
        ));
    }
    Ok(())
}
