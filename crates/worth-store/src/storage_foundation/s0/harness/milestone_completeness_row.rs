use super::super::artifacts::S0ArtifactRowStatus;
use super::super::capability::Roadmap2SequenceId;
use super::super::evidence::{S0ArtifactKind, S0EvidenceRef};
use super::digest::stable_digest;
use super::fixtures::S1ForbiddenShortcut;
use super::maturity::{
    ForbiddenShortcutDetectionStatus, HarnessMaturityLevel, HarnessSubsystemMaturity,
};
use super::row::HarnessMaturityRow;
use super::validation::{harness_row_id, S0HarnessMaturityBuildRejection};

pub(super) fn milestone_completeness_row(
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
