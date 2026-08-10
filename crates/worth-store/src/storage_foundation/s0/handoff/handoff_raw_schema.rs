use super::super::artifacts::S0NondeterministicMetadata;
use super::super::capability::Roadmap2SequenceId;
use super::super::evidence::S0ArtifactKind;
use super::super::harness::{
    HarnessMaturityLevel, HarnessSubsystemMaturity, S1CompileTimeBoundaryFixture,
    S1CompileTimeBoundaryStatus, S1ForbiddenShortcut,
};
use super::super::milestones::RoadmapGateReadinessWitness;
use super::accepted_evidence_provenance::S0AcceptedEvidenceProvenance;
use super::compile_time_boundary_rows::{
    S1CompileTimeBoundaryFixtureStatusRow, S1NonPlatformGradeDebtRow,
};
use super::handoff_validation::{S0S1HandoffBuildRejection, S0S1HandoffParseRejection};
use super::s1_blocking_predicate::S1BlockingPredicateRow;
use super::sequence_harness_dependency::SequenceHarnessDependency;

#[derive(serde::Deserialize)]
pub(super) struct RawStorageFoundationS1Handoff {
    #[serde(flatten)]
    pub(super) envelope: RawHandoffEnvelope,
    pub(super) backend_tier_matrix_digest: String,
    pub(super) deferred_guarantee_map_digest: String,
    pub(super) terminology_scan_digest: String,
    pub(super) audit_input_manifest_digest: String,
    pub(super) complexity_contract_summary_digest: String,
    pub(super) required_forbidden_shortcuts: Vec<S1ForbiddenShortcut>,
    pub(super) required_harness_subsystems: Vec<RawSequenceHarnessDependency>,
    pub(super) allowed_backend_candidates: Vec<String>,
    pub(super) legacy_backend_fences: Vec<String>,
    pub(super) compile_time_boundary_fixtures: Vec<RawS1CompileTimeBoundaryFixtureStatusRow>,
    pub(super) non_platform_grade_debt_rows: Vec<RawS1NonPlatformGradeDebtRow>,
    pub(super) blocking_predicates: Vec<S1BlockingPredicateRow>,
    pub(super) gate_readiness: RawRoadmapGateReadinessWitness,
    pub(super) accepted_evidence_provenance: RawAcceptedEvidenceProvenance,
}

#[derive(serde::Deserialize)]
pub(super) struct RawHandoffEnvelope {
    pub(super) schema_version: String,
    pub(super) artifact_kind: S0ArtifactKind,
    pub(super) source_revision: String,
    pub(super) roadmap_parent_digest: String,
    pub(super) generated_by: String,
    pub(super) deterministic_digest: String,
    pub(super) nondeterministic_metadata: RawHandoffNondeterministicMetadata,
}

#[derive(serde::Deserialize)]
pub(super) struct RawHandoffNondeterministicMetadata {
    generated_at_policy: String,
    local_path_hint: Option<String>,
    host_hint: Option<String>,
}

impl RawHandoffNondeterministicMetadata {
    pub(super) fn into_validated(
        self,
    ) -> Result<S0NondeterministicMetadata, S0S1HandoffParseRejection> {
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

#[derive(serde::Deserialize)]
pub(super) struct RawSequenceHarnessDependency {
    sequence_id: String,
    subsystem: HarnessSubsystemMaturity,
    minimum_level: HarnessMaturityLevel,
}

impl RawSequenceHarnessDependency {
    pub(super) fn into_validated(
        self,
    ) -> Result<SequenceHarnessDependency, S0S1HandoffParseRejection> {
        Ok(SequenceHarnessDependency::new(
            Roadmap2SequenceId::new(self.sequence_id)
                .map_err(|_| S0S1HandoffParseRejection::InvalidDigest)?,
            self.subsystem,
            self.minimum_level,
        ))
    }
}

#[derive(serde::Deserialize)]
pub(super) struct RawS1CompileTimeBoundaryFixtureStatusRow {
    fixture: S1CompileTimeBoundaryFixture,
    status: S1CompileTimeBoundaryStatus,
}

impl RawS1CompileTimeBoundaryFixtureStatusRow {
    pub(super) fn into_validated(self) -> S1CompileTimeBoundaryFixtureStatusRow {
        S1CompileTimeBoundaryFixtureStatusRow {
            fixture: self.fixture,
            status: self.status,
        }
    }
}

#[derive(serde::Deserialize)]
pub(super) struct RawS1NonPlatformGradeDebtRow {
    subject: String,
    deferred_s_sequences: Vec<String>,
    required_wording: String,
}

impl RawS1NonPlatformGradeDebtRow {
    pub(super) fn into_validated(
        self,
    ) -> Result<S1NonPlatformGradeDebtRow, S0S1HandoffParseRejection> {
        Ok(S1NonPlatformGradeDebtRow {
            subject: self.subject,
            deferred_s_sequences: self
                .deferred_s_sequences
                .into_iter()
                .map(|sequence| {
                    Roadmap2SequenceId::new(sequence)
                        .map_err(|_| S0S1HandoffParseRejection::InvalidDigest)
                })
                .collect::<Result<Vec<_>, _>>()?,
            required_wording: self.required_wording,
        })
    }
}

#[derive(serde::Deserialize)]
pub(super) struct RawRoadmapGateReadinessWitness {
    milestone_id: String,
    predecessor_evidence_count: u64,
}

impl RawRoadmapGateReadinessWitness {
    pub(super) fn into_validated(self) -> RoadmapGateReadinessWitness {
        RoadmapGateReadinessWitness::new(self.milestone_id, self.predecessor_evidence_count)
    }
}

#[derive(serde::Deserialize)]
pub(super) struct RawAcceptedEvidenceProvenance {
    source_revision: String,
    roadmap_parent_digest: String,
    audit_input_manifest_digest: String,
}

impl RawAcceptedEvidenceProvenance {
    pub(super) fn into_validated(
        self,
    ) -> Result<S0AcceptedEvidenceProvenance, S0S1HandoffParseRejection> {
        Ok(S0AcceptedEvidenceProvenance::from_parts(
            self.source_revision,
            super::super::evidence::S0StableDigest::new(self.roadmap_parent_digest)
                .map_err(|_| S0S1HandoffParseRejection::InvalidDigest)?,
            super::super::evidence::S0StableDigest::new(self.audit_input_manifest_digest)
                .map_err(|_| S0S1HandoffParseRejection::InvalidDigest)?,
        ))
    }
}
