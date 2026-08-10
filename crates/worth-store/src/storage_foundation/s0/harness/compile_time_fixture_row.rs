use super::super::artifacts::S0ArtifactRowStatus;
use super::super::capability::Roadmap2SequenceId;
use super::super::evidence::{S0ArtifactKind, S0EvidenceRef};
use super::digest::stable_digest;
use super::fixtures::{S1CompileTimeBoundaryFixture, S1ForbiddenShortcut};
use super::maturity::{
    ForbiddenShortcutDetectionStatus, HarnessMaturityLevel, HarnessSubsystemMaturity,
};
use super::row::HarnessMaturityRow;
use super::validation::{harness_row_id, S0HarnessMaturityBuildRejection};
use std::collections::BTreeSet;

pub(super) fn compile_time_fixture_row(
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
