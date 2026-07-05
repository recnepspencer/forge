use forge_store_readiness::{S10CompactionReadinessNonClaim, S6LaterMilestoneDestination};

use crate::{
    BackgroundPacingCounterSnapshot, IoSchedulerBackgroundMaintenanceAssumption,
    IoSchedulerForegroundInterferenceSurface, IoSchedulerS6CounterSnapshot,
    IoSchedulerS6ReadinessAdmission,
};

use super::{
    core::S6LaterReadinessEvidenceCore, S10CompactionPacingEvidence,
    S6LaterReadinessReadmissionState,
};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct S10CompactionIoReadinessHandoff {
    core: S6LaterReadinessEvidenceCore,
    background_pacing: BackgroundPacingCounterSnapshot,
    non_claims: [S10CompactionReadinessNonClaim; 3],
}

pub fn publish_s10_compaction_io_readiness_handoff(
    readiness: &IoSchedulerS6ReadinessAdmission,
    background_pacing: S10CompactionPacingEvidence,
) -> S10CompactionIoReadinessHandoff {
    S10CompactionIoReadinessHandoff {
        core: S6LaterReadinessEvidenceCore::from_current_readiness(readiness),
        background_pacing: background_pacing.counters(),
        non_claims: S10CompactionReadinessNonClaim::required(),
    }
}

pub const fn readmit_s10_compaction_io_readiness_after_publication(
    handoff: S10CompactionIoReadinessHandoff,
) -> S10CompactionIoReadinessHandoff {
    handoff.with_readmission(S6LaterReadinessReadmissionState::ReadmittedAfterPublication)
}

impl S10CompactionIoReadinessHandoff {
    pub const fn destination(&self) -> S6LaterMilestoneDestination {
        S6LaterMilestoneDestination::S10Compaction
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

    pub const fn background_pacing_counters(&self) -> BackgroundPacingCounterSnapshot {
        self.background_pacing
    }

    pub const fn non_claims(&self) -> &[S10CompactionReadinessNonClaim; 3] {
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
