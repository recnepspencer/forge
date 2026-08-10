use super::super::artifacts::{BackendCapabilityMatrix, S0ArtifactRowStatus};
use super::super::capability::{Roadmap2SequenceId, StoreBackendCapabilityTier};
use super::super::evidence::{S0ArtifactKind, S0EvidenceRef};
use super::fixtures::S1ForbiddenShortcut;
use super::maturity::{
    ForbiddenShortcutDetectionStatus, HarnessMaturityLevel, HarnessSubsystemMaturity,
};
use super::row::HarnessMaturityRow;
use super::validation::{harness_row_id, S0HarnessMaturityBuildRejection};

pub(super) fn backend_tier_fence_row(
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
