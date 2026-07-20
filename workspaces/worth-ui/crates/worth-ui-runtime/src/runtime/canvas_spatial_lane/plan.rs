use crate::runtime::{
    WorthUiCanvasDrawHook, WorthUiCanvasSpatialCertification, WorthUiCanvasSpatialCounters,
    WorthUiCanvasSpatialNode, WorthUiRuntimeHandleAllocationReceipt, WorthUiSpatialHitTestHook,
    WorthUiSpatialToolStateHook,
};

#[derive(Clone, Debug)]
pub struct WorthUiCanvasSpatialPlan {
    handle_receipt: WorthUiRuntimeHandleAllocationReceipt,
    support_digest: u64,
    canvas_plan_digest: u64,
    host_binding: crate::facade::WorthUiHostPlanBinding,
    region_store: crate::runtime::plan_topology::WorthUiPlanRegionStore,
    spatial_slots: crate::runtime::plan_topology::WorthUiPlanRegionSlotSetView<1>,
    draw_hooks: [WorthUiCanvasDrawHook; 1],
    hit_test_hooks: [WorthUiSpatialHitTestHook; 1],
    tool_state_hooks: [WorthUiSpatialToolStateHook; 1],
    counters: WorthUiCanvasSpatialCounters,
}

pub(crate) struct WorthUiCanvasSpatialPlanInput {
    pub handle_receipt: WorthUiRuntimeHandleAllocationReceipt,
    pub support_digest: u64,
    pub canvas_plan_digest: u64,
    pub host_binding: crate::facade::WorthUiHostPlanBinding,
    pub region_store: crate::runtime::plan_topology::WorthUiPlanRegionStore,
    pub spatial_slots: crate::runtime::plan_topology::WorthUiPlanRegionSlotSetView<1>,
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
        let draw_hooks = [WorthUiCanvasDrawHook::from_host_binding(
            input.host_binding,
            input.canvas_plan_digest,
        )];
        let hit_test_hooks = [WorthUiSpatialHitTestHook::from_host_binding(
            input.host_binding,
            input.canvas_plan_digest,
        )];
        let tool_state_hooks = [WorthUiSpatialToolStateHook::from_host_binding(
            input.host_binding,
            input.canvas_plan_digest,
        )];
        Self {
            handle_receipt: input.handle_receipt,
            support_digest: input.support_digest,
            canvas_plan_digest: input.canvas_plan_digest,
            host_binding: input.host_binding,
            region_store: input.region_store,
            spatial_slots: input.spatial_slots,
            draw_hooks,
            hit_test_hooks,
            tool_state_hooks,
            counters: input.counters,
        }
    }

    pub fn handle_receipt(&self) -> WorthUiRuntimeHandleAllocationReceipt {
        self.handle_receipt
    }
    pub fn support_digest(&self) -> u64 {
        self.support_digest
    }
    pub fn canvas_plan_digest(&self) -> u64 {
        self.canvas_plan_digest
    }
    pub fn row_count(&self) -> usize {
        self.spatial_slots.len()
    }
    pub fn draw_hooks(&self) -> &[WorthUiCanvasDrawHook] {
        &self.draw_hooks
    }
    pub fn hit_test_hooks(&self) -> &[WorthUiSpatialHitTestHook] {
        &self.hit_test_hooks
    }
    pub fn tool_state_hooks(&self) -> &[WorthUiSpatialToolStateHook] {
        &self.tool_state_hooks
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
