use crate::runtime::{
    WorthUiCanvasSpatialCertification, WorthUiCanvasSpatialCounters, WorthUiCanvasSpatialNode,
    WorthUiRuntimeHandleAllocationReceipt,
};

#[derive(Clone, Debug)]
pub struct WorthUiCanvasSpatialPlan {
    handle_receipt: WorthUiRuntimeHandleAllocationReceipt,
    support_digest: u64,
    canvas_plan_digest: u64,
    host_binding: crate::facade::WorthUiHostPlanBinding,
    region_store: crate::runtime::planning::plan_topology::WorthUiPlanRegionStore,
    spatial_slots: crate::runtime::planning::plan_topology::WorthUiPlanRegionSlotSetView<1>,
    counters: WorthUiCanvasSpatialCounters,
}

pub(crate) struct WorthUiCanvasSpatialPlanInput {
    pub handle_receipt: WorthUiRuntimeHandleAllocationReceipt,
    pub support_digest: u64,
    pub canvas_plan_digest: u64,
    pub host_binding: crate::facade::WorthUiHostPlanBinding,
    pub region_store: crate::runtime::planning::plan_topology::WorthUiPlanRegionStore,
    pub spatial_slots: crate::runtime::planning::plan_topology::WorthUiPlanRegionSlotSetView<1>,
    pub counters: WorthUiCanvasSpatialCounters,
}

impl PartialEq for WorthUiCanvasSpatialPlan {
    fn eq(&self, other: &Self) -> bool {
        self.handle_receipt == other.handle_receipt
            && self.support_digest == other.support_digest
            && self.canvas_plan_digest == other.canvas_plan_digest
            && self.host_binding == other.host_binding
            && self.spatial_slots == other.spatial_slots
    }
}

impl Eq for WorthUiCanvasSpatialPlan {}

impl WorthUiCanvasSpatialPlan {
    pub(crate) fn new(input: WorthUiCanvasSpatialPlanInput) -> Self {
        Self {
            handle_receipt: input.handle_receipt,
            support_digest: input.support_digest,
            canvas_plan_digest: input.canvas_plan_digest,
            host_binding: input.host_binding,
            region_store: input.region_store,
            spatial_slots: input.spatial_slots,
            counters: input.counters,
        }
    }

    pub fn handle_receipt(&self) -> WorthUiRuntimeHandleAllocationReceipt {
        self.handle_receipt
    }
    pub fn counters(&self) -> WorthUiCanvasSpatialCounters {
        self.counters
    }

    pub(crate) fn first_row(&self) -> Option<WorthUiCanvasSpatialNode> {
        self.spatial_slots
            .first()
            .and_then(|slot| u32::try_from(slot).ok())
            .and_then(|index| self.row_for_plan_index(index))
    }

    pub(crate) fn row_for_plan_index(&self, plan_index: u32) -> Option<WorthUiCanvasSpatialNode> {
        let slot = u64::from(plan_index);
        if !self.spatial_slots.contains(slot) {
            return None;
        }
        let executable = self.region_store.executable_for_stable_slot(slot)?;
        let meaning = executable.spatial_meaning_reference()?;
        let handle = self
            .region_store
            .runtime_handle_for_stable_slot(slot, self.handle_receipt.arena_identity())?;
        Some(WorthUiCanvasSpatialNode::new(
            handle,
            meaning.contract(),
            self.host_binding,
            self.handle_receipt.basis_digest(),
        ))
    }

    pub(crate) fn certification(&self) -> WorthUiCanvasSpatialCertification {
        WorthUiCanvasSpatialCertification::new(
            self.canvas_plan_digest,
            self.support_digest,
            self.handle_receipt,
            self.host_binding,
        )
    }
}
