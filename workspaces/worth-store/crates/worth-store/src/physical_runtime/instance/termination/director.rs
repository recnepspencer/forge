use super::{PhysicalStoreCloseObservation, PhysicalStoreCloseProgressOwner};
use crate::physical_runtime::instance::PhysicalStoreInstanceParts;

pub(super) struct PhysicalStoreTerminationDirector {
    parts: PhysicalStoreInstanceParts,
    progress: PhysicalStoreCloseProgressOwner,
}

impl PhysicalStoreTerminationDirector {
    pub(super) fn new(parts: PhysicalStoreInstanceParts) -> Self {
        Self {
            parts,
            progress: PhysicalStoreCloseProgressOwner::new(),
        }
    }

    pub(super) fn execute(
        self,
    ) -> crate::physical_runtime::record_serving::ServingShutdownOutcome<
        crate::physical_runtime::ClosedRuntime,
    > {
        self.parts.close(self.progress)
    }

    pub(super) fn observation(&self) -> PhysicalStoreCloseObservation {
        self.progress.observation()
    }

    #[cfg(feature = "certification-test-authority")]
    pub(super) fn certification_pause_at(
        &self,
        phase: super::PhysicalStoreClosePhase,
    ) -> super::CertificationPhysicalClosePauseGate {
        self.progress.certification_pause_at(phase)
    }
}
