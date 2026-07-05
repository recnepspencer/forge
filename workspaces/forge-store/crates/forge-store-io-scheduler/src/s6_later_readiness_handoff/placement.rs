use forge_store_readiness::{S6LaterMilestoneDestination, S7PlacementReadinessNonClaim};

use crate::{
    IoSchedulerBackgroundMaintenanceAssumption, IoSchedulerForegroundInterferenceSurface,
    IoSchedulerS6CounterSnapshot, IoSchedulerS6ReadinessAdmission,
};

use super::{core::S6LaterReadinessEvidenceCore, S6LaterReadinessReadmissionState};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct S7PlacementIoReadinessHandoff {
    core: S6LaterReadinessEvidenceCore,
    non_claims: [S7PlacementReadinessNonClaim; 3],
}

pub fn publish_s7_placement_io_readiness_handoff(
    readiness: &IoSchedulerS6ReadinessAdmission,
) -> S7PlacementIoReadinessHandoff {
    S7PlacementIoReadinessHandoff {
        core: S6LaterReadinessEvidenceCore::from_current_readiness(readiness),
        non_claims: S7PlacementReadinessNonClaim::required(),
    }
}

pub const fn readmit_s7_placement_io_readiness_after_publication(
    handoff: S7PlacementIoReadinessHandoff,
) -> S7PlacementIoReadinessHandoff {
    handoff.with_readmission(S6LaterReadinessReadmissionState::ReadmittedAfterPublication)
}

impl S7PlacementIoReadinessHandoff {
    pub const fn destination(&self) -> S6LaterMilestoneDestination {
        S6LaterMilestoneDestination::S7Placement
    }

    pub const fn counters(&self) -> IoSchedulerS6CounterSnapshot {
        self.core.counters()
    }

    pub const fn foreground_interference(&self) -> IoSchedulerForegroundInterferenceSurface {
        self.core.foreground_interference()
    }

    pub const fn background_maintenance(&self) -> IoSchedulerBackgroundMaintenanceAssumption {
        self.core.background_maintenance()
    }

    pub const fn non_claims(&self) -> &[S7PlacementReadinessNonClaim; 3] {
        &self.non_claims
    }

    pub const fn readmission_state(&self) -> S6LaterReadinessReadmissionState {
        self.core.readmission()
    }

    const fn with_readmission(mut self, readmission: S6LaterReadinessReadmissionState) -> Self {
        self.core = self.core.with_readmission(readmission);
        self
    }
}
