use crate::runtime::{
    WorthUiRuntimeHandleAllocationReceipt, WorthUiVirtualizedDataCertification,
    WorthUiVirtualizedDataCounters, WorthUiVirtualizedDataNode,
};

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct WorthUiVirtualizedDataPlan {
    handle_receipt: WorthUiRuntimeHandleAllocationReceipt,
    support_digest: u64,
    data_plan_digest: u64,
    region_store: crate::runtime::planning::plan_topology::WorthUiPlanRegionStore,
    query_slots: crate::runtime::planning::plan_topology::WorthUiPlanRegionSlotSetView<1>,
    counters: WorthUiVirtualizedDataCounters,
}

pub(crate) struct WorthUiVirtualizedDataPlanInput {
    pub handle_receipt: WorthUiRuntimeHandleAllocationReceipt,
    pub support_digest: u64,
    pub data_plan_digest: u64,
    pub region_store: crate::runtime::planning::plan_topology::WorthUiPlanRegionStore,
    pub query_slots: crate::runtime::planning::plan_topology::WorthUiPlanRegionSlotSetView<1>,
    pub counters: WorthUiVirtualizedDataCounters,
}

impl WorthUiVirtualizedDataPlan {
    pub(crate) fn new(input: WorthUiVirtualizedDataPlanInput) -> Self {
        Self {
            handle_receipt: input.handle_receipt,
            support_digest: input.support_digest,
            data_plan_digest: input.data_plan_digest,
            region_store: input.region_store,
            query_slots: input.query_slots,
            counters: input.counters,
        }
    }

    pub fn handle_receipt(&self) -> WorthUiRuntimeHandleAllocationReceipt {
        self.handle_receipt
    }

    pub fn row_count(&self) -> usize {
        self.query_slots.len()
    }

    pub(crate) fn row_for_plan_index(&self, plan_index: u32) -> Option<WorthUiVirtualizedDataNode> {
        let stable_slot = u64::from(plan_index);
        if !self.query_slots.contains(stable_slot) {
            return None;
        }
        let executable = self.region_store.executable_for_stable_slot(stable_slot)?;
        let runtime_handle = self
            .region_store
            .runtime_handle_for_stable_slot(stable_slot, self.handle_receipt.arena_identity())?;
        Some(WorthUiVirtualizedDataNode::new(
            runtime_handle,
            executable.query_binding_identity_reference()?,
            executable.query_settled_fact_link()?,
        ))
    }

    pub(crate) fn first_row(&self) -> Option<WorthUiVirtualizedDataNode> {
        let stable_slot = self.query_slots.first()?;
        self.row_for_plan_index(u32::try_from(stable_slot).ok()?)
    }

    pub(crate) fn certification(&self) -> WorthUiVirtualizedDataCertification {
        WorthUiVirtualizedDataCertification::new(
            self.data_plan_digest,
            self.support_digest,
            self.handle_receipt,
        )
    }
}
