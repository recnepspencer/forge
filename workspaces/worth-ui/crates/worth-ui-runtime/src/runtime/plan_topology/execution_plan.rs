use crate::runtime::{
    WorthUiPlanLanePartition, WorthUiPlanLookupIndex, WorthUiPlanTopology,
    WorthUiPlanTopologyCounters, WorthUiRuntimeHandleAllocationReceipt,
};

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct WorthUiExecutionPlan {
    handle_receipt: WorthUiRuntimeHandleAllocationReceipt,
    topology: WorthUiPlanTopology,
    lane_partitions: Vec<WorthUiPlanLanePartition>,
    lookup_index: WorthUiPlanLookupIndex,
    counters: WorthUiPlanTopologyCounters,
}

impl WorthUiExecutionPlan {
    pub(crate) fn new(
        handle_receipt: WorthUiRuntimeHandleAllocationReceipt,
        topology: WorthUiPlanTopology,
        lane_partitions: Vec<WorthUiPlanLanePartition>,
        lookup_index: WorthUiPlanLookupIndex,
        counters: WorthUiPlanTopologyCounters,
    ) -> Self {
        Self {
            handle_receipt,
            topology,
            lane_partitions,
            lookup_index,
            counters,
        }
    }

    pub fn handle_receipt(&self) -> WorthUiRuntimeHandleAllocationReceipt {
        self.handle_receipt
    }

    pub fn topology(&self) -> &WorthUiPlanTopology {
        &self.topology
    }

    pub fn lane_partitions(&self) -> &[WorthUiPlanLanePartition] {
        &self.lane_partitions
    }

    pub fn lookup_index(&self) -> &WorthUiPlanLookupIndex {
        &self.lookup_index
    }

    pub fn counters(&self) -> WorthUiPlanTopologyCounters {
        self.counters
    }
}
