use super::super::artifacts::{BackendCapabilityMatrix, S0ArtifactRowStatus};
use super::super::capability::Roadmap2SequenceId;
use super::super::deferred::DeferredPhysicalGuaranteeMap;
use super::super::evidence::{S0ArtifactKind, S0EvidenceRef};
use super::super::terminology::TerminologyRiskReport;
use super::digest::stable_digest;
use super::maturity::{
    ForbiddenShortcutDetectionStatus, HarnessMaturityLevel, HarnessSubsystemMaturity,
};
use super::row::HarnessMaturityRow;
use super::validation::{harness_row_id, S0HarnessMaturityBuildRejection};

pub(super) fn stale_handoff_row(
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
