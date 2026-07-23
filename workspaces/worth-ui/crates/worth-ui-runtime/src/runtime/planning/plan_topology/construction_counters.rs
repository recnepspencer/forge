use crate::runtime::{
    WorthUiLaneAdmissionCounters, WorthUiPlanLoweringCounters, WorthUiPlanRegionStorageCounters,
    WorthUiPlanTopologyCounters, WorthUiRuntimeHandleAllocationCounters,
};

/// Exact work performed while constructing one complete candidate plan.
///
/// Regional locality is not reported separately from the flat work that still
/// surrounds it. Callers inspecting replacement cost therefore cannot mistake
/// a bounded regional update for a bounded complete successor construction.
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct WorthUiPlanConstructionCounters {
    lowering: WorthUiPlanLoweringCounters,
    handle_allocation: WorthUiRuntimeHandleAllocationCounters,
    lane_admission: WorthUiLaneAdmissionCounters,
    topology: WorthUiPlanTopologyCounters,
    regional_storage: WorthUiPlanRegionStorageCounters,
}

impl WorthUiPlanConstructionCounters {
    pub(crate) fn new(
        lowering: WorthUiPlanLoweringCounters,
        handle_allocation: WorthUiRuntimeHandleAllocationCounters,
        lane_admission: WorthUiLaneAdmissionCounters,
        topology: WorthUiPlanTopologyCounters,
        regional_storage: WorthUiPlanRegionStorageCounters,
    ) -> Self {
        Self {
            lowering,
            handle_allocation,
            lane_admission,
            topology,
            regional_storage,
        }
    }

    pub fn lowering(self) -> WorthUiPlanLoweringCounters {
        self.lowering
    }

    pub fn handle_allocation(self) -> WorthUiRuntimeHandleAllocationCounters {
        self.handle_allocation
    }

    pub fn lane_admission(self) -> WorthUiLaneAdmissionCounters {
        self.lane_admission
    }

    pub fn topology(self) -> WorthUiPlanTopologyCounters {
        self.topology
    }

    pub fn regional_storage(self) -> WorthUiPlanRegionStorageCounters {
        self.regional_storage
    }

    pub fn full_candidate_node_visit_count(self) -> usize {
        self.lowering.staged_node_input_count()
            + self.lowering.query_binding_input_count()
            + self.lowering.component_hook_input_count()
            + self.handle_allocation.plan_node_input_count()
            + self.lane_admission.plan_node_visit_count()
            + self.topology.plan_node_input_count()
    }
}
