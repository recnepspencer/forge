use super::super::claims::SemanticPhysicalClaimStatus;
use super::super::evidence::S0StableDigest;
use super::super::milestones::MilestonePhysicalStatusRow;
use super::migration_notes::TestMigrationNotes;
use super::test_migration_note_row::TestMigrationNoteRow;
use super::validation::{migration_evidence_ref, migration_row_id, S0TestMigrationBuildRejection};
use std::collections::BTreeSet;

impl TestMigrationNotes {
    pub fn from_milestone_rows(
        source_revision: impl Into<String>,
        roadmap_parent_digest: S0StableDigest,
        generated_by: impl Into<String>,
        nondeterministic_metadata: super::super::artifacts::S0NondeterministicMetadata,
        milestone_rows: &[MilestonePhysicalStatusRow],
    ) -> Result<Self, S0TestMigrationBuildRejection> {
        let rows = milestone_rows
            .iter()
            .map(row_for_milestone)
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

fn row_for_milestone(
    row: &MilestonePhysicalStatusRow,
) -> Result<TestMigrationNoteRow, S0TestMigrationBuildRejection> {
    let evidence_scope = milestone_scope(row);
    let required_followup_guarantees = row
        .forbidden_claims()
        .iter()
        .map(|claim| format!("{:?}", claim.claim_kind()))
        .collect::<BTreeSet<_>>()
        .into_iter()
        .collect::<Vec<_>>();
    let status = match evidence_scope {
        SemanticPhysicalClaimStatus::FoundationBacked
        | SemanticPhysicalClaimStatus::PlatformGrade => {
            super::super::artifacts::S0ArtifactRowStatus::Admitted
        }
        _ => super::super::artifacts::S0ArtifactRowStatus::Deferred,
    };
    TestMigrationNoteRow::new(
        migration_row_id(row.milestone_id(), row.named_suite())?,
        row.milestone_id(),
        vec![migration_evidence_ref(
            row.closeout_or_planned_source(),
            row.named_suite(),
        )?],
        row.forbidden_claims().to_vec(),
        row.deferred_s_sequences().to_vec(),
        status,
        format!(
            "{} remains {:?} evidence until deferred Roadmap 2 guarantees close.",
            row.named_suite(),
            evidence_scope
        ),
        row.named_suite(),
        evidence_scope,
        if required_followup_guarantees.is_empty() {
            vec!["no additional followup guarantee required".to_string()]
        } else {
            required_followup_guarantees
        },
    )
}

fn milestone_scope(row: &MilestonePhysicalStatusRow) -> SemanticPhysicalClaimStatus {
    let strongest = row
        .claim_families()
        .iter()
        .map(|family| row.physical_status_for_claim_family(*family))
        .max()
        .unwrap_or(super::super::milestones::S0PhysicalStatus::SemanticOnly);
    match strongest {
        super::super::milestones::S0PhysicalStatus::NotApplicable
        | super::super::milestones::S0PhysicalStatus::NotStarted
        | super::super::milestones::S0PhysicalStatus::SemanticOnly => {
            SemanticPhysicalClaimStatus::SemanticOnly
        }
        super::super::milestones::S0PhysicalStatus::BootstrapPhysical => {
            SemanticPhysicalClaimStatus::BootstrapPhysical
        }
        super::super::milestones::S0PhysicalStatus::PhysicalDebt => {
            SemanticPhysicalClaimStatus::PhysicalDebt
        }
        super::super::milestones::S0PhysicalStatus::PartiallyFoundationBacked => {
            SemanticPhysicalClaimStatus::PartiallyFoundationBacked
        }
        super::super::milestones::S0PhysicalStatus::FoundationBacked => {
            SemanticPhysicalClaimStatus::FoundationBacked
        }
        super::super::milestones::S0PhysicalStatus::PlatformGrade => {
            SemanticPhysicalClaimStatus::PlatformGrade
        }
    }
}
