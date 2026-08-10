use super::super::artifacts::{
    S0ArtifactEnvelopeMetadata, S0ArtifactRowId, S0ArtifactSubjectKind, S0NondeterministicMetadata,
};
use super::super::evidence::{S0ArtifactKind, S0EvidenceRef, S0StableDigest};
use super::super::milestones::{MilestonePhysicalStatusRow, SemanticPhysicalClaimFamily};
use super::claim_policy::{artifact_status_for, claim_status_for};
use super::claim_report_row::SemanticPhysicalClaimReportRow;
use super::claim_validation::{
    reject_duplicate_rows, report_digest, require_non_empty, S0ClaimReportBuildRejection,
};

#[derive(Debug, Clone, PartialEq, Eq, serde::Serialize)]
pub struct SemanticPhysicalClaimReport {
    #[serde(flatten)]
    pub(super) envelope: S0ArtifactEnvelopeMetadata,
    pub(super) rows: Vec<SemanticPhysicalClaimReportRow>,
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
        rows.sort_by(|left, right| left.row_id().cmp(right.row_id()));
        reject_duplicate_rows(&rows)?;
        let deterministic_digest = report_digest(
            &source_revision,
            &roadmap_parent_digest,
            &generated_by,
            &rows,
        )?;
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
