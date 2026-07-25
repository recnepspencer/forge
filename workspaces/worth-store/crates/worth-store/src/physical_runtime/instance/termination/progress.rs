use std::sync::{
    atomic::{AtomicU8, Ordering},
    Arc,
};

use super::phase::{PhysicalStoreClosePhase, CLOSE_PHASES};

#[derive(Clone)]
pub struct PhysicalStoreCloseObservation {
    completed_phase_count: Arc<AtomicU8>,
}

pub(in crate::physical_runtime) struct PhysicalStoreCloseProgressOwner {
    completed_phase_count: Arc<AtomicU8>,
    #[cfg(feature = "certification-test-authority")]
    yieldpoint: super::yieldpoint::PhysicalStoreCloseYieldpointOwner,
}

impl PhysicalStoreCloseObservation {
    pub fn reached(&self, phase: PhysicalStoreClosePhase) -> bool {
        self.completed_phase_count() >= phase.sequence_number()
    }

    pub fn completed_phase_count(&self) -> u8 {
        self.completed_phase_count.load(Ordering::Acquire)
    }

    pub fn latest_phase(&self) -> Option<PhysicalStoreClosePhase> {
        self.completed_phase_count()
            .checked_sub(1)
            .and_then(|index| CLOSE_PHASES.get(usize::from(index)))
            .copied()
    }
}

impl PhysicalStoreCloseProgressOwner {
    pub(super) fn new() -> Self {
        Self {
            completed_phase_count: Arc::new(AtomicU8::new(0)),
            #[cfg(feature = "certification-test-authority")]
            yieldpoint: super::yieldpoint::PhysicalStoreCloseYieldpointOwner::new(),
        }
    }

    pub(super) fn observation(&self) -> PhysicalStoreCloseObservation {
        PhysicalStoreCloseObservation {
            completed_phase_count: Arc::clone(&self.completed_phase_count),
        }
    }

    pub(in crate::physical_runtime) fn record(&self, phase: PhysicalStoreClosePhase) {
        self.completed_phase_count
            .store(phase.sequence_number(), Ordering::Release);
        #[cfg(feature = "certification-test-authority")]
        self.yieldpoint.pause(phase);
    }

    #[cfg(feature = "certification-test-authority")]
    pub(super) fn certification_pause_at(
        &self,
        phase: PhysicalStoreClosePhase,
    ) -> super::CertificationPhysicalClosePauseGate {
        self.yieldpoint.install(phase)
    }
}
