use crate::runtime::{
    WorthUiCanvasDrawHook, WorthUiCanvasSpatialCertification, WorthUiCanvasSpatialCounters,
    WorthUiCanvasSpatialNode, WorthUiRuntimeHandleAllocationReceipt, WorthUiSpatialHitTestHook,
    WorthUiSpatialToolStateHook, WorthUiStateSlotHandle,
};

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct WorthUiCanvasSpatialPlan {
    handle_receipt: WorthUiRuntimeHandleAllocationReceipt,
    support_digest: u64,
    canvas_plan_digest: u64,
    rows: Vec<WorthUiCanvasSpatialNode>,
    command_plan_indexes: Vec<u32>,
    diagnostics_plan_indexes: Vec<u32>,
    selection_state_slot_handles: Vec<WorthUiStateSlotHandle>,
    draw_hooks: Vec<WorthUiCanvasDrawHook>,
    hit_test_hooks: Vec<WorthUiSpatialHitTestHook>,
    tool_state_hooks: Vec<WorthUiSpatialToolStateHook>,
    counters: WorthUiCanvasSpatialCounters,
}

pub(crate) struct WorthUiCanvasSpatialPlanInput {
    pub handle_receipt: WorthUiRuntimeHandleAllocationReceipt,
    pub support_digest: u64,
    pub canvas_plan_digest: u64,
    pub rows: Vec<WorthUiCanvasSpatialNode>,
    pub command_plan_indexes: Vec<u32>,
    pub diagnostics_plan_indexes: Vec<u32>,
    pub selection_state_slot_handles: Vec<WorthUiStateSlotHandle>,
    pub draw_hooks: Vec<WorthUiCanvasDrawHook>,
    pub hit_test_hooks: Vec<WorthUiSpatialHitTestHook>,
    pub tool_state_hooks: Vec<WorthUiSpatialToolStateHook>,
    pub counters: WorthUiCanvasSpatialCounters,
}

impl WorthUiCanvasSpatialPlan {
    pub(crate) fn new(input: WorthUiCanvasSpatialPlanInput) -> Self {
        let WorthUiCanvasSpatialPlanInput {
            handle_receipt,
            support_digest,
            canvas_plan_digest,
            rows,
            command_plan_indexes,
            diagnostics_plan_indexes,
            selection_state_slot_handles,
            draw_hooks,
            hit_test_hooks,
            tool_state_hooks,
            counters,
        } = input;
        Self {
            handle_receipt,
            support_digest,
            canvas_plan_digest,
            rows,
            command_plan_indexes,
            diagnostics_plan_indexes,
            selection_state_slot_handles,
            draw_hooks,
            hit_test_hooks,
            tool_state_hooks,
            counters,
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

    pub fn rows(&self) -> &[WorthUiCanvasSpatialNode] {
        &self.rows
    }

    pub fn command_plan_indexes(&self) -> &[u32] {
        &self.command_plan_indexes
    }

    pub fn diagnostics_plan_indexes(&self) -> &[u32] {
        &self.diagnostics_plan_indexes
    }

    pub fn selection_state_slot_handles(&self) -> &[WorthUiStateSlotHandle] {
        &self.selection_state_slot_handles
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

    pub(crate) fn row_for_plan_index(&self, plan_index: u32) -> Option<&WorthUiCanvasSpatialNode> {
        self.rows
            .binary_search_by_key(&plan_index, WorthUiCanvasSpatialNode::plan_index)
            .ok()
            .map(|index| &self.rows[index])
    }

    pub(crate) fn certification(&self) -> WorthUiCanvasSpatialCertification {
        WorthUiCanvasSpatialCertification::new(
            self.canvas_plan_digest,
            self.support_digest,
            self.handle_receipt,
        )
    }
}
