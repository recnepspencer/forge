use super::super::artifacts::{
    S0ArtifactEnvelopeMetadata, S0NondeterministicMetadata,
    S0ValidatedBackendCapabilityMatrixArtifact, S0_ARTIFACT_SCHEMA_VERSION,
};
use super::super::claims::S0ValidatedSemanticPhysicalClaimReportArtifact;
use super::super::counters::{S0ComplexityContractReport, S0CounterSnapshot};
use super::super::deferred::S0ValidatedDeferredPhysicalGuaranteeMapArtifact;
use super::super::evidence::{
    S0ArtifactKind, S0ArtifactValidationReport, S0CanonicalArtifactSpec, S0RequiredArtifactSet,
    S0StableDigest,
};
use super::super::handoff::S0ValidatedStorageFoundationS1HandoffArtifact;
use super::super::harness::S0ValidatedHarnessMaturityReportArtifact;
use super::super::manifest::S0AuditInputManifest;
use super::super::migration::S0ValidatedTestMigrationNotesArtifact;
use super::super::milestones::{
    RoadmapGateReadinessWitness, S0ValidatedMilestonePhysicalStatusMatrixArtifact,
};
use super::super::terminology::{ReleaseClaimReport, S0ValidatedTerminologyRiskReportArtifact};
use super::aggregate::S0EvidenceBundle;
use super::certification_matrix::{build_certification_matrix, CertificationInputs};
use super::digests::{
    complexity_summary_digest, failure_digest, stable_digest, S0EvidenceBundleDigestBasis,
};
use super::provenance::{
    S0ArtifactStalenessReport, S0EvidenceProvenance, S0RegenerationRequirement,
};
use super::upstream_artifacts::build_upstream_artifact_specs;
use super::validation::{
    artifact_spec, enforce_shared_provenance, reject_stale_handoff_inputs, require_non_empty,
    S0EvidenceBundleBuildRejection, SharedProvenanceInputs, StaleHandoffInputs,
};

pub(super) struct CertifiedBundleRequest<'a> {
    pub(super) source_revision: String,
    pub(super) roadmap_parent_digest: S0StableDigest,
    pub(super) generated_by: String,
    pub(super) nondeterministic_metadata: S0NondeterministicMetadata,
    pub(super) backend_matrix: &'a S0ValidatedBackendCapabilityMatrixArtifact,
    pub(super) milestone_matrix: &'a S0ValidatedMilestonePhysicalStatusMatrixArtifact,
    pub(super) claim_report: &'a S0ValidatedSemanticPhysicalClaimReportArtifact,
    pub(super) deferred_map: &'a S0ValidatedDeferredPhysicalGuaranteeMapArtifact,
    pub(super) terminology_report: &'a S0ValidatedTerminologyRiskReportArtifact,
    pub(super) migration_notes: &'a S0ValidatedTestMigrationNotesArtifact,
    pub(super) harness_report: &'a S0ValidatedHarnessMaturityReportArtifact,
    pub(super) s1_handoff: &'a S0ValidatedStorageFoundationS1HandoffArtifact,
    pub(super) manifest: &'a S0AuditInputManifest,
    pub(super) complexity_report: &'a S0ComplexityContractReport,
    pub(super) release_claim_report: &'a ReleaseClaimReport,
    pub(super) regeneration_requirement: S0RegenerationRequirement,
}

struct BundleAssembly {
    certification_rows: Vec<super::certification::S0CertificationMatrixRow>,
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
        assemble_bundle(CertifiedBundleRequest {
            source_revision: require_non_empty(source_revision)?,
            roadmap_parent_digest,
            generated_by: require_non_empty(generated_by)?,
            nondeterministic_metadata,
            backend_matrix,
            milestone_matrix,
            claim_report,
            deferred_map,
            terminology_report,
            migration_notes,
            harness_report,
            s1_handoff,
            manifest,
            complexity_report,
            release_claim_report,
            regeneration_requirement,
        })
    }
}

fn assemble_bundle(
    request: CertifiedBundleRequest<'_>,
) -> Result<S0EvidenceBundle, S0EvidenceBundleBuildRejection> {
    validate_input_provenance(&request)?;
    validate_input_freshness(&request)?;
    let upstream_artifacts = build_upstream_artifact_specs(&request);
    let artifact_validation = validate_required_artifacts(&upstream_artifacts)?;
    let evidence_provenance = build_evidence_provenance(&request, upstream_artifacts);
    let release_claim_report_digest = stable_digest(request.release_claim_report)
        .map_err(|_| S0EvidenceBundleBuildRejection::InvalidDigest)?;
    let complexity_contract_summary_digest =
        complexity_summary_digest(request.complexity_report)
            .map_err(|_| S0EvidenceBundleBuildRejection::InvalidDigest)?;
    let counter_snapshot = build_counter_snapshot(&request, &artifact_validation);
    let certification_rows =
        build_certification_rows(&request, &artifact_validation, &counter_snapshot)?;
    let failure_digest = failure_digest(&certification_rows)
        .map_err(|_| S0EvidenceBundleBuildRejection::InvalidDigest)?;
    let assembly = BundleAssembly {
        certification_rows,
        artifact_validation,
        evidence_provenance,
        staleness_report: clean_staleness_report(),
        regeneration_requirement: request.regeneration_requirement.clone(),
        accepted_handoff_digest: request
            .s1_handoff
            .handoff()
            .envelope()
            .deterministic_digest()
            .clone(),
        release_claim_report_digest,
        complexity_contract_summary_digest,
        roadmap_gate_readiness: request.s1_handoff.handoff().gate_readiness().clone(),
        counter_snapshot,
        failure_digest,
    };
    let deterministic_digest = assembly.deterministic_digest(&request)?;
    Ok(assembly.into_bundle(request, deterministic_digest))
}

impl BundleAssembly {
    fn deterministic_digest(
        &self,
        request: &CertifiedBundleRequest<'_>,
    ) -> Result<S0StableDigest, S0EvidenceBundleBuildRejection> {
        stable_digest(&S0EvidenceBundleDigestBasis {
            schema_version: S0_ARTIFACT_SCHEMA_VERSION,
            artifact_kind: S0ArtifactKind::S0EvidenceBundle,
            source_revision: &request.source_revision,
            roadmap_parent_digest: &request.roadmap_parent_digest,
            generated_by: &request.generated_by,
            certification_rows: &self.certification_rows,
            artifact_validation: &self.artifact_validation,
            evidence_provenance: &self.evidence_provenance,
            staleness_report: &self.staleness_report,
            regeneration_requirement: &self.regeneration_requirement,
            accepted_handoff_digest: &self.accepted_handoff_digest,
            release_claim_report_digest: &self.release_claim_report_digest,
            complexity_contract_summary_digest: &self.complexity_contract_summary_digest,
            roadmap_gate_readiness: &self.roadmap_gate_readiness,
            counter_snapshot: &self.counter_snapshot,
            failure_digest: &self.failure_digest,
        })
        .map_err(|_| S0EvidenceBundleBuildRejection::InvalidDigest)
    }

    fn into_bundle(
        self,
        request: CertifiedBundleRequest<'_>,
        deterministic_digest: S0StableDigest,
    ) -> S0EvidenceBundle {
        S0EvidenceBundle {
            envelope: S0ArtifactEnvelopeMetadata::new(
                S0ArtifactKind::S0EvidenceBundle,
                request.source_revision,
                request.roadmap_parent_digest,
                request.generated_by,
                deterministic_digest,
                request.nondeterministic_metadata,
            ),
            certification_rows: self.certification_rows,
            artifact_validation: self.artifact_validation,
            evidence_provenance: self.evidence_provenance,
            staleness_report: self.staleness_report,
            regeneration_requirement: self.regeneration_requirement,
            accepted_handoff_digest: self.accepted_handoff_digest,
            release_claim_report_digest: self.release_claim_report_digest,
            complexity_contract_summary_digest: self.complexity_contract_summary_digest,
            roadmap_gate_readiness: self.roadmap_gate_readiness,
            counter_snapshot: self.counter_snapshot,
            failure_digest: self.failure_digest,
        }
    }
}

fn validate_input_provenance(
    request: &CertifiedBundleRequest<'_>,
) -> Result<(), S0EvidenceBundleBuildRejection> {
    enforce_shared_provenance(SharedProvenanceInputs {
        source_revision: &request.source_revision,
        roadmap_parent_digest: &request.roadmap_parent_digest,
        backend_matrix: request.backend_matrix.matrix().envelope(),
        milestone_matrix: request.milestone_matrix.matrix().envelope(),
        claim_report: request.claim_report.report().envelope(),
        deferred_map: request.deferred_map.map().envelope(),
        terminology_report: request.terminology_report.report().envelope(),
        migration_notes: request.migration_notes.report().envelope(),
        harness_report: request.harness_report.report().envelope(),
        s1_handoff: request.s1_handoff.handoff().envelope(),
        manifest: request.manifest,
    })
}

fn validate_input_freshness(
    request: &CertifiedBundleRequest<'_>,
) -> Result<(), S0EvidenceBundleBuildRejection> {
    reject_stale_handoff_inputs(StaleHandoffInputs {
        handoff: request.s1_handoff.handoff(),
        backend_matrix: request.backend_matrix.matrix(),
        deferred_map: request.deferred_map.map(),
        terminology_report: request.terminology_report.report(),
        manifest: request.manifest,
        complexity_report: request.complexity_report,
    })
}

fn validate_required_artifacts(
    upstream_artifacts: &[S0CanonicalArtifactSpec],
) -> Result<S0ArtifactValidationReport, S0EvidenceBundleBuildRejection> {
    Ok(
        S0RequiredArtifactSet::canonical().validate_present_artifacts(
            upstream_artifacts
                .iter()
                .cloned()
                .chain(std::iter::once(artifact_spec(
                    S0ArtifactKind::S0EvidenceBundle,
                    S0StableDigest::new("generated:self")
                        .map_err(|_| S0EvidenceBundleBuildRejection::InvalidDigest)?,
                ))),
        ),
    )
}

fn build_evidence_provenance(
    request: &CertifiedBundleRequest<'_>,
    upstream_artifacts: Vec<S0CanonicalArtifactSpec>,
) -> S0EvidenceProvenance {
    S0EvidenceProvenance {
        source_revision: request.source_revision.clone(),
        roadmap_parent_digest: request.roadmap_parent_digest.clone(),
        audit_input_manifest_digest: request.manifest.manifest_digest().clone(),
        upstream_artifact_digests: upstream_artifacts,
    }
}

fn build_counter_snapshot(
    request: &CertifiedBundleRequest<'_>,
    artifact_validation: &S0ArtifactValidationReport,
) -> S0CounterSnapshot {
    let mut counter_snapshot = S0CounterSnapshot::from_artifact_and_complexity_reports(
        artifact_validation,
        request.complexity_report,
    )
    .with_input_manifest(request.manifest, None)
    .with_sequence_matrix(request.milestone_matrix.matrix().roadmap_sequence_status())
    .with_milestone_status_rows(
        request.milestone_matrix.matrix().rows(),
        request
            .milestone_matrix
            .matrix()
            .roadmap_sequence_status()
            .declarations()
            .len() as u64,
    )
    .with_claim_report(request.claim_report.report())
    .with_deferred_guarantee_map(request.deferred_map.map())
    .with_terminology_report(request.terminology_report.report())
    .with_release_claim_report(request.release_claim_report);
    counter_snapshot = counter_snapshot.with_validation_costs([
        request.backend_matrix.validation_cost(),
        request.milestone_matrix.validation_cost(),
        request.claim_report.validation_cost(),
        request.deferred_map.validation_cost(),
        request.terminology_report.validation_cost(),
        request.migration_notes.validation_cost(),
        request.harness_report.validation_cost(),
        request.s1_handoff.validation_cost(),
    ]);
    counter_snapshot
}

fn build_certification_rows(
    request: &CertifiedBundleRequest<'_>,
    artifact_validation: &S0ArtifactValidationReport,
    counter_snapshot: &S0CounterSnapshot,
) -> Result<Vec<super::certification::S0CertificationMatrixRow>, S0EvidenceBundleBuildRejection> {
    build_certification_matrix(&CertificationInputs {
        artifact_validation,
        counters: counter_snapshot,
        backend_matrix: request.backend_matrix,
        milestone_matrix: request.milestone_matrix,
        claim_report: request.claim_report,
        deferred_map: request.deferred_map,
        terminology_report: request.terminology_report,
        migration_notes: request.migration_notes,
        harness_report: request.harness_report,
        s1_handoff: request.s1_handoff,
    })
}

fn clean_staleness_report() -> S0ArtifactStalenessReport {
    S0ArtifactStalenessReport {
        stale_artifacts: Vec::new(),
        manually_edited_artifacts: Vec::new(),
    }
}
