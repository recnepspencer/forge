use crate::runtime::{
    WorthUiCanvasSpatialCertification, WorthUiCanvasSpatialCounters,
    WorthUiCanvasSpatialFrameTarget, WorthUiCanvasSpatialLane, WorthUiFrameWorkScope,
    WorthUiHandleResolutionEvidence, WorthUiRuntimeHandle,
};

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct WorthUiCanvasSpatialFrameReceipt {
    target: WorthUiCanvasSpatialFrameTarget,
    lane: WorthUiCanvasSpatialLane,
    touched_plan_indexes: [u32; 1],
    touched_runtime_handles: [WorthUiRuntimeHandle; 1],
    visible_primitive_count: u32,
    queried_hit_test_region_count: u32,
    touched_overlay_row_count: u16,
    touched_tool_state_row_count: u16,
    touch_digest: u64,
    counters: WorthUiCanvasSpatialCounters,
    certification: WorthUiCanvasSpatialCertification,
    resolution_evidence: WorthUiHandleResolutionEvidence,
    work_scope: WorthUiFrameWorkScope,
}

pub(crate) struct WorthUiCanvasSpatialFrameReceiptInput {
    pub target: WorthUiCanvasSpatialFrameTarget,
    pub lane: WorthUiCanvasSpatialLane,
    pub touched_plan_index: u32,
    pub touched_runtime_handle: WorthUiRuntimeHandle,
    pub visible_primitive_count: u32,
    pub queried_hit_test_region_count: u32,
    pub touched_overlay_row_count: u16,
    pub touched_tool_state_row_count: u16,
    pub counters: WorthUiCanvasSpatialCounters,
    pub certification: WorthUiCanvasSpatialCertification,
    pub resolution_evidence: WorthUiHandleResolutionEvidence,
    pub work_scope: WorthUiFrameWorkScope,
}

impl WorthUiCanvasSpatialFrameReceipt {
    pub(crate) fn new(input: WorthUiCanvasSpatialFrameReceiptInput) -> Self {
        let touch_digest = u64::from(input.touched_plan_index)
            ^ u64::from(input.visible_primitive_count).rotate_left(11)
            ^ u64::from(input.queried_hit_test_region_count).rotate_left(23)
            ^ u64::from(input.touched_overlay_row_count).rotate_left(37)
            ^ u64::from(input.touched_tool_state_row_count).rotate_left(49);
        Self {
            target: input.target,
            lane: input.lane,
            touched_plan_indexes: [input.touched_plan_index],
            touched_runtime_handles: [input.touched_runtime_handle],
            visible_primitive_count: input.visible_primitive_count,
            queried_hit_test_region_count: input.queried_hit_test_region_count,
            touched_overlay_row_count: input.touched_overlay_row_count,
            touched_tool_state_row_count: input.touched_tool_state_row_count,
            touch_digest,
            counters: input.counters,
            certification: input.certification,
            resolution_evidence: input.resolution_evidence,
            work_scope: input.work_scope,
        }
    }
    pub fn target(&self) -> WorthUiCanvasSpatialFrameTarget {
        self.target
    }
    pub fn lane(&self) -> WorthUiCanvasSpatialLane {
        self.lane
    }
    pub fn touched_plan_indexes(&self) -> &[u32] {
        &self.touched_plan_indexes
    }
    pub fn touched_runtime_handles(&self) -> &[WorthUiRuntimeHandle] {
        &self.touched_runtime_handles
    }
    pub fn visible_primitive_count(&self) -> u32 {
        self.visible_primitive_count
    }
    pub fn queried_hit_test_region_count(&self) -> u32 {
        self.queried_hit_test_region_count
    }
    pub fn touched_overlay_row_count(&self) -> u16 {
        self.touched_overlay_row_count
    }
    pub fn touched_tool_state_row_count(&self) -> u16 {
        self.touched_tool_state_row_count
    }
    pub fn touch_digest(&self) -> u64 {
        self.touch_digest
    }
    pub fn counters(&self) -> WorthUiCanvasSpatialCounters {
        self.counters
    }
    pub fn certification(&self) -> WorthUiCanvasSpatialCertification {
        self.certification
    }
    pub fn resolution_evidence(&self) -> WorthUiHandleResolutionEvidence {
        self.resolution_evidence
    }
    pub fn work_scope(&self) -> WorthUiFrameWorkScope {
        self.work_scope
    }
}
