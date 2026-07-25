use super::{
    director::PhysicalStoreTerminationDirector, outcome::PhysicalStoreCloseOutcome,
    PhysicalStoreCloseObservation,
};
use crate::physical_runtime::instance::PhysicalStoreInstanceParts;

pub struct PhysicalStoreClosePlan {
    director: PhysicalStoreTerminationDirector,
}

impl PhysicalStoreClosePlan {
    pub(in crate::physical_runtime) fn new(parts: PhysicalStoreInstanceParts) -> Self {
        Self {
            director: PhysicalStoreTerminationDirector::new(parts),
        }
    }

    pub fn execute(self) -> PhysicalStoreCloseOutcome {
        PhysicalStoreCloseOutcome::from_shutdown(self.director.execute())
    }

    pub fn observation(&self) -> PhysicalStoreCloseObservation {
        self.director.observation()
    }

    #[cfg(feature = "certification-test-authority")]
    pub fn certification_pause_at(
        &self,
        phase: super::PhysicalStoreClosePhase,
    ) -> super::CertificationPhysicalClosePauseGate {
        self.director.certification_pause_at(phase)
    }
}
