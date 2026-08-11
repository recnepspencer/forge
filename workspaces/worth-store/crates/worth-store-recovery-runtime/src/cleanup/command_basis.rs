use worth_store::physical_runtime::{
    PhysicalRecoveryFreshnessPort, StoreRecoveryCleanupFreshnessAdmission,
    StoreRecoveryCleanupFreshnessFailure,
};
use worth_store_physical_format::VerifiedCheckpointStream;

use crate::progression::ReopenedPhysicalRecovery;

use super::RecoveryCleanupEligibility;

/// Borrowed, proof-bearing Store inputs used to construct one exact cleanup
/// command. Runtime eligibility narrows the candidate set; the Store command
/// independently revalidates checkpoint coverage before accepting it.
pub(super) struct RecoveryCleanupCommandBasis<'a> {
    reopened: &'a worth_store::physical_runtime::CompletedPhysicalRecoveryFreshReopen,
    checkpoint: &'a VerifiedCheckpointStream,
}

impl<'a> RecoveryCleanupCommandBasis<'a> {
    pub(super) fn from_reopened(reopened: &'a ReopenedPhysicalRecovery) -> Option<Self> {
        Some(Self {
            reopened: &reopened.reopened,
            checkpoint: reopened.state.selection.checkpoint()?.checkpoint(),
        })
    }

    pub(super) fn admit(
        &self,
        coordination: &worth_store::physical_runtime::PhysicalRecoveryCoordination,
        media: &worth_store::physical_runtime::AdmittedRecoveryFilesystemMedia,
        cleanup_plan_identity: [u8; 32],
        candidate: RecoveryCleanupEligibility,
    ) -> Result<StoreRecoveryCleanupFreshnessAdmission, StoreRecoveryCleanupFreshnessFailure> {
        PhysicalRecoveryFreshnessPort::sample_cleanup(
            coordination,
            media,
            cleanup_plan_identity,
            self.reopened,
            self.checkpoint,
            candidate.into_verified_artifact(),
        )
    }
}
