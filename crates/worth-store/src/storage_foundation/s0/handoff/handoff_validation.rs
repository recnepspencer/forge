use super::super::artifacts::{BackendCapabilityMatrix, S0ArtifactEnvelopeMetadata};
use super::super::counters::S0ComplexityContractReport;
use super::super::deferred::DeferredPhysicalGuaranteeMap;
use super::super::evidence::{S0ArtifactKind, S0StableDigest};
use super::super::harness::HarnessMaturityReport;
use super::super::harness::S1ForbiddenShortcut;
use super::super::manifest::S0AuditInputManifest;
use super::super::milestones::RoadmapGateReadinessWitness;
use super::super::terminology::{ReleaseClaimReport, TerminologyAllowedUse, TerminologyRiskReport};
use super::accepted_evidence_provenance::S0AcceptedEvidenceProvenance;
use super::compile_time_boundary_rows::S1CompileTimeBoundaryFixtureStatusRow;
use super::s1_blocking_predicate::S1BlockingPredicateRow;
use super::sequence_harness_dependency::SequenceHarnessDependency;

#[derive(Debug, Clone, Copy, PartialEq, Eq, serde::Serialize)]
pub enum S0S1HandoffBuildRejection {
    EmptyRequiredField,
    MissingHarnessReadinessRows,
    MissingForbiddenShortcutList,
    MissingAllowedBackendCandidate,
    StaleAcceptedInput,
    BlockingPredicate(super::s1_blocking_predicate::S1BlockingPredicate),
    InvalidDigest,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, serde::Serialize)]
pub enum S0S1HandoffParseRejection {
    NonParseable,
    SerializationFailed,
    SchemaVersionMismatch,
    ArtifactKindMismatch,
    InvalidDigest,
    HandoffBuildRejected(S0S1HandoffBuildRejection),
    DeterministicDigestMismatch,
}

impl From<S0S1HandoffBuildRejection> for S0S1HandoffParseRejection {
    fn from(value: S0S1HandoffBuildRejection) -> Self {
        Self::HandoffBuildRejected(value)
    }
}

pub(super) struct SharedProvenanceInputs<'a> {
    pub(super) expected_source_revision: &'a str,
    pub(super) expected_roadmap_parent_digest: &'a S0StableDigest,
    pub(super) backend_envelope: &'a S0ArtifactEnvelopeMetadata,
    pub(super) deferred_envelope: &'a S0ArtifactEnvelopeMetadata,
    pub(super) terminology_envelope: &'a S0ArtifactEnvelopeMetadata,
    pub(super) harness_envelope: &'a S0ArtifactEnvelopeMetadata,
    pub(super) manifest: &'a S0AuditInputManifest,
}

pub(super) fn enforce_shared_provenance(
    inputs: SharedProvenanceInputs<'_>,
) -> Result<(), S0S1HandoffBuildRejection> {
    let same_revision = [
        inputs.backend_envelope.source_revision(),
        inputs.deferred_envelope.source_revision(),
        inputs.terminology_envelope.source_revision(),
        inputs.harness_envelope.source_revision(),
        inputs.manifest.source_revision(),
    ]
    .into_iter()
    .all(|revision| revision == inputs.expected_source_revision);
    let same_roadmap = [
        inputs.backend_envelope.roadmap_parent_digest(),
        inputs.deferred_envelope.roadmap_parent_digest(),
        inputs.terminology_envelope.roadmap_parent_digest(),
        inputs.harness_envelope.roadmap_parent_digest(),
    ]
    .into_iter()
    .all(|digest| digest == inputs.expected_roadmap_parent_digest);
    if same_revision && same_roadmap {
        Ok(())
    } else {
        Err(S0S1HandoffBuildRejection::StaleAcceptedInput)
    }
}

pub(super) struct AcceptedInputValidation<'a> {
    pub(super) source_revision: &'a str,
    pub(super) roadmap_parent_digest: &'a S0StableDigest,
    pub(super) backend_matrix: &'a BackendCapabilityMatrix,
    pub(super) deferred_map: &'a DeferredPhysicalGuaranteeMap,
    pub(super) terminology_report: &'a TerminologyRiskReport,
    pub(super) manifest: &'a S0AuditInputManifest,
    pub(super) harness_report: &'a HarnessMaturityReport,
    pub(super) release_claim_report: &'a ReleaseClaimReport,
}

pub(super) fn validate_accepted_inputs(
    inputs: AcceptedInputValidation<'_>,
) -> Result<(), S0S1HandoffBuildRejection> {
    enforce_shared_provenance(SharedProvenanceInputs {
        expected_source_revision: inputs.source_revision,
        expected_roadmap_parent_digest: inputs.roadmap_parent_digest,
        backend_envelope: inputs.backend_matrix.envelope(),
        deferred_envelope: inputs.deferred_map.envelope(),
        terminology_envelope: inputs.terminology_report.envelope(),
        harness_envelope: inputs.harness_report.envelope(),
        manifest: inputs.manifest,
    })?;
    if inputs.harness_report.rows().is_empty() {
        return Err(S0S1HandoffBuildRejection::MissingHarnessReadinessRows);
    }
    if inputs.terminology_report.rows().iter().any(|row| {
        matches!(
            row.allowed_use(),
            TerminologyAllowedUse::OverclaimedPhysicalPosture
        )
    }) {
        return Err(S0S1HandoffBuildRejection::BlockingPredicate(
            super::s1_blocking_predicate::S1BlockingPredicate::OverclaimedPhysicalPosturePresent,
        ));
    }
    if inputs.release_claim_report.scanned_surface_count() == 0 {
        return Err(S0S1HandoffBuildRejection::MissingForbiddenShortcutList);
    }
    Ok(())
}

#[derive(serde::Serialize)]
pub(super) struct S1HandoffDigestBasis<'a> {
    pub(super) schema_version: &'static str,
    pub(super) artifact_kind: S0ArtifactKind,
    pub(super) source_revision: &'a str,
    pub(super) roadmap_parent_digest: &'a S0StableDigest,
    pub(super) generated_by: &'a str,
    pub(super) backend_tier_matrix_digest: &'a S0StableDigest,
    pub(super) deferred_guarantee_map_digest: &'a S0StableDigest,
    pub(super) terminology_scan_digest: &'a S0StableDigest,
    pub(super) audit_input_manifest_digest: &'a S0StableDigest,
    pub(super) complexity_contract_summary_digest: &'a S0StableDigest,
    pub(super) required_forbidden_shortcuts: &'a [S1ForbiddenShortcut],
    pub(super) required_harness_subsystems: &'a [SequenceHarnessDependency],
    pub(super) allowed_backend_candidates: &'a [String],
    pub(super) legacy_backend_fences: &'a [String],
    pub(super) compile_time_boundary_fixtures: &'a [S1CompileTimeBoundaryFixtureStatusRow],
    pub(super) non_platform_grade_debt_rows:
        &'a [super::compile_time_boundary_rows::S1NonPlatformGradeDebtRow],
    pub(super) blocking_predicates: &'a [S1BlockingPredicateRow],
    pub(super) gate_readiness: &'a RoadmapGateReadinessWitness,
    pub(super) accepted_evidence_provenance: &'a S0AcceptedEvidenceProvenance,
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
    use sha2::{Digest, Sha256};
    let mut hasher = Sha256::new();
    hasher.update(value);
    S0StableDigest::new(format!("{:x}", hasher.finalize()))
        .map_err(|_| serde_json::Error::io(std::io::Error::other("invalid digest")))
}

pub(super) fn stable_digest<T: serde::Serialize + ?Sized>(
    value: &T,
) -> Result<S0StableDigest, serde_json::Error> {
    let value = serde_json::to_vec(value)?;
    use sha2::{Digest, Sha256};
    let mut hasher = Sha256::new();
    hasher.update(value);
    S0StableDigest::new(format!("{:x}", hasher.finalize()))
        .map_err(|_| serde_json::Error::io(std::io::Error::other("invalid digest")))
}

pub(super) fn require_non_empty(value: impl Into<String>) -> Result<String, String> {
    let value = value.into();
    if value.trim().is_empty() {
        return Err(value);
    }
    Ok(value)
}
