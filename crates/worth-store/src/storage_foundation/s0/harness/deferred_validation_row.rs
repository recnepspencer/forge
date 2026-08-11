use super::super::artifacts::S0ArtifactRowStatus;
use super::super::capability::Roadmap2SequenceId;
use super::super::deferred::DeferredPhysicalGuaranteeMap;
use super::super::evidence::{S0ArtifactKind, S0EvidenceRef};
use super::fixtures::S1ForbiddenShortcut;
use super::maturity::{
    ForbiddenShortcutDetectionStatus, HarnessMaturityLevel, HarnessSubsystemMaturity,
};
use super::row::HarnessMaturityRow;
use super::validation::{harness_row_id, S0HarnessMaturityBuildRejection};

pub(super) fn deferred_validation_row(
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
