use super::super::artifacts::S0ArtifactRowStatus;
use super::super::capability::Roadmap2SequenceId;
use super::super::evidence::{S0ArtifactKind, S0EvidenceRef};
use super::super::terminology::{ReleaseClaimReport, TerminologyAllowedUse, TerminologyRiskReport};
use super::fixtures::S1ForbiddenShortcut;
use super::maturity::{
    ForbiddenShortcutDetectionStatus, HarnessMaturityLevel, HarnessSubsystemMaturity,
};
use super::row::HarnessMaturityRow;
use super::validation::{harness_row_id, S0HarnessMaturityBuildRejection};

pub(super) fn terminology_claim_gate_row(
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
