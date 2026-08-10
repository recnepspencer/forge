use super::super::artifacts::{
    S0ArtifactEnvelopeMetadata, S0ArtifactRowId, S0ArtifactRowStatus, S0ArtifactSubjectKind,
    S0NondeterministicMetadata,
};
use super::super::evidence::{S0ArtifactKind, S0EvidenceRef, S0StableDigest};
use super::super::milestones::MilestonePhysicalStatusRow;
use super::deferred_category_policy::{
    current_status_for_category, deferred_category_from_claim_family,
    supplementary_category_from_forbidden_claim_kind, DeferredPhysicalGuaranteeCategory,
};
use super::deferred_guarantee_row::DeferredPhysicalGuaranteeRow;
use super::deferred_validation::{
    map_digest, reject_duplicate_rows, require_non_empty, S0DeferredGuaranteeBuildRejection,
};

#[derive(Debug, Clone, PartialEq, Eq, serde::Serialize)]
pub struct DeferredPhysicalGuaranteeMap {
    #[serde(flatten)]
    pub(super) envelope: S0ArtifactEnvelopeMetadata,
    pub(super) rows: Vec<DeferredPhysicalGuaranteeRow>,
}

impl DeferredPhysicalGuaranteeMap {
    pub fn new(
        source_revision: impl Into<String>,
        roadmap_parent_digest: S0StableDigest,
        generated_by: impl Into<String>,
        nondeterministic_metadata: S0NondeterministicMetadata,
        mut rows: Vec<DeferredPhysicalGuaranteeRow>,
    ) -> Result<Self, S0DeferredGuaranteeBuildRejection> {
        let source_revision = require_non_empty(source_revision)?;
        let generated_by = require_non_empty(generated_by)?;
        rows.sort_by(|left, right| left.row_id().cmp(right.row_id()));
        reject_duplicate_rows(&rows)?;
        let deterministic_digest = map_digest(
            &source_revision,
            &roadmap_parent_digest,
            &generated_by,
            &rows,
        )?;
        let envelope = S0ArtifactEnvelopeMetadata::new(
            S0ArtifactKind::DeferredPhysicalGuaranteeMap,
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
    ) -> Result<Self, S0DeferredGuaranteeBuildRejection> {
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
) -> impl Iterator<Item = Result<DeferredPhysicalGuaranteeRow, S0DeferredGuaranteeBuildRejection>> + '_
{
    let mut categories = row
        .claim_families()
        .iter()
        .filter_map(|family| deferred_category_from_claim_family(*family, row))
        .collect::<Vec<_>>();
    categories.extend(
        row.forbidden_claims().iter().filter_map(|claim| {
            supplementary_category_from_forbidden_claim_kind(claim.claim_kind())
        }),
    );
    categories.sort();
    categories.dedup();

    categories.into_iter().map(|category| {
        let current_status = current_status_for_category(row, category);
        DeferredPhysicalGuaranteeRow::new(
            guarantee_row_id(row.milestone_id(), category)?,
            S0ArtifactSubjectKind::Milestone,
            row.milestone_id(),
            "deferred-physical-guarantee",
            vec![milestone_evidence_ref(row.milestone_id(), category)],
            row.forbidden_claims().to_vec(),
            row.deferred_s_sequences().to_vec(),
            S0ArtifactRowStatus::Deferred,
            guarantee_notes(row),
            category,
            current_status,
            category.missing_proof_summary(),
            row.named_suite(),
            row.evidence_lanes().to_vec(),
        )
    })
}

fn guarantee_row_id(
    milestone_id: &str,
    category: DeferredPhysicalGuaranteeCategory,
) -> Result<S0ArtifactRowId, S0DeferredGuaranteeBuildRejection> {
    let milestone = milestone_id.replace('.', "_");
    let category = match category {
        DeferredPhysicalGuaranteeCategory::PageSegmentExtentSubstrate => {
            "PageSegmentExtentSubstrate"
        }
        DeferredPhysicalGuaranteeCategory::MemoryAllocationBoundedness => {
            "MemoryAllocationBoundedness"
        }
        DeferredPhysicalGuaranteeCategory::PageFrameChunkIntegrityAndCorruptionLocalization => {
            "PageFrameChunkIntegrity"
        }
        DeferredPhysicalGuaranteeCategory::WalCheckpointLsnRecoveryPhysics => {
            "WalCheckpointRecoveryPhysics"
        }
        DeferredPhysicalGuaranteeCategory::PhysicalReadStabilityDuringMaintenance => {
            "PhysicalReadStability"
        }
        DeferredPhysicalGuaranteeCategory::HardwareAwareIoAndForegroundQos => "HardwareAwareIoQos",
        DeferredPhysicalGuaranteeCategory::NativeBlobObjectChunkStore => "NativeBlobChunkStore",
        DeferredPhysicalGuaranteeCategory::IndexLayoutAccessPathDiscipline => {
            "IndexLayoutAccessPathDiscipline"
        }
        DeferredPhysicalGuaranteeCategory::FormalCrashConcurrencyModels => {
            "FormalCrashConcurrencyModels"
        }
        DeferredPhysicalGuaranteeCategory::BackupPitrRepairAndForensics => {
            "BackupPitrRepairForensics"
        }
        DeferredPhysicalGuaranteeCategory::SecurityTenantBoundariesKeysAndAuditability => {
            "SecurityTenantKeysAuditability"
        }
        DeferredPhysicalGuaranteeCategory::PhysicalDatabaseCertificationAndPerformance => {
            "PhysicalDatabaseCertification"
        }
    };
    S0ArtifactRowId::new(format!("Milestone{milestone}{category}"))
        .map_err(|_| S0DeferredGuaranteeBuildRejection::EmptyRequiredField)
}

fn guarantee_notes(row: &MilestonePhysicalStatusRow) -> String {
    if row.required_wording_cleanup().is_empty() {
        "S.0 deferred physical guarantee row.".to_string()
    } else {
        format!(
            "S.0 deferred physical guarantee row. Required wording cleanup: {}",
            row.required_wording_cleanup().join("; ")
        )
    }
}

fn milestone_evidence_ref(
    milestone_id: &str,
    category: DeferredPhysicalGuaranteeCategory,
) -> S0EvidenceRef {
    S0EvidenceRef::new(
        S0ArtifactKind::MilestonePhysicalStatusMatrix,
        S0StableDigest::new(format!("deferred:{milestone_id}:{category:?}"))
            .expect("synthetic deferred evidence digest is non-empty"),
    )
}
