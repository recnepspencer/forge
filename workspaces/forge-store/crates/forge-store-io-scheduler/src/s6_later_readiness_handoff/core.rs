use crate::{
    IoSchedulerBackgroundMaintenanceAssumption, IoSchedulerForegroundInterferenceSurface,
    IoSchedulerIsolationCounterSnapshot, IoSchedulerIsolationAdmission,
};

use super::S6LaterReadinessReadmissionState;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) struct S6LaterReadinessEvidenceCore {
    counters: IoSchedulerIsolationCounterSnapshot,
    foreground_interference: IoSchedulerForegroundInterferenceSurface,
    background_maintenance: IoSchedulerBackgroundMaintenanceAssumption,
    readmission: S6LaterReadinessReadmissionState,
}

impl S6LaterReadinessEvidenceCore {
    pub(crate) const fn from_current_readiness(
        readiness: &IoSchedulerIsolationAdmission,
    ) -> Self {
        Self {
            counters: readiness.counters(),
            foreground_interference: readiness.foreground_interference(),
            background_maintenance: readiness.background_maintenance(),
            readmission: S6LaterReadinessReadmissionState::CurrentStoreAuthority,
        }
    }

    pub(crate) const fn counters(self) -> IoSchedulerIsolationCounterSnapshot {
        self.counters
    }

    pub(crate) const fn foreground_interference(self) -> IoSchedulerForegroundInterferenceSurface {
        self.foreground_interference
    }

    pub(crate) const fn background_maintenance(self) -> IoSchedulerBackgroundMaintenanceAssumption {
        self.background_maintenance
    }

    pub(crate) const fn readmission(self) -> S6LaterReadinessReadmissionState {
        self.readmission
    }

    pub(crate) const fn with_readmission(
        mut self,
        readmission: S6LaterReadinessReadmissionState,
    ) -> Self {
        self.readmission = readmission;
        self
    }
}
