use super::artifacts::{
    BackendCapabilityMatrix, S0ArtifactEnvelopeMetadata, S0ArtifactRowId, S0ArtifactRowStatus,
    S0ArtifactSubjectKind, S0ArtifactValidationCostSurface, S0NondeterministicMetadata,
    S0_ARTIFACT_SCHEMA_VERSION,
};
use super::capability::{BackendForbiddenClaim, Roadmap2SequenceId, StoreBackendCapabilityTier};
use super::deferred::DeferredPhysicalGuaranteeMap;
use super::evidence::{S0ArtifactKind, S0EvidenceRef, S0StableDigest};
use super::terminology::{ReleaseClaimReport, TerminologyAllowedUse, TerminologyRiskReport};
use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};
use std::collections::BTreeSet;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
pub enum HarnessMaturityLevel {
    Missing,
    Exists,
    SmokeWorks,
    CiCertifiable,
    ReleaseCertifiable,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
pub enum HarnessSubsystemMaturity {
    TerminologyClaimGate,
    BackendTierFenceEnforcement,
    DeferredGuaranteeValidation,
    MilestoneStatusCompleteness,
    CompileTimeBoundaryFixtures,
    StaleHandoffRejection,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
pub enum EvidenceBundleReadiness {
    Insufficient,
    ReadyForS1Planning,
    ReadyForS1Closeout,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
pub enum ForbiddenShortcutDetectionStatus {
    Missing,
    Exists,
    CiEnforced,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
pub enum S1ForbiddenShortcut {
    OverclaimedPhysicalPosture,
    BackendTierMismatch,
    UnmappedDeferredGuarantee,
    MissingMilestonePhysicalStatusRow,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
pub enum S1CompileTimeBoundaryFixture {
    PlatformGradeClaimConstructorPrivate,
    PlatformGradeEvidenceWitnessConstructorPrivate,
    PhysicalDebtCannotPromoteToPlatform,
    BackendDeclarationRequiresTier,
    NonPlatformBackendRequiresForbiddenClaims,
    PhysicalDebtRequiresSequenceMapping,
    S1HandoffRequiresAcceptedDigests,
}

impl S1CompileTimeBoundaryFixture {
    pub fn required_by_s0() -> Vec<Self> {
        vec![
            Self::PlatformGradeClaimConstructorPrivate,
            Self::PlatformGradeEvidenceWitnessConstructorPrivate,
            Self::PhysicalDebtCannotPromoteToPlatform,
            Self::BackendDeclarationRequiresTier,
            Self::NonPlatformBackendRequiresForbiddenClaims,
            Self::PhysicalDebtRequiresSequenceMapping,
            Self::S1HandoffRequiresAcceptedDigests,
        ]
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
pub enum S1CompileTimeBoundaryStatus {
    Present,
    MissingS0Debt,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct HarnessMaturityRow {
    row_id: S0ArtifactRowId,
    subject_kind: S0ArtifactSubjectKind,
    subject_path_or_symbol: String,
    classification: String,
    evidence_refs: Vec<S0EvidenceRef>,
    forbidden_claims: Vec<BackendForbiddenClaim>,
    deferred_s_sequences: Vec<Roadmap2SequenceId>,
    status: S0ArtifactRowStatus,
    notes: String,
    subsystem: HarnessSubsystemMaturity,
    maturity_level: HarnessMaturityLevel,
    required_for_sequences: Vec<Roadmap2SequenceId>,
    forbidden_shortcuts_covered: Vec<S1ForbiddenShortcut>,
    detection_status: ForbiddenShortcutDetectionStatus,
}

impl HarnessMaturityRow {
    #[allow(clippy::too_many_arguments)]
    pub fn new(
        row_id: S0ArtifactRowId,
        subject_path_or_symbol: impl Into<String>,
        evidence_refs: Vec<S0EvidenceRef>,
        status: S0ArtifactRowStatus,
        notes: impl Into<String>,
        subsystem: HarnessSubsystemMaturity,
        maturity_level: HarnessMaturityLevel,
        required_for_sequences: Vec<Roadmap2SequenceId>,
        forbidden_shortcuts_covered: Vec<S1ForbiddenShortcut>,
        detection_status: ForbiddenShortcutDetectionStatus,
    ) -> Result<Self, S0HarnessMaturityBuildRejection> {
        let subject_path_or_symbol = require_non_empty(subject_path_or_symbol)
            .map_err(|_| S0HarnessMaturityBuildRejection::EmptyRequiredField)?;
        let notes = require_non_empty(notes)
            .map_err(|_| S0HarnessMaturityBuildRejection::EmptyRequiredField)?;
        if evidence_refs.is_empty() {
            return Err(S0HarnessMaturityBuildRejection::MissingEvidenceRef);
        }
        if required_for_sequences.is_empty() {
            return Err(S0HarnessMaturityBuildRejection::MissingRequiredSequence);
        }
        Ok(Self {
            row_id,
            subject_kind: S0ArtifactSubjectKind::Harness,
            subject_path_or_symbol,
            classification: "harness-maturity".to_string(),
            evidence_refs,
            forbidden_claims: Vec::new(),
            deferred_s_sequences: Vec::new(),
            status,
            notes,
            subsystem,
            maturity_level,
            required_for_sequences,
            forbidden_shortcuts_covered,
            detection_status,
        })
    }

    pub fn row_id(&self) -> &S0ArtifactRowId {
        &self.row_id
    }

    pub fn subsystem(&self) -> HarnessSubsystemMaturity {
        self.subsystem
    }

    pub fn maturity_level(&self) -> HarnessMaturityLevel {
        self.maturity_level
    }

    pub fn forbidden_shortcuts_covered(&self) -> &[S1ForbiddenShortcut] {
        &self.forbidden_shortcuts_covered
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct HarnessMaturityReport {
    #[serde(flatten)]
    envelope: S0ArtifactEnvelopeMetadata,
    rows: Vec<HarnessMaturityRow>,
    evidence_bundle_readiness: EvidenceBundleReadiness,
}

impl HarnessMaturityReport {
    pub fn new(
        source_revision: impl Into<String>,
        roadmap_parent_digest: S0StableDigest,
        generated_by: impl Into<String>,
        nondeterministic_metadata: S0NondeterministicMetadata,
        mut rows: Vec<HarnessMaturityRow>,
        evidence_bundle_readiness: EvidenceBundleReadiness,
    ) -> Result<Self, S0HarnessMaturityBuildRejection> {
        let source_revision = require_non_empty(source_revision)
            .map_err(|_| S0HarnessMaturityBuildRejection::EmptyRequiredField)?;
        let generated_by = require_non_empty(generated_by)
            .map_err(|_| S0HarnessMaturityBuildRejection::EmptyRequiredField)?;
        rows.sort_by(|left, right| left.row_id.cmp(&right.row_id));
        reject_duplicate_rows(&rows)?;
        ensure_required_harness_subsystems(&rows)?;
        let deterministic_digest = stable_digest(&HarnessMaturityDigestBasis {
            schema_version: S0_ARTIFACT_SCHEMA_VERSION,
            artifact_kind: S0ArtifactKind::HarnessMaturityReport,
            source_revision: &source_revision,
            roadmap_parent_digest: &roadmap_parent_digest,
            generated_by: &generated_by,
            readiness: evidence_bundle_readiness,
            rows: &rows,
        })
        .map_err(|_| S0HarnessMaturityBuildRejection::InvalidDigest)?;
        Ok(Self {
            envelope: S0ArtifactEnvelopeMetadata::new(
                S0ArtifactKind::HarnessMaturityReport,
                source_revision,
                roadmap_parent_digest,
                generated_by,
                deterministic_digest,
                nondeterministic_metadata,
            ),
            rows,
            evidence_bundle_readiness,
        })
    }

    pub fn baseline_for_s1(
        source_revision: impl Into<String>,
        roadmap_parent_digest: S0StableDigest,
        generated_by: impl Into<String>,
        nondeterministic_metadata: S0NondeterministicMetadata,
        backend_matrix: &BackendCapabilityMatrix,
        deferred_map: &DeferredPhysicalGuaranteeMap,
        terminology_report: &TerminologyRiskReport,
        release_claim_report: &ReleaseClaimReport,
        milestone_row_count: u64,
        required_milestone_row_count: u64,
        available_fixtures: &[S1CompileTimeBoundaryFixture],
    ) -> Result<Self, S0HarnessMaturityBuildRejection> {
        let rows = vec![
            terminology_claim_gate_row(terminology_report, release_claim_report)?,
            backend_tier_fence_row(backend_matrix)?,
            deferred_validation_row(deferred_map)?,
            milestone_completeness_row(milestone_row_count, required_milestone_row_count)?,
            compile_time_fixture_row(available_fixtures)?,
            stale_handoff_row(backend_matrix, deferred_map, terminology_report)?,
        ];
        let readiness = if rows
            .iter()
            .filter(|row| {
                row.required_for_sequences
                    .iter()
                    .any(|sequence| sequence.as_str() == "S1")
            })
            .all(|row| row.maturity_level >= HarnessMaturityLevel::Exists)
        {
            EvidenceBundleReadiness::ReadyForS1Planning
        } else {
            EvidenceBundleReadiness::Insufficient
        };
        Self::new(
            source_revision,
            roadmap_parent_digest,
            generated_by,
            nondeterministic_metadata,
            rows,
            readiness,
        )
    }

    pub fn envelope(&self) -> &S0ArtifactEnvelopeMetadata {
        &self.envelope
    }

    pub fn rows(&self) -> &[HarnessMaturityRow] {
        &self.rows
    }

    pub fn evidence_bundle_readiness(&self) -> EvidenceBundleReadiness {
        self.evidence_bundle_readiness
    }

    pub fn to_canonical_json_bytes(&self) -> Result<Vec<u8>, S0HarnessMaturityParseRejection> {
        serde_json::to_vec_pretty(self)
            .map_err(|_| S0HarnessMaturityParseRejection::SerializationFailed)
    }

    pub fn validate_canonical_json_bytes(
        bytes: &[u8],
    ) -> Result<S0ValidatedHarnessMaturityReportArtifact, S0HarnessMaturityParseRejection> {
        let raw = serde_json::from_slice::<RawHarnessMaturityReport>(bytes)
            .map_err(|_| S0HarnessMaturityParseRejection::NonParseable)?;
        if raw.envelope.schema_version != S0_ARTIFACT_SCHEMA_VERSION {
            return Err(S0HarnessMaturityParseRejection::SchemaVersionMismatch);
        }
        if raw.envelope.artifact_kind != S0ArtifactKind::HarnessMaturityReport {
            return Err(S0HarnessMaturityParseRejection::ArtifactKindMismatch);
        }
        let roadmap_parent_digest = S0StableDigest::new(raw.envelope.roadmap_parent_digest)
            .map_err(|_| S0HarnessMaturityParseRejection::InvalidDigest)?;
        let expected_digest = S0StableDigest::new(raw.envelope.deterministic_digest)
            .map_err(|_| S0HarnessMaturityParseRejection::InvalidDigest)?;
        let report = Self::new(
            raw.envelope.source_revision,
            roadmap_parent_digest,
            raw.envelope.generated_by,
            raw.envelope.nondeterministic_metadata.into_validated()?,
            raw.rows
                .into_iter()
                .map(RawHarnessMaturityRow::into_validated)
                .collect::<Result<Vec<_>, _>>()?,
            raw.evidence_bundle_readiness,
        )?;
        let row_count = report.rows().len() as u64;
        if report.envelope().deterministic_digest() != &expected_digest {
            return Err(S0HarnessMaturityParseRejection::DeterministicDigestMismatch);
        }
        let canonicalized_row_byte_count = serde_json::to_vec(report.rows())
            .map_err(|_| S0HarnessMaturityParseRejection::SerializationFailed)?
            .len() as u64;
        Ok(S0ValidatedHarnessMaturityReportArtifact {
            report,
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
pub struct S0ValidatedHarnessMaturityReportArtifact {
    report: HarnessMaturityReport,
    validation_cost: S0ArtifactValidationCostSurface,
}

impl S0ValidatedHarnessMaturityReportArtifact {
    pub fn report(&self) -> &HarnessMaturityReport {
        &self.report
    }

    pub fn validation_cost(&self) -> &S0ArtifactValidationCostSurface {
        &self.validation_cost
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize)]
pub enum S0HarnessMaturityBuildRejection {
    EmptyRequiredField,
    MissingEvidenceRef,
    MissingRequiredSequence,
    DuplicateRowId,
    MissingRequiredHarnessSubsystem,
    InvalidDigest,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize)]
pub enum S0HarnessMaturityParseRejection {
    NonParseable,
    SerializationFailed,
    SchemaVersionMismatch,
    ArtifactKindMismatch,
    InvalidDigest,
    InvalidEvidenceRef,
    InvalidDeferredSequence,
    RowBuildRejected(S0HarnessMaturityBuildRejection),
    ReportBuildRejected(S0HarnessMaturityBuildRejection),
    DeterministicDigestMismatch,
}

impl From<S0HarnessMaturityBuildRejection> for S0HarnessMaturityParseRejection {
    fn from(value: S0HarnessMaturityBuildRejection) -> Self {
        Self::ReportBuildRejected(value)
    }
}

#[derive(Serialize)]
struct HarnessMaturityDigestBasis<'a> {
    schema_version: &'static str,
    artifact_kind: S0ArtifactKind,
    source_revision: &'a str,
    roadmap_parent_digest: &'a S0StableDigest,
    generated_by: &'a str,
    readiness: EvidenceBundleReadiness,
    rows: &'a [HarnessMaturityRow],
}

#[derive(Deserialize)]
struct RawHarnessMaturityReport {
    #[serde(flatten)]
    envelope: RawS0ArtifactEnvelope,
    rows: Vec<RawHarnessMaturityRow>,
    evidence_bundle_readiness: EvidenceBundleReadiness,
}

#[derive(Deserialize)]
struct RawHarnessMaturityRow {
    row_id: String,
    subject_path_or_symbol: String,
    evidence_refs: Vec<RawS0EvidenceRef>,
    status: S0ArtifactRowStatus,
    notes: String,
    subsystem: HarnessSubsystemMaturity,
    maturity_level: HarnessMaturityLevel,
    required_for_sequences: Vec<String>,
    forbidden_shortcuts_covered: Vec<S1ForbiddenShortcut>,
    detection_status: ForbiddenShortcutDetectionStatus,
}

impl RawHarnessMaturityRow {
    fn into_validated(self) -> Result<HarnessMaturityRow, S0HarnessMaturityParseRejection> {
        let row_id = S0ArtifactRowId::new(self.row_id).map_err(|_| {
            S0HarnessMaturityParseRejection::RowBuildRejected(
                S0HarnessMaturityBuildRejection::EmptyRequiredField,
            )
        })?;
        let evidence_refs = self
            .evidence_refs
            .into_iter()
            .map(RawS0EvidenceRef::into_validated)
            .collect::<Result<Vec<_>, _>>()?;
        let required_for_sequences = self
            .required_for_sequences
            .into_iter()
            .map(|sequence| {
                Roadmap2SequenceId::new(sequence)
                    .map_err(|_| S0HarnessMaturityParseRejection::InvalidDeferredSequence)
            })
            .collect::<Result<Vec<_>, _>>()?;
        HarnessMaturityRow::new(
            row_id,
            self.subject_path_or_symbol,
            evidence_refs,
            self.status,
            self.notes,
            self.subsystem,
            self.maturity_level,
            required_for_sequences,
            self.forbidden_shortcuts_covered,
            self.detection_status,
        )
        .map_err(S0HarnessMaturityParseRejection::RowBuildRejected)
    }
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
    fn into_validated(self) -> Result<S0NondeterministicMetadata, S0HarnessMaturityParseRejection> {
        S0NondeterministicMetadata::excluded(
            self.generated_at_policy,
            self.local_path_hint,
            self.host_hint,
        )
        .map_err(|_| {
            S0HarnessMaturityParseRejection::RowBuildRejected(
                S0HarnessMaturityBuildRejection::EmptyRequiredField,
            )
        })
    }
}

#[derive(Deserialize)]
struct RawS0EvidenceRef {
    artifact_kind: S0ArtifactKind,
    digest: String,
}

impl RawS0EvidenceRef {
    fn into_validated(self) -> Result<S0EvidenceRef, S0HarnessMaturityParseRejection> {
        let digest = S0StableDigest::new(self.digest)
            .map_err(|_| S0HarnessMaturityParseRejection::InvalidDigest)?;
        Ok(S0EvidenceRef::new(self.artifact_kind, digest))
    }
}

fn terminology_claim_gate_row(
    terminology_report: &TerminologyRiskReport,
    release_claim_report: &ReleaseClaimReport,
) -> Result<HarnessMaturityRow, S0HarnessMaturityBuildRejection> {
    let release_ready = release_claim_report.scanned_surface_count() > 0
        && release_claim_report.rejection_count() == 0
        && terminology_report.rows().iter().all(|row| {
            !matches!(
                row.allowed_use(),
                TerminologyAllowedUse::OverclaimedPhysicalPosture
            )
        });
    HarnessMaturityRow::new(
        harness_row_id("terminology-claim-gate")?,
        "worth_store::storage_foundation::s0::terminology",
        vec![S0EvidenceRef::new(
            S0ArtifactKind::TerminologyRiskReport,
            terminology_report.envelope().deterministic_digest().clone(),
        )],
        if release_ready {
            S0ArtifactRowStatus::Admitted
        } else {
            S0ArtifactRowStatus::Deferred
        },
        "Terminology scanning must qualify public physical language before S.1 closes.",
        HarnessSubsystemMaturity::TerminologyClaimGate,
        if release_ready {
            HarnessMaturityLevel::CiCertifiable
        } else {
            HarnessMaturityLevel::SmokeWorks
        },
        vec![Roadmap2SequenceId::new("S1")
            .map_err(|_| S0HarnessMaturityBuildRejection::EmptyRequiredField)?],
        vec![S1ForbiddenShortcut::OverclaimedPhysicalPosture],
        ForbiddenShortcutDetectionStatus::CiEnforced,
    )
}

fn backend_tier_fence_row(
    backend_matrix: &BackendCapabilityMatrix,
) -> Result<HarnessMaturityRow, S0HarnessMaturityBuildRejection> {
    let fenced = backend_matrix
        .rows()
        .iter()
        .all(|row| match row.capability_tier() {
            StoreBackendCapabilityTier::PlatformGrade => true,
            StoreBackendCapabilityTier::PhysicalFoundation => {
                !row.deferred_s_sequences().is_empty()
            }
            StoreBackendCapabilityTier::Bootstrap
            | StoreBackendCapabilityTier::SemanticCertification
            | StoreBackendCapabilityTier::Compatibility => !row.forbidden_claims().is_empty(),
        });
    HarnessMaturityRow::new(
        harness_row_id("backend-tier-fence-enforcement")?,
        "worth_store::storage_foundation::s0::artifacts",
        vec![S0EvidenceRef::new(
            S0ArtifactKind::BackendCapabilityMatrix,
            backend_matrix.envelope().deterministic_digest().clone(),
        )],
        if fenced {
            S0ArtifactRowStatus::Admitted
        } else {
            S0ArtifactRowStatus::Deferred
        },
        "Backend tiers must fence legacy and semantic-only backends from platform claims.",
        HarnessSubsystemMaturity::BackendTierFenceEnforcement,
        if fenced {
            HarnessMaturityLevel::CiCertifiable
        } else {
            HarnessMaturityLevel::Missing
        },
        vec![Roadmap2SequenceId::new("S1")
            .map_err(|_| S0HarnessMaturityBuildRejection::EmptyRequiredField)?],
        vec![S1ForbiddenShortcut::BackendTierMismatch],
        ForbiddenShortcutDetectionStatus::CiEnforced,
    )
}

fn deferred_validation_row(
    deferred_map: &DeferredPhysicalGuaranteeMap,
) -> Result<HarnessMaturityRow, S0HarnessMaturityBuildRejection> {
    let complete = !deferred_map.rows().is_empty();
    HarnessMaturityRow::new(
        harness_row_id("deferred-guarantee-validation")?,
        "worth_store::storage_foundation::s0::deferred",
        vec![S0EvidenceRef::new(
            S0ArtifactKind::DeferredPhysicalGuaranteeMap,
            deferred_map.envelope().deterministic_digest().clone(),
        )],
        if complete {
            S0ArtifactRowStatus::Admitted
        } else {
            S0ArtifactRowStatus::Deferred
        },
        "Deferred physical guarantees must map to named Roadmap 2 sequences.",
        HarnessSubsystemMaturity::DeferredGuaranteeValidation,
        if complete {
            HarnessMaturityLevel::CiCertifiable
        } else {
            HarnessMaturityLevel::Missing
        },
        vec![Roadmap2SequenceId::new("S1")
            .map_err(|_| S0HarnessMaturityBuildRejection::EmptyRequiredField)?],
        vec![S1ForbiddenShortcut::UnmappedDeferredGuarantee],
        ForbiddenShortcutDetectionStatus::CiEnforced,
    )
}

fn milestone_completeness_row(
    milestone_row_count: u64,
    required_milestone_row_count: u64,
) -> Result<HarnessMaturityRow, S0HarnessMaturityBuildRejection> {
    let complete = milestone_row_count == required_milestone_row_count;
    HarnessMaturityRow::new(
        harness_row_id("milestone-status-completeness")?,
        "worth_store::storage_foundation::s0::milestones",
        vec![S0EvidenceRef::new(
            S0ArtifactKind::MilestonePhysicalStatusMatrix,
            stable_digest(&(milestone_row_count, required_milestone_row_count))
                .map_err(|_| S0HarnessMaturityBuildRejection::InvalidDigest)?,
        )],
        if complete {
            S0ArtifactRowStatus::Admitted
        } else {
            S0ArtifactRowStatus::Deferred
        },
        "Milestone physical-status coverage must stay complete before S.1 closeout.",
        HarnessSubsystemMaturity::MilestoneStatusCompleteness,
        if complete {
            HarnessMaturityLevel::CiCertifiable
        } else {
            HarnessMaturityLevel::Missing
        },
        vec![Roadmap2SequenceId::new("S1")
            .map_err(|_| S0HarnessMaturityBuildRejection::EmptyRequiredField)?],
        vec![S1ForbiddenShortcut::MissingMilestonePhysicalStatusRow],
        ForbiddenShortcutDetectionStatus::CiEnforced,
    )
}

fn compile_time_fixture_row(
    available_fixtures: &[S1CompileTimeBoundaryFixture],
) -> Result<HarnessMaturityRow, S0HarnessMaturityBuildRejection> {
    let available = available_fixtures.iter().copied().collect::<BTreeSet<_>>();
    let required = S1CompileTimeBoundaryFixture::required_by_s0();
    let present_required = required
        .iter()
        .filter(|fixture| available.contains(fixture))
        .count();
    HarnessMaturityRow::new(
        harness_row_id("compile-time-boundary-fixtures")?,
        "worth_store::tests::ui",
        vec![S0EvidenceRef::new(
            S0ArtifactKind::S1HandoffReadiness,
            stable_digest(&required).map_err(|_| S0HarnessMaturityBuildRejection::InvalidDigest)?,
        )],
        if present_required == 0 {
            S0ArtifactRowStatus::Deferred
        } else {
            S0ArtifactRowStatus::Present
        },
        "Compile-time S.0 boundary fixtures are tracked for S.1 closeout readiness.",
        HarnessSubsystemMaturity::CompileTimeBoundaryFixtures,
        if present_required == required.len() {
            HarnessMaturityLevel::CiCertifiable
        } else {
            HarnessMaturityLevel::SmokeWorks
        },
        vec![Roadmap2SequenceId::new("S1")
            .map_err(|_| S0HarnessMaturityBuildRejection::EmptyRequiredField)?],
        vec![S1ForbiddenShortcut::BackendTierMismatch],
        ForbiddenShortcutDetectionStatus::Exists,
    )
}

fn stale_handoff_row(
    backend_matrix: &BackendCapabilityMatrix,
    deferred_map: &DeferredPhysicalGuaranteeMap,
    terminology_report: &TerminologyRiskReport,
) -> Result<HarnessMaturityRow, S0HarnessMaturityBuildRejection> {
    let shared = backend_matrix.envelope().source_revision()
        == deferred_map.envelope().source_revision()
        && backend_matrix.envelope().source_revision()
            == terminology_report.envelope().source_revision();
    HarnessMaturityRow::new(
        harness_row_id("stale-handoff-rejection")?,
        "worth_store::storage_foundation::s0::handoff",
        vec![S0EvidenceRef::new(
            S0ArtifactKind::S1HandoffReadiness,
            stable_digest(&(
                backend_matrix.envelope().source_revision(),
                deferred_map.envelope().source_revision(),
                terminology_report.envelope().source_revision(),
            ))
            .map_err(|_| S0HarnessMaturityBuildRejection::InvalidDigest)?,
        )],
        if shared {
            S0ArtifactRowStatus::Present
        } else {
            S0ArtifactRowStatus::Deferred
        },
        "S.1 handoff will reject stale accepted inputs across S.0 artifacts.",
        HarnessSubsystemMaturity::StaleHandoffRejection,
        if shared {
            HarnessMaturityLevel::SmokeWorks
        } else {
            HarnessMaturityLevel::Missing
        },
        vec![Roadmap2SequenceId::new("S1")
            .map_err(|_| S0HarnessMaturityBuildRejection::EmptyRequiredField)?],
        vec![],
        ForbiddenShortcutDetectionStatus::Exists,
    )
}

fn ensure_required_harness_subsystems(
    rows: &[HarnessMaturityRow],
) -> Result<(), S0HarnessMaturityBuildRejection> {
    let present = rows
        .iter()
        .map(|row| row.subsystem())
        .collect::<BTreeSet<_>>();
    for required in [
        HarnessSubsystemMaturity::TerminologyClaimGate,
        HarnessSubsystemMaturity::BackendTierFenceEnforcement,
        HarnessSubsystemMaturity::DeferredGuaranteeValidation,
        HarnessSubsystemMaturity::MilestoneStatusCompleteness,
        HarnessSubsystemMaturity::CompileTimeBoundaryFixtures,
        HarnessSubsystemMaturity::StaleHandoffRejection,
    ] {
        if !present.contains(&required) {
            return Err(S0HarnessMaturityBuildRejection::MissingRequiredHarnessSubsystem);
        }
    }
    Ok(())
}

fn harness_row_id(label: &str) -> Result<S0ArtifactRowId, S0HarnessMaturityBuildRejection> {
    let mut hasher = Sha256::new();
    hasher.update(label.as_bytes());
    let digest = format!("{:x}", hasher.finalize());
    S0ArtifactRowId::new(format!("Harness{}", &digest[..16]))
        .map_err(|_| S0HarnessMaturityBuildRejection::EmptyRequiredField)
}

fn reject_duplicate_rows(
    rows: &[HarnessMaturityRow],
) -> Result<(), S0HarnessMaturityBuildRejection> {
    let mut seen = BTreeSet::new();
    if rows.iter().any(|row| !seen.insert(row.row_id().clone())) {
        return Err(S0HarnessMaturityBuildRejection::DuplicateRowId);
    }
    Ok(())
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
