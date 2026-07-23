use crate::runtime::{
    WorthUiOrdinaryLaneCertification, WorthUiOrdinaryLaneCounters, WorthUiOrdinaryLaneNode,
    WorthUiRuntimeHandleAllocationReceipt,
};

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct WorthUiOrdinaryLanePlan {
    handle_receipt: WorthUiRuntimeHandleAllocationReceipt,
    support_digest: u64,
    ordinary_plan_digest: u64,
    region_store: crate::runtime::planning::plan_topology::WorthUiPlanRegionStore,
    root_shell_slots: crate::runtime::planning::plan_topology::WorthUiPlanRegionSlotSetView<1>,
    counters: WorthUiOrdinaryLaneCounters,
}

pub(crate) struct WorthUiOrdinaryLanePlanInput {
    pub handle_receipt: WorthUiRuntimeHandleAllocationReceipt,
    pub support_digest: u64,
    pub ordinary_plan_digest: u64,
    pub region_store: crate::runtime::planning::plan_topology::WorthUiPlanRegionStore,
    pub root_shell_slots: crate::runtime::planning::plan_topology::WorthUiPlanRegionSlotSetView<1>,
    pub counters: WorthUiOrdinaryLaneCounters,
}

impl WorthUiOrdinaryLanePlan {
    pub(crate) fn new(input: WorthUiOrdinaryLanePlanInput) -> Self {
        Self {
            handle_receipt: input.handle_receipt,
            support_digest: input.support_digest,
            ordinary_plan_digest: input.ordinary_plan_digest,
            region_store: input.region_store,
            root_shell_slots: input.root_shell_slots,
            counters: input.counters,
        }
    }

    pub fn handle_receipt(&self) -> WorthUiRuntimeHandleAllocationReceipt {
        self.handle_receipt
    }

    pub fn counters(&self) -> WorthUiOrdinaryLaneCounters {
        self.counters
    }

    pub(crate) fn root_shell_slots(
        &self,
    ) -> &crate::runtime::planning::plan_topology::WorthUiPlanRegionSlotSetView<1> {
        &self.root_shell_slots
    }

    #[cfg(test)]
    pub(crate) fn first_runtime_handle_for_family(
        &self,
        family: crate::runtime::WorthUiPlanNodeInputFamily,
    ) -> Option<crate::runtime::WorthUiRuntimeHandle> {
        self.first_row_for_family(family)
            .1
            .map(|row| row.runtime_handle())
    }

    pub(crate) fn first_row_for_family(
        &self,
        family: crate::runtime::WorthUiPlanNodeInputFamily,
    ) -> (usize, Option<WorthUiOrdinaryLaneNode>) {
        let view = self.region_store.family_slot_view([family]);
        let row = view
            .first()
            .and_then(|slot| u32::try_from(slot).ok())
            .and_then(|plan_index| self.row_for_plan_index(plan_index));
        (view.len(), row)
    }

    pub(crate) fn row_for_plan_index(&self, plan_index: u32) -> Option<WorthUiOrdinaryLaneNode> {
        let stable_slot = u64::from(plan_index);
        let executable = self.region_store.executable_for_stable_slot(stable_slot)?;
        let runtime_handle = self
            .region_store
            .runtime_handle_for_stable_slot(stable_slot, self.handle_receipt.arena_identity())?;
        super::ordinary_node_from_regional(executable, runtime_handle)
    }

    pub(crate) fn certification(
        &self,
        lane: crate::runtime::WorthUiOrdinaryExecutionLane,
    ) -> WorthUiOrdinaryLaneCertification {
        WorthUiOrdinaryLaneCertification::new(
            lane,
            self.ordinary_plan_digest,
            self.support_digest,
            self.handle_receipt,
        )
    }
}
