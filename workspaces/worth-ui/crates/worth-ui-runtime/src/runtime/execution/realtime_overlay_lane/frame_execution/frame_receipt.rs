use crate::runtime::{
    WorthUiFrameWorkScope, WorthUiHandleResolutionEvidence, WorthUiRealtimeCertification,
    WorthUiRealtimeFrameTarget, WorthUiRealtimeLaneCounters, WorthUiRealtimeOverlayLane,
    WorthUiRendererSurfaceAdmission, WorthUiRuntimeHandle,
};

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct WorthUiRealtimeFrameReceipt {
    target: WorthUiRealtimeFrameTarget,
    lane: WorthUiRealtimeOverlayLane,
    renderer_surface_admission: WorthUiRendererSurfaceAdmission,
    touched_plan_indexes: [u32; 1],
    touched_runtime_handles: [WorthUiRuntimeHandle; 1],
    touched_overlay_row_count: u16,
    touch_digest: u64,
    counters: WorthUiRealtimeLaneCounters,
    certification: WorthUiRealtimeCertification,
    resolution_evidence: WorthUiHandleResolutionEvidence,
    work_scope: WorthUiFrameWorkScope,
}

pub(crate) struct WorthUiRealtimeFrameReceiptInput {
    pub target: WorthUiRealtimeFrameTarget,
    pub lane: WorthUiRealtimeOverlayLane,
    pub renderer_surface_admission: WorthUiRendererSurfaceAdmission,
    pub touched_plan_index: u32,
    pub touched_runtime_handle: WorthUiRuntimeHandle,
    pub touched_overlay_row_count: u16,
    pub counters: WorthUiRealtimeLaneCounters,
    pub certification: WorthUiRealtimeCertification,
    pub resolution_evidence: WorthUiHandleResolutionEvidence,
    pub work_scope: WorthUiFrameWorkScope,
}

impl WorthUiRealtimeFrameReceipt {
    pub(crate) fn new(input: WorthUiRealtimeFrameReceiptInput) -> Self {
        Self {
            target: input.target,
            lane: input.lane,
            renderer_surface_admission: input.renderer_surface_admission,
            touched_plan_indexes: [input.touched_plan_index],
            touched_runtime_handles: [input.touched_runtime_handle],
            touched_overlay_row_count: input.touched_overlay_row_count,
            touch_digest: u64::from(input.touched_plan_index)
                ^ u64::from(input.touched_overlay_row_count).rotate_left(29),
            counters: input.counters,
            certification: input.certification,
            resolution_evidence: input.resolution_evidence,
            work_scope: input.work_scope,
        }
    }

    pub fn target(&self) -> WorthUiRealtimeFrameTarget {
        self.target
    }
    pub fn lane(&self) -> WorthUiRealtimeOverlayLane {
        self.lane
    }
    pub fn renderer_surface_admission(&self) -> WorthUiRendererSurfaceAdmission {
        self.renderer_surface_admission
    }
    pub fn touched_plan_indexes(&self) -> &[u32] {
        &self.touched_plan_indexes
    }
    pub fn touched_runtime_handles(&self) -> &[WorthUiRuntimeHandle] {
        &self.touched_runtime_handles
    }
    pub fn touched_overlay_row_count(&self) -> u16 {
        self.touched_overlay_row_count
    }
    pub fn touch_digest(&self) -> u64 {
        self.touch_digest
    }
    pub fn counters(&self) -> WorthUiRealtimeLaneCounters {
        self.counters
    }
    pub fn certification(&self) -> WorthUiRealtimeCertification {
        self.certification
    }
    pub fn resolution_evidence(&self) -> WorthUiHandleResolutionEvidence {
        self.resolution_evidence
    }
    pub fn work_scope(&self) -> WorthUiFrameWorkScope {
        self.work_scope
    }
}
