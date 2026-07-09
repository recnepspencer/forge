use super::artifacts::{
    S0ArtifactEnvelopeMetadata, S0ArtifactRowId, S0ArtifactRowStatus, S0ArtifactSubjectKind,
    S0ArtifactValidationCostSurface, S0NondeterministicMetadata, S0_ARTIFACT_SCHEMA_VERSION,
};
use super::evidence::{S0ArtifactKind, S0EvidenceRef, S0StableDigest};
use super::milestones::{
    MilestonePhysicalStatusRow, S0PhysicalStatus, SemanticPhysicalClaimFamily,
};
use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};
use std::collections::BTreeSet;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum SemanticPhysicalClaimStatus {
    SemanticProven,
    NotApplicable,
    NotStarted,
    SemanticOnly,
    BootstrapPhysical,
    PhysicalDebt,
    PartiallyFoundationBacked,
    FoundationBacked,
    PlatformGrade,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct SemanticPhysicalClaimReportRow {
    row_id: S0ArtifactRowId,
    subject_kind: S0ArtifactSubjectKind,
    subject_path_or_symbol: String,
    classification: String,
    evidence_refs: Vec<S0EvidenceRef>,
    forbidden_claims: Vec<super::capability::BackendForbiddenClaim>,
    deferred_s_sequences: Vec<super::capability::Roadmap2SequenceId>,
    status: S0ArtifactRowStatus,
    notes: String,
    claim_family: SemanticPhysicalClaimFamily,
    claim_status: SemanticPhysicalClaimStatus,
    semantic_capability_proven: String,
    closeout_or_planned_source: String,
    named_suite: String,
    evidence_lanes: Vec<String>,
}

impl SemanticPhysicalClaimReportRow {
    #[allow(clippy::too_many_arguments)]
    pub fn new(
        row_id: S0ArtifactRowId,
        subject_kind: S0ArtifactSubjectKind,
        subject_path_or_symbol: impl Into<String>,
        classification: impl Into<String>,
        evidence_refs: Vec<S0EvidenceRef>,
        forbidden_claims: Vec<super::capability::BackendForbiddenClaim>,
        deferred_s_sequences: Vec<super::capability::Roadmap2SequenceId>,
        status: S0ArtifactRowStatus,
        notes: impl Into<String>,
        claim_family: SemanticPhysicalClaimFamily,
        claim_status: SemanticPhysicalClaimStatus,
        semantic_capability_proven: impl Into<String>,
        closeout_or_planned_source: impl Into<String>,
        named_suite: impl Into<String>,
        evidence_lanes: Vec<String>,
    ) -> Result<Self, S0ClaimReportBuildRejection> {
        let subject_path_or_symbol = require_non_empty(subject_path_or_symbol)?;
        let classification = require_non_empty(classification)?;
        let notes = require_non_empty(notes)?;
        let semantic_capability_proven = require_non_empty(semantic_capability_proven)?;
        let closeout_or_planned_source = require_non_empty(closeout_or_planned_source)?;
        let named_suite = require_non_empty(named_suite)?;
        if evidence_refs.is_empty() {
            return Err(S0ClaimReportBuildRejection::MissingEvidenceRef);
        }
        if evidence_lanes.is_empty() {
            return Err(S0ClaimReportBuildRejection::MissingEvidenceLane);
        }
        if claim_status_requires_deferred_mapping(claim_family, claim_status)
            && deferred_s_sequences.is_empty()
        {
            return Err(S0ClaimReportBuildRejection::DeferredSequenceMissing);
        }
        Ok(Self {
            row_id,
            subject_kind,
            subject_path_or_symbol,
            classification,
            evidence_refs,
            forbidden_claims,
            deferred_s_sequences,
            status,
            notes,
            claim_family,
            claim_status,
            semantic_capability_proven,
            closeout_or_planned_source,
            named_suite,
            evidence_lanes,
        })
    }

    pub fn row_id(&self) -> &S0ArtifactRowId {
        &self.row_id
    }

    pub fn claim_family(&self) -> SemanticPhysicalClaimFamily {
        self.claim_family
    }

    pub fn claim_status(&self) -> SemanticPhysicalClaimStatus {
        self.claim_status
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct S0ValidatedSemanticPhysicalClaimReportArtifact {
    report: SemanticPhysicalClaimReport,
    validation_cost: S0ArtifactValidationCostSurface,
}

impl S0ValidatedSemanticPhysicalClaimReportArtifact {
    pub fn report(&self) -> &SemanticPhysicalClaimReport {
        &self.report
    }

    pub fn validation_cost(&self) -> &S0ArtifactValidationCostSurface {
        &self.validation_cost
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct SemanticPhysicalClaimReport {
    #[serde(flatten)]
    envelope: S0ArtifactEnvelopeMetadata,
    rows: Vec<SemanticPhysicalClaimReportRow>,
}

impl SemanticPhysicalClaimReport {
    pub fn new(
        source_revision: impl Into<String>,
        roadmap_parent_digest: S0StableDigest,
        generated_by: impl Into<String>,
        nondeterministic_metadata: S0NondeterministicMetadata,
        mut rows: Vec<SemanticPhysicalClaimReportRow>,
    ) -> Result<Self, S0ClaimReportBuildRejection> {
        let source_revision = require_non_empty(source_revision)?;
        let generated_by = require_non_empty(generated_by)?;
        rows.sort_by(|left, right| left.row_id.cmp(&right.row_id));
        reject_duplicate_rows(&rows)?;
        let deterministic_digest = stable_digest(&SemanticPhysicalClaimReportDigestBasis {
            schema_version: S0_ARTIFACT_SCHEMA_VERSION,
            artifact_kind: S0ArtifactKind::SemanticPhysicalClaimReport,
            source_revision: &source_revision,
            roadmap_parent_digest: &roadmap_parent_digest,
            generated_by: &generated_by,
            rows: &rows,
        })?;
        let envelope = S0ArtifactEnvelopeMetadata::new(
            S0ArtifactKind::SemanticPhysicalClaimReport,
            source_revision,
            roadmap_parent_digest,
            generated_by,
            deterministic_digest,
            nondeterministic_metadata,
        );
        Ok(Self { envelope, rows })
    }

    pub fn from_milestone_rows(
        source_revision: impl Into<String>,
        roadmap_parent_digest: S0StableDigest,
        generated_by: impl Into<String>,
        nondeterministic_metadata: S0NondeterministicMetadata,
        milestone_rows: &[MilestonePhysicalStatusRow],
    ) -> Result<Self, S0ClaimReportBuildRejection> {
        let rows = milestone_rows
            .iter()
            .flat_map(rows_for_milestone)
            .collect::<Result<Vec<_>, _>>()?;
        Self::new(
            source_revision,
            roadmap_parent_digest,
            generated_by,
            nondeterministic_metadata,
            rows,
        )
    }

    pub fn envelope(&self) -> &S0ArtifactEnvelopeMetadata {
        &self.envelope
    }

    pub fn rows(&self) -> &[SemanticPhysicalClaimReportRow] {
        &self.rows
    }

    pub fn to_canonical_json_bytes(&self) -> Result<Vec<u8>, S0ClaimReportParseRejection> {
        serde_json::to_vec_pretty(self)
            .map_err(|_| S0ClaimReportParseRejection::SerializationFailed)
    }

    pub fn validate_canonical_json_bytes(
        bytes: &[u8],
    ) -> Result<S0ValidatedSemanticPhysicalClaimReportArtifact, S0ClaimReportParseRejection> {
        let raw = serde_json::from_slice::<RawSemanticPhysicalClaimReport>(bytes)
            .map_err(|_| S0ClaimReportParseRejection::NonParseable)?;
        if raw.envelope.schema_version != S0_ARTIFACT_SCHEMA_VERSION {
            return Err(S0ClaimReportParseRejection::SchemaVersionMismatch);
        }
        if raw.envelope.artifact_kind != S0ArtifactKind::SemanticPhysicalClaimReport {
            return Err(S0ClaimReportParseRejection::ArtifactKindMismatch);
        }
        let roadmap_parent_digest = S0StableDigest::new(raw.envelope.roadmap_parent_digest)
            .map_err(|_| S0ClaimReportParseRejection::InvalidDigest)?;
        let expected_digest = S0StableDigest::new(raw.envelope.deterministic_digest)
            .map_err(|_| S0ClaimReportParseRejection::InvalidDigest)?;
        let rows = raw
            .rows
            .into_iter()
            .map(RawSemanticPhysicalClaimReportRow::into_validated)
            .collect::<Result<Vec<_>, _>>()?;
        let report = Self::new(
            raw.envelope.source_revision,
            roadmap_parent_digest,
            raw.envelope.generated_by,
            raw.envelope.nondeterministic_metadata.into_validated()?,
            rows,
        )?;
        let row_count = report.rows().len() as u64;
        if report.envelope().deterministic_digest() != &expected_digest {
            return Err(S0ClaimReportParseRejection::DeterministicDigestMismatch);
        }
        let canonicalized_row_byte_count = serde_json::to_vec(report.rows())
            .map_err(|_| S0ClaimReportParseRejection::SerializationFailed)?
            .len() as u64;
        Ok(S0ValidatedSemanticPhysicalClaimReportArtifact {
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

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize)]
pub enum S0ClaimReportBuildRejection {
    EmptyRequiredField,
    MissingEvidenceRef,
    MissingEvidenceLane,
    DeferredSequenceMissing,
    DuplicateRowId,
    DigestConstructionFailed,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize)]
pub enum S0ClaimReportParseRejection {
    NonParseable,
    SerializationFailed,
    SchemaVersionMismatch,
    ArtifactKindMismatch,
    InvalidDigest,
    InvalidEvidenceRef,
    InvalidDeferredSequence,
    RowBuildRejected(S0ClaimReportBuildRejection),
    ReportBuildRejected(S0ClaimReportBuildRejection),
    DeterministicDigestMismatch,
}

impl From<S0ClaimReportBuildRejection> for S0ClaimReportParseRejection {
    fn from(value: S0ClaimReportBuildRejection) -> Self {
        Self::ReportBuildRejected(value)
    }
}

#[derive(Serialize)]
struct SemanticPhysicalClaimReportDigestBasis<'a> {
    schema_version: &'static str,
    artifact_kind: S0ArtifactKind,
    source_revision: &'a str,
    roadmap_parent_digest: &'a S0StableDigest,
    generated_by: &'a str,
    rows: &'a [SemanticPhysicalClaimReportRow],
}

#[derive(Deserialize)]
struct RawSemanticPhysicalClaimReport {
    #[serde(flatten)]
    envelope: RawS0ArtifactEnvelope,
    rows: Vec<RawSemanticPhysicalClaimReportRow>,
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
    fn into_validated(self) -> Result<S0NondeterministicMetadata, S0ClaimReportParseRejection> {
        S0NondeterministicMetadata::excluded(
            self.generated_at_policy,
            self.local_path_hint,
            self.host_hint,
        )
        .map_err(|_| {
            S0ClaimReportParseRejection::RowBuildRejected(
                S0ClaimReportBuildRejection::EmptyRequiredField,
            )
        })
    }
}

#[derive(Deserialize)]
struct RawSemanticPhysicalClaimReportRow {
    row_id: String,
    subject_kind: S0ArtifactSubjectKind,
    subject_path_or_symbol: String,
    classification: String,
    evidence_refs: Vec<RawS0EvidenceRef>,
    forbidden_claims: Vec<RawBackendForbiddenClaim>,
    deferred_s_sequences: Vec<String>,
    status: S0ArtifactRowStatus,
    notes: String,
    claim_family: SemanticPhysicalClaimFamily,
    claim_status: SemanticPhysicalClaimStatus,
    semantic_capability_proven: String,
    closeout_or_planned_source: String,
    named_suite: String,
    evidence_lanes: Vec<String>,
}

impl RawSemanticPhysicalClaimReportRow {
    fn into_validated(self) -> Result<SemanticPhysicalClaimReportRow, S0ClaimReportParseRejection> {
        let row_id = S0ArtifactRowId::new(self.row_id).map_err(|_| {
            S0ClaimReportParseRejection::RowBuildRejected(
                S0ClaimReportBuildRejection::EmptyRequiredField,
            )
        })?;
        let evidence_refs = self
            .evidence_refs
            .into_iter()
            .map(RawS0EvidenceRef::into_validated)
            .collect::<Result<Vec<_>, _>>()?;
        let forbidden_claims = self
            .forbidden_claims
            .into_iter()
            .map(RawBackendForbiddenClaim::into_validated)
            .collect::<Result<Vec<_>, _>>()?;
        let deferred_s_sequences = self
            .deferred_s_sequences
            .into_iter()
            .map(|sequence| {
                super::capability::Roadmap2SequenceId::new(sequence)
                    .map_err(|_| S0ClaimReportParseRejection::InvalidDeferredSequence)
            })
            .collect::<Result<Vec<_>, _>>()?;
        SemanticPhysicalClaimReportRow::new(
            row_id,
            self.subject_kind,
            self.subject_path_or_symbol,
            self.classification,
            evidence_refs,
            forbidden_claims,
            deferred_s_sequences,
            self.status,
            self.notes,
            self.claim_family,
            self.claim_status,
            self.semantic_capability_proven,
            self.closeout_or_planned_source,
            self.named_suite,
            self.evidence_lanes,
        )
        .map_err(S0ClaimReportParseRejection::RowBuildRejected)
    }
}

#[derive(Deserialize)]
struct RawS0EvidenceRef {
    artifact_kind: S0ArtifactKind,
    digest: String,
}

impl RawS0EvidenceRef {
    fn into_validated(self) -> Result<S0EvidenceRef, S0ClaimReportParseRejection> {
        let digest = S0StableDigest::new(self.digest)
            .map_err(|_| S0ClaimReportParseRejection::InvalidDigest)?;
        Ok(S0EvidenceRef::new(self.artifact_kind, digest))
    }
}

#[derive(Deserialize)]
struct RawBackendForbiddenClaim {
    claim_kind: super::capability::BackendForbiddenClaimKind,
    deferred_sequence: String,
}

impl RawBackendForbiddenClaim {
    fn into_validated(
        self,
    ) -> Result<super::capability::BackendForbiddenClaim, S0ClaimReportParseRejection> {
        super::capability::BackendForbiddenClaim::new(self.claim_kind, self.deferred_sequence)
            .map_err(|_| S0ClaimReportParseRejection::InvalidDeferredSequence)
    }
}

fn rows_for_milestone(
    row: &MilestonePhysicalStatusRow,
) -> impl Iterator<Item = Result<SemanticPhysicalClaimReportRow, S0ClaimReportBuildRejection>> + '_
{
    row.claim_families().iter().copied().map(|family| {
        let claim_status = claim_status_for(row, family);
        SemanticPhysicalClaimReportRow::new(
            claim_row_id(row.milestone_id(), family)?,
            S0ArtifactSubjectKind::Milestone,
            row.milestone_id(),
            "semantic-vs-physical-claim",
            vec![milestone_evidence_ref(row.milestone_id(), family)],
            row.forbidden_claims().to_vec(),
            row.deferred_s_sequences().to_vec(),
            artifact_status_for(claim_status),
            claim_notes(row),
            family,
            claim_status,
            row.semantic_capability_proven(),
            row.closeout_or_planned_source(),
            row.named_suite(),
            row.evidence_lanes().to_vec(),
        )
    })
}

fn claim_status_for(
    row: &MilestonePhysicalStatusRow,
    family: SemanticPhysicalClaimFamily,
) -> SemanticPhysicalClaimStatus {
    match family {
        SemanticPhysicalClaimFamily::SemanticAuthority
        | SemanticPhysicalClaimFamily::RecoverySemantics
        | SemanticPhysicalClaimFamily::RetentionSemantics
        | SemanticPhysicalClaimFamily::SubscriptionSupport
        | SemanticPhysicalClaimFamily::CompatibilitySemantics
        | SemanticPhysicalClaimFamily::TieringPlacement
        | SemanticPhysicalClaimFamily::ReplicationSemantics => {
            SemanticPhysicalClaimStatus::SemanticProven
        }
        _ => match row.physical_status_for_claim_family(family) {
            S0PhysicalStatus::NotApplicable => SemanticPhysicalClaimStatus::NotApplicable,
            S0PhysicalStatus::NotStarted => SemanticPhysicalClaimStatus::NotStarted,
            S0PhysicalStatus::SemanticOnly => SemanticPhysicalClaimStatus::SemanticOnly,
            S0PhysicalStatus::BootstrapPhysical => SemanticPhysicalClaimStatus::BootstrapPhysical,
            S0PhysicalStatus::PhysicalDebt => SemanticPhysicalClaimStatus::PhysicalDebt,
            S0PhysicalStatus::PartiallyFoundationBacked => {
                SemanticPhysicalClaimStatus::PartiallyFoundationBacked
            }
            S0PhysicalStatus::FoundationBacked => SemanticPhysicalClaimStatus::FoundationBacked,
            S0PhysicalStatus::PlatformGrade => SemanticPhysicalClaimStatus::PlatformGrade,
        },
    }
}

fn claim_status_requires_deferred_mapping(
    family: SemanticPhysicalClaimFamily,
    status: SemanticPhysicalClaimStatus,
) -> bool {
    is_physical_family(family)
        && matches!(
            status,
            SemanticPhysicalClaimStatus::NotStarted
                | SemanticPhysicalClaimStatus::SemanticOnly
                | SemanticPhysicalClaimStatus::BootstrapPhysical
                | SemanticPhysicalClaimStatus::PhysicalDebt
                | SemanticPhysicalClaimStatus::PartiallyFoundationBacked
        )
}

fn is_physical_family(family: SemanticPhysicalClaimFamily) -> bool {
    matches!(
        family,
        SemanticPhysicalClaimFamily::PhysicalSubstrate
            | SemanticPhysicalClaimFamily::PhysicalBoundedness
            | SemanticPhysicalClaimFamily::PhysicalIntegrity
            | SemanticPhysicalClaimFamily::PhysicalRecoveryPhysics
            | SemanticPhysicalClaimFamily::PhysicalIsolation
            | SemanticPhysicalClaimFamily::PhysicalIo
            | SemanticPhysicalClaimFamily::PhysicalOperationalSafety
            | SemanticPhysicalClaimFamily::PhysicalSecurity
    )
}

fn artifact_status_for(status: SemanticPhysicalClaimStatus) -> S0ArtifactRowStatus {
    match status {
        SemanticPhysicalClaimStatus::NotApplicable => S0ArtifactRowStatus::NotApplicable,
        SemanticPhysicalClaimStatus::FoundationBacked
        | SemanticPhysicalClaimStatus::PlatformGrade
        | SemanticPhysicalClaimStatus::SemanticProven => S0ArtifactRowStatus::Admitted,
        SemanticPhysicalClaimStatus::NotStarted
        | SemanticPhysicalClaimStatus::SemanticOnly
        | SemanticPhysicalClaimStatus::BootstrapPhysical
        | SemanticPhysicalClaimStatus::PhysicalDebt
        | SemanticPhysicalClaimStatus::PartiallyFoundationBacked => S0ArtifactRowStatus::Deferred,
    }
}

fn claim_row_id(
    milestone_id: &str,
    family: SemanticPhysicalClaimFamily,
) -> Result<S0ArtifactRowId, S0ClaimReportBuildRejection> {
    let milestone = milestone_id.replace('.', "_");
    let family = match family {
        SemanticPhysicalClaimFamily::SemanticAuthority => "SemanticAuthorityClaim",
        SemanticPhysicalClaimFamily::RecoverySemantics => "RecoverySemanticsClaim",
        SemanticPhysicalClaimFamily::RetentionSemantics => "RetentionSemanticsClaim",
        SemanticPhysicalClaimFamily::SubscriptionSupport => "SubscriptionSupportClaim",
        SemanticPhysicalClaimFamily::CompatibilitySemantics => "CompatibilitySemanticsClaim",
        SemanticPhysicalClaimFamily::TieringPlacement => "TieringPlacementClaim",
        SemanticPhysicalClaimFamily::ReplicationSemantics => "ReplicationSemanticsClaim",
        SemanticPhysicalClaimFamily::PhysicalSubstrate => "PhysicalSubstrateClaim",
        SemanticPhysicalClaimFamily::PhysicalBoundedness => "PhysicalBoundednessClaim",
        SemanticPhysicalClaimFamily::PhysicalIntegrity => "PhysicalIntegrityClaim",
        SemanticPhysicalClaimFamily::PhysicalRecoveryPhysics => "PhysicalRecoveryPhysicsClaim",
        SemanticPhysicalClaimFamily::PhysicalIsolation => "PhysicalIsolationClaim",
        SemanticPhysicalClaimFamily::PhysicalIo => "PhysicalIoClaim",
        SemanticPhysicalClaimFamily::PhysicalOperationalSafety => "PhysicalOperationalSafetyClaim",
        SemanticPhysicalClaimFamily::PhysicalSecurity => "PhysicalSecurityClaim",
    };
    S0ArtifactRowId::new(format!("Milestone{milestone}{family}"))
        .map_err(|_| S0ClaimReportBuildRejection::EmptyRequiredField)
}

fn claim_notes(row: &MilestonePhysicalStatusRow) -> String {
    if row.required_wording_cleanup().is_empty() {
        "S.0 claim classification row.".to_string()
    } else {
        format!(
            "S.0 claim classification row. Required wording cleanup: {}",
            row.required_wording_cleanup().join("; ")
        )
    }
}

fn milestone_evidence_ref(
    milestone_id: &str,
    family: SemanticPhysicalClaimFamily,
) -> S0EvidenceRef {
    S0EvidenceRef::new(
        S0ArtifactKind::MilestonePhysicalStatusMatrix,
        S0StableDigest::new(format!("claim:{milestone_id}:{family:?}"))
            .expect("synthetic claim evidence digest is non-empty"),
    )
}

fn reject_duplicate_rows(
    rows: &[SemanticPhysicalClaimReportRow],
) -> Result<(), S0ClaimReportBuildRejection> {
    let mut seen = BTreeSet::new();
    if rows.iter().any(|row| !seen.insert(row.row_id().clone())) {
        return Err(S0ClaimReportBuildRejection::DuplicateRowId);
    }
    Ok(())
}

fn stable_digest<T: Serialize + ?Sized>(
    value: &T,
) -> Result<S0StableDigest, S0ClaimReportBuildRejection> {
    let bytes = serde_json::to_vec(value)
        .map_err(|_| S0ClaimReportBuildRejection::DigestConstructionFailed)?;
    let mut hasher = Sha256::new();
    hasher.update(bytes);
    S0StableDigest::new(format!("{:x}", hasher.finalize()))
        .map_err(|_| S0ClaimReportBuildRejection::DigestConstructionFailed)
}

fn require_non_empty(value: impl Into<String>) -> Result<String, S0ClaimReportBuildRejection> {
    let value = value.into();
    if value.trim().is_empty() {
        return Err(S0ClaimReportBuildRejection::EmptyRequiredField);
    }
    Ok(value)
}
