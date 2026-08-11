use super::super::artifacts::{
    BackendCapabilityMatrix, S0ArtifactEnvelopeMetadata, S0NondeterministicMetadata,
    S0_ARTIFACT_SCHEMA_VERSION,
};
use super::super::counters::S0ComplexityContractReport;
use super::super::deferred::DeferredPhysicalGuaranteeMap;
use super::super::harness::{
    HarnessMaturityReport, S1CompileTimeBoundaryFixture, S1ForbiddenShortcut,
};
use super::super::manifest::S0AuditInputManifest;
use super::super::milestones::RoadmapGateReadinessWitness;
use super::super::terminology::{ReleaseClaimReport, TerminologyRiskReport};
use super::accepted_evidence_provenance::S0AcceptedEvidenceProvenance;
use super::compile_time_boundary_rows::{
    S1CompileTimeBoundaryFixtureStatusRow, S1NonPlatformGradeDebtRow,
};
use super::handoff_requirements::{
    derive_handoff_requirements, DerivedHandoffRequirements, HandoffRequirementsInput,
};
use super::handoff_validation::{
    complexity_summary_digest, require_non_empty, stable_digest, validate_accepted_inputs,
    AcceptedInputValidation, S0S1HandoffBuildRejection, S1HandoffDigestBasis,
};
use super::s1_blocking_predicate::S1BlockingPredicateRow;
use super::sequence_harness_dependency::SequenceHarnessDependency;

#[derive(Debug, Clone, PartialEq, Eq, serde::Serialize)]
pub struct StorageFoundationS1Handoff {
    #[serde(flatten)]
    pub(super) envelope: S0ArtifactEnvelopeMetadata,
    pub(super) backend_tier_matrix_digest: super::super::evidence::S0StableDigest,
    pub(super) deferred_guarantee_map_digest: super::super::evidence::S0StableDigest,
    pub(super) terminology_scan_digest: super::super::evidence::S0StableDigest,
    pub(super) audit_input_manifest_digest: super::super::evidence::S0StableDigest,
    pub(super) complexity_contract_summary_digest: super::super::evidence::S0StableDigest,
    pub(super) required_forbidden_shortcuts: Vec<S1ForbiddenShortcut>,
    pub(super) required_harness_subsystems: Vec<SequenceHarnessDependency>,
    pub(super) allowed_backend_candidates: Vec<String>,
    pub(super) legacy_backend_fences: Vec<String>,
    pub(super) compile_time_boundary_fixtures: Vec<S1CompileTimeBoundaryFixtureStatusRow>,
    pub(super) non_platform_grade_debt_rows: Vec<S1NonPlatformGradeDebtRow>,
    pub(super) blocking_predicates: Vec<S1BlockingPredicateRow>,
    pub(super) gate_readiness: RoadmapGateReadinessWitness,
    pub(super) accepted_evidence_provenance: S0AcceptedEvidenceProvenance,
}

impl StorageFoundationS1Handoff {
    #[allow(clippy::too_many_arguments)]
    pub fn from_accepted_inputs(
        source_revision: impl Into<String>,
        roadmap_parent_digest: super::super::evidence::S0StableDigest,
        generated_by: impl Into<String>,
        nondeterministic_metadata: S0NondeterministicMetadata,
        backend_matrix: &BackendCapabilityMatrix,
        deferred_map: &DeferredPhysicalGuaranteeMap,
        terminology_report: &TerminologyRiskReport,
        manifest: &S0AuditInputManifest,
        complexity_report: &S0ComplexityContractReport,
        harness_report: &HarnessMaturityReport,
        gate_readiness: RoadmapGateReadinessWitness,
        release_claim_report: &ReleaseClaimReport,
        available_fixtures: &[S1CompileTimeBoundaryFixture],
    ) -> Result<Self, S0S1HandoffBuildRejection> {
        let source_revision = require_non_empty(source_revision)
            .map_err(|_| S0S1HandoffBuildRejection::EmptyRequiredField)?;
        let generated_by = require_non_empty(generated_by)
            .map_err(|_| S0S1HandoffBuildRejection::EmptyRequiredField)?;
        validate_accepted_inputs(AcceptedInputValidation {
            source_revision: &source_revision,
            roadmap_parent_digest: &roadmap_parent_digest,
            backend_matrix,
            deferred_map,
            terminology_report,
            manifest,
            harness_report,
            release_claim_report,
        })?;
        let requirements = derive_handoff_requirements(HandoffRequirementsInput {
            backend_matrix,
            deferred_map,
            available_fixtures,
        })?;
        let complexity_contract_summary_digest = complexity_summary_digest(complexity_report)
            .map_err(|_| S0S1HandoffBuildRejection::InvalidDigest)?;
        let accepted_evidence_provenance = S0AcceptedEvidenceProvenance::from_parts(
            source_revision.clone(),
            roadmap_parent_digest.clone(),
            manifest.manifest_digest().clone(),
        );
        assemble_handoff(HandoffAssemblyInputs {
            source_revision,
            roadmap_parent_digest,
            generated_by,
            nondeterministic_metadata,
            backend_tier_matrix_digest: backend_matrix.envelope().deterministic_digest().clone(),
            deferred_guarantee_map_digest: deferred_map.envelope().deterministic_digest().clone(),
            terminology_scan_digest: terminology_report.scan_digest().clone(),
            audit_input_manifest_digest: manifest.manifest_digest().clone(),
            complexity_contract_summary_digest,
            requirements,
            gate_readiness,
            accepted_evidence_provenance,
        })
    }

    pub fn envelope(&self) -> &S0ArtifactEnvelopeMetadata {
        &self.envelope
    }

    pub fn allowed_backend_candidates(&self) -> &[String] {
        &self.allowed_backend_candidates
    }

    pub fn compile_time_boundary_fixtures(&self) -> &[S1CompileTimeBoundaryFixtureStatusRow] {
        &self.compile_time_boundary_fixtures
    }

    pub fn blocking_predicates(&self) -> &[S1BlockingPredicateRow] {
        &self.blocking_predicates
    }

    pub fn backend_tier_matrix_digest(&self) -> &super::super::evidence::S0StableDigest {
        &self.backend_tier_matrix_digest
    }

    pub fn deferred_guarantee_map_digest(&self) -> &super::super::evidence::S0StableDigest {
        &self.deferred_guarantee_map_digest
    }

    pub fn terminology_scan_digest(&self) -> &super::super::evidence::S0StableDigest {
        &self.terminology_scan_digest
    }

    pub fn audit_input_manifest_digest(&self) -> &super::super::evidence::S0StableDigest {
        &self.audit_input_manifest_digest
    }

    pub fn complexity_contract_summary_digest(&self) -> &super::super::evidence::S0StableDigest {
        &self.complexity_contract_summary_digest
    }

    pub fn accepted_evidence_provenance(&self) -> &S0AcceptedEvidenceProvenance {
        &self.accepted_evidence_provenance
    }

    pub fn gate_readiness(&self) -> &RoadmapGateReadinessWitness {
        &self.gate_readiness
    }
}

struct HandoffAssemblyInputs {
    source_revision: String,
    roadmap_parent_digest: super::super::evidence::S0StableDigest,
    generated_by: String,
    nondeterministic_metadata: S0NondeterministicMetadata,
    backend_tier_matrix_digest: super::super::evidence::S0StableDigest,
    deferred_guarantee_map_digest: super::super::evidence::S0StableDigest,
    terminology_scan_digest: super::super::evidence::S0StableDigest,
    audit_input_manifest_digest: super::super::evidence::S0StableDigest,
    complexity_contract_summary_digest: super::super::evidence::S0StableDigest,
    requirements: DerivedHandoffRequirements,
    gate_readiness: RoadmapGateReadinessWitness,
    accepted_evidence_provenance: S0AcceptedEvidenceProvenance,
}

fn assemble_handoff(
    inputs: HandoffAssemblyInputs,
) -> Result<StorageFoundationS1Handoff, S0S1HandoffBuildRejection> {
    let deterministic_digest = assembly_digest(&inputs)?;
    Ok(StorageFoundationS1Handoff {
        envelope: S0ArtifactEnvelopeMetadata::new(
            super::super::evidence::S0ArtifactKind::S1HandoffReadiness,
            inputs.source_revision,
            inputs.roadmap_parent_digest,
            inputs.generated_by,
            deterministic_digest,
            inputs.nondeterministic_metadata,
        ),
        backend_tier_matrix_digest: inputs.backend_tier_matrix_digest,
        deferred_guarantee_map_digest: inputs.deferred_guarantee_map_digest,
        terminology_scan_digest: inputs.terminology_scan_digest,
        audit_input_manifest_digest: inputs.audit_input_manifest_digest,
        complexity_contract_summary_digest: inputs.complexity_contract_summary_digest,
        required_forbidden_shortcuts: inputs.requirements.required_forbidden_shortcuts,
        required_harness_subsystems: inputs.requirements.required_harness_subsystems,
        allowed_backend_candidates: inputs.requirements.allowed_backend_candidates,
        legacy_backend_fences: inputs.requirements.legacy_backend_fences,
        compile_time_boundary_fixtures: inputs.requirements.compile_time_boundary_fixtures,
        non_platform_grade_debt_rows: inputs.requirements.non_platform_grade_debt_rows,
        blocking_predicates: inputs.requirements.blocking_predicates,
        gate_readiness: inputs.gate_readiness,
        accepted_evidence_provenance: inputs.accepted_evidence_provenance,
    })
}

fn assembly_digest(
    inputs: &HandoffAssemblyInputs,
) -> Result<super::super::evidence::S0StableDigest, S0S1HandoffBuildRejection> {
    stable_digest(&S1HandoffDigestBasis {
        schema_version: S0_ARTIFACT_SCHEMA_VERSION,
        artifact_kind: super::super::evidence::S0ArtifactKind::S1HandoffReadiness,
        source_revision: &inputs.source_revision,
        roadmap_parent_digest: &inputs.roadmap_parent_digest,
        generated_by: &inputs.generated_by,
        backend_tier_matrix_digest: &inputs.backend_tier_matrix_digest,
        deferred_guarantee_map_digest: &inputs.deferred_guarantee_map_digest,
        terminology_scan_digest: &inputs.terminology_scan_digest,
        audit_input_manifest_digest: &inputs.audit_input_manifest_digest,
        complexity_contract_summary_digest: &inputs.complexity_contract_summary_digest,
        required_forbidden_shortcuts: &inputs.requirements.required_forbidden_shortcuts,
        required_harness_subsystems: &inputs.requirements.required_harness_subsystems,
        allowed_backend_candidates: &inputs.requirements.allowed_backend_candidates,
        legacy_backend_fences: &inputs.requirements.legacy_backend_fences,
        compile_time_boundary_fixtures: &inputs.requirements.compile_time_boundary_fixtures,
        non_platform_grade_debt_rows: &inputs.requirements.non_platform_grade_debt_rows,
        blocking_predicates: &inputs.requirements.blocking_predicates,
        gate_readiness: &inputs.gate_readiness,
        accepted_evidence_provenance: &inputs.accepted_evidence_provenance,
    })
    .map_err(|_| S0S1HandoffBuildRejection::InvalidDigest)
}
