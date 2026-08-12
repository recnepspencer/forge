mod admission;
mod freshness;
mod removal;

pub use freshness::{
    CompletedPhysicalRecoveryCleanupFreshnessRead, PhysicalRecoveryCleanupFreshnessReadDenial,
    PhysicalRecoveryCleanupFreshnessReadDenialKind, PhysicalRecoveryCleanupFreshnessReadOutcome,
    PhysicalRecoveryCleanupFreshnessReadProgress,
};
pub(in crate::physical_runtime) use removal::PhysicalRecoveryCleanupRemovalCommand;
pub use removal::{
    CompletedPhysicalRecoveryCleanupRemoval, PhysicalRecoveryCleanupRemovalDenial,
    PhysicalRecoveryCleanupRemovalDenialKind, PhysicalRecoveryCleanupRemovalIndeterminate,
    PhysicalRecoveryCleanupRemovalOutcome,
};

pub use admission::{
    PhysicalRecoveryCleanupAdmissionDenial, PhysicalRecoveryCleanupAdmissionDenialKind,
};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PhysicalRecoveryCleanupCommandStage {
    FreshnessRead,
    Removal,
}

use super::PhysicalRecoveryCoordination;
use crate::physical_runtime::{
    CompletedPhysicalRecoveryFreshReopen, StoreRecoveryCleanupAttempt,
    StoreRecoveryCleanupFreshnessFailure, StoreRecoveryCleanupPlan,
};

impl PhysicalRecoveryCoordination {
    pub fn admit_cleanup_plan<'e>(
        &self,
        media: &worth_store_physical_backend::AdmittedRecoveryFilesystemMedia,
        reopened: &CompletedPhysicalRecoveryFreshReopen,
        checkpoint: &'e worth_store_physical_format::VerifiedCheckpointStream,
        wal: impl IntoIterator<Item = worth_store_wal::VerifiedWalArtifact>,
    ) -> Result<StoreRecoveryCleanupPlan<'e>, StoreRecoveryCleanupFreshnessFailure> {
        crate::physical_runtime::recovery_freshness::admit_cleanup_plan(
            self, media, reopened, checkpoint, wal,
        )
    }

    pub(in crate::physical_runtime) fn read_cleanup_current_selector(
        &self,
        media: &worth_store_physical_backend::AdmittedRecoveryFilesystemMedia,
    ) -> PhysicalRecoveryCleanupFreshnessReadOutcome {
        freshness::read(self, media)
    }

    pub fn execute_cleanup_candidate(
        &self,
        media: &worth_store_physical_backend::AdmittedRecoveryFilesystemMedia,
        plan: &mut StoreRecoveryCleanupPlan<'_>,
        artifact: worth_store_wal::WalSegmentArtifactIdentity,
    ) -> StoreRecoveryCleanupAttempt {
        let admission = match crate::physical_runtime::recovery_freshness::cleanup::sample(
            self.freshness(),
            self,
            media,
            plan,
            artifact,
        ) {
            Ok(admission) => admission,
            Err(failure) => return StoreRecoveryCleanupAttempt::FreshnessDenied(failure),
        };
        let (freshness, command) = admission.into_parts();
        match command {
            Some(command) => StoreRecoveryCleanupAttempt::Removal {
                freshness,
                outcome: removal::execute(self, media, command),
            },
            None => StoreRecoveryCleanupAttempt::PublishedGenerationChanged(freshness),
        }
    }
}
