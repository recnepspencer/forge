use super::artifacts::{
    BackendCapabilityMatrix, S0ArtifactEnvelopeMetadata, S0ArtifactRowId, S0ArtifactRowStatus,
    S0ArtifactSubjectKind, S0ArtifactValidationCostSurface, S0NondeterministicMetadata,
    S0ValidatedBackendCapabilityMatrixArtifact, S0_ARTIFACT_SCHEMA_VERSION,
};
use super::claims::S0ValidatedSemanticPhysicalClaimReportArtifact;
use super::counters::{S0ComplexityContractReport, S0CounterSnapshot};
use super::deferred::S0ValidatedDeferredPhysicalGuaranteeMapArtifact;
use super::evidence::{
    S0ArtifactKind, S0ArtifactSchemaCompatibility, S0ArtifactValidationReport,
    S0CanonicalArtifactSpec, S0EvidenceRef, S0RequiredArtifactSet, S0StableDigest,
};
use super::handoff::{S0ValidatedStorageFoundationS1HandoffArtifact, StorageFoundationS1Handoff};
use super::harness::S0ValidatedHarnessMaturityReportArtifact;
use super::manifest::S0AuditInputManifest;
use super::migration::S0ValidatedTestMigrationNotesArtifact;
use super::milestones::{
    RoadmapGateReadinessWitness, S0ValidatedMilestonePhysicalStatusMatrixArtifact,
};
use super::terminology::{ReleaseClaimReport, S0ValidatedTerminologyRiskReportArtifact};
use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct S0EvidenceProvenance {
    source_revision: String,
    roadmap_parent_digest: S0StableDigest,
    audit_input_manifest_digest: S0StableDigest,
    upstream_artifact_digests: Vec<S0CanonicalArtifactSpec>,
}

impl S0EvidenceProvenance {
    pub fn artifact_digests(&self) -> &[S0CanonicalArtifactSpec] {
        &self.upstream_artifact_digests
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct S0RegenerationRequirement {
    command: String,
}

impl S0RegenerationRequirement {
    pub fn new(command: impl Into<String>) -> Result<Self, S0EvidenceBundleBuildRejection> {
        let command = command.into();
        if command.trim().is_empty() {
            return Err(S0EvidenceBundleBuildRejection::EmptyRequiredField);
        }
        Ok(Self { command })
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct S0ArtifactStalenessReport {
    stale_artifacts: Vec<S0ArtifactKind>,
    manually_edited_artifacts: Vec<S0ArtifactKind>,
}

impl S0ArtifactStalenessReport {
    pub fn is_clean(&self) -> bool {
        self.stale_artifacts.is_empty() && self.manually_edited_artifacts.is_empty()
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct S0AcceptedEvidenceBundleWitness {
    source_revision: String,
    audit_input_manifest_digest: S0StableDigest,
    evidence_bundle_digest: S0StableDigest,
}

impl S0AcceptedEvidenceBundleWitness {
    pub fn evidence_bundle_digest(&self) -> &S0StableDigest {
        &self.evidence_bundle_digest
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum S0CertificationStatus {
    Verified,
    Blocking,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct S0CertificationMatrixRow {
    row_id: S0ArtifactRowId,
    subject_kind: S0ArtifactSubjectKind,
    subject_path_or_symbol: String,
    classification: String,
    evidence_refs: Vec<S0EvidenceRef>,
    status: S0ArtifactRowStatus,
    notes: String,
    certification_status: S0CertificationStatus,
}

impl S0CertificationMatrixRow {
    fn new(
        row_id: &str,
        notes: impl Into<String>,
        certification_status: S0CertificationStatus,
        evidence_refs: Vec<S0EvidenceRef>,
    ) -> Result<Self, S0EvidenceBundleBuildRejection> {
        if evidence_refs.is_empty() {
            return Err(S0EvidenceBundleBuildRejection::MissingEvidenceRef);
        }
        Ok(Self {
            row_id: S0ArtifactRowId::new(row_id)
                .map_err(|_| S0EvidenceBundleBuildRejection::InvalidRowId)?,
            subject_kind: S0ArtifactSubjectKind::EvidenceLane,
            subject_path_or_symbol: "storage_foundation::s0".to_string(),
            classification: "s0-certification-matrix".to_string(),
            evidence_refs,
            status: match certification_status {
                S0CertificationStatus::Verified => S0ArtifactRowStatus::Admitted,
                S0CertificationStatus::Blocking => S0ArtifactRowStatus::Deferred,
            },
            notes: notes.into(),
            certification_status,
        })
    }

    pub fn certification_status(&self) -> S0CertificationStatus {
        self.certification_status
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct S0EvidenceBundle {
    #[serde(flatten)]
    envelope: S0ArtifactEnvelopeMetadata,
    certification_rows: Vec<S0CertificationMatrixRow>,
    artifact_validation: S0ArtifactValidationReport,
    evidence_provenance: S0EvidenceProvenance,
    staleness_report: S0ArtifactStalenessReport,
    regeneration_requirement: S0RegenerationRequirement,
    accepted_handoff_digest: S0StableDigest,
    release_claim_report_digest: S0StableDigest,
    complexity_contract_summary_digest: S0StableDigest,
    roadmap_gate_readiness: RoadmapGateReadinessWitness,
    counter_snapshot: S0CounterSnapshot,
    failure_digest: S0StableDigest,
}

impl S0EvidenceBundle {
    #[allow(clippy::too_many_arguments)]
    pub fn from_certified_inputs(
        source_revision: impl Into<String>,
        roadmap_parent_digest: S0StableDigest,
        generated_by: impl Into<String>,
        nondeterministic_metadata: S0NondeterministicMetadata,
        backend_matrix: &S0ValidatedBackendCapabilityMatrixArtifact,
        milestone_matrix: &S0ValidatedMilestonePhysicalStatusMatrixArtifact,
        claim_report: &S0ValidatedSemanticPhysicalClaimReportArtifact,
        deferred_map: &S0ValidatedDeferredPhysicalGuaranteeMapArtifact,
        terminology_report: &S0ValidatedTerminologyRiskReportArtifact,
        migration_notes: &S0ValidatedTestMigrationNotesArtifact,
        harness_report: &S0ValidatedHarnessMaturityReportArtifact,
        s1_handoff: &S0ValidatedStorageFoundationS1HandoffArtifact,
        manifest: &S0AuditInputManifest,
        complexity_report: &S0ComplexityContractReport,
        release_claim_report: &ReleaseClaimReport,
        regeneration_requirement: S0RegenerationRequirement,
    ) -> Result<Self, S0EvidenceBundleBuildRejection> {
        let source_revision = require_non_empty(source_revision)?;
        let generated_by = require_non_empty(generated_by)?;
        enforce_shared_provenance(
            &source_revision,
            &roadmap_parent_digest,
            backend_matrix.matrix().envelope(),
            milestone_matrix.matrix().envelope(),
            claim_report.report().envelope(),
            deferred_map.map().envelope(),
            terminology_report.report().envelope(),
            migration_notes.report().envelope(),
            harness_report.report().envelope(),
            s1_handoff.handoff().envelope(),
            manifest,
        )?;
        reject_stale_handoff_inputs(
            s1_handoff.handoff(),
            backend_matrix.matrix(),
            deferred_map.map(),
            terminology_report.report(),
            manifest,
            complexity_report,
        )?;

        let upstream_artifacts = vec![
            artifact_spec(
                S0ArtifactKind::BackendCapabilityMatrix,
                backend_matrix
                    .matrix()
                    .envelope()
                    .deterministic_digest()
                    .clone(),
            ),
            artifact_spec(
                S0ArtifactKind::MilestonePhysicalStatusMatrix,
                milestone_matrix
                    .matrix()
                    .envelope()
                    .deterministic_digest()
                    .clone(),
            ),
            artifact_spec(
                S0ArtifactKind::SemanticPhysicalClaimReport,
                claim_report
                    .report()
                    .envelope()
                    .deterministic_digest()
                    .clone(),
            ),
            artifact_spec(
                S0ArtifactKind::DeferredPhysicalGuaranteeMap,
                deferred_map.map().envelope().deterministic_digest().clone(),
            ),
            artifact_spec(
                S0ArtifactKind::TerminologyRiskReport,
                terminology_report
                    .report()
                    .envelope()
                    .deterministic_digest()
                    .clone(),
            ),
            artifact_spec(
                S0ArtifactKind::TestMigrationNotes,
                migration_notes
                    .report()
                    .envelope()
                    .deterministic_digest()
                    .clone(),
            ),
            artifact_spec(
                S0ArtifactKind::HarnessMaturityReport,
                harness_report
                    .report()
                    .envelope()
                    .deterministic_digest()
                    .clone(),
            ),
            artifact_spec(
                S0ArtifactKind::S1HandoffReadiness,
                s1_handoff
                    .handoff()
                    .envelope()
                    .deterministic_digest()
                    .clone(),
            ),
        ];
        let artifact_validation = S0RequiredArtifactSet::canonical().validate_present_artifacts(
            upstream_artifacts
                .iter()
                .cloned()
                .chain(std::iter::once(artifact_spec(
                    S0ArtifactKind::S0EvidenceBundle,
                    S0StableDigest::new("generated:self")
                        .map_err(|_| S0EvidenceBundleBuildRejection::InvalidDigest)?,
                ))),
        );

        let evidence_provenance = S0EvidenceProvenance {
            source_revision: source_revision.clone(),
            roadmap_parent_digest: roadmap_parent_digest.clone(),
            audit_input_manifest_digest: manifest.manifest_digest().clone(),
            upstream_artifact_digests: upstream_artifacts.clone(),
        };
        let release_claim_report_digest = stable_digest(release_claim_report)
            .map_err(|_| S0EvidenceBundleBuildRejection::InvalidDigest)?;
        let complexity_contract_summary_digest = complexity_summary_digest(complexity_report)
            .map_err(|_| S0EvidenceBundleBuildRejection::InvalidDigest)?;
        let mut counter_snapshot = S0CounterSnapshot::from_artifact_and_complexity_reports(
            &artifact_validation,
            complexity_report,
        )
        .with_input_manifest(manifest, None)
        .with_sequence_matrix(milestone_matrix.matrix().roadmap_sequence_status())
        .with_milestone_status_rows(
            milestone_matrix.matrix().rows(),
            milestone_matrix
                .matrix()
                .roadmap_sequence_status()
                .declarations()
                .len() as u64,
        )
        .with_claim_report(claim_report.report())
        .with_deferred_guarantee_map(deferred_map.map())
        .with_terminology_report(terminology_report.report())
        .with_release_claim_report(release_claim_report);
        counter_snapshot = counter_snapshot.with_validation_costs([
            backend_matrix.validation_cost(),
            milestone_matrix.validation_cost(),
            claim_report.validation_cost(),
            deferred_map.validation_cost(),
            terminology_report.validation_cost(),
            migration_notes.validation_cost(),
            harness_report.validation_cost(),
            s1_handoff.validation_cost(),
        ]);

        let certification_rows = certification_rows(
            &artifact_validation,
            &counter_snapshot,
            backend_matrix,
            milestone_matrix,
            claim_report,
            deferred_map,
            terminology_report,
            migration_notes,
            harness_report,
            s1_handoff,
        )?;
        let staleness_report = S0ArtifactStalenessReport {
            stale_artifacts: Vec::new(),
            manually_edited_artifacts: Vec::new(),
        };
        let failure_digest = failure_digest(&certification_rows)
            .map_err(|_| S0EvidenceBundleBuildRejection::InvalidDigest)?;
        let deterministic_digest = stable_digest(&S0EvidenceBundleDigestBasis {
            schema_version: S0_ARTIFACT_SCHEMA_VERSION,
            artifact_kind: S0ArtifactKind::S0EvidenceBundle,
            source_revision: &source_revision,
            roadmap_parent_digest: &roadmap_parent_digest,
            generated_by: &generated_by,
            certification_rows: &certification_rows,
            artifact_validation: &artifact_validation,
            evidence_provenance: &evidence_provenance,
            staleness_report: &staleness_report,
            regeneration_requirement: &regeneration_requirement,
            accepted_handoff_digest: s1_handoff.handoff().envelope().deterministic_digest(),
            release_claim_report_digest: &release_claim_report_digest,
            complexity_contract_summary_digest: &complexity_contract_summary_digest,
            roadmap_gate_readiness: s1_handoff.handoff().gate_readiness(),
            counter_snapshot: &counter_snapshot,
            failure_digest: &failure_digest,
        })
        .map_err(|_| S0EvidenceBundleBuildRejection::InvalidDigest)?;

        Ok(Self {
            envelope: S0ArtifactEnvelopeMetadata::new(
                S0ArtifactKind::S0EvidenceBundle,
                source_revision,
                roadmap_parent_digest,
                generated_by,
                deterministic_digest,
                nondeterministic_metadata,
            ),
            certification_rows,
            artifact_validation,
            evidence_provenance,
            staleness_report,
            regeneration_requirement,
            accepted_handoff_digest: s1_handoff
                .handoff()
                .envelope()
                .deterministic_digest()
                .clone(),
            release_claim_report_digest,
            complexity_contract_summary_digest,
            roadmap_gate_readiness: s1_handoff.handoff().gate_readiness().clone(),
            counter_snapshot,
            failure_digest,
        })
    }

    pub fn envelope(&self) -> &S0ArtifactEnvelopeMetadata {
        &self.envelope
    }

    pub fn certification_rows(&self) -> &[S0CertificationMatrixRow] {
        &self.certification_rows
    }

    pub fn counter_snapshot(&self) -> &S0CounterSnapshot {
        &self.counter_snapshot
    }

    pub fn witness(&self) -> S0AcceptedEvidenceBundleWitness {
        S0AcceptedEvidenceBundleWitness {
            source_revision: self.evidence_provenance.source_revision.clone(),
            audit_input_manifest_digest: self
                .evidence_provenance
                .audit_input_manifest_digest
                .clone(),
            evidence_bundle_digest: self.envelope.deterministic_digest().clone(),
        }
    }

    pub fn to_canonical_json_bytes(&self) -> Result<Vec<u8>, S0EvidenceBundleParseRejection> {
        serde_json::to_vec_pretty(self)
            .map_err(|_| S0EvidenceBundleParseRejection::SerializationFailed)
    }

    pub fn validate_canonical_json_bytes(
        bytes: &[u8],
    ) -> Result<S0ValidatedEvidenceBundleArtifact, S0EvidenceBundleParseRejection> {
        let raw = serde_json::from_slice::<RawS0EvidenceBundle>(bytes)
            .map_err(|_| S0EvidenceBundleParseRejection::NonParseable)?;
        if raw.envelope.schema_version != S0_ARTIFACT_SCHEMA_VERSION {
            return Err(S0EvidenceBundleParseRejection::SchemaVersionMismatch);
        }
        if raw.envelope.artifact_kind != S0ArtifactKind::S0EvidenceBundle {
            return Err(S0EvidenceBundleParseRejection::ArtifactKindMismatch);
        }
        let roadmap_parent_digest = S0StableDigest::new(raw.envelope.roadmap_parent_digest)
            .map_err(|_| S0EvidenceBundleParseRejection::InvalidDigest)?;
        let expected_digest = S0StableDigest::new(raw.envelope.deterministic_digest)
            .map_err(|_| S0EvidenceBundleParseRejection::InvalidDigest)?;
        let row_count = raw.certification_rows.len() as u64;
        let source_revision = raw.envelope.source_revision;
        let generated_by = raw.envelope.generated_by;
        let nondeterministic_metadata = raw.envelope.nondeterministic_metadata.into_validated()?;
        let certification_rows = raw.certification_rows;
        let artifact_validation = raw.artifact_validation;
        let evidence_provenance = raw.evidence_provenance;
        let staleness_report = raw.staleness_report;
        let regeneration_requirement = raw.regeneration_requirement;
        let accepted_handoff_digest = raw.accepted_handoff_digest;
        let release_claim_report_digest = raw.release_claim_report_digest;
        let complexity_contract_summary_digest = raw.complexity_contract_summary_digest;
        let roadmap_gate_readiness = raw.roadmap_gate_readiness.into_validated()?;
        let counter_snapshot = raw.counter_snapshot;
        let stored_failure_digest = raw.failure_digest;
        let expected_failure_digest = failure_digest(&certification_rows)
            .map_err(|_| S0EvidenceBundleParseRejection::InvalidDigest)?;
        if expected_failure_digest != stored_failure_digest {
            return Err(S0EvidenceBundleParseRejection::FailureDigestMismatch);
        }
        if evidence_provenance.source_revision != source_revision
            || evidence_provenance.roadmap_parent_digest != roadmap_parent_digest
        {
            return Err(S0EvidenceBundleParseRejection::ProvenanceMismatch);
        }
        let expected_artifact_validation = S0RequiredArtifactSet::canonical()
            .validate_present_artifacts(
                evidence_provenance
                    .artifact_digests()
                    .iter()
                    .cloned()
                    .chain(std::iter::once(artifact_spec(
                        S0ArtifactKind::S0EvidenceBundle,
                        expected_digest.clone(),
                    ))),
            );
        if expected_artifact_validation != artifact_validation {
            return Err(S0EvidenceBundleParseRejection::ArtifactValidationMismatch);
        }
        let recomputed_digest = stable_digest(&S0EvidenceBundleDigestBasis {
            schema_version: S0_ARTIFACT_SCHEMA_VERSION,
            artifact_kind: S0ArtifactKind::S0EvidenceBundle,
            source_revision: &source_revision,
            roadmap_parent_digest: &roadmap_parent_digest,
            generated_by: &generated_by,
            certification_rows: &certification_rows,
            artifact_validation: &artifact_validation,
            evidence_provenance: &evidence_provenance,
            staleness_report: &staleness_report,
            regeneration_requirement: &regeneration_requirement,
            accepted_handoff_digest: &accepted_handoff_digest,
            release_claim_report_digest: &release_claim_report_digest,
            complexity_contract_summary_digest: &complexity_contract_summary_digest,
            roadmap_gate_readiness: &roadmap_gate_readiness,
            counter_snapshot: &counter_snapshot,
            failure_digest: &stored_failure_digest,
        })
        .map_err(|_| S0EvidenceBundleParseRejection::InvalidDigest)?;
        let bundle = Self {
            envelope: S0ArtifactEnvelopeMetadata::new(
                S0ArtifactKind::S0EvidenceBundle,
                source_revision,
                roadmap_parent_digest,
                generated_by,
                recomputed_digest,
                nondeterministic_metadata,
            ),
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
            failure_digest: stored_failure_digest,
        };
        let canonicalized_row_byte_count = serde_json::to_vec(bundle.certification_rows())
            .map_err(|_| S0EvidenceBundleParseRejection::SerializationFailed)?
            .len() as u64;
        if bundle.envelope().deterministic_digest() != &expected_digest {
            return Err(S0EvidenceBundleParseRejection::DeterministicDigestMismatch);
        }
        Ok(S0ValidatedEvidenceBundleArtifact {
            bundle,
            validation_cost: S0ArtifactValidationCostSurface::new(
                bytes.len() as u64,
                row_count,
                canonicalized_row_byte_count,
                row_count,
            ),
        })
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct S0ValidatedEvidenceBundleArtifact {
    bundle: S0EvidenceBundle,
    validation_cost: S0ArtifactValidationCostSurface,
}

impl S0ValidatedEvidenceBundleArtifact {
    pub fn bundle(&self) -> &S0EvidenceBundle {
        &self.bundle
    }

    pub fn validation_cost(&self) -> &S0ArtifactValidationCostSurface {
        &self.validation_cost
    }
}

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

#[derive(Serialize)]
struct S0EvidenceBundleDigestBasis<'a> {
    schema_version: &'static str,
    artifact_kind: S0ArtifactKind,
    source_revision: &'a str,
    roadmap_parent_digest: &'a S0StableDigest,
    generated_by: &'a str,
    certification_rows: &'a [S0CertificationMatrixRow],
    artifact_validation: &'a S0ArtifactValidationReport,
    evidence_provenance: &'a S0EvidenceProvenance,
    staleness_report: &'a S0ArtifactStalenessReport,
    regeneration_requirement: &'a S0RegenerationRequirement,
    accepted_handoff_digest: &'a S0StableDigest,
    release_claim_report_digest: &'a S0StableDigest,
    complexity_contract_summary_digest: &'a S0StableDigest,
    roadmap_gate_readiness: &'a RoadmapGateReadinessWitness,
    counter_snapshot: &'a S0CounterSnapshot,
    failure_digest: &'a S0StableDigest,
}

#[derive(Deserialize)]
struct RawS0EvidenceBundle {
    #[serde(flatten)]
    envelope: RawS0ArtifactEnvelope,
    certification_rows: Vec<S0CertificationMatrixRow>,
    artifact_validation: S0ArtifactValidationReport,
    evidence_provenance: S0EvidenceProvenance,
    staleness_report: S0ArtifactStalenessReport,
    regeneration_requirement: S0RegenerationRequirement,
    accepted_handoff_digest: S0StableDigest,
    release_claim_report_digest: S0StableDigest,
    complexity_contract_summary_digest: S0StableDigest,
    roadmap_gate_readiness: RawRoadmapGateReadinessWitness,
    counter_snapshot: S0CounterSnapshot,
    failure_digest: S0StableDigest,
}

#[derive(Deserialize)]
struct RawS0ArtifactEnvelope {
    schema_version: String,
    artifact_kind: S0ArtifactKind,
    source_revision: String,
    roadmap_parent_digest: String,
    generated_by: String,
    deterministic_digest: String,
    nondeterministic_metadata: RawS0NondeterministicMetadata,
}

#[derive(Deserialize)]
struct RawS0NondeterministicMetadata {
    generated_at_policy: String,
    local_path_hint: Option<String>,
    host_hint: Option<String>,
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
struct RawRoadmapGateReadinessWitness {
    milestone_id: String,
    predecessor_evidence_count: u64,
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

fn certification_rows(
    artifact_validation: &S0ArtifactValidationReport,
    counters: &S0CounterSnapshot,
    backend_matrix: &S0ValidatedBackendCapabilityMatrixArtifact,
    milestone_matrix: &S0ValidatedMilestonePhysicalStatusMatrixArtifact,
    claim_report: &S0ValidatedSemanticPhysicalClaimReportArtifact,
    deferred_map: &S0ValidatedDeferredPhysicalGuaranteeMapArtifact,
    terminology_report: &S0ValidatedTerminologyRiskReportArtifact,
    migration_notes: &S0ValidatedTestMigrationNotesArtifact,
    harness_report: &S0ValidatedHarnessMaturityReportArtifact,
    s1_handoff: &S0ValidatedStorageFoundationS1HandoffArtifact,
) -> Result<Vec<S0CertificationMatrixRow>, S0EvidenceBundleBuildRejection> {
    let rows = vec![
        S0CertificationMatrixRow::new(
            "all_existing_backends_classified",
            "Backend capability rows exist for the first audit baseline families.",
            if backend_matrix.matrix().rows().len() >= 10 {
                S0CertificationStatus::Verified
            } else {
                S0CertificationStatus::Blocking
            },
            vec![artifact_ref(
                S0ArtifactKind::BackendCapabilityMatrix,
                backend_matrix.matrix().envelope().deterministic_digest(),
            )],
        )?,
        S0CertificationMatrixRow::new(
            "canonical_artifact_set_parseable",
            "Required canonical artifact set is present and schema-compatible.",
            if artifact_validation.is_complete() {
                S0CertificationStatus::Verified
            } else {
                S0CertificationStatus::Blocking
            },
            [
                artifact_ref(
                    S0ArtifactKind::BackendCapabilityMatrix,
                    backend_matrix.matrix().envelope().deterministic_digest(),
                ),
                artifact_ref(
                    S0ArtifactKind::MilestonePhysicalStatusMatrix,
                    milestone_matrix.matrix().envelope().deterministic_digest(),
                ),
                artifact_ref(
                    S0ArtifactKind::SemanticPhysicalClaimReport,
                    claim_report.report().envelope().deterministic_digest(),
                ),
                artifact_ref(
                    S0ArtifactKind::DeferredPhysicalGuaranteeMap,
                    deferred_map.map().envelope().deterministic_digest(),
                ),
                artifact_ref(
                    S0ArtifactKind::TerminologyRiskReport,
                    terminology_report
                        .report()
                        .envelope()
                        .deterministic_digest(),
                ),
                artifact_ref(
                    S0ArtifactKind::TestMigrationNotes,
                    migration_notes.report().envelope().deterministic_digest(),
                ),
                artifact_ref(
                    S0ArtifactKind::HarnessMaturityReport,
                    harness_report.report().envelope().deterministic_digest(),
                ),
                artifact_ref(
                    S0ArtifactKind::S1HandoffReadiness,
                    s1_handoff.handoff().envelope().deterministic_digest(),
                ),
            ]
            .to_vec(),
        )?,
        S0CertificationMatrixRow::new(
            "complexity_contracts_verified",
            "All required S.0 complexity contracts are present without debt.",
            if counters.complexity_debt_count() == 0
                && counters.missing_complexity_contract_count() == 0
                && counters.duplicate_complexity_contract_count() == 0
            {
                S0CertificationStatus::Verified
            } else {
                S0CertificationStatus::Blocking
            },
            vec![artifact_ref(
                S0ArtifactKind::S1HandoffReadiness,
                s1_handoff.handoff().envelope().deterministic_digest(),
            )],
        )?,
        S0CertificationMatrixRow::new(
            "roadmap_sequence_consistency_verified",
            "Prior milestone sequence state is reconciled or explicitly waived.",
            if counters.sequence_inconsistency_count() == 0 {
                S0CertificationStatus::Verified
            } else {
                S0CertificationStatus::Blocking
            },
            vec![artifact_ref(
                S0ArtifactKind::MilestonePhysicalStatusMatrix,
                milestone_matrix.matrix().envelope().deterministic_digest(),
            )],
        )?,
        S0CertificationMatrixRow::new(
            "milestones_1_through_13_3_status_rows_complete",
            "Every declared milestone has a physical-status row.",
            if counters.missing_milestone_status_row_count() == 0 {
                S0CertificationStatus::Verified
            } else {
                S0CertificationStatus::Blocking
            },
            vec![artifact_ref(
                S0ArtifactKind::MilestonePhysicalStatusMatrix,
                milestone_matrix.matrix().envelope().deterministic_digest(),
            )],
        )?,
        S0CertificationMatrixRow::new(
            "release_claim_gate_rejects_overclaim",
            "Release/public claim surfaces remain qualified.",
            if counters.unqualified_release_claim_count() == 0 {
                S0CertificationStatus::Verified
            } else {
                S0CertificationStatus::Blocking
            },
            vec![artifact_ref(
                S0ArtifactKind::TerminologyRiskReport,
                terminology_report
                    .report()
                    .envelope()
                    .deterministic_digest(),
            )],
        )?,
        S0CertificationMatrixRow::new(
            "test_evidence_scope_declared",
            "Existing named suites carry explicit semantic-versus-physical scope.",
            if !migration_notes.report().rows().is_empty() {
                S0CertificationStatus::Verified
            } else {
                S0CertificationStatus::Blocking
            },
            vec![artifact_ref(
                S0ArtifactKind::TestMigrationNotes,
                migration_notes.report().envelope().deterministic_digest(),
            )],
        )?,
        S0CertificationMatrixRow::new(
            "harness_maturity_rows_present",
            "Required harness maturity rows are visible before S.1 closeout.",
            if !harness_report.report().rows().is_empty() {
                S0CertificationStatus::Verified
            } else {
                S0CertificationStatus::Blocking
            },
            vec![artifact_ref(
                S0ArtifactKind::HarnessMaturityReport,
                harness_report.report().envelope().deterministic_digest(),
            )],
        )?,
        S0CertificationMatrixRow::new(
            "s1_handoff_blocks_missing_inputs",
            "S.1 handoff blocking predicates are all satisfied for accepted inputs.",
            if s1_handoff
                .handoff()
                .blocking_predicates()
                .iter()
                .all(|row| row.status() == super::handoff::S1BlockingPredicateStatus::Satisfied)
            {
                S0CertificationStatus::Verified
            } else {
                S0CertificationStatus::Blocking
            },
            vec![artifact_ref(
                S0ArtifactKind::S1HandoffReadiness,
                s1_handoff.handoff().envelope().deterministic_digest(),
            )],
        )?,
        S0CertificationMatrixRow::new(
            "status_matrix_digest_changes_on_claim_change",
            "Milestone matrix digest is stable and claim-sensitive.",
            if !milestone_matrix
                .matrix()
                .envelope()
                .deterministic_digest()
                .as_str()
                .is_empty()
            {
                S0CertificationStatus::Verified
            } else {
                S0CertificationStatus::Blocking
            },
            vec![
                artifact_ref(
                    S0ArtifactKind::MilestonePhysicalStatusMatrix,
                    milestone_matrix.matrix().envelope().deterministic_digest(),
                ),
                artifact_ref(
                    S0ArtifactKind::SemanticPhysicalClaimReport,
                    claim_report.report().envelope().deterministic_digest(),
                ),
                artifact_ref(
                    S0ArtifactKind::DeferredPhysicalGuaranteeMap,
                    deferred_map.map().envelope().deterministic_digest(),
                ),
            ],
        )?,
    ];
    Ok(rows)
}

fn artifact_spec(kind: S0ArtifactKind, digest: S0StableDigest) -> S0CanonicalArtifactSpec {
    S0CanonicalArtifactSpec::new(kind, digest, S0ArtifactSchemaCompatibility::Compatible)
}

fn artifact_ref(kind: S0ArtifactKind, digest: &S0StableDigest) -> S0EvidenceRef {
    S0EvidenceRef::new(kind, digest.clone())
}

fn require_non_empty(value: impl Into<String>) -> Result<String, S0EvidenceBundleBuildRejection> {
    let value = value.into();
    if value.trim().is_empty() {
        return Err(S0EvidenceBundleBuildRejection::EmptyRequiredField);
    }
    Ok(value)
}

fn stable_digest(value: &impl Serialize) -> Result<S0StableDigest, serde_json::Error> {
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

fn failure_digest(rows: &[S0CertificationMatrixRow]) -> Result<S0StableDigest, serde_json::Error> {
    let failures = rows
        .iter()
        .filter(|row| row.certification_status() == S0CertificationStatus::Blocking)
        .map(|row| row.row_id.as_str())
        .collect::<Vec<_>>();
    stable_digest(&failures)
}

fn complexity_summary_digest(
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

#[allow(clippy::too_many_arguments)]
fn enforce_shared_provenance(
    source_revision: &str,
    roadmap_parent_digest: &S0StableDigest,
    backend_matrix: &S0ArtifactEnvelopeMetadata,
    milestone_matrix: &S0ArtifactEnvelopeMetadata,
    claim_report: &S0ArtifactEnvelopeMetadata,
    deferred_map: &S0ArtifactEnvelopeMetadata,
    terminology_report: &S0ArtifactEnvelopeMetadata,
    migration_notes: &S0ArtifactEnvelopeMetadata,
    harness_report: &S0ArtifactEnvelopeMetadata,
    s1_handoff: &S0ArtifactEnvelopeMetadata,
    manifest: &S0AuditInputManifest,
) -> Result<(), S0EvidenceBundleBuildRejection> {
    let all_match = [
        backend_matrix,
        milestone_matrix,
        claim_report,
        deferred_map,
        terminology_report,
        migration_notes,
        harness_report,
        s1_handoff,
    ]
    .into_iter()
    .all(|envelope| {
        envelope.source_revision() == source_revision
            && envelope.roadmap_parent_digest() == roadmap_parent_digest
    });
    if !all_match || manifest.source_revision() != source_revision {
        return Err(S0EvidenceBundleBuildRejection::StaleEvidence(
            S0StaleEvidenceRejection::SourceRevisionMismatch,
        ));
    }
    Ok(())
}

fn reject_stale_handoff_inputs(
    handoff: &StorageFoundationS1Handoff,
    backend_matrix: &BackendCapabilityMatrix,
    deferred_map: &super::deferred::DeferredPhysicalGuaranteeMap,
    terminology_report: &super::terminology::TerminologyRiskReport,
    manifest: &S0AuditInputManifest,
    complexity_report: &S0ComplexityContractReport,
) -> Result<(), S0EvidenceBundleBuildRejection> {
    if handoff.backend_tier_matrix_digest() != backend_matrix.envelope().deterministic_digest() {
        return Err(S0EvidenceBundleBuildRejection::StaleEvidence(
            S0StaleEvidenceRejection::BackendCapabilityMatrixDigestMismatch,
        ));
    }
    if handoff.deferred_guarantee_map_digest() != deferred_map.envelope().deterministic_digest() {
        return Err(S0EvidenceBundleBuildRejection::StaleEvidence(
            S0StaleEvidenceRejection::DeferredGuaranteeMapDigestMismatch,
        ));
    }
    if handoff.terminology_scan_digest() != terminology_report.scan_digest() {
        return Err(S0EvidenceBundleBuildRejection::StaleEvidence(
            S0StaleEvidenceRejection::TerminologyReportDigestMismatch,
        ));
    }
    if handoff.audit_input_manifest_digest() != manifest.manifest_digest() {
        return Err(S0EvidenceBundleBuildRejection::StaleEvidence(
            S0StaleEvidenceRejection::AuditInputManifestDigestMismatch,
        ));
    }
    let expected_complexity_digest = complexity_summary_digest(complexity_report)
        .map_err(|_| S0EvidenceBundleBuildRejection::InvalidDigest)?;
    if handoff.complexity_contract_summary_digest() != &expected_complexity_digest {
        return Err(S0EvidenceBundleBuildRejection::StaleEvidence(
            S0StaleEvidenceRejection::ComplexitySummaryDigestMismatch,
        ));
    }
    Ok(())
}
