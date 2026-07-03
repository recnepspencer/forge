use crate::{PhysicalScenarioSchedule, PhysicalSimulationScenarioFamily};

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum S51SecurityScopeHarnessSchedule {
    StableReadPlanAdmission,
    RootSwapBeforeLogicalDecode,
    CheckpointPublicationReplay,
    RepairReadAdmission,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct S51SecurityScopePhysicalScheduleBinding {
    schedule: S51SecurityScopeHarnessSchedule,
    s5_lane_name: &'static str,
    s5_scenario_family: PhysicalSimulationScenarioFamily,
}

impl S51SecurityScopeHarnessSchedule {
    pub fn physical_schedule(self) -> PhysicalScenarioSchedule {
        PhysicalScenarioSchedule::named_boundary_yieldpoint(self.yieldpoint_name())
    }

    pub const fn physical_replay_binding(self) -> S51SecurityScopePhysicalScheduleBinding {
        let (s5_lane_name, s5_scenario_family) = match self {
            Self::StableReadPlanAdmission => (
                "future-chunk-stability",
                PhysicalSimulationScenarioFamily::S5FutureChunkStability,
            ),
            Self::RootSwapBeforeLogicalDecode => (
                "compaction-interlock",
                PhysicalSimulationScenarioFamily::S5CompactionInterlock,
            ),
            Self::CheckpointPublicationReplay => (
                "checkpoint-publication",
                PhysicalSimulationScenarioFamily::S5CheckpointPublicationInterlock,
            ),
            Self::RepairReadAdmission => (
                "reclaim-reachability",
                PhysicalSimulationScenarioFamily::S5ReclaimReachability,
            ),
        };
        S51SecurityScopePhysicalScheduleBinding {
            schedule: self,
            s5_lane_name,
            s5_scenario_family,
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

impl S51SecurityScopePhysicalScheduleBinding {
    pub const fn schedule(self) -> S51SecurityScopeHarnessSchedule {
        self.schedule
    }

    pub const fn s5_lane_name(self) -> &'static str {
        self.s5_lane_name
    }

    pub const fn s5_scenario_family(self) -> PhysicalSimulationScenarioFamily {
        self.s5_scenario_family
    }
}
