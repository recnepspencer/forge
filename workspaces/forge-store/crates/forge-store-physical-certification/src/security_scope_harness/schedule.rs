use crate::{PhysicalScenarioSchedule, PhysicalSimulationScenarioFamily};

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum SecurityScopeHarnessSchedule {
    StableReadPlanAdmission,
    RootSwapBeforeLogicalDecode,
    CheckpointPublicationReplay,
    RepairReadAdmission,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct SecurityScopePhysicalScheduleBinding {
    schedule: SecurityScopeHarnessSchedule,
    physical_isolation_lane_name: &'static str,
    physical_isolation_scenario_family: PhysicalSimulationScenarioFamily,
}

impl SecurityScopeHarnessSchedule {
    pub fn physical_schedule(self) -> PhysicalScenarioSchedule {
        PhysicalScenarioSchedule::named_boundary_yieldpoint(self.yieldpoint_name())
    }

    pub const fn physical_replay_binding(self) -> SecurityScopePhysicalScheduleBinding {
        let (physical_isolation_lane_name, physical_isolation_scenario_family) = match self {
            Self::StableReadPlanAdmission => (
                "future-chunk-stability",
                PhysicalSimulationScenarioFamily::PhysicalIsolationFutureChunkStability,
            ),
            Self::RootSwapBeforeLogicalDecode => (
                "compaction-interlock",
                PhysicalSimulationScenarioFamily::PhysicalIsolationCompactionInterlock,
            ),
            Self::CheckpointPublicationReplay => (
                "checkpoint-publication",
                PhysicalSimulationScenarioFamily::PhysicalIsolationCheckpointPublicationInterlock,
            ),
            Self::RepairReadAdmission => (
                "reclaim-reachability",
                PhysicalSimulationScenarioFamily::PhysicalIsolationReclaimReachability,
            ),
        };
        SecurityScopePhysicalScheduleBinding {
            schedule: self,
            physical_isolation_lane_name,
            physical_isolation_scenario_family,
        }
    }

    pub const fn yieldpoint_name(self) -> &'static str {
        match self {
            Self::StableReadPlanAdmission => "s5.1-security-scope-stable-read-admission",
            Self::RootSwapBeforeLogicalDecode => "s5.1-security-scope-root-swap-before-decode",
            Self::CheckpointPublicationReplay => "s5.1-security-scope-checkpoint-replay",
            Self::RepairReadAdmission => "s5.1-security-scope-repair-read-admission",
        }
    }
}

impl SecurityScopePhysicalScheduleBinding {
    pub const fn schedule(self) -> SecurityScopeHarnessSchedule {
        self.schedule
    }

    pub const fn physical_isolation_lane_name(self) -> &'static str {
        self.physical_isolation_lane_name
    }

    pub const fn physical_isolation_scenario_family(self) -> PhysicalSimulationScenarioFamily {
        self.physical_isolation_scenario_family
    }
}
