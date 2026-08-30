use crate::observation::PhysicalIntegrityObservationOutcome;
use crate::validation::PhysicalArtifactScope;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct PhysicalIntegrityScrubInspection {
    outcome: PhysicalIntegrityObservationOutcome,
}

impl PhysicalIntegrityScrubInspection {
    pub const fn new(outcome: PhysicalIntegrityObservationOutcome) -> Self {
        Self { outcome }
    }

    pub const fn scope(self) -> PhysicalArtifactScope {
        self.outcome.scope()
    }

    pub const fn outcome(self) -> PhysicalIntegrityObservationOutcome {
        self.outcome
    }
}
