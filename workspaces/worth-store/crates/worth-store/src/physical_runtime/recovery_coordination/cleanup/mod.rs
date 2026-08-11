mod admission;
mod freshness;
mod removal;

pub use freshness::{
    CompletedPhysicalRecoveryCleanupFreshnessRead, PhysicalRecoveryCleanupFreshnessReadDenial,
    PhysicalRecoveryCleanupFreshnessReadDenialKind, PhysicalRecoveryCleanupFreshnessReadOutcome,
    PhysicalRecoveryCleanupFreshnessReadProgress,
};
pub use removal::{
    CompletedPhysicalRecoveryCleanupRemoval, PhysicalRecoveryCleanupRemovalCommand,
    PhysicalRecoveryCleanupRemovalDenial, PhysicalRecoveryCleanupRemovalDenialKind,
    PhysicalRecoveryCleanupRemovalIndeterminate, PhysicalRecoveryCleanupRemovalOutcome,
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

impl PhysicalRecoveryCoordination {
    pub(in crate::physical_runtime) fn read_cleanup_current_selector(
        &self,
        media: &worth_store_physical_backend::AdmittedRecoveryFilesystemMedia,
    ) -> PhysicalRecoveryCleanupFreshnessReadOutcome {
        freshness::read(self, media)
    }

    pub fn execute_cleanup_removal(
        &self,
        media: &worth_store_physical_backend::AdmittedRecoveryFilesystemMedia,
        command: PhysicalRecoveryCleanupRemovalCommand,
    ) -> PhysicalRecoveryCleanupRemovalOutcome {
        removal::execute(self, media, command)
    }
}
