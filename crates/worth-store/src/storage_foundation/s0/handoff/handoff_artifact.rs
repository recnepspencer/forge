use super::super::artifacts::{
    S0ArtifactEnvelopeMetadata, S0ArtifactValidationCostSurface, S0NondeterministicMetadata,
    S0_ARTIFACT_SCHEMA_VERSION,
};
use super::super::evidence::{S0ArtifactKind, S0StableDigest};
use super::super::harness::S1ForbiddenShortcut;
use super::super::milestones::RoadmapGateReadinessWitness;
use super::accepted_evidence_provenance::S0AcceptedEvidenceProvenance;
use super::compile_time_boundary_rows::{
    S1CompileTimeBoundaryFixtureStatusRow, S1NonPlatformGradeDebtRow,
};
use super::handoff_raw_schema::RawStorageFoundationS1Handoff;
use super::handoff_validation::{
    stable_digest, S0S1HandoffBuildRejection, S0S1HandoffParseRejection, S1HandoffDigestBasis,
};
use super::s1_blocking_predicate::S1BlockingPredicateRow;
use super::sequence_harness_dependency::SequenceHarnessDependency;
use super::storage_foundation_s1_handoff::StorageFoundationS1Handoff;

#[derive(Debug, Clone, PartialEq, Eq, serde::Serialize)]
pub struct S0ValidatedStorageFoundationS1HandoffArtifact {
    handoff: StorageFoundationS1Handoff,
    validation_cost: S0ArtifactValidationCostSurface,
}

impl S0ValidatedStorageFoundationS1HandoffArtifact {
    pub fn handoff(&self) -> &StorageFoundationS1Handoff {
        &self.handoff
    }

    pub fn validation_cost(&self) -> &S0ArtifactValidationCostSurface {
        &self.validation_cost
    }
}

impl StorageFoundationS1Handoff {
    pub fn to_canonical_json_bytes(&self) -> Result<Vec<u8>, S0S1HandoffParseRejection> {
        serde_json::to_vec_pretty(self).map_err(|_| S0S1HandoffParseRejection::SerializationFailed)
    }

    pub fn validate_canonical_json_bytes(
        bytes: &[u8],
    ) -> Result<S0ValidatedStorageFoundationS1HandoffArtifact, S0S1HandoffParseRejection> {
        let (raw, roadmap_parent_digest, expected_digest) = parse_handoff_envelope(bytes)?;
        let handoff = Self::from_parts(HandoffConstructionParts::from_raw(
            raw,
            roadmap_parent_digest,
        )?)?;
        if handoff.envelope().deterministic_digest() != &expected_digest {
            return Err(S0S1HandoffParseRejection::DeterministicDigestMismatch);
        }
        let byte_count = serde_json::to_vec(&handoff)
            .map_err(|_| S0S1HandoffParseRejection::SerializationFailed)?
            .len() as u64;
        Ok(S0ValidatedStorageFoundationS1HandoffArtifact {
            handoff,
            validation_cost: S0ArtifactValidationCostSurface::new(
                bytes.len() as u64,
                1,
                byte_count,
                1,
            ),
        })
    }

    fn from_parts(parts: HandoffConstructionParts) -> Result<Self, S0S1HandoffParseRejection> {
        let digests = parse_handoff_digests(&parts)?;
        if parts.required_forbidden_shortcuts.is_empty() {
            return Err(S0S1HandoffParseRejection::HandoffBuildRejected(
                S0S1HandoffBuildRejection::MissingForbiddenShortcutList,
            ));
        }
        if parts.required_harness_subsystems.is_empty() {
            return Err(S0S1HandoffParseRejection::HandoffBuildRejected(
                S0S1HandoffBuildRejection::MissingHarnessReadinessRows,
            ));
        }
        let deterministic_digest = construction_digest(&parts, &digests)?;
        Ok(Self {
            envelope: S0ArtifactEnvelopeMetadata::new(
                S0ArtifactKind::S1HandoffReadiness,
                parts.source_revision,
                parts.roadmap_parent_digest,
                parts.generated_by,
                deterministic_digest,
                parts.nondeterministic_metadata,
            ),
            backend_tier_matrix_digest: digests.backend_tier_matrix_digest,
            deferred_guarantee_map_digest: digests.deferred_guarantee_map_digest,
            terminology_scan_digest: digests.terminology_scan_digest,
            audit_input_manifest_digest: digests.audit_input_manifest_digest,
            complexity_contract_summary_digest: digests.complexity_contract_summary_digest,
            required_forbidden_shortcuts: parts.required_forbidden_shortcuts,
            required_harness_subsystems: parts.required_harness_subsystems,
            allowed_backend_candidates: parts.allowed_backend_candidates,
            legacy_backend_fences: parts.legacy_backend_fences,
            compile_time_boundary_fixtures: parts.compile_time_boundary_fixtures,
            non_platform_grade_debt_rows: parts.non_platform_grade_debt_rows,
            blocking_predicates: parts.blocking_predicates,
            gate_readiness: parts.gate_readiness,
            accepted_evidence_provenance: parts.accepted_evidence_provenance,
        })
    }
}

struct ParsedHandoffDigests {
    backend_tier_matrix_digest: S0StableDigest,
    deferred_guarantee_map_digest: S0StableDigest,
    terminology_scan_digest: S0StableDigest,
    audit_input_manifest_digest: S0StableDigest,
    complexity_contract_summary_digest: S0StableDigest,
}

fn parse_handoff_digests(
    parts: &HandoffConstructionParts,
) -> Result<ParsedHandoffDigests, S0S1HandoffParseRejection> {
    Ok(ParsedHandoffDigests {
        backend_tier_matrix_digest: S0StableDigest::new(parts.backend_tier_matrix_digest.clone())
            .map_err(|_| S0S1HandoffParseRejection::InvalidDigest)?,
        deferred_guarantee_map_digest: S0StableDigest::new(
            parts.deferred_guarantee_map_digest.clone(),
        )
        .map_err(|_| S0S1HandoffParseRejection::InvalidDigest)?,
        terminology_scan_digest: S0StableDigest::new(parts.terminology_scan_digest.clone())
            .map_err(|_| S0S1HandoffParseRejection::InvalidDigest)?,
        audit_input_manifest_digest: S0StableDigest::new(parts.audit_input_manifest_digest.clone())
            .map_err(|_| S0S1HandoffParseRejection::InvalidDigest)?,
        complexity_contract_summary_digest: S0StableDigest::new(
            parts.complexity_contract_summary_digest.clone(),
        )
        .map_err(|_| S0S1HandoffParseRejection::InvalidDigest)?,
    })
}

fn construction_digest(
    parts: &HandoffConstructionParts,
    digests: &ParsedHandoffDigests,
) -> Result<S0StableDigest, S0S1HandoffParseRejection> {
    stable_digest(&S1HandoffDigestBasis {
        schema_version: S0_ARTIFACT_SCHEMA_VERSION,
        artifact_kind: S0ArtifactKind::S1HandoffReadiness,
        source_revision: &parts.source_revision,
        roadmap_parent_digest: &parts.roadmap_parent_digest,
        generated_by: &parts.generated_by,
        backend_tier_matrix_digest: &digests.backend_tier_matrix_digest,
        deferred_guarantee_map_digest: &digests.deferred_guarantee_map_digest,
        terminology_scan_digest: &digests.terminology_scan_digest,
        audit_input_manifest_digest: &digests.audit_input_manifest_digest,
        complexity_contract_summary_digest: &digests.complexity_contract_summary_digest,
        required_forbidden_shortcuts: &parts.required_forbidden_shortcuts,
        required_harness_subsystems: &parts.required_harness_subsystems,
        allowed_backend_candidates: &parts.allowed_backend_candidates,
        legacy_backend_fences: &parts.legacy_backend_fences,
        compile_time_boundary_fixtures: &parts.compile_time_boundary_fixtures,
        non_platform_grade_debt_rows: &parts.non_platform_grade_debt_rows,
        blocking_predicates: &parts.blocking_predicates,
        gate_readiness: &parts.gate_readiness,
        accepted_evidence_provenance: &parts.accepted_evidence_provenance,
    })
    .map_err(|_| S0S1HandoffParseRejection::InvalidDigest)
}

struct HandoffConstructionParts {
    source_revision: String,
    roadmap_parent_digest: S0StableDigest,
    generated_by: String,
    nondeterministic_metadata: S0NondeterministicMetadata,
    backend_tier_matrix_digest: String,
    deferred_guarantee_map_digest: String,
    terminology_scan_digest: String,
    audit_input_manifest_digest: String,
    complexity_contract_summary_digest: String,
    required_forbidden_shortcuts: Vec<S1ForbiddenShortcut>,
    required_harness_subsystems: Vec<SequenceHarnessDependency>,
    allowed_backend_candidates: Vec<String>,
    legacy_backend_fences: Vec<String>,
    compile_time_boundary_fixtures: Vec<S1CompileTimeBoundaryFixtureStatusRow>,
    non_platform_grade_debt_rows: Vec<S1NonPlatformGradeDebtRow>,
    blocking_predicates: Vec<S1BlockingPredicateRow>,
    gate_readiness: RoadmapGateReadinessWitness,
    accepted_evidence_provenance: S0AcceptedEvidenceProvenance,
}

impl HandoffConstructionParts {
    fn from_raw(
        raw: RawStorageFoundationS1Handoff,
        roadmap_parent_digest: S0StableDigest,
    ) -> Result<Self, S0S1HandoffParseRejection> {
        Ok(Self {
            source_revision: raw.envelope.source_revision,
            roadmap_parent_digest,
            generated_by: raw.envelope.generated_by,
            nondeterministic_metadata: raw.envelope.nondeterministic_metadata.into_validated()?,
            backend_tier_matrix_digest: raw.backend_tier_matrix_digest,
            deferred_guarantee_map_digest: raw.deferred_guarantee_map_digest,
            terminology_scan_digest: raw.terminology_scan_digest,
            audit_input_manifest_digest: raw.audit_input_manifest_digest,
            complexity_contract_summary_digest: raw.complexity_contract_summary_digest,
            required_forbidden_shortcuts: raw.required_forbidden_shortcuts,
            required_harness_subsystems: raw
                .required_harness_subsystems
                .into_iter()
                .map(super::handoff_raw_schema::RawSequenceHarnessDependency::into_validated)
                .collect::<Result<Vec<_>, _>>()?,
            allowed_backend_candidates: raw.allowed_backend_candidates,
            legacy_backend_fences: raw.legacy_backend_fences,
            compile_time_boundary_fixtures: raw
                .compile_time_boundary_fixtures
                .into_iter()
                .map(
                    super::handoff_raw_schema::RawS1CompileTimeBoundaryFixtureStatusRow::into_validated,
                )
                .collect::<Vec<_>>(),
            non_platform_grade_debt_rows: raw
                .non_platform_grade_debt_rows
                .into_iter()
                .map(super::handoff_raw_schema::RawS1NonPlatformGradeDebtRow::into_validated)
                .collect::<Result<Vec<_>, _>>()?,
            blocking_predicates: raw.blocking_predicates,
            gate_readiness: raw.gate_readiness.into_validated(),
            accepted_evidence_provenance: raw.accepted_evidence_provenance.into_validated()?,
        })
    }
}

fn parse_handoff_envelope(
    bytes: &[u8],
) -> Result<
    (
        RawStorageFoundationS1Handoff,
        S0StableDigest,
        S0StableDigest,
    ),
    S0S1HandoffParseRejection,
> {
    let raw = serde_json::from_slice::<RawStorageFoundationS1Handoff>(bytes)
        .map_err(|_| S0S1HandoffParseRejection::NonParseable)?;
    if raw.envelope.schema_version != S0_ARTIFACT_SCHEMA_VERSION {
        return Err(S0S1HandoffParseRejection::SchemaVersionMismatch);
    }
    if raw.envelope.artifact_kind != S0ArtifactKind::S1HandoffReadiness {
        return Err(S0S1HandoffParseRejection::ArtifactKindMismatch);
    }
    let roadmap_parent_digest = S0StableDigest::new(raw.envelope.roadmap_parent_digest.clone())
        .map_err(|_| S0S1HandoffParseRejection::InvalidDigest)?;
    let expected_digest = S0StableDigest::new(raw.envelope.deterministic_digest.clone())
        .map_err(|_| S0S1HandoffParseRejection::InvalidDigest)?;
    Ok((raw, roadmap_parent_digest, expected_digest))
}
