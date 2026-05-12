use super::artifacts::{
    BackendCapabilityMatrix, S0ArtifactEnvelopeMetadata, S0ArtifactValidationCostSurface,
    S0NondeterministicMetadata, S0_ARTIFACT_SCHEMA_VERSION,
};
use super::counters::S0ComplexityContractReport;
use super::deferred::DeferredPhysicalGuaranteeMap;
use super::evidence::{S0ArtifactKind, S0StableDigest};
use super::harness::{
    HarnessMaturityLevel, HarnessMaturityReport, HarnessSubsystemMaturity,
    S1CompileTimeBoundaryFixture, S1CompileTimeBoundaryStatus, S1ForbiddenShortcut,
};
use super::manifest::S0AuditInputManifest;
use super::milestones::RoadmapGateReadinessWitness;
use super::terminology::{ReleaseClaimReport, TerminologyAllowedUse, TerminologyRiskReport};
use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};
use std::collections::BTreeSet;

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct SequenceHarnessDependency {
    sequence_id: super::capability::Roadmap2SequenceId,
    subsystem: HarnessSubsystemMaturity,
    minimum_level: HarnessMaturityLevel,
}

impl SequenceHarnessDependency {
    pub fn new(
        sequence_id: super::capability::Roadmap2SequenceId,
        subsystem: HarnessSubsystemMaturity,
        minimum_level: HarnessMaturityLevel,
    ) -> Self {
        Self {
            sequence_id,
            subsystem,
            minimum_level,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct S0AcceptedEvidenceProvenance {
    source_revision: String,
    roadmap_parent_digest: S0StableDigest,
    audit_input_manifest_digest: S0StableDigest,
}

impl S0AcceptedEvidenceProvenance {
    pub fn source_revision(&self) -> &str {
        &self.source_revision
    }

    pub fn roadmap_parent_digest(&self) -> &S0StableDigest {
        &self.roadmap_parent_digest
    }

    pub fn audit_input_manifest_digest(&self) -> &S0StableDigest {
        &self.audit_input_manifest_digest
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct S1NonPlatformGradeDebtRow {
    subject: String,
    deferred_s_sequences: Vec<super::capability::Roadmap2SequenceId>,
    required_wording: String,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct S1CompileTimeBoundaryFixtureStatusRow {
    fixture: S1CompileTimeBoundaryFixture,
    status: S1CompileTimeBoundaryStatus,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum S1BlockingPredicate {
    MissingBackendTierMatrix,
    MissingDeferredGuaranteeMap,
    MissingTerminologyScanDigest,
    MissingForbiddenShortcutList,
    MissingHarnessReadinessRows,
    OverclaimedPhysicalPosturePresent,
    UnmappedDeferredGuaranteePresent,
    StaleAcceptedInput,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum S1BlockingPredicateStatus {
    Satisfied,
    Blocking,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct S1BlockingPredicateRow {
    predicate: S1BlockingPredicate,
    status: S1BlockingPredicateStatus,
}

impl S1BlockingPredicateRow {
    pub fn predicate(&self) -> S1BlockingPredicate {
        self.predicate
    }

    pub fn status(&self) -> S1BlockingPredicateStatus {
        self.status
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct StorageFoundationS1Handoff {
    #[serde(flatten)]
    envelope: S0ArtifactEnvelopeMetadata,
    backend_tier_matrix_digest: S0StableDigest,
    deferred_guarantee_map_digest: S0StableDigest,
    terminology_scan_digest: S0StableDigest,
    audit_input_manifest_digest: S0StableDigest,
    complexity_contract_summary_digest: S0StableDigest,
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

impl StorageFoundationS1Handoff {
    #[allow(clippy::too_many_arguments)]
    pub fn from_accepted_inputs(
        source_revision: impl Into<String>,
        roadmap_parent_digest: S0StableDigest,
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
        enforce_shared_provenance(
            &source_revision,
            &roadmap_parent_digest,
            backend_matrix.envelope(),
            deferred_map.envelope(),
            terminology_report.envelope(),
            harness_report.envelope(),
            manifest,
        )?;
        if harness_report.rows().is_empty() {
            return Err(S0S1HandoffBuildRejection::MissingHarnessReadinessRows);
        }
        if terminology_report.rows().iter().any(|row| {
            matches!(
                row.allowed_use(),
                TerminologyAllowedUse::OverclaimedPhysicalPosture
            )
        }) {
            return Err(S0S1HandoffBuildRejection::BlockingPredicate(
                S1BlockingPredicate::OverclaimedPhysicalPosturePresent,
            ));
        }
        if release_claim_report.scanned_surface_count() == 0 {
            return Err(S0S1HandoffBuildRejection::MissingForbiddenShortcutList);
        }
        let required_forbidden_shortcuts = vec![
            S1ForbiddenShortcut::OverclaimedPhysicalPosture,
            S1ForbiddenShortcut::BackendTierMismatch,
            S1ForbiddenShortcut::UnmappedDeferredGuarantee,
            S1ForbiddenShortcut::MissingMilestonePhysicalStatusRow,
        ];
        let required_harness_subsystems = vec![
            SequenceHarnessDependency::new(
                super::capability::Roadmap2SequenceId::new("S1")
                    .map_err(|_| S0S1HandoffBuildRejection::EmptyRequiredField)?,
                HarnessSubsystemMaturity::TerminologyClaimGate,
                HarnessMaturityLevel::Exists,
            ),
            SequenceHarnessDependency::new(
                super::capability::Roadmap2SequenceId::new("S1")
                    .map_err(|_| S0S1HandoffBuildRejection::EmptyRequiredField)?,
                HarnessSubsystemMaturity::DeferredGuaranteeValidation,
                HarnessMaturityLevel::Exists,
            ),
            SequenceHarnessDependency::new(
                super::capability::Roadmap2SequenceId::new("S1")
                    .map_err(|_| S0S1HandoffBuildRejection::EmptyRequiredField)?,
                HarnessSubsystemMaturity::CompileTimeBoundaryFixtures,
                HarnessMaturityLevel::Exists,
            ),
        ];
        let allowed_backend_candidates = backend_matrix
            .rows()
            .iter()
            .filter(|row| {
                matches!(
                    row.capability_tier(),
                    super::capability::StoreBackendCapabilityTier::PhysicalFoundation
                        | super::capability::StoreBackendCapabilityTier::PlatformGrade
                )
            })
            .map(|row| row.subject_path_or_symbol().to_string())
            .collect::<Vec<_>>();
        if allowed_backend_candidates.is_empty() {
            return Err(S0S1HandoffBuildRejection::MissingAllowedBackendCandidate);
        }
        let legacy_backend_fences = backend_matrix
            .rows()
            .iter()
            .filter(|row| {
                matches!(
                    row.capability_tier(),
                    super::capability::StoreBackendCapabilityTier::Bootstrap
                        | super::capability::StoreBackendCapabilityTier::SemanticCertification
                        | super::capability::StoreBackendCapabilityTier::Compatibility
                )
            })
            .map(|row| row.subject_path_or_symbol().to_string())
            .collect::<Vec<_>>();
        let compile_time_boundary_fixtures = compile_time_fixture_rows(available_fixtures);
        let non_platform_grade_debt_rows = backend_matrix
            .rows()
            .iter()
            .filter(|row| {
                row.capability_tier()
                    != super::capability::StoreBackendCapabilityTier::PlatformGrade
            })
            .filter(|row| !row.deferred_s_sequences().is_empty())
            .map(|row| S1NonPlatformGradeDebtRow {
                subject: row.subject_path_or_symbol().to_string(),
                deferred_s_sequences: row.deferred_s_sequences().to_vec(),
                required_wording:
                    "Legal only as explicit non-platform-grade debt until Roadmap 2 closes."
                        .to_string(),
            })
            .collect::<Vec<_>>();
        let blocking_predicates = vec![
            S1BlockingPredicateRow {
                predicate: S1BlockingPredicate::MissingBackendTierMatrix,
                status: S1BlockingPredicateStatus::Satisfied,
            },
            S1BlockingPredicateRow {
                predicate: S1BlockingPredicate::MissingDeferredGuaranteeMap,
                status: if deferred_map.rows().is_empty() {
                    S1BlockingPredicateStatus::Blocking
                } else {
                    S1BlockingPredicateStatus::Satisfied
                },
            },
            S1BlockingPredicateRow {
                predicate: S1BlockingPredicate::MissingTerminologyScanDigest,
                status: S1BlockingPredicateStatus::Satisfied,
            },
            S1BlockingPredicateRow {
                predicate: S1BlockingPredicate::MissingForbiddenShortcutList,
                status: S1BlockingPredicateStatus::Satisfied,
            },
            S1BlockingPredicateRow {
                predicate: S1BlockingPredicate::MissingHarnessReadinessRows,
                status: S1BlockingPredicateStatus::Satisfied,
            },
            S1BlockingPredicateRow {
                predicate: S1BlockingPredicate::OverclaimedPhysicalPosturePresent,
                status: S1BlockingPredicateStatus::Satisfied,
            },
            S1BlockingPredicateRow {
                predicate: S1BlockingPredicate::UnmappedDeferredGuaranteePresent,
                status: S1BlockingPredicateStatus::Satisfied,
            },
        ];
        let complexity_contract_summary_digest = complexity_summary_digest(complexity_report)
            .map_err(|_| S0S1HandoffBuildRejection::InvalidDigest)?;
        let accepted_evidence_provenance = S0AcceptedEvidenceProvenance {
            source_revision: source_revision.clone(),
            roadmap_parent_digest: roadmap_parent_digest.clone(),
            audit_input_manifest_digest: manifest.manifest_digest().clone(),
        };
        let deterministic_digest = stable_digest(&S1HandoffDigestBasis {
            schema_version: S0_ARTIFACT_SCHEMA_VERSION,
            artifact_kind: S0ArtifactKind::S1HandoffReadiness,
            source_revision: &source_revision,
            roadmap_parent_digest: &roadmap_parent_digest,
            generated_by: &generated_by,
            backend_tier_matrix_digest: backend_matrix.envelope().deterministic_digest(),
            deferred_guarantee_map_digest: deferred_map.envelope().deterministic_digest(),
            terminology_scan_digest: terminology_report.scan_digest(),
            audit_input_manifest_digest: manifest.manifest_digest(),
            complexity_contract_summary_digest: &complexity_contract_summary_digest,
            required_forbidden_shortcuts: &required_forbidden_shortcuts,
            required_harness_subsystems: &required_harness_subsystems,
            allowed_backend_candidates: &allowed_backend_candidates,
            legacy_backend_fences: &legacy_backend_fences,
            compile_time_boundary_fixtures: &compile_time_boundary_fixtures,
            non_platform_grade_debt_rows: &non_platform_grade_debt_rows,
            blocking_predicates: &blocking_predicates,
            gate_readiness: &gate_readiness,
            accepted_evidence_provenance: &accepted_evidence_provenance,
        })
        .map_err(|_| S0S1HandoffBuildRejection::InvalidDigest)?;
        Ok(Self {
            envelope: S0ArtifactEnvelopeMetadata::new(
                S0ArtifactKind::S1HandoffReadiness,
                source_revision,
                roadmap_parent_digest,
                generated_by,
                deterministic_digest,
                nondeterministic_metadata,
            ),
            backend_tier_matrix_digest: backend_matrix.envelope().deterministic_digest().clone(),
            deferred_guarantee_map_digest: deferred_map.envelope().deterministic_digest().clone(),
            terminology_scan_digest: terminology_report.scan_digest().clone(),
            audit_input_manifest_digest: manifest.manifest_digest().clone(),
            complexity_contract_summary_digest,
            required_forbidden_shortcuts,
            required_harness_subsystems,
            allowed_backend_candidates,
            legacy_backend_fences,
            compile_time_boundary_fixtures,
            non_platform_grade_debt_rows,
            blocking_predicates,
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

    pub fn backend_tier_matrix_digest(&self) -> &S0StableDigest {
        &self.backend_tier_matrix_digest
    }

    pub fn deferred_guarantee_map_digest(&self) -> &S0StableDigest {
        &self.deferred_guarantee_map_digest
    }

    pub fn terminology_scan_digest(&self) -> &S0StableDigest {
        &self.terminology_scan_digest
    }

    pub fn audit_input_manifest_digest(&self) -> &S0StableDigest {
        &self.audit_input_manifest_digest
    }

    pub fn complexity_contract_summary_digest(&self) -> &S0StableDigest {
        &self.complexity_contract_summary_digest
    }

    pub fn accepted_evidence_provenance(&self) -> &S0AcceptedEvidenceProvenance {
        &self.accepted_evidence_provenance
    }

    pub fn gate_readiness(&self) -> &RoadmapGateReadinessWitness {
        &self.gate_readiness
    }

    pub fn to_canonical_json_bytes(&self) -> Result<Vec<u8>, S0S1HandoffParseRejection> {
        serde_json::to_vec_pretty(self).map_err(|_| S0S1HandoffParseRejection::SerializationFailed)
    }

    pub fn validate_canonical_json_bytes(
        bytes: &[u8],
    ) -> Result<S0ValidatedStorageFoundationS1HandoffArtifact, S0S1HandoffParseRejection> {
        let raw = serde_json::from_slice::<RawStorageFoundationS1Handoff>(bytes)
            .map_err(|_| S0S1HandoffParseRejection::NonParseable)?;
        if raw.envelope.schema_version != S0_ARTIFACT_SCHEMA_VERSION {
            return Err(S0S1HandoffParseRejection::SchemaVersionMismatch);
        }
        if raw.envelope.artifact_kind != S0ArtifactKind::S1HandoffReadiness {
            return Err(S0S1HandoffParseRejection::ArtifactKindMismatch);
        }
        let roadmap_parent_digest = S0StableDigest::new(raw.envelope.roadmap_parent_digest)
            .map_err(|_| S0S1HandoffParseRejection::InvalidDigest)?;
        let expected_digest = S0StableDigest::new(raw.envelope.deterministic_digest)
            .map_err(|_| S0S1HandoffParseRejection::InvalidDigest)?;
        let handoff = StorageFoundationS1Handoff::from_parts(
            raw.envelope.source_revision,
            roadmap_parent_digest,
            raw.envelope.generated_by,
            raw.envelope.nondeterministic_metadata.into_validated()?,
            raw.backend_tier_matrix_digest,
            raw.deferred_guarantee_map_digest,
            raw.terminology_scan_digest,
            raw.audit_input_manifest_digest,
            raw.complexity_contract_summary_digest,
            raw.required_forbidden_shortcuts,
            raw.required_harness_subsystems
                .into_iter()
                .map(RawSequenceHarnessDependency::into_validated)
                .collect::<Result<Vec<_>, _>>()?,
            raw.allowed_backend_candidates,
            raw.legacy_backend_fences,
            raw.compile_time_boundary_fixtures
                .into_iter()
                .map(RawS1CompileTimeBoundaryFixtureStatusRow::into_validated)
                .collect::<Vec<_>>(),
            raw.non_platform_grade_debt_rows
                .into_iter()
                .map(RawS1NonPlatformGradeDebtRow::into_validated)
                .collect::<Result<Vec<_>, _>>()?,
            raw.blocking_predicates,
            raw.gate_readiness.into_validated(),
            raw.accepted_evidence_provenance.into_validated()?,
        )?;
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

    #[allow(clippy::too_many_arguments)]
    fn from_parts(
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
    ) -> Result<Self, S0S1HandoffParseRejection> {
        let backend_tier_matrix_digest = S0StableDigest::new(backend_tier_matrix_digest)
            .map_err(|_| S0S1HandoffParseRejection::InvalidDigest)?;
        let deferred_guarantee_map_digest = S0StableDigest::new(deferred_guarantee_map_digest)
            .map_err(|_| S0S1HandoffParseRejection::InvalidDigest)?;
        let terminology_scan_digest = S0StableDigest::new(terminology_scan_digest)
            .map_err(|_| S0S1HandoffParseRejection::InvalidDigest)?;
        let audit_input_manifest_digest = S0StableDigest::new(audit_input_manifest_digest)
            .map_err(|_| S0S1HandoffParseRejection::InvalidDigest)?;
        let complexity_contract_summary_digest =
            S0StableDigest::new(complexity_contract_summary_digest)
                .map_err(|_| S0S1HandoffParseRejection::InvalidDigest)?;
        if required_forbidden_shortcuts.is_empty() {
            return Err(S0S1HandoffParseRejection::HandoffBuildRejected(
                S0S1HandoffBuildRejection::MissingForbiddenShortcutList,
            ));
        }
        if required_harness_subsystems.is_empty() {
            return Err(S0S1HandoffParseRejection::HandoffBuildRejected(
                S0S1HandoffBuildRejection::MissingHarnessReadinessRows,
            ));
        }
        let deterministic_digest = stable_digest(&S1HandoffDigestBasis {
            schema_version: S0_ARTIFACT_SCHEMA_VERSION,
            artifact_kind: S0ArtifactKind::S1HandoffReadiness,
            source_revision: &source_revision,
            roadmap_parent_digest: &roadmap_parent_digest,
            generated_by: &generated_by,
            backend_tier_matrix_digest: &backend_tier_matrix_digest,
            deferred_guarantee_map_digest: &deferred_guarantee_map_digest,
            terminology_scan_digest: &terminology_scan_digest,
            audit_input_manifest_digest: &audit_input_manifest_digest,
            complexity_contract_summary_digest: &complexity_contract_summary_digest,
            required_forbidden_shortcuts: &required_forbidden_shortcuts,
            required_harness_subsystems: &required_harness_subsystems,
            allowed_backend_candidates: &allowed_backend_candidates,
            legacy_backend_fences: &legacy_backend_fences,
            compile_time_boundary_fixtures: &compile_time_boundary_fixtures,
            non_platform_grade_debt_rows: &non_platform_grade_debt_rows,
            blocking_predicates: &blocking_predicates,
            gate_readiness: &gate_readiness,
            accepted_evidence_provenance: &accepted_evidence_provenance,
        })
        .map_err(|_| S0S1HandoffParseRejection::InvalidDigest)?;
        Ok(Self {
            envelope: S0ArtifactEnvelopeMetadata::new(
                S0ArtifactKind::S1HandoffReadiness,
                source_revision,
                roadmap_parent_digest,
                generated_by,
                deterministic_digest,
                nondeterministic_metadata,
            ),
            backend_tier_matrix_digest,
            deferred_guarantee_map_digest,
            terminology_scan_digest,
            audit_input_manifest_digest,
            complexity_contract_summary_digest,
            required_forbidden_shortcuts,
            required_harness_subsystems,
            allowed_backend_candidates,
            legacy_backend_fences,
            compile_time_boundary_fixtures,
            non_platform_grade_debt_rows,
            blocking_predicates,
            gate_readiness,
            accepted_evidence_provenance,
        })
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
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

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize)]
pub enum S0S1HandoffBuildRejection {
    EmptyRequiredField,
    MissingHarnessReadinessRows,
    MissingForbiddenShortcutList,
    MissingAllowedBackendCandidate,
    StaleAcceptedInput,
    BlockingPredicate(S1BlockingPredicate),
    InvalidDigest,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize)]
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

#[derive(Serialize)]
struct S1HandoffDigestBasis<'a> {
    schema_version: &'static str,
    artifact_kind: S0ArtifactKind,
    source_revision: &'a str,
    roadmap_parent_digest: &'a S0StableDigest,
    generated_by: &'a str,
    backend_tier_matrix_digest: &'a S0StableDigest,
    deferred_guarantee_map_digest: &'a S0StableDigest,
    terminology_scan_digest: &'a S0StableDigest,
    audit_input_manifest_digest: &'a S0StableDigest,
    complexity_contract_summary_digest: &'a S0StableDigest,
    required_forbidden_shortcuts: &'a [S1ForbiddenShortcut],
    required_harness_subsystems: &'a [SequenceHarnessDependency],
    allowed_backend_candidates: &'a [String],
    legacy_backend_fences: &'a [String],
    compile_time_boundary_fixtures: &'a [S1CompileTimeBoundaryFixtureStatusRow],
    non_platform_grade_debt_rows: &'a [S1NonPlatformGradeDebtRow],
    blocking_predicates: &'a [S1BlockingPredicateRow],
    gate_readiness: &'a RoadmapGateReadinessWitness,
    accepted_evidence_provenance: &'a S0AcceptedEvidenceProvenance,
}

#[derive(Deserialize)]
struct RawStorageFoundationS1Handoff {
    #[serde(flatten)]
    envelope: RawS0ArtifactEnvelope,
    backend_tier_matrix_digest: String,
    deferred_guarantee_map_digest: String,
    terminology_scan_digest: String,
    audit_input_manifest_digest: String,
    complexity_contract_summary_digest: String,
    required_forbidden_shortcuts: Vec<S1ForbiddenShortcut>,
    required_harness_subsystems: Vec<RawSequenceHarnessDependency>,
    allowed_backend_candidates: Vec<String>,
    legacy_backend_fences: Vec<String>,
    compile_time_boundary_fixtures: Vec<RawS1CompileTimeBoundaryFixtureStatusRow>,
    non_platform_grade_debt_rows: Vec<RawS1NonPlatformGradeDebtRow>,
    blocking_predicates: Vec<S1BlockingPredicateRow>,
    gate_readiness: RawRoadmapGateReadinessWitness,
    accepted_evidence_provenance: RawS0AcceptedEvidenceProvenance,
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
    fn into_validated(self) -> Result<S0NondeterministicMetadata, S0S1HandoffParseRejection> {
        S0NondeterministicMetadata::excluded(
            self.generated_at_policy,
            self.local_path_hint,
            self.host_hint,
        )
        .map_err(|_| {
            S0S1HandoffParseRejection::HandoffBuildRejected(
                S0S1HandoffBuildRejection::EmptyRequiredField,
            )
        })
    }
}

#[derive(Deserialize)]
struct RawSequenceHarnessDependency {
    sequence_id: String,
    subsystem: HarnessSubsystemMaturity,
    minimum_level: HarnessMaturityLevel,
}

impl RawSequenceHarnessDependency {
    fn into_validated(self) -> Result<SequenceHarnessDependency, S0S1HandoffParseRejection> {
        Ok(SequenceHarnessDependency::new(
            super::capability::Roadmap2SequenceId::new(self.sequence_id)
                .map_err(|_| S0S1HandoffParseRejection::InvalidDigest)?,
            self.subsystem,
            self.minimum_level,
        ))
    }
}

#[derive(Deserialize)]
struct RawS1CompileTimeBoundaryFixtureStatusRow {
    fixture: S1CompileTimeBoundaryFixture,
    status: S1CompileTimeBoundaryStatus,
}

impl RawS1CompileTimeBoundaryFixtureStatusRow {
    fn into_validated(self) -> S1CompileTimeBoundaryFixtureStatusRow {
        S1CompileTimeBoundaryFixtureStatusRow {
            fixture: self.fixture,
            status: self.status,
        }
    }
}

#[derive(Deserialize)]
struct RawS1NonPlatformGradeDebtRow {
    subject: String,
    deferred_s_sequences: Vec<String>,
    required_wording: String,
}

impl RawS1NonPlatformGradeDebtRow {
    fn into_validated(self) -> Result<S1NonPlatformGradeDebtRow, S0S1HandoffParseRejection> {
        Ok(S1NonPlatformGradeDebtRow {
            subject: self.subject,
            deferred_s_sequences: self
                .deferred_s_sequences
                .into_iter()
                .map(|sequence| {
                    super::capability::Roadmap2SequenceId::new(sequence)
                        .map_err(|_| S0S1HandoffParseRejection::InvalidDigest)
                })
                .collect::<Result<Vec<_>, _>>()?,
            required_wording: self.required_wording,
        })
    }
}

#[derive(Deserialize)]
struct RawRoadmapGateReadinessWitness {
    milestone_id: String,
    predecessor_evidence_count: u64,
}

impl RawRoadmapGateReadinessWitness {
    fn into_validated(self) -> RoadmapGateReadinessWitness {
        RoadmapGateReadinessWitness::new(self.milestone_id, self.predecessor_evidence_count)
    }
}

#[derive(Deserialize)]
struct RawS0AcceptedEvidenceProvenance {
    source_revision: String,
    roadmap_parent_digest: String,
    audit_input_manifest_digest: String,
}

impl RawS0AcceptedEvidenceProvenance {
    fn into_validated(self) -> Result<S0AcceptedEvidenceProvenance, S0S1HandoffParseRejection> {
        Ok(S0AcceptedEvidenceProvenance {
            source_revision: self.source_revision,
            roadmap_parent_digest: S0StableDigest::new(self.roadmap_parent_digest)
                .map_err(|_| S0S1HandoffParseRejection::InvalidDigest)?,
            audit_input_manifest_digest: S0StableDigest::new(self.audit_input_manifest_digest)
                .map_err(|_| S0S1HandoffParseRejection::InvalidDigest)?,
        })
    }
}

fn compile_time_fixture_rows(
    available_fixtures: &[S1CompileTimeBoundaryFixture],
) -> Vec<S1CompileTimeBoundaryFixtureStatusRow> {
    let available = available_fixtures.iter().copied().collect::<BTreeSet<_>>();
    let mut rows = S1CompileTimeBoundaryFixture::required_by_s0()
        .into_iter()
        .map(|fixture| S1CompileTimeBoundaryFixtureStatusRow {
            fixture,
            status: if available.contains(&fixture) {
                S1CompileTimeBoundaryStatus::Present
            } else {
                S1CompileTimeBoundaryStatus::MissingS0Debt
            },
        })
        .collect::<Vec<_>>();
    rows.sort_by_key(|row| row.fixture);
    rows
}

fn enforce_shared_provenance(
    expected_source_revision: &str,
    expected_roadmap_parent_digest: &S0StableDigest,
    backend_envelope: &S0ArtifactEnvelopeMetadata,
    deferred_envelope: &S0ArtifactEnvelopeMetadata,
    terminology_envelope: &S0ArtifactEnvelopeMetadata,
    harness_envelope: &S0ArtifactEnvelopeMetadata,
    manifest: &S0AuditInputManifest,
) -> Result<(), S0S1HandoffBuildRejection> {
    let same_revision = [
        backend_envelope.source_revision(),
        deferred_envelope.source_revision(),
        terminology_envelope.source_revision(),
        harness_envelope.source_revision(),
        manifest.source_revision(),
    ]
    .into_iter()
    .all(|revision| revision == expected_source_revision);
    let same_roadmap = [
        backend_envelope.roadmap_parent_digest(),
        deferred_envelope.roadmap_parent_digest(),
        terminology_envelope.roadmap_parent_digest(),
        harness_envelope.roadmap_parent_digest(),
    ]
    .into_iter()
    .all(|digest| digest == expected_roadmap_parent_digest);
    if same_revision && same_roadmap {
        Ok(())
    } else {
        Err(S0S1HandoffBuildRejection::StaleAcceptedInput)
    }
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
    S0StableDigest::new(format!("{:x}", hasher.finalize()))
        .map_err(|_| serde_json::Error::io(std::io::Error::other("invalid digest")))
}

fn stable_digest<T: Serialize + ?Sized>(value: &T) -> Result<S0StableDigest, serde_json::Error> {
    let value = serde_json::to_vec(value)?;
    let mut hasher = Sha256::new();
    hasher.update(value);
    S0StableDigest::new(format!("{:x}", hasher.finalize()))
        .map_err(|_| serde_json::Error::io(std::io::Error::other("invalid digest")))
}

fn require_non_empty(value: impl Into<String>) -> Result<String, String> {
    let value = value.into();
    if value.trim().is_empty() {
        return Err(value);
    }
    Ok(value)
}
